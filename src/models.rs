use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Credentail {
    pub username: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: String,
    pub client_name: String,
    pub client_url: String, 
    pub client_callback_url: String
}