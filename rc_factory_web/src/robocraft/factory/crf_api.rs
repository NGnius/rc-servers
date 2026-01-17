use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse};
use base64::Engine;

use libfj::robocraft::{
    FactoryInfo, RoboShopItemsInfo, FactoryRobotGetInfo, FactoryRobotListInfo,
    ListPayload, ListQuery, FactoryOrderType, FactoryTextSearchField,
};
use oj_rc_factory::{VehicleFactoryAdapter, VehicleInfo, VehicleQueryInfo};

fn parse_filter(filter: &str) -> Vec<u32> {
    if filter.trim().is_empty() {
        return Vec::new();
    }
    filter.split(',')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect()
}

fn payload_to_query(payload: &ListPayload) -> ListQuery {
    ListQuery {
        page: (payload.page.max(1)) as usize,
        page_size: (payload.page_size.clamp(1, 100)) as usize,
        order: FactoryOrderType::try_from(payload.order.clamp(0, u8::MAX.into()) as u8).unwrap_or(FactoryOrderType::Suggested),
        player_filter: payload.player_filter,
        movement_filter: parse_filter(&payload.movement_filter),
        movement_category_filter: parse_filter(&payload.movement_category_filter),
        weapon_filter: parse_filter(&payload.weapon_filter),
        weapon_category_filter: parse_filter(&payload.weapon_category_filter),
        minimum_cpu: if payload.minimum_cpu <= 0 { 0 } else { payload.minimum_cpu as usize },
        maximum_cpu: if payload.maximum_cpu <= 0 { usize::MAX } else { payload.maximum_cpu as usize },
        text_filter: payload.text_filter.clone(),
        text_search_field: FactoryTextSearchField::try_from(payload.text_search_field.clamp(0, u8::MAX.into()) as u8).unwrap_or(FactoryTextSearchField::All),
        buyable: payload.buyable,
        prepend_featured_robot: payload.prepend_featured_robot,
        featured_only: payload.featured_only,
        default_page: payload.default_page,
    }
}

fn list_info(vi: VehicleQueryInfo) -> FactoryRobotListInfo {
    FactoryRobotListInfo {
        item_id: vi.id as usize,
        item_name: vi.name,
        item_description: vi.description,
        thumbnail: vi.thumbnail,
        added_by: vi.added_by,
        added_by_display_name: vi.added_by_display_name,
        added_date: vi.added_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
        expiry_date: vi.expiry_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
        cpu: vi.cpu as usize,
        total_robot_ranking: vi.total_robot_ranking as usize,
        rent_count: vi.rent_count as usize,
        buy_count: vi.buy_count as usize,
        buyable: vi.buyable,
        removed_date: vi.removed_date.as_ref().map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        ban_date: vi.ban_date.as_ref().map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        featured: vi.featured,
        banner_message: vi.banner_message,
        combat_rating: vi.combat_rating as f32,
        cosmetic_rating: vi.cosmetic_rating as f32,
        cube_amounts: serde_json::to_string(&vi.cube_amounts).unwrap_or_else(|_| "{}".to_string()),
    }
}

fn get_info(qi: VehicleQueryInfo, vi: VehicleInfo) -> FactoryRobotGetInfo {
    FactoryRobotGetInfo {
        item_id: qi.id as usize,
        item_name: qi.name,
        item_description: qi.description,
        thumbnail: qi.thumbnail,
        added_by: qi.added_by,
        added_by_display_name: qi.added_by_display_name,
        added_date: qi.added_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
        expiry_date: qi.expiry_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
        cpu: qi.cpu as usize,
        total_robot_ranking: qi.total_robot_ranking as usize,
        rent_count: qi.rent_count as usize,
        buy_count: qi.buy_count as usize,
        buyable: qi.buyable,
        removed_date: qi.removed_date.as_ref().map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        ban_date: qi.ban_date.as_ref().map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        featured: qi.featured,
        banner_message: qi.banner_message,
        combat_rating: qi.combat_rating as f32,
        cosmetic_rating: qi.cosmetic_rating as f32,
        cube_data: base64::prelude::BASE64_STANDARD.encode(&vi.cube_data),
        colour_data: base64::prelude::BASE64_STANDARD.encode(&vi.colour_data),
        cube_amounts: serde_json::to_string(&qi.cube_amounts).unwrap_or_else(|_| "{}".to_string()),
    }
}

#[post("/api/roboShopItems/list")]
pub async fn list(factory: Data<oj_rc_core::factory::Factory>, body: Json<ListPayload>) -> HttpResponse {
    if let oj_rc_core::factory::Factory::None = factory.get_ref() {
        return HttpResponse::ServiceUnavailable().body("factory adapter disabled");
    }

    let query = payload_to_query(&body.into_inner());

    match factory.list(query).await {
        Ok(items) => {
            let out: Vec<FactoryRobotListInfo> = items.into_iter().map(list_info).collect();
            HttpResponse::Ok().json(FactoryInfo {
                response: RoboShopItemsInfo { roboshop_items: out },
                status_code: 200,
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("factory list error: {e}")),
    }
}

#[get("/api/roboShopItems/list")]
pub async fn list_default(factory: Data<oj_rc_core::factory::Factory>) -> HttpResponse {
    if let oj_rc_core::factory::Factory::None = factory.get_ref() {
        return HttpResponse::ServiceUnavailable().body("factory adapter disabled");
    }

    let query = payload_to_query(&ListPayload::default());

    match factory.list(query).await {
        Ok(items) => {
            let out: Vec<FactoryRobotListInfo> = items.into_iter().map(list_info).collect();
            HttpResponse::Ok().json(FactoryInfo {
                response: RoboShopItemsInfo { roboshop_items: out },
                status_code: 200,
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("factory list error: {e}")),
    }
}

#[get("/api/roboShopItems/get/{id}")]
pub async fn get(factory: Data<oj_rc_core::factory::Factory>, id: Path<i32>) -> HttpResponse {
    if let oj_rc_core::factory::Factory::None = factory.get_ref() {
        return HttpResponse::ServiceUnavailable().body("factory adapter disabled");
    }
    match factory.vehicle(*id).await {
        Ok(Some((vehicle_info, query_info))) => HttpResponse::Ok().json(FactoryInfo {
            response: get_info(query_info, vehicle_info),
            status_code: 200,
        }),
        Ok(None) => HttpResponse::NotFound().body("robot not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("factory get error: {e}")),
    }
}
