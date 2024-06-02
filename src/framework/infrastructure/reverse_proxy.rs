use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::{Request, State},
    response::Response,
    routing::{get, MethodRouter},
    Router,
};
use hyper::{StatusCode, Uri};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

async fn reverse_proxy_handler(
    State(state): State<(String, String)>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path)
        .trim_end_matches('/');

    let uri = format!("http://{}/x/{}{}", state.1, state.0, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_response())
}

fn reverse_proxy_method_router(extension_number: String, ip_port: String) -> MethodRouter {
    let state = (extension_number, ip_port);
    get(reverse_proxy_handler)
        .post(reverse_proxy_handler)
        .put(reverse_proxy_handler)
        .patch(reverse_proxy_handler)
        .delete(reverse_proxy_handler)
        .head(reverse_proxy_handler)
        .options(reverse_proxy_handler)
        .trace(reverse_proxy_handler)
        .with_state(state)
}

pub fn reverse_proxy_router(extension_number: String, ip_port: String) -> Router {
    Router::new()
        .route(
            "/",
            reverse_proxy_method_router(extension_number.clone(), ip_port.clone()),
        )
        .route(
            "/*path",
            reverse_proxy_method_router(extension_number.clone(), ip_port.clone()),
        )
}
