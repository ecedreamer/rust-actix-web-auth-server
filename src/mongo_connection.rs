use mongodb::{Client, Database};

pub async fn establish_connection() -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("rs_oauth_db");
    Ok(database)
}