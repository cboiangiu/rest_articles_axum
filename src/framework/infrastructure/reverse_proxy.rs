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

async fn handle(
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

fn method_router(extension_number: String, ip_port: String) -> MethodRouter {
    let state = (extension_number, ip_port);
    get(handle)
        .post(handle)
        .put(handle)
        .patch(handle)
        .delete(handle)
        .head(handle)
        .options(handle)
        .trace(handle)
        .with_state(state)
}

pub fn reverse_proxy_router(extension_number: String, ip_port: String) -> Router {
    Router::new()
        .route(
            "/",
            method_router(extension_number.clone(), ip_port.clone()),
        )
        .route(
            "/*path",
            method_router(extension_number.clone(), ip_port.clone()),
        )
}
