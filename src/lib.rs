use worker::*;


#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {

    let router = Router::new();
    router
        .get_async("/", |_req, _ctx| async move {
            Response::ok(INDEX_HTML)
        })
        .get_async("/ip", |req, _ctx| async move {
            client_ip(&req).await
        })
        .get_async("/user_agent", |req, _ctx| async move {
            user_agent(&req).await
        })
        .get_async("/country", |req, _ctx| async move {
            country(req.cf()).await
        })
        .get_async("/http_protocol", |req, _ctx| async move {
            http_protocol(req.cf()).await
        })
        .get_async("/tls_cipher", |req, _ctx| async move {
            tls_cipher(req.cf()).await
        })
        .get_async("/coordinates", |req, _ctx| async move {
            coordinates(req.cf()).await
        })
        .run(req, env)
        .await
}

async fn client_ip(req: &Request) -> Result<Response> {
    match req.headers().get("cf-connecting-ip").unwrap() {
        Some(x) => Response::ok(x),
        None => Response::error(ERROR_MESSAGE_APPLICATION_ERROR, 418)
    }
}

async fn user_agent(req: &Request) -> Result<Response> {
    match req.headers().get("user-agent").unwrap() {
        Some(x) => Response::ok(x),
        None => Response::error(ERROR_MESSAGE_APPLICATION_ERROR, 418)
    }
}

async fn country(cf: &Cf) -> Result<Response> {
    match cf.country() {
        Some(x) => Response::ok(x),
        None => Response::error(ERROR_MESSAGE_APPLICATION_ERROR, 418)
    }
}

async fn http_protocol(cf: &Cf) -> Result<Response> {
    Response::ok(cf.http_protocol())
}

async fn tls_cipher(cf: &Cf) -> Result<Response> {
    Response::ok(cf.tls_cipher())
}

async fn coordinates(cf: &Cf) -> Result<Response> {
    match cf.coordinates() {
        Some(x) => Response::ok(format!("lat: {}\nlong: {}", x.0, x.1)),
        None => Response::error(ERROR_MESSAGE_APPLICATION_ERROR, 418)
    }
}

// Error messages
const ERROR_MESSAGE_APPLICATION_ERROR: &str = "An application error occurred";

// - HTML-Code
const INDEX_HTML: &str = include_str!("assets/html/index.html");