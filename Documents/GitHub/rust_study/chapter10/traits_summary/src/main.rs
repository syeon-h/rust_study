// bring the Summary trait into scope 
use traits_summary::{NewsArticle, Summary, Tweet}; 

fn main() {
    let tweet = Tweet {
        username: String::from("rust_study"), 
        content: String::from(
            "traits are similar to interfaces in other languages"
        ), 
        reply: false, 
        retweet: false, 

    }; 

    println!("1 new tweet: {}", tweet.summarize()); 

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"), 
        location: String::from("Pittsburgh, PA"), 
        author: String::from("iceburgh"), 
        content: String::from(
            "The pittsburgh Penguins once again are the best \
            hockey team in the NHL."
        ), 
    }; 

    println!("New article available! {}", article.summarize()); 
}

