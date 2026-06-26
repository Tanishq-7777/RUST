struct News_Article {
    headline:String,
    content:String,
    author:String,
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
}
// ! No Default every struct implementing this need to write their own summarize function
trait Summary {
    fn summarize(&self) -> String; // It is like an abstract function
}

// trait Summary {
//     fn summarizeEveryThing(&self) -> String {
//         format!("(Read more from {}...)", self.summarize())
//     }
//     fn summarize(&self) -> String {
//         String::from("This is the whole summary.")
//     }
// }

impl Summary for News_Article {
    fn summarize(&self) -> String {
        String::from("Hey News Summary")
    }
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from("Hey Tweet Summary")
    }
}

// impl Summary for Tweet {} -> This will take default summarize function


fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_generic<T: Summary>(item :&T) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    // They are interfaces of RUST or abstract class of RUST.
    let news_letter = News_Article {
        author:String::from("Tanishq"),
        headline:String::from("Tanishq is the new leader"),
        content:String::from("It is a news."),
    };
    let tweets = Tweet {
        username: String::from("Tani"),
        content:String::from("It is a tweet"),
        reply:false,
    };

    let news = news_letter.summarize();
    println!("{news}");
    // let summary_Everything = news_letter.summarizeEveryThing();
    // println!("{summary_Everything}");

    let tweet = tweets.summarize();
    println!("{tweet}");
    // let summary_Everything = tweets.summarizeEveryThing();
    // println!("{summary_Everything}");

    notify(&news_letter);
    notify_generic(&tweets);



    
}
