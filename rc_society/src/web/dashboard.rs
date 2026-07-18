use actix_web::{get, post, web::Data, Responder, HttpRequest};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};

const FORM_NAME: &str = "dashboard";

#[derive(Serialize, Deserialize)]
struct RenderData {
    display_name: String,
    public_id: String,
    debug: DebugData,
    perms: PermissionData,
    garage: GarageData,
    factory: FactoryData,
    account: AccountData,
    sanction: SanctionData,
    social: SocialData,
    fediverse: FederationData,
}

#[derive(Serialize, Deserialize)]
struct DebugData {
    user_id: i32,
    creation_time_unix: i64,
    creation_time_iso: String,
    cdn: String,
    factory: String,
}

#[derive(Serialize, Deserialize)]
struct PermissionData {
    r#mod: bool,
    admin: bool,
    dev: bool,
    royal: bool,
    banned: bool,
}

#[derive(Serialize, Deserialize)]
struct GarageData {
    total: u64,
    regular_total: u64,
    mega_total: u64,
    empty_total: u64,
    bytes_total: Option<u64>, // not always supported
    selected_slot: i32,
}

#[derive(Serialize, Deserialize)]
struct FactoryData {
    garages_from_factory: u64,
    uploads_total: usize,
}

#[derive(Serialize, Deserialize)]
struct AccountData {
    currencies: std::collections::HashMap<String, u64>,
    games: u64,
    avatar_id: String,
    is_custom_avatar: bool,
    premium_until_unix: i64,
    premium_until_iso: String,
    rank: u32,
}

