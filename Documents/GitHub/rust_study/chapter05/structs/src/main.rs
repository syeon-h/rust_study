/*
    structs are similar to tuples in that both hold multiple related values 
        1. the pieces (fields) can be different types 
        2. will name each piece of data (more flexible) 
        3. thus, do not have to rely on the order of the data 
*/

// to define a struct: 
struct User {
    // fields: 
    active: bool, 
    username: String,   // using String rather than &str is a delibrate choice 
    email: String,      // because we want each instance to own all of its data 
    sign_in_count: u64, 
}

// tuple structs without name fileds to create different types: 
struct Colour(i32, i32, i32); 
struct Point(i32, i32, i32); 

fn main() {
    // instance can be mutable (but only the entire) 
    let mut user1 = User {
        active: true, 
        username: String::from("someuser123"), 
        email: String::from("someone@example.com"), 
        sign_in_count: 1, 
    }; 

    // use dot notation to get a specific value 
    user1.email = String::from("another@example.com"); 

    // creating instances from other instances: 
    let user2 = User {
        active: user1.active, 
        username: user1.username, 
        email: String::from("another2@example.com"), 
        sign_in_count: user1.sign_in_count, 
    }; 

    /*
        note that the struct update syntax uses = like an assignment 
            it "moves" the data, thus, can no longer use user1 as a whole 
            after creating user2 because the String in the username filed 
            was "moved" into user2 
        
        if we had given user2 new String values for both email and username, 
            then user1 would still be valid after creating user2 
    */

    // .. specifies that the remaining fields have the same value 
    let user3 = User {
        email: String::from("another3@example.com"), 
        ..user2
    }; 

    // black and origin balues are different "types"  
    let black = Colour(0, 0, 0); 
    let origin = Point(0, 0, 0); 
}

// function that returns a User instance: 
fn build_user(email: String, username: String) -> User {
    User {
        active: true, 
        username,   // identical to username: username 
        email,      // identical to email: email 
        sign_in_count: 1, 
    }
}