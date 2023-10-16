//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-sqlite
//! ```

use axum::error_handling::HandleErrorLayer;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::post;
use axum::{routing::get, Router};
use axum::{Extension, Form};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
};

use rand::Rng;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = SqlitePoolOptions::new()
        .connect("sqlite/users.db")
        .await
        .unwrap();

    let user_store = SqliteStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let state = AppState { pool };

    let default_path_service =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/NotFound.html"));
    let app = Router::new()
        .route("/protected", get(protected_handler))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .route("/login", post(login_handler))
        .route("/logout", get(logout_handler))
        .nest_service("/", default_path_service)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(state));

    let addr = "0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login_handler(
    mut auth: AuthContext,
    Extension(state): Extension<AppState>,
    Form(login): Form<Login>,
) -> impl IntoResponse {
    let user: User = sqlx::query_as("select * from users where email = ? and password_hash=?")
        .bind(&login.email)
        .bind(&login.password)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    auth.login(&user).await.unwrap();

    Redirect::to("/protected")
}

async fn logout_handler(mut auth: AuthContext) -> impl IntoResponse {
    match &auth.current_user {
        Some(current_user) => {
            println!("Logging out user: {:?}", current_user);

            auth.logout().await;
        }
        None => {
            println!("user not logged in");
        }
    }

    Redirect::to("/")
}

async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.email);
    let output = format!(
        "<h1>Hello, {}</h1><br /><a href='/logout'>logout</a>",
        user.email
    );
    Html(output)
}

#[derive(Clone, Debug)]
struct AppState {
    pool: Pool<Sqlite>,
}

#[derive(Deserialize, Debug)]
struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Default, Clone, sqlx::FromRow)]
struct User {
    id: i64,
    password_hash: String,
    email: String,
}

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;
