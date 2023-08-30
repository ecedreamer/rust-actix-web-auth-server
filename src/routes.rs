use super::models::{Client, Credentail};

use actix_web::{get, post, web, Responder, Result};
use log::warn;
use serde_json::json;
use std::collections::HashMap;

#[get("/")]
async fn index() -> Result<impl Responder> {
    warn!("Checking health of the system!");
    let health = HashMap::from([("status", "ok")]);
    Ok(web::Json(health))
}

#[post("/client-registration")]
pub async fn client_registration(client_info: web::Json<Client>) -> Result<impl Responder> {
    Ok(web::Json(
        json!({"success": false, "message": "Login Failed."}),
    ))
}

#[post("/login")]
pub async fn login(credential: web::Json<Credentail>) -> Result<impl Responder> {
    Ok(web::Json(
        json!({"success": false, "message": "Login Failed."}),
    ))
}

#[post("/token")]
pub async fn get_access_token() -> Result<impl Responder> {
    Ok(web::Json(
        json!({"success": false, "message": "Login Failed."}),
    ))
}

#[post("/verify-token")]
pub async fn verify_token() -> Result<impl Responder> {
    Ok(web::Json(
        json!({"success": false, "message": "Login Failed."}),
    ))
}
