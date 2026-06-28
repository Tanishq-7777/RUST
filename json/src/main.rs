use serde::{Deserialize,Serialize};
use serde_json::{to_string, to_string_pretty,from_str};

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(deny_unknown_fields)]
#[derive(Debug)]
struct Dog {
    name:String,
    year_born:i32,
    owner:Owner,
}
#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(deny_unknown_fields)]
#[derive(Debug)]
struct Owner {
    first_name:String,
    last_name:String,
}
fn main() {
    //serde crate is used to play with json data.
    //ser -> serialization
    //de -> deserialization
    let owner = Owner {
        first_name:"Tanishq".to_string(),
        last_name:"Saxena".to_string(),
    };
    let dog_01 = Dog {
        name:"blacky".to_string(),
        year_born:2026,
        owner:owner,
    };

    //converting DOG struct instance into json string
    // make a broorwed instance of DOG
    let dog_ser = to_string_pretty(&dog_01);
    match dog_ser {
        Ok(dog) => println!("{dog}"),
        Err(err) => panic!("Wrong implementation {err}"),
    }

    //If we deserialize it ,
    let json_string = r#"{"name":"blacky","color":"black","yearBorn":2026,"owner":{"firstName":"Tanishq","lastName":"Saxena"}}"#;
    let struct_json_string = from_str::<Dog>(json_string);
    match struct_json_string {
        Ok(dog) => println!("{dog:#?}"),
        Err(err) => panic!("Wrong implementation {err}"),
    }
}
