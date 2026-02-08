pub struct WebAdapter {
    api: libfj::robocraft::FactoryAPI,
}

// TODO do some real authentication
struct FederatedFactoryAuthProvider {
    target_domain: String,
    my_auth_url: String,
}

impl libfj::robocraft::ITokenProvider for FederatedFactoryAuthProvider {
    fn token(&self) -> Result<String, ()> {
        Ok(format!("{},{}", self.target_domain, self.my_auth_url))
    }

    fn scheme(&self) -> &'static str {
        "OJWeb"
    }
}

impl WebAdapter {
    pub async fn init(url: &str, auth_url: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Ok(Self {
            api: libfj::robocraft::FactoryAPI::with_auth(FederatedFactoryAuthProvider {
                    target_domain: url.to_owned(),
                    my_auth_url: auth_url.to_owned(),
                })
                .with_domain(url)?,
        })
    }
}

#[async_trait::async_trait]
impl crate::VehicleFactoryAdapter for WebAdapter {
    async fn vehicle(&self, id: i32) -> Result<Option<(crate::VehicleInfo, crate::VehicleQueryInfo)>, Box<dyn std::error::Error>> {
        use base64::Engine;
        let id_usize: usize = id.try_into()?;
        let response = self.api.get(id_usize).await?;
        let response_data = response.response;
        Ok(Some((
            crate::VehicleInfo {
                id,
                cube_data: base64::prelude::BASE64_STANDARD.decode(response_data.cube_data.as_bytes()).unwrap_or_default(),
                colour_data: base64::prelude::BASE64_STANDARD.decode(response_data.colour_data.as_bytes()).unwrap_or_default(),
            },
            crate::VehicleQueryInfo {
                id: response_data.item_id as _,
                name: response_data.item_name,
                description: response_data.item_description,
                thumbnail: response_data.thumbnail,
                added_by: response_data.added_by,
                added_by_display_name: response_data.added_by_display_name,
                added_date: crate::traits::parse_rc_date(&response_data.added_date).unwrap_or_default(),
                expiry_date: crate::traits::parse_rc_date(&response_data.expiry_date).unwrap_or_default(),
                cpu: response_data.cpu as _,
                total_robot_ranking: response_data.total_robot_ranking as _,
                rent_count: response_data.rent_count as _,
                buy_count: response_data.buy_count as _,
                buyable: response_data.buyable,
                removed_date: response_data.removed_date.and_then(|x| crate::traits::parse_rc_date(&x).ok()),
                ban_date: response_data.ban_date.and_then(|x| crate::traits::parse_rc_date(&x).ok()),
                featured: response_data.featured,
                banner_message: response_data.banner_message,
                combat_rating: response_data.combat_rating as _,
                cosmetic_rating: response_data.cosmetic_rating as _,
                cube_amounts: serde_json::from_str(&response_data.cube_amounts).unwrap_or_default(),
            }
        )))
    }

    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<crate::VehicleQueryInfo>, Box<dyn std::error::Error>> {
        let response = if query.default_page {
            self.api.list().await?
        } else {
            self.api.list_builder()
                .page(query.page as _)
                .items_per_page(query.page_size as _)
                .order(query.order)
                .movement_raw(concat_u32_enums_to_str(&query.movement_filter))
                .weapon_raw(concat_u32_enums_to_str(&query.weapon_filter))
                .min_cpu(query.minimum_cpu as _)
                .max_cpu(query.maximum_cpu as _)
                .text(query.text_filter)
                .text_search_type(if query.player_filter { libfj::robocraft::FactoryTextSearchField::Player } else { query.text_search_field })
                .buyable(query.buyable)
                .prepend_featured(query.prepend_featured_robot)
                .default_page(query.default_page)
                .send().await?
        };
        Ok(response.response.roboshop_items.into_iter().map(|response_data| crate::VehicleQueryInfo {
            id: response_data.item_id as _,
            name: response_data.item_name,
            description: response_data.item_description,
            thumbnail: response_data.thumbnail,
            added_by: response_data.added_by,
            added_by_display_name: response_data.added_by_display_name,
            added_date: crate::traits::parse_rc_date(&response_data.added_date).unwrap_or_default(),
            expiry_date: crate::traits::parse_rc_date(&response_data.expiry_date).unwrap_or_default(),
            cpu: response_data.cpu as _,
            total_robot_ranking: response_data.total_robot_ranking as _,
            rent_count: response_data.rent_count as _,
            buy_count: response_data.buy_count as _,
            buyable: response_data.buyable,
            removed_date: response_data.removed_date.and_then(|x| crate::traits::parse_rc_date(&x).ok()),
            ban_date: response_data.ban_date.and_then(|x| crate::traits::parse_rc_date(&x).ok()),
            featured: response_data.featured,
            banner_message: response_data.banner_message,
            combat_rating: response_data.combat_rating as _,
            cosmetic_rating: response_data.cosmetic_rating as _,
            cube_amounts: serde_json::from_str(&response_data.cube_amounts).unwrap_or_default(),
        }).collect())
    }

    async fn upload(&self, _vehicle: crate::VehicleUploadInfo) -> Result<crate::VehicleThumbnailInfo, Box<dyn std::error::Error>>{
        Err("Uploading is not supported in the web adapter".into())
    }

    async fn rate_vehicle(&self, _id: i32, _combat: i32, _cosmetic: i32) -> Result<(), Box<dyn std::error::Error>> {
        Err("Rating is not supported in the web adapter".into())
    }

    /// Just update any purchase trackers
    async fn purchase(&self, _id: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn update_vehicle(&self, _id: i32, _cube_data: Option<Vec<u8>>, _colour_data: Option<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        Err("Updating vehicles is not supported in the web adapter".into())
    }

    async fn remove_vehicle(&self, _id: i32, _user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        Err("Removing vehicles is not supported in the web adapter".into())
    }

    async fn set_featured(&self, _id: i32, _is_featured: bool) -> Result<(), Box<dyn std::error::Error>> {
        Err("Featuring vehicles is not supported in the web adapter".into())
    }
}

pub fn concat_u32_enums_to_str(enums: &[u32]) -> String {
    let mut out = String::new();
    for num in enums.iter() {
        if out.is_empty() {
            out += &num.to_string();
        } else {
            out += ",";
            out += &num.to_string();
        }
    }
    out
}
