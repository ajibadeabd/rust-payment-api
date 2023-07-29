#[macro_use]
extern crate rocket;


mod app;
mod modules;
mod database;

use std::sync::Arc;

use modules::{middleware,middleware_copy};
use app::{
    user::user_route::{add_user,sign_in},
    account::account_route::{ account_creation ,deposit,withdraw}
    
};
 

#[launch]
fn rocket() -> _ {
    // rocket= mount_user_route(rocket);
let db=database::Database::init();
    let rocket = rocket::build()

    .mount("/api", routes![add_user,sign_in]);
    let rocket = rocket.mount("/api/account", routes![account_creation,deposit,withdraw])
    .manage(db);

     rocket
}


