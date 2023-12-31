use chrono::{DateTime, Utc};
use mongodb::{
    bson::{oid::ObjectId,doc, Document, self},
    sync::{Collection, ClientSession},
    results::{InsertOneResult, UpdateResult}, options::{FindOneOptions, UpdateModifications, UpdateOptions, FindOptions}
};
use rocket::http::ext::IntoCollection;
use serde::{Serialize, Deserialize, Serializer};
use serde_json::Value;

use crate::app::user::user_model::{serialize_object_ids, serialize_object_id};

use super::transaction_model::Transaction;

#[derive(Debug, Serialize, Deserialize,Clone)]
// pub struct Account {
//         #[serde(rename = "_id", skip_serializing_if = "Option::is_none",serialize_with = "serialize_object_id")]
//         pub id: Option<ObjectId>,
//         pub balance: f64,
//         pub locked_balance: f64,
//         #[serde( skip_serializing_if = "Option::is_none",serialize_with = "serialize_object_id")]
//         pub user_id: Option<ObjectId>,
//         pub channel: String,
//         pub currency: String,
//         #[serde(skip_serializing_if = "Option::is_none",serialize_with = "serialize_object_ids")]
//         pub transactions: Option<Vec<ObjectId>>,
//         pub updated_at: Option<DateTime<Utc>>,
//         pub created_at: Option<DateTime<Utc>>,
// }




// Update the Account struct to use the TransactionReference enum
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub balance: f64,
    pub locked_balance: f64,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_object_id")]
    pub user_id: Option<ObjectId>,
    pub channel: String,
    pub currency: String,
    // #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_object_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<TransactionReference>>, // Use TransactionReference enum
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}


// Define an enum to represent either an ObjectId or a Transaction
#[derive(Debug,  Deserialize,Clone)]
#[serde(untagged)]
pub enum TransactionReference {
    ObjectId(ObjectId),
    Transaction(Transaction),
}


impl Serialize for TransactionReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TransactionReference::ObjectId(obj_id) => {
                serialize_object_id(&Some(*obj_id), serializer)
            }
            TransactionReference::Transaction(transaction) => {
                transaction.serialize(serializer)
            }
        }
    }
}

impl Account {
    pub fn new(channel:String,currency:String,user_id:Option<ObjectId>)->Self{
        Self {
            locked_balance:0.0,
            balance:0.0,
            currency,
            channel,
            user_id,
            transactions: None,
            id:None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }

    }
}
 
pub struct Init<'a> {
    col: &'a Collection<Account>,
}

impl<'a> Init<'a> {
    pub fn init(col: &'a Collection<Account>) -> Self {
        Init { col }
    }

    pub fn save(&self, account: &Account)->Result<InsertOneResult, mongodb::error::Error> {
        self.col.insert_one(account, None)
    }
    pub fn find_one(&self, find_by:Document,filter_by:Option<FindOneOptions>)->Result<std::option::Option<Account>, mongodb::error::Error> {
        self.col.find_one(find_by,filter_by)
    }
    pub fn find_by_id(&self, object_id: &ObjectId)->Result<std::option::Option<Account>, mongodb::error::Error> {
        self.col.find_one(doc!{"_id":object_id}, None)
    }
    
    // pub fn accounts_with_transactions(&self,pipeline:Vec<Document>,options:Option<Document>,)
    // ->Result<Vec<Document>,mongodb::error::Error>
    //     { 
        
    //     let result  = self.col.aggregate(pipeline, None).ok();

    //     // match result {
    //     // Some(result)=>{
    //             let response  :Vec<Document> = result.unwrap().map(|c|
    //                 c.unwrap()).collect();
    //                 return Ok(response);
    //                 // let collected_results: Vec<Document> = s;
    //                 // println!("{:?}", response);
    //             // println!("{:?}",s.collect());

    //         // },
    //         // None=>{
    //         //     println!("{}","error ");
    //         // }
    //     // }
    
    // }

    pub fn accounts_with_transactions(
        &self,
        pipeline: Vec<Document>,
        options: Option<Document>,
    ) -> Result<Vec<Account>, mongodb::error::Error> {
        let result = self.col.aggregate(pipeline,  None).ok();
    
        let response: Result<Vec<Account>, mongodb::error::Error> = result
            .unwrap()
            .map(|doc| {
                let account: Account = bson::from_bson(bson::Bson::Document(doc.unwrap()))?;
                Ok(account)
            })
            .collect();
    
        return response;
    }
    


    pub fn find(&self, find_by:Option<Document>,filter_by:Option<FindOptions>)


    -> Result<Vec<Account>, mongodb::error::Error> { 
    let cursors = self
    .col
    .find(find_by, filter_by)
    .ok()
    .expect("Error getting list of Transactions");
    let response  = cursors.map(|doc| doc.unwrap());
    Ok(response.collect())
    }


    pub fn update_one(&self, filter_by:&Document,update:&UpdateModifications,update_option:Option<UpdateOptions>,session: Option<&mut ClientSession>)->Result<UpdateResult, mongodb::error::Error> {
        if let Some(session) = session {
        //  return  self.col.update_one_with_session(filter_by,update,update_option,session)
        }
         self.col.update_one(filter_by.to_owned(),update.clone(),update_option)
 }
}
 