use chrono::{Utc, DateTime};
use mongodb::{
    bson::{oid::ObjectId,doc, Document},
    sync::{Collection, ClientSession},
    results::{InsertOneResult, UpdateResult}, options::{UpdateOptions, UpdateModifications}
};

use serde::{Serialize, Deserialize};

use crate::app::user::user_model::serialize_object_id;

use super::account_type::{TransactionStatus, TransactionType};
#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct Transaction {
        #[serde(
            rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_object_id"
        )]
        pub id: Option<ObjectId>,
        pub amount: f64,
        pub currency: String,
        pub fee: f64,
        pub receiver_id: String,
        pub giver_id: String,
        pub description:Option<String>,
        pub provider_name: String,
        pub transaction_type: TransactionType,
        pub status: TransactionStatus,
        pub provider_reference : Option<String>,
        pub provider_fee :Option<f64>,
        updated_at: Option<DateTime<Utc>>,
        created_at: Option<DateTime<Utc>>,
}
 


pub struct Init<'a> {
    col: &'a Collection<Transaction>,
}

impl Transaction {
    pub fn new(
        amount: f64,
        currency: String,
        fee: f64,
        receiver_id: String,
        giver_id: String,
        provider_name: String,
        transaction_type: TransactionType,
        status:TransactionStatus,
        description:String
    ) -> Self {
        Self {
            amount,
            currency,
            id:None,
            provider_reference:None,
            provider_fee:None,
            fee,
            receiver_id,
            giver_id,
            description:Some(description),
            provider_name,
            transaction_type,
            status,
            updated_at:  Some(Utc::now()),
            created_at:  Some(Utc::now()),
        }
    }
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<Transaction>) -> Self {
        Init { col }
    }

    pub fn save(&self, transaction: &Transaction)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(transaction, None)
    }
    pub fn create(&self, transaction: &Transaction,session: Option<&mut ClientSession>)->Result<InsertOneResult, mongodb::error::Error> {
    if let Some(session) = session {
      return self.col.insert_one_with_session(transaction, None,session)
       }
      self.col.insert_one(transaction, None)
    }
    pub fn find_one(&self, filter_by:Document)->Result<std::option::Option<Transaction>, mongodb::error::Error> {
        self.col.find_one(filter_by, None)
    }
    pub fn find_all(&self,filter_by:Option<Document>)
    -> Result<Vec<Transaction>, mongodb::error::Error> {
    let cursors = self
    .col
    .find(filter_by, None)
    .ok()
    .expect("Error getting list of Transactions");
    Ok(cursors.map(|doc| doc.unwrap()).collect())
    }

    pub fn update_one(&self, filter_by:Document,update:&UpdateModifications,update_option:Option<UpdateOptions>,session: Option<&mut ClientSession>)->Result<UpdateResult, mongodb::error::Error> {
           if let Some(session) = session {
            return  self.col.update_one_with_session(filter_by,update.to_owned(),update_option,session)
           }
            self.col.update_one(filter_by,update.to_owned(),update_option)
    }
}
 