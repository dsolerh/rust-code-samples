use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result, api::AUTH_TOKEN};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

// NOTE: Json payload extracted from the body of the request needs to be the last argument
async fn login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFailed);
    }

    // FIXME: Implement real auth token generation/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
