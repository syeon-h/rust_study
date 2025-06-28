/*
    trait defines the functionality a particular type has and can share with other 
        to define shared behaviour in an abstract way 
        to specify that a generic can be any type that has certain behaviour 

    trait is a collection of methods defined for an unknown type: Self 

    type's behaviour consists of the methods we can call on that type 
        Ex. different types that call the same methods (same behaviour) 
        trait definitions are a way to group method sigs together 
    
    Ex. to make a media aggregator libary crate named "aggregator" 
        there are multiple structs that hold various kinds and amounts of text 
        aggregator: a media aggregator library crate that can display summaries 

    coherence: can implement a trait on a type only if either  
        the trait or the type, or both, are local to our create 
        Ex. can implement standard library traits on a custom type (local) 
        Ex. can implement Summary (local) on Vec<T> in our aggregator crate 
        cannot implement external traits on external types 
        Ex. cannot implement standard library traits on Vec<T> within 
            our aggregator crate because both are not local 

    default implementations can call other methods in the same trait 
        trait can provide a lot of useful functionality and 
        only require implementors to specify a small part of it 

    traits as parameters: to define functions that accept many different types 

    can use the impl Trait syntax in the return position to return a value of 
        some type that implements a trait without naming the concrete type 
        especially useful in the context of closures and iterators 
        however, can only use if fn is returning a single type 
        Ex. code that returns either a NewsArticle or a Tweet would not work 
        fn returns_summarizable(switchL bool) -> impl Summary {
            if switch {
                NewsArticle { 
                }
            } else {
                Tweet { 
                }
            }
        }

*/

use std::fmt::{Display, Debug};

// consists of the behaviour provided by a summarize 
pub trait Summary {
    // can have multiple methods in its body 
    // no default implementation: 
    fn summarize_author(&self) -> String; 

    // with a default implementation: 
    // that call other methods in the same trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// implementing a trait on a type: 
pub struct NewsArticle {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String, 
}

// put the trait name and use the for keyword 
impl Summary for NewsArticle {
    // put the method sigs within the block:   
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location) 
    }

    fn summarize_author(&self) -> String {
        self.author.clone() 
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) 
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username) 
    }
}

// calls the summarize method on its item parameter: 
// specify the impl keyword and the trait name 
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize()); 
}

// trait bound syntax with the generic type parameter: 
// can express more complexity in other cases 
// ex. to force both parameters to have the same type 
pub fn notify_gen<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// specifying multiple trait bounds with the + syntax: 
// ex. to use diplay formatting as well as summarize 
pub fn notify_mult(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize()); 
} 

// + syntax is also valid on generic types: 
pub fn notify_mult_gen<T: Summary + Display>(item: &T) { 
    println!("Breaking news! {}", item.summarize()); 
}

// clearer trait bounds with where clauses: 
// fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {} 
fn some_fn<T, U>(t: &T, u: &U) 
    where T: Display + Clone, 
        U: Clone + Debug, 
{}

// returning types that implement traits: 
// that implements the Summary trait without the concrete type 
fn returns_summarizable() -> impl Summary {
    // returns 
    Tweet {
        username: String::from("rust_study"), 
        content: String::from(
            "traits are similar to interfaces in other languages"
        ), 
        reply: false, 
        retweet: false, 
    }
}