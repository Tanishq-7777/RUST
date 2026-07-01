use reqwest::blocking::Client;


fn main() {
    let http_client = Client::new();
    let  result = http_client.get("https://muvies.tanishqsaxena.xyz").send();
    match result {
        // Ok(res) => println!("hey this is the response {:#?}",res.text()),
        // Ok(res) => println!("hey this is the response {:#?}",res),
        Ok(res) => println!("hey this is the response {:#?}",res.headers()),
        Err(err) => println!("there is an error {err}"),
    }

    //You can also do a post request
    // let res = http_client.post("your api").body("your body").send();

    //You can also contruct your own client
}
