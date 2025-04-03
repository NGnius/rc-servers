use rocket::{get, routes, serde::json::Json, State};

#[get("/live/data.json")]
pub fn static_live_data(cli: &State<crate::cli::CliArgs>) -> Json<libfj::robocraft::StaticDataRaw> {
    Json(libfj::robocraft::StaticDataRaw {
        MaintenanceMode: "false".into(),
        MaintenanceRegex: "".into(),
        EacEnabled: "true".into(),
        MinimumVersion: "2855".into(),
        PhotonSocialServer: "rc-backend.servers.robocraftgame.com:4534".into(),
        PhotonServicesServer: "rc-backend.servers.robocraftgame.com:4532".into(),
        PhotonChatServer: "rc-backend.servers.robocraftgame.com:4530".into(),
        PhotonSinglePlayerServer: "rc-backend.servers.robocraftgame.com:4536".into(),
        GameplayServerServiceAddress: "rc-backend.servers.robocraftgame.com:4538".into(),
        PhotonLobbyServer: "rc-backend.servers.robocraftgame.com:4540".into(),
        ErrorLogAddress: "logs.freejamgames.com:4561".into(),
        ServerErrorLogAddress: "logs.freejamgames.com:4562".into(),
        authUrl: cli.auth.clone(), // originally "https://auth-backend.freejamgames.com/"
        paymentUrl: cli.pay.clone(),
        enterBattleLogGenerationTimeout: "60".into(),
        GameServerConnectionTestTimeout: 10,
        AvatarCdnUrl: "https://rc-cdn-images.robocraftgame.com/customavatar/Live/".into(),
        ClanAvatarCdnUrl: "https://rc-cdn-images.robocraftgame.com/clanavatar/Live/".into(),
        FeatureThrottlerOnPercent: "100".into(),
        EmailCaptureEnabled: "true".into(),
        UnreliableMessages: "true".into(),
        MessageQueueEnabled: "true".into(),
        BrawlDataUrl: "https://rc-cdn-images.robocraftgame.com/brawldata/Live/".into(),
        CampaignDataUrl: "https://rc-cdn-images.robocraftgame.com/campaigndata/Live/".into(),
        LeaderboardsUrl: "https://leaderboards.robocraftgame.com".into(),
        NetworkChannelTypes: "3113".into(),
        MaxSentMessageQueueSize: 64,
        IsAcksLong: 1,
        NetworkDropThreshold: 80,
        PacketSize: 1200,
        MaxPacketSize: 5888,
        MaxCombinedReliableMessageCount: 20,
        MaxCombinedReliableMessageSize: 200,
        MaxDelay: 1,
        OverflowThreshold: 10,
        MinUpdateTimeout: 1,
        DevMessageRefresh: 60,
        MaintenanceRefresh: 30,
        SaveRequestOnPhoton: "false".into(),
        UseS3System: "true".into(),
        authMigrationUrl: "http://88.150.159.132:3000/auth-migration".into(),
        xsollaEnabled: "true".into(),
        MaintenanceMessage: "Robocraft is currently undergoing server maintenance. ".into(),
        DevMessage: "NGnius says hello".into(),
        DevMessageDisplayTime: "10".into(),
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Robocraft live data.json", |rocket| async {
        rocket.mount("/", routes![static_live_data])
    })
}
