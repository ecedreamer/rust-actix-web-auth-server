use crate::models::{Client, ClientRegister, Credentail};
use crate::utils::{authenticate};
use mongodb::{Database, bson::{self, Document}, Collection, Cursor};
use crate::mongo_connection;
use tera::{Tera, Context};
use uuid::Uuid;
use futures::stream::TryStreamExt;

use actix_files::NamedFile;
use std::path::Path;

use actix_web::{get, post, web, Responder, Result, HttpRequest, HttpResponse};
use log::warn;
use serde_json::json;
use std::collections::HashMap;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::options::Collation;


#[get("/")]
async fn index(tera: web::Data<Tera>) -> HttpResponse {
    warn!("Checking health of the system!");
    let mut context = Context::new();

    let db = mongo_connection::establish_connection().await.unwrap();
    let user_collection: Collection<ClientRegister> = db.collection("users");

    let mut user_cursor = user_collection.find(None, None).await?;
    while let Ok(Some(user)) = user_cursor.expect("hello").try_next().await {
        warn!("{:?}", user);
    }
    context.insert("client_list", "users");
    let rendered = tera.render("index.html", &context).unwrap(); // Handle error appropriately

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/client-registration")]
pub async fn page_client_registration(req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("src/templates/client_registration.html"))?)
}

#[post("/client-registration")]
pub async fn handle_client_registration(tera: web::Data<Tera>, mut form_data: web::Form<ClientRegister>) -> HttpResponse {
    warn!("name: {}, email: {}", form_data.client_name, form_data.client_email);
    let db = mongo_connection::establish_connection().await.unwrap();
    let collection = db.collection("users");
    let uuid = Uuid::new_v4();
    form_data.uuid = Some(uuid.to_string());
    let bson_document = bson::to_document(&form_data.into_inner()).unwrap();
    let inserted_id = collection.insert_one(bson_document, None).await;
    let inserted_user = collection.find_one(doc! { "_id": inserted_id.unwrap().inserted_id }, None).await.unwrap();
    let mut context = Context::new();
    context.insert("client_data", &inserted_user);

    // Render the template with the context
    let rendered = tera.render("client_registration_successful.html", &context).unwrap(); // Handle error appropriately

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/login")]
pub async fn login(credential: web::Json<Credentail>) -> Result<impl Responder> {
    if authenticate(credential.username.as_str(), credential.password.as_str()) {
        Ok(web::Json(
            json!({"success": true, "message": "Login successful."})
        ))
    } else {
        Ok(web::Json(
            json!({"success": false, "message": "Login Failed."})
        ))
    }
}


#[get("/login")]
pub async fn login_form(req: HttpRequest) -> Result<NamedFile> {
    println!("{:?}", req.headers());
    Ok(NamedFile::open(Path::new("templates/login.html"))?)
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
