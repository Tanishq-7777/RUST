pub mod models;
pub fn login(cred: models::Credentials) {
//try to login the user
    super::database::get_user()//or crate::database::get_user()
}

