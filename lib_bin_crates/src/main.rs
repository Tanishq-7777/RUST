// src/main.rs -> Root Binary Crate
// src/lib.rs -> Root library Crate
//every thing in rust is private by default
//field of struct are also by default private

use lib_bin_crates::authenticate;
use lib_bin_crates::auth_utils::models::Credentials;
//OR 
// use lib_bin_crates::{Credentials,authenticate};
fn main() {
    let cred = Credentials {
        username:String::from("Tanishq"),
        password:String::from("Tanishq@123"),
    };
    authenticate(cred); 
}
