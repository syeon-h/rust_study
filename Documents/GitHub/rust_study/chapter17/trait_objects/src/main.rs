// someone decides to implement a SelectBox struct: 
// another create using gui 
use trait_objects::Draw; 

// implementing the Draw trait on a SelectBox struct: 
struct SelectBox {
    width: u32, 
    height: u32, 
    options: Vec<String>, 
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box 
    }
}

// lib user can now create a Screen instance: 
use trait_objects::{Button, Screen}; 

fn main() {
    // using trait objects to store vals: 
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75, 
                height: 10, 
                options: vec![
                    String::from("Yes"), 
                    String::from("Maybe"), 
                    String::from("No"), 
                ], 
            }), 
            Box::new(Button {
                width: 50, 
                height: 10, 
                label: String::from("OK"), 
            }), 
        ], 
    };

    screen.run(); 
}

/*
    when the lib was first written, did not know that some might add 
        the SelectBox but the Screen implementation was able to operate 
        on the new type because SelectBox implements the Draw trait 

        this concept is similar to duck typing: if it walks like a duck 
        and quacks like a duck, then it must be a duck 

        by specifying Box<dyn Draw> as the type of the vals, we have 
        defined Screen to need vals that we can call the draw method on; 
        never have to check whether a val implements a particular method 
        as Rust will not compile if the vals do not implement the traits 

    trait objects perform dynamic dispatch: when the compiler cannot tell 
        at compile time which method you are calling 
        
        recall the monomorphization process when we use trait bounds on 
        generics: the code that results from monomorphization is doing 
        static dispatch, which is when the compiler knows what method you 
        are calling at compile time; this is opposed to dynamic dispatch 

        also prevents the compiler from choosing to inline a method's code 
        which in turn prevents some optimizations (trade-off: flexibility) 

*/