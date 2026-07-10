use futures::TryStreamExt;
use mongodb::{Client, bson::oid::ObjectId};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
#[derive(Debug,Deserialize,Serialize)]

struct User {
    uid:Option<ObjectId>,
    first_name:String,
    last_name:String,
    email:String,
    age:i32,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Step 1 -> creating a mongoDb client like mongoose.connect(uri)
    let client = Client::with_uri_str("mongodb+srv://TanishqSaxena:tani2007@moviespace.mj2nkz4.mongodb.net/").await?;
    println!("Connected to MongoDB client!");
    // STEP 2: Verify the connection
    client.database("Rust").run_command(doc! { "ping": 1 }).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    // STEP 3: Get a handle to the database
    let db = client.database("Rust");
    // STEP 4: Get a collection handle
    let user = db.collection::<User>("user");
    println!("Collection handle obtained!");
    // STEP 5: Insert a document
    // let user1 = User {
    //     uid:Some(ObjectId::new()),
    //     first_name:String::from("Tani"),
    //     last_name:String::from("Saxena"),
    //     email:String::from("sax@gmail.com"),
    //     age:19,
    // };
    // user.insert_one(user1).await?;
    //STEP 6 Find doc
    let mut cursor:Vec<User> = user.find(doc! {}).await?.try_collect().await?;
    println!("{cursor:#?}");
    Ok(())
}