#[derive(Serialize, Deserialize)]
struct SanctionData {
    total: u64,
    pending: u64,
    warns: u64,
    mutes: u64,
    muted_until_unix: Option<i64>,
    muted_until_iso: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ClanData {
    name: String,
    description: String,
    ty: String,
}

#[derive(Serialize, Deserialize)]
struct SocialData {
    clan: Option<ClanData>,
    friends: u64,
    friends_of: u64,
    chats: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct FederationData {
    enabled: bool,
    defederated: Vec<String>,
}

pub async fn dashboard_impl(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, factory: Data<oj_rc_core::factory::Factory>, server_config: Data<oj_rc_core::persist::config::ServerConfig>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    match super::try_auth_user(user_opt, auth.as_ref(), &req).await? {
        super::LoginReturn::AuthFail(resp) => Ok(resp),
        super::LoginReturn::Success(user) => {
            log::debug!("Rendering user's dashboard");
            let creation_time = user.creation();
            let creation_time_chrono = chrono::DateTime::<chrono::Utc>::from_timestamp_secs(creation_time).unwrap_or_default();
            let (garage_stats, factory_stats) = match build_garage_data(user.as_ref(), factory.as_ref()).await {
                Ok(x) => x,
                Err(e) => return Ok(fallback_render(e, user.as_ref(), handlebars_ref.as_ref(), &req)),
            };
            let account_stats = match build_account_data(user.as_ref()).await {
                Ok(x) => x,
                Err(e) => return Ok(fallback_render(e, user.as_ref(), handlebars_ref.as_ref(), &req)),
            };
            let sanction_stats = match build_sanction_data(user.as_ref()).await {
                Ok(x) => x,
                Err(e) => return Ok(fallback_render(e, user.as_ref(), handlebars_ref.as_ref(), &req)),
            };
            let social_stats = match build_social_data(user.as_ref()).await {
                Ok(x) => x,
                Err(e) => return Ok(fallback_render(e, user.as_ref(), handlebars_ref.as_ref(), &req)),
            };
            let fedi_stats = match build_fedi_data(user.as_ref()).await {
                Ok(x) => x,
                Err(e) => return Ok(fallback_render(e, user.as_ref(), handlebars_ref.as_ref(), &req)),
            };
            Ok(super::render_ok(
                RenderData {
                    display_name: user.display_name().to_owned(),
                    public_id: user.public_id().to_owned(),
                    debug: DebugData {
                        user_id: user.account_id(),
                        creation_time_unix: creation_time,
                        creation_time_iso: creation_time_chrono.to_rfc3339(),
                        cdn: server_config.cdn_url.clone(),
                        factory: server_config.factory_url.clone(),
                    },
                    perms: build_perm_data(user.as_ref()),
                    garage: garage_stats,
                    factory: factory_stats,
                    account: account_stats,
                    sanction: sanction_stats,
                    social: social_stats,
                    fediverse: fedi_stats,
                },
                handlebars_ref.as_ref(),
                FORM_NAME,
            )
                .respond_to(&req)
                .map_into_boxed_body()
            )
        }
    }
}

fn fallback_render(error: Box<dyn std::error::Error>, user: &dyn oj_rc_core::persist::user::WebUser, handlebars_ref: &handlebars::Handlebars<'_>, req: &HttpRequest) -> actix_web::HttpResponse {
    let creation_time = user.creation();
    let creation_time_chrono = chrono::DateTime::<chrono::Utc>::from_timestamp_secs(creation_time).unwrap_or_default();
    super::render_err(
        RenderData {
            display_name: user.display_name().to_owned(),
            public_id: user.public_id().to_owned(),
            debug: DebugData {
                user_id: user.account_id(),
                creation_time_unix: creation_time,
                creation_time_iso: creation_time_chrono.to_rfc3339(),
                cdn: String::default(),
                factory: String::default(),
            },
            perms: PermissionData {
                r#mod: false,
                admin: false,
                dev: false,
                royal: false,
                banned: false,
            },
            garage: GarageData {
                total: 0,
                regular_total: 0,
                mega_total: 0,
                empty_total: 0,
                bytes_total: None,
                selected_slot: 0,
            },
            factory: FactoryData {
                garages_from_factory: 0,
                uploads_total: 0,
            },
            account: AccountData {
                currencies: std::collections::HashMap::default(),
                games: 0,
                avatar_id: "???".to_owned(),
                is_custom_avatar: false,
                premium_until_unix: 0,
                premium_until_iso: String::default(),
                rank: 0,
            },
            sanction: SanctionData {
                total: 0,
                pending: 0,
                warns: 0,
                mutes: 0,
                muted_until_unix: None,
                muted_until_iso: None,
            },
            social: SocialData {
                clan: None,
                friends: 0,
                friends_of: 0,
                chats: Vec::default(),
            },
            fediverse: FederationData {
                enabled: false,
                defederated: Vec::default(),
            }
        },
        format!("Dashboard loading failed: {}", error),
        handlebars_ref,
        FORM_NAME,
    )
    .respond_to(req)
    .map_into_boxed_body()
}

fn build_perm_data(user: &dyn oj_rc_core::persist::user::WebUser) -> PermissionData {
    PermissionData {
        r#mod: user.is_mod(),
        admin: user.is_admin(),
        dev: user.is_dev(),
        royal: user.is_royal(),
        banned: user.is_banned(),
    }
}

async fn build_garage_data(user: &dyn oj_rc_core::persist::user::WebUser, factory: &dyn oj_rc_factory::VehicleFactoryAdapter) -> Result<(GarageData, FactoryData), Box<dyn std::error::Error>> {
    let stats = user.garage_stats().await?;
    let uploads_count = factory.count(libfj::robocraft::ListQuery {
        page: 0,
        page_size: 1_000,
        order: libfj::robocraft::FactoryOrderType::Added,
        player_filter: true,
        movement_filter: Vec::default(),
        movement_category_filter: Vec::default(),
        weapon_filter: Vec::default(),
        weapon_category_filter: Vec::default(),
        minimum_cpu: 0,
        maximum_cpu: i32::MAX as _,
        text_filter: user.public_id().to_owned(),
        text_search_field: libfj::robocraft::FactoryTextSearchField::Player,
        buyable: false,
        prepend_featured_robot: false,
        featured_only: false,
        default_page: false,
    }).await?;
    Ok((GarageData {
        total: stats.vehicle_total,
        regular_total: stats.regular_vehicle_total,
        mega_total: stats.mega_vehicle_total,
        empty_total: stats.empty_vehicle_total,
        bytes_total: stats.storage_bytes_total,
        selected_slot: stats.selected_garage,
    },
    FactoryData {
        garages_from_factory: stats.factory_vehicle_total,
        uploads_total: uploads_count,
    }))
}

async fn build_account_data(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<AccountData, Box<dyn std::error::Error>> {
    let stats = user.account_stats().await?;
    Ok(AccountData {
        currencies: stats.currencies.into_iter()
            .map(|(key, val)| (currency_to_str(key).to_owned(), val))
            .collect(),
        games: stats.games_played,
        avatar_id: stats.avatar_id.map(|id| format!("#{}", id)).unwrap_or_else(|| "Custom".to_owned()),
        is_custom_avatar: stats.avatar_id.is_none(),
        premium_until_unix: stats.premium_until,
        premium_until_iso: chrono::DateTime::<chrono::Utc>::from_timestamp_secs(stats.premium_until)
                .unwrap_or_default()
                .to_rfc3339(),
        rank: stats.rank,
    })
}

fn currency_to_str(currency: oj_rc_core::persist::user::CurrencyType) -> &'static str {
    match currency {
        oj_rc_core::persist::user::CurrencyType::Free => "robits",
        oj_rc_core::persist::user::CurrencyType::Paid => "gc",
        oj_rc_core::persist::user::CurrencyType::TechPoints => "tp",
        oj_rc_core::persist::user::CurrencyType::Experience => "xp"
    }
}

async fn build_sanction_data(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<SanctionData, Box<dyn std::error::Error>> {
    let stats = user.sanction_stats().await?;
    Ok(SanctionData {
        total: stats.total,
        pending: stats.pending_total,
        warns: stats.warn_total,
        mutes: stats.mute_total,
        muted_until_unix: stats.muted_until,
        muted_until_iso: stats.muted_until
            .map(|t| chrono::DateTime::<chrono::Utc>::from_timestamp_secs(t)
                .unwrap_or_default()
                .to_rfc3339()
            ),
    })
}

async fn build_social_data(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<SocialData, Box<dyn std::error::Error>> {
    let stats = user.social_stats().await?;
    Ok(SocialData {
        clan: stats.clan.map(|clan| ClanData {
            name: clan.name,
            description: clan.description,
            ty: format!("{:?}", clan.ty),
        }),
        friends: stats.friends_total,
        friends_of: stats.friends_of_total,
        chats: stats.chats,
    })
}

async fn build_fedi_data(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<FederationData, Box<dyn std::error::Error>> {
    let info = user.fedi_get().await;
    Ok(FederationData {
        enabled: info.enabled,
       defederated: info.defederated,
    })
}

#[get("/dashboard")]
pub async fn get(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, factory: Data<oj_rc_core::factory::Factory>, server_config: Data<oj_rc_core::persist::config::ServerConfig>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    dashboard_impl(handlebars_ref, auth, factory, server_config, user_opt, req).await
}

#[post("/dashboard")]
pub async fn post(handlebars_ref: Data<handlebars::Handlebars<'_>>, auth: Data<Box<oj_rc_core::UserImpl>>, factory: Data<oj_rc_core::factory::Factory>, server_config: Data<oj_rc_core::persist::config::ServerConfig>, user_opt: Option<Identity>, req: HttpRequest) -> Result<impl Responder, actix_web::error::Error> {
    dashboard_impl(handlebars_ref, auth, factory, server_config, user_opt, req).await
}
