#![allow(dead_code,unused_variables)]

mod database;
// mod database {
//     pub enum Status {
//         Connected,
//         Interupted,
//     }
//     pub fn connect_to_database() -> Status {
//         //connect to db
//         Status::Connected
//     }
//     pub  fn get_user() {
//         //fetch the user from db and return
//     }
// }




pub mod auth_utils;
// pub  mod auth_utils {
//     pub fn login(cred: models::Credentials) {
//     //try to login the user
//     super::database::get_user()//or crate::database::get_user()
//     }
//     pub mod models {
//         pub struct Credentials {
//             pub username:String,
//             pub password:String,
//         }
//     }
// }

use auth_utils::login;
use database::{connect_to_database,Status};

pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred);
    }
    println!("Your Credentials Are Verified");
}

pub mod util;//now compiler will check it inside util.rs or util/mod.rs