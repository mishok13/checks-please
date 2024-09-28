use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header, request::Parts, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use minijinja::{context, path_loader, Environment};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState<'a> {
    pub templates: Environment<'a>,
}

impl AppState<'_> {
    fn new() -> Self {
        let mut templates = Environment::new();
        templates.set_loader(path_loader("src/templates"));
        Self{templates}
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let app = Router::new()
        .route("/", get(index))
        .route("/groups", post(groups_create).get(groups_list))
        // .route("/login", post(login))
        // .route("/logout", post(logout))
        // .route("/expenses", post(expenses_add).get(expenses_list))
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

struct LoggedOutRedirect;

impl IntoResponse for LoggedOutRedirect {
    fn into_response(self) -> axum::response::Response {
        Redirect::temporary("/").into_response()
    }
}

#[derive(Debug)]
struct User {
    name: String,
}

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
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

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = LoggedOutRedirect;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get(header::AUTHORIZATION)
            .map(|val| {
                tracing::info!("Authenticated user {:?}", val);
                User {name: val.to_str().unwrap().to_owned()}
            })
            .ok_or(LoggedOutRedirect)
    }
}

async fn index<'a>(State(state): State<AppState<'a>>) -> Result<Html<String>, AppError> {
    let template = state.templates.get_template("index.html")?;
    Ok(template.render(context! {username => "Bob"}).map(|s| s.into())?)
}

async fn login() -> impl IntoResponse {}
async fn logout() -> impl IntoResponse {}

async fn groups_create(user: User) -> Result<Html<String>, AppError> {
    tracing::debug!("Creating new group for {:?}", user);
    let message = format!("Thanks, {}", user.name);
    Ok(Html::from(message.to_owned()))
}
async fn groups_list(user: User, State(state): State<AppState<'_>>) -> Result<Html<String>, AppError> {
    let template = state.templates.get_template("groups.html")?;
    Ok(template.render(context! {username => user.name}).map(|s| s.into())?)

}
async fn expenses_add() -> impl IntoResponse {}
async fn expenses_list(user: User) -> impl IntoResponse {}
async fn expenses_delete() -> impl IntoResponse {}
