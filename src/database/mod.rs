use std::env;
use dotenv::dotenv;

use mongodb::{
    bson::doc,
    options::IndexOptions,
    sync::{Client, Collection}, IndexModel,
};

use super::app::{
    user::{user_model as user},
    account::{
        account_model as account,
        transaction_model as transaction
    },
    
};




#[derive(Debug)]
pub struct Database {
    user_schema: Collection<user::User>,
    transaction_schema:Collection<transaction::Transaction>,
    account_schema:Collection<account::Account>
}
impl Database {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(database_url) => database_url.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");

        let user_schema: Collection<user::User> = db.collection("User");
        let email_index = IndexModel::builder()
        .keys(doc! {"email": 1})
        .options(IndexOptions::builder()
            .unique(true)
            .build())
        .build();
    let password_index = IndexModel::builder()
        .keys(doc! {"password": 1})
        .options(IndexOptions::builder()
            .hidden(true)
            .build())
        .build();

    let user_schema_index_models = vec![email_index,password_index];

    user_schema.create_indexes(user_schema_index_models,None);
        let account_schema: Collection<account::Account> = db.collection("Account");
        let transaction_schema: Collection<transaction::Transaction> = db.collection("Transaction");
        Database { user_schema, transaction_schema ,account_schema }
    }
    pub fn copy(&self) -> Database {
        Database {
            user_schema: self.user_schema.clone(),
            transaction_schema: self.transaction_schema.clone(),
            account_schema: self.account_schema.clone(),
        }
    }
    pub fn user(&self)->user::Init{
        user::Init::init(&self.user_schema)
    }
    pub fn account(&self)->account::Init{
        account::Init::init(&self.account_schema)
    }
    pub fn transaction(&self)->transaction::Init{
        transaction::Init::init(&self.transaction_schema)
    }
}
// fn db()-> Database{
//      Database::init()
//     }