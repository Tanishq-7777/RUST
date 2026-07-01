use axum::{Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize)]
struct User {
    first_name:String,
    email:String,
    password:String,
}
#[tokio::main]
async fn main() {
    println!("Hello, world!"); 
    
    //1. Create the axum router
    let authRouter = Router::new().route("/auth",get(getUser)).route("/login", post(login));
   
    //2. Define the IP and port to listen(TCP);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    //3. axum serve launch to web server
    axum::serve(listener, authRouter).await.unwrap();
}
async fn getUser() -> Json<User> {
    let user = User {
        first_name:"Tanishq".to_string(),
        email:"saxena.gmail.com".to_string(),
        password:"123456".to_string(),
    };
    Json(user)
}
async fn login(Json(body): Json<User>) -> Json<User> {
    Json(body)
}