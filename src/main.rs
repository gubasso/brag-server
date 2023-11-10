use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use listenfd::ListenFd;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .nest("/error", api_error());
    let mut listenfd = ListenFd::from_env();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => Server::from_tcp(listener).unwrap(),
        None => Server::bind(&addr),
    };
    println!("listening on {}", addr);
    server.serve(app.into_make_service()).await.unwrap();
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Helloooooww, woooorrrlldd!!</h1>")
}

fn api_error() -> Router {
    Router::new().route("/", get(handle_error))
}

async fn handle_error() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}
