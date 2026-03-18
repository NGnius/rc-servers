use actix_web::{rt, web::{Payload, Data, Json, Path}, Error, HttpRequest, HttpResponse, get, post};
//use actix_ws::AggregatedMessage;
//use futures::StreamExt as _;

#[get("/intercom/userless/.oj_lobby")]
pub async fn lobby_state_ws(req: HttpRequest, stream: Payload, auth: Data<super::IntercomAuth>, reg: Data<super::Users>) -> Result<HttpResponse, Error> {
    auth.validate(&req, "state/.oj_lobby")?;
    let (res, mut session, _stream) = actix_ws::handle(&req, stream)?;

    /*let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20)); // aggregate continuation frames up to 1MiB
        */

    let (tx, mut rx) = tokio::sync::mpsc::channel(16);
    reg.register_lobby_state_service(tx).await;
    log::debug!("Registered lobby state intercom websocket");

    // start task but don't wait for it
    rt::spawn(async move {
        let mut is_ok = false;
        while let Some(op) = rx.recv().await {
            match op {
                super::IntercomOp::Message(msg) => {
                    if let Err(e) = session.text(serde_json::to_string(&msg).unwrap()).await {
                        log::warn!("Failed to send lobby state intercom message: {}", e);
                        break;
                    }
                },
                super::IntercomOp::Info(info) => {
                    match info {
                        super::IntercomInfo::Close => {
                            is_ok = true;
                            break;
                        },
                    }
                }
            }

        }
        if !is_ok {
            reg.remove_lobby_state_service().await;
        }
        rx.close();
        session.close(Some(actix_ws::CloseReason {
            code: actix_ws::CloseCode::Normal,
            description: Some("End of channel".to_owned()),
        })).await.expect("Failed to close a services intercom websocket session");
        log::debug!("Lobby state intercom websocket closed");
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

#[post("/intercom/.oj_lobby/{name}/state")]
pub async fn lobby_state_msg(req: HttpRequest, body: Json<oj_rc_core::persist::user::intercom::IntercomLobbyStateMessage>, auth: Data<super::IntercomAuth>, reg: Data<super::Users>, name: Path<String>) -> Result<HttpResponse, super::IntercomOpError> {
    log::debug!("Got lobby state intercom message");
    auth.validate(&req, &format!(".oj_lobby/{}/state", name))?;
    log::debug!("Authenticated intercom lobby state message from {}", name);
    reg.broadcast_lobby_message(body.0).await;
    Ok(HttpResponse::NoContent().finish())
}
