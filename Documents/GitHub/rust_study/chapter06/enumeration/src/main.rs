/*
    enum gives a way of saying a value is one of a possible set of values 
        where struct gives a way of grouping together related fields and data 
        Ex. Rectangle is one of a set of possible shapes (Circle, Triangle, etc.) 
    can put any kind of data inside an enum variant: strings, numerics, structs 
        evan can include another enum too 
*/

// IP address can be either a version 4 or 6, but not both:  
enum IpAddrKind {
    V4, // variant 
    V6, // variant 
}
// attaching data to each variant directly is more concise 
enum IpAddr {
    // each can have different types and amounts of data 
    V4(u8, u8, u8, u8), 
    V6(String), 
}

// struct like enum: variants are grouped under the Message 
enum Message {
    Quit,                       // no data associated 
    Move { x: i32, y: i32 },    // named fields like a stuct 
    Write(String),              // single String 
    ChangeColour(i32, i32, i32),// three i32 values 
}

// also able to define methods on enum: 
impl Message {
    fn call(&self) {
        // method body 
    }
}

// struct version of Message enum: 
struct QuitMessage;         // unit struct 
struct MoveMessage{
    x: i32, 
    y: i32, 
} 
struct WriteMessage(String); // tuple struct 
struct ChangeColourMessage(i32, i32, i32); 

/*
    Option is another enum defined by the standard library 
        encodes the common scenario in which a value could be something or nothing 
        Ex. if you request the first item in a non-empty list, you would get a value 
        Ex. if you request the first item in an empty list, you would get nothing 
    
    the compiler can check whether you have handled all the cases you should be 
        Rust does not have the NULL feature: problem with NULL is that 
        if you try to use a null value as a not-null value, you will get an error 

    NULL is a value that is currently invalid or absent for some reason 
        as such, Rust does have an Option<T> enum that can encode this concept 
        enum Option<T> { 
            None,       // do not have a valid value 
            Some(T),    // know that a value is present 
        }

    why is having Option<T> any better than having NULL? 
        because Option<T> and T (any type) are different types, the compiler 
        will not let us use an Option<T> value as if it were definitely a valid value 
        Ex. following will not compile: 
            let x: i8 = 5; 
            let y: Option<i8> = Some(5); 
            let sum = x + y; 
        have to convert an Option<T> to a T before performing T operation 
        thus, required to explicitly handle the case when the value is NULL 
*/


// match compare a value against a series of patterns and then execute code 
enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter, 
}

// with if, the condition needs to evaluate to a Boolean with if 
// but match can be any type 
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // match arms: pattern => some code 
        Coin::Penny => 1, 
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Quarter => 25, 
    }
}

// patterns that bind to values: 
#[derive(Debug)] 
enum UsState {
    Alabama, 
    Alaska, 
}

enum CoinState {
    Penny, 
    Nickel, 
    Dime, 
    Quarter(UsState), 
}

fn value_in_cents_states(coin: CoinState) -> u8 {
    match coin {
        CoinState::Penny => 1, 
        CoinState::Nickel => 5, 
        CoinState::Dime => 10, 
        CoinState::Quarter(state) => {
            println!("State quarter from {state:?}!"); 
            25 
        }
    }
}

// matching with Option<T>: mathces are "exhaustive" 
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // the arms' patterns must cover all possibilities 
        None => None, // not handling None case will cause a bug    
        Some(i) => Some(i + 1), 
    }
}

/*
    _ palceholder covers every other possible value 
        catch-all pattern meets the requirement that match must be exhaustive 
    
    catch-all arm (_) have to be the last as the patterns are evaluated in order 
    Ex. let dice_roll = 9; 
        match dice_roll {
            3 => add_fancy_hat(), 
            7 => remove_fancy_hat(),
            _ => reroll(), 
        }
*/

fn main() {
    // enum values of IpAddrKind: 
    let four = IpAddrKind::V4; 
    let six = IpAddrKind::V6; 

    // call the function with either variant: 
    route(IpAddrKind::V4); 
    route(IpAddrKind::V6); 

    let home = IpAddr::V4(127, 0, 0, 1);  
    let loopback = IpAddr::V6(String::from("::1")); 

    let m = Message::Write(String::from("hello")); 
    m.call(); 

    let some_number = Some(5); // Option<i32>
    let some_char = Some('e');// Option<char> 
    // for absent, need to annotate the overall Option type: 
    let absent_number: Option<i32> = None; 

    let five = Some(5); 
    let six = plus_one(five); 
    let none = plus_one(None); 

    // concise control flow with if let (loss the exhaustive checking)  
    let config_max = Some(3u8); 
    // case#1: match that only cares about Some 
    match config_max {
        Some(max) => println!("The max is configured to be {max}"), 
        _ => (), 
    }
    // case#2: if let version of case#1 
    if let Some(max) = config_max {
        println!("The max is configured to be {max}"); 
    } // can include an else here {} 
}

// can define a function that takes any IpAddrKind: 
fn route(ip_kind: IpAddrKind) {} 