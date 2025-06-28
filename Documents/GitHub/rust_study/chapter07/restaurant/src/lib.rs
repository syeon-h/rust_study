/*
    modules let us organize code within a crate 
        allow us to control the privacy of items  
    
    by using moduels, we can group related definitions together 
        and name why they are related 

    the contents of the crate root form a module named crate 
        at the root of the crate's module structure: "module tree"  
    
    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist 
        │   └── seat_at_table    
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
*/

// organize its functions into nested modules:  
mod front_of_house {    // parent module 
    // modules cab hold definitions for other itmes 
    // such as structs, enums, constants, traits, functions 
    pub mod hosting {   // child module 
        // making the module pub does not make its contents pub 
        // recall: items are private to parent modules by default 
        pub fn add_to_waitlist() {} 

        fn seat_at_table() {}
    }

    pub mod serving {   
        fn take_order() {} 

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {} 

mod back_of_house {

    /*
        making structs and enums public 

        pub struct make the struct public 
            but it's files will still be private (seasonal_fruit) 

        enums are not very useful unless their variants are public 
            so the default for enum variants is to be public 
    */
    pub struct Breakfast {
        pub toast: String, 
        seasonal_fruit: String, 
    }

    impl Breakfast {
        // pub associated fn that constrcts an instance 
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }

    // make an enum public: 
    pub enum Appetizer {
        Soup, 
        Salad, 
    }

    fn fix_incorrect_order() {
        cook_order(); 
        // relative path starting with super (parent mod): 
        // fewer places to update code in the future if moved 
        super::deliver_order(); 
    }

    fn cook_order() {} 
}

/*
    bringing paths into scope: use keyword 
    Ex. hosting is now a valid name in that scope 
        but if eat_at_restaurant fn moves into a new child mod, 
        which is then a different scope, these will not compile 

    1. bring the function's parent module into scope 
        to make it clear that the fn is not locally defined 
    2. bring structs, enums, and other items with the full path 
        no strong reason behind; just the convention 
    
    providing new names: as keyword 
        to bring two types of the same name into the same scope 

    re-exporing names: pub use 
        the bane available in the new scope is private 
        make that item available for others to bring into their scope 
    thus, can write a code with one structure but expose a different structure 
    Ex. now external code can use restaurant::hosting::add_to_waitlist() path 
        re-exported the hosting module from the root module 

    using external packages: add a dependency in Cargo.toml 
        tells Cargo to download the package and any dependencies (available) 
        to bring the definition into the scope, add a use line 

    using nested paths: to clean up large use lists 
        if using multiple items defined in the same crate or module, 
        use nested paths to bring the same items in one line 
    Ex. use std::cmp::Ordering; into use std::{cmp::Ordering, io}; 
        use std::io; 
    Ex. use std::io;    into use std::io::{self, Write}; 
        use std::Write; 

    brinf all public items defind in a path: * glob operator 
        but be careful as glob can make it harder to tell 
        what names are in scope and where a name used was defined 
*/

// bringing fn into the scope:  
use crate::front_of_house::hosting; 

// bringing struct into the scope: 
use std::collections::HashMap; 

// providing new names with as keyword: 
use std::fmt::Result; 
use std::io::Result as IoResult; 

fn function1() -> Result {} 
fn funtion2() -> IoResult<()> {}

// making a name avilable for any code to use: 
// pub use crate::front_of_house::hosting; 

// bring all public items with glob operator: 
use std::collections::*; 

/*
    path can take two forms: 
        1. absolute path is the full path starting from a crate root 
        2. relative path starts from the current module 
            uses self, super, or an identifier in the current module 

    choosing whether to use is a decisiion based on the project 
        depends on whether you are more likely to move item definition code 
        separately from or together with the code 
    Ex. if front_of_house mod and eat_at_restaurant fn moved into another mod: 
        need to update the absolute path to add_to_waitlist 
        but the realtive path would still be valid 
    Ex. if eat_at_restaurant fn moved seprataely into a mod named dining: 
        absolute path to add_to_waitlist call would stay the same 
        but the relative path would need to be updated 
    
    preference in general is to specify absolute paths 
        more likely to move code definitions and item calls independently  
*/

pub fn eat_at_restaurant() {
    // absolute path: can use the crate keyword as 
    // fn is defined in the same crate as eat_at_restuarant 
    crate::front_of_house::hosting::add_to_waitlist(); 

    // relative path: starts with front_of_house 
    front_of_house::hosting::add_to_waitlist(); 

    // order a breakfast in the summer with Rye 
    let mut meal = back_of_house::Breakfast::summer("Rye"); 
    // change the bread 
    meal.toast = String::from("Wheat"); 
    println!("I'd like {} toast please", meal.toast); 

    // meal.seasonal fruit = String::from("bluberries"); will not work 
    // as not allowed to see or modify the seasonal fruit (private) 

    // because Appetizer enum is public, can use the variants: 
    let order1 = back_of_house::Appetizer::Soup; 
    let order2 = back_of_house::Appetizer::Salad; 

    hosting::add_to_waitlist(); 
}