use actix_web::{rt, web::{Payload, Data, Path, Json}, Error, HttpRequest, HttpResponse, get, post};
//use actix_ws::AggregatedMessage;
//use futures::StreamExt as _;

#[get("/intercom/.oj_services/{name}")]
pub async fn services_ws(req: HttpRequest, stream: Payload, auth: Data<super::IntercomAuth>, reg: Data<super::Users>, name: Path<String>) -> Result<HttpResponse, Error> {
    auth.validate(&req, &format!(".oj_services/{}", urlencoding::encode(&*name)))?;
    let (res, mut session, _stream) = actix_ws::handle(&req, stream)?;

    /*let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20)); // aggregate continuation frames up to 1MiB
        */

    let (tx, mut rx) = tokio::sync::mpsc::channel(16);
    reg.register_web_service(name.clone(), tx).await;
    log::debug!("Registered web services intercom websocket for user {}", name);

    // start task but don't wait for it
    rt::spawn(async move {
        let mut is_ok = false;
        while let Some(op) = rx.recv().await {
            match op {
                super::IntercomOp::Message(msg) => {
                    if let Err(e) = session.text(serde_json::to_string(&msg).unwrap()).await {
                        log::warn!("Failed to send services intercom to user {}: {}", name, e);
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
            reg.remove_web_service(name.clone()).await;
        }
        rx.close();
        session.close(Some(actix_ws::CloseReason {
            code: actix_ws::CloseCode::Normal,
            description: Some("End of channel".to_owned()),
        })).await.expect("Failed to close a services intercom websocket session");
        log::debug!("Web services intercom websocket closed for {}", name);
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

#[post("/intercom/.oj_services/{name}/messages")]
pub async fn service_msg(req: HttpRequest, body: Json<oj_rc_core::persist::user::intercom::IntercomWebServiceMessage>, auth: Data<super::IntercomAuth>, reg: Data<super::Users>, name: Path<String>) -> Result<HttpResponse, super::IntercomOpError> {
    log::debug!("Got intercom message from {} to {:?}", name, body.public_ids.as_slice());
    auth.validate(&req, &format!(".oj_services/{}/messages", name))?;
    log::debug!("Authenticated intercom message from {} to {:?}", name, body.public_ids.as_slice());
    reg.broadcast_web_service_message(body.0).await;
    Ok(HttpResponse::NoContent().finish())
}
