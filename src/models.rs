use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Credentail {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientRegister {
    pub client_name: String,
    pub client_email: String,
    pub client_website: String,
    pub uuid: Option<String>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: String,
    pub client_name: String,
    pub client_url: String, 
    pub client_callback_url: String
}