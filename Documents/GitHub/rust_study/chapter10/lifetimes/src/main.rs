/*
    lifetime is an another kind of generic, which ensure 
        that references are valid as long as it is needed 

    every reference in Rust has a lifetime, which is the 
        scope for which that reference is valid 
        lifetimes are implicit and inferred (types are inferred) 
        recall: must annotate types only when multiple are possible 
        thus, must annotate lifetimes when the lifetimes of refs 
        could be related in a few different ways 
    
    Rust requires to annotate the relationships using generic lifetime params 
        to ensure the acutal refs used at runtime will definitely be valid 

        let r;                        | 'a lifetime block 
        {                             |
            let x = 5;      | 'b      | 
            r = &x;         |         | 
        }                             | 
        println!("r: {r}");           | 
        
    borrow checker: compares scopes to determine whether all borrows are valid 
        Rust compares the size of the lifetimes ('a and 'b) at a compile time 
        inner 'b block is much smaller than the outer 'a lifetime block 
        r has a lifetime of 'a but code refers to memeory with a lifetime of 'b 
        thus, the program is rejected because 'b is shorer than 'a 
        ** the subject of the ref does not live as long as the ref ** 
    
    lifetime annotation: start with an apostrophe(') ana all lowercase 
        Ex. &'a i32 is a ref with an explicit liftime 
        Ex. &'a mut i32 is a mutable ref with an explicit lifetime 
    
    
*/

use std::fmt::Display; 

// lifetime annotations in struct definitions 
// struct that holds a string slice (ref):  
struct ImportantExcerpt<'a> {
    // instance cannot outlive the ref it holds 
    part: &'a str, 
}

/*
    lifetime elision rules: do not need to write lifetime explicitly 
        lifetimes on fn or method params are called input lifetimes 
        lifetimes on return are called output lifetimes 
    
    if the compiler gets to the end of the three rules and there are 
        still refs for which it cannot figure out lifetimes, will stop 
        1. compiler assigns a lifetime param to each ref param 
            Ex. fn foo<'a>(x: &'a i32) gets one lifetime param 
            Ex. fn foo<'a, 'b>(x: &'a i32, y: &'b i32) gets two 
        2. if there is exactly one input lifetime param, 
            that lifetime is assigned to all output lifetime params 
        3. if there are multiple input lifetime params, 
            but one of them is &self or &mut self (in method), 
            their lifetime is assigned to all output lifetime params 
    
    Ex. fn longest<'a, 'b>(x: &'a str, y:&'b str) -> &str { 
        second rule does not apply: more than one input lifetime 
        third rule does not apply: longest is a fn rather than a method 
        thus, have not figured out what the return type's lifetime is 

    lifetime annotations in method definitions 

*/
// lifetime annotations in method definitions:  
impl<'a> ImportantExcerpt<'a> {
    // rule#1: only one input param 
    fn level(&self) -> i32 {
        3
    }

    // rule#3: one of the params is &self 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}"); 
        self.part 
    }
}

/*
    'static lifetime denotes that the affected ref CAN live 
        for the entire duration of the program 
        Ex. all string literals have the 'static lifetime: 
            let s: &'static str = "I have a static lifetime."; 
        stored directly in the program's binary (always available) 
*/

// generic type params, trait bounds, lifetimes: 
fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    ann: T, // any type that implements the Display trait 
) -> &'a str 
where T: Display, {
    println!("Announcement! {ann}"); 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // preventing dangling refs with lifetimes: 
    // valid ref as the data has a longer lifetime 
    let x = 5;         // | 'b 
    let r = &x;       // |    | 'a 
    println!("r: {r}");     // |    | 

    // generic lifetimes in functions: 
    let string1 = String::from("abcd"); 
    let string2 = "xyz"; 

    let result = longest(string1.as_str(), string2); 
    println!("The longest string is {result}"); 

    // refs to String that have different concrete lifetimes: 
    let string1 = String::from("long string is long"); 
    {
        let string2 = String::from("xyz"); 
        let result = longest(string1.as_str(), string2.as_str()); 
        println!("The longest string is {result}"); 
    }

    let novel = String::from("Call me Ismael. Some years ago..."); 
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); 
    let i = ImportantExcerpt {
        part: first_sentence, 
    }; 
    println!("{}", i.part); 
}

/*
    lifetime annotations in function sigs: declare the generic lifetime params 
        inside angle brackets btw the function name and the param list 
    
    sig express the contraint that the returned ref will be valid 
        as long as both the params are valid (relationship) 
    
    Ex. fn longest(x: &str, y:&str) -> &str { does not work 
        because whether the ref being returned refers to x or y is not known 

    Ex. fn longest<'a>(x: &'a str, y:&'a str) -> &'a str { 
        1. for some lifetime 'a, the function takes two params, both of which 
            are string slices that live at least as long as lifetime 'a 
        2. also that the string slice returned from the function will live 
            at least as long as lifetime 'a 
        3. in practice, the lifetime of the ref returned is the same as 
            the smaller of the lifetime the values referred to  
    
    specifying the lifetime params DOES NOT change the lifetime of any values 
        rather, specify the borrower checker to reject any values that 
        do not adhere to these contraints 

*/

// generic lifetime 'a will get the concrete lifetime 
// that is equal to the smaller of x or y 
fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
    specifying lifetime params depends on what the function is doing 
        Ex. fn longest<'a>(x: &'a str, y: &str) -> &'a str { x } 
            would not need to specify a flietime on the y param because 
            y does not have any relationship with x or the return value 
    
    if the ref returned refer to a value created within the fn, this 
        would be a dangling ref as it will go out of scope at the end
        fn longest<'a>(x: &str, y: &str) -> &a' str {
            let result = String::from("really long string"); 
            result.as_str() 
        } // result goes out of scope and gets cleaned up here 
*/


