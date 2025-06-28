/*
    default generic type parameter eliminates the need for implementors 
        of the trait to specify a concrete type if the default type works; 
        specify with the <PlaceholderType=ConcreteType> syntax  

        useful with operator overloading, which to customize the behaviour 
        of an operator that listed in std::ops by implementing the traits 

        Ex. overload the + operator to add two Point instances together: 
            Rhs=Self is a syntax called default type parameters; the Rhs 
            (Righthand side) defines the type of the rhs param in the add 
            method; if not specified, the type of Rhs will default to Self

        Ex. add values in millimetres to values in metres and have the 
        implementation of Add trait where we want to customize the Rhs: 
            struct Millimetres(u32); 
            struct Metres(u32); 
            impl Add<Metres> for Millimetres { 
                type Output = Millimetres; 
                fn add(self, other: Metres) -> Millimetres {
                    Millimetres(self.0 + (other.0 * 1000))
                }
            }

        use default type params in two main ways: 
            1. to extend a type without breaking existing code 
            2. to allow customization in specific cases (rarely)  

    fully qualified syntax for calling methods with the same name; nothing
        in Rust prevents a trait from having a method with the same name 
        as another trait's, nor implementing both traits on one type; 

        Ex. when fly is called on an instance of Human, the compiler 
        defaults to calling that is directly implemented on the type 

        specifying the trait name before the method name clarifies; 
        because the method takes a self param, Rust could figure out 
        which implementation of a trait to use based on the type of self 

        however, associated functions that are not methods, do not have a 
        self param; thus, need to use fully qualified syntax of: 
        <Type as Trait>::function(receiver_if_method, next_arg, ...);   

        Ex. Animal trait with an associated non-method function baby_name: 
        becasue Animal::baby_name does not have a self param, and there 
        could be other types that implement the Animal trait, Rust cannot 
        figure out which to call (ex. Animal::baby_name(); not compile) 

    supertrait is the trait that one trait definition is relying on; 
        might write a trait definition that depends on another trait, 
        thus, to implement the first trait, want to requrie that type to 
        also implement the second trait 

        Ex. OutlinePrint trait with an outline_print method that print a
        given value formatted so that it is framed in astrerisks; in the 
        implementation, want to use the Disply trait's functionality, 
        therefore, need to specify that the Outline Print trait will work 
        only for types that also implement Dispaly 

    newtype pattern implements external traits on external types
        recall that Rust only allow to implement a trait on a type if 
        either the trait or the type are local to the crate 

        newtype pattern involves creating a new type in a tuple struct: 
        that will have one field and be a thin wrapper around the type 
        we want to implement a trait for; then the wrapper type is local 

        there is no runtime performance penalty and the wrapper type is 
        elided at compile time 

        Ex. to implement Display on Vec<T>, which the orphan rule prevents
        because the Display trait and the Vec<T> type are defined outside; 
        make a Wrapper struct that holds an instance of Vec<T>, then 
        implement Display on Wrapper and use the Vec<T> val 

        the downside is that Wrapper is a new type, so it does not have 
        the methods of the value it is holding; have to implement all the 
        methods of Vec<T> directly on Wrapper 

        if we wanted the new type to have every method the inner type has, 
        implement the Deref trait on the Wrapper to return the inner type 

        also useful even when traits are not involved 
*/

use std::ops::Add; 

/*
    definition with the default generic type: 
    trait Add<Rhs=Self> {
        type Output; 

        fn add(self, rhs: Rhs) -> Self::Output;     
    }
*/


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32, 
    y: i32, 
}

impl Add for Point {
    // associated type that determines the type returned: 
    type Output = Point; 

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x, 
            y: self.y + other.y, 
        }
    }
}

// two traits having a fly method: 
trait Pilot {
    fn fly(&self); 
}

trait Wizard {
    fn fly(&self); 
}

struct Human; 

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking."); 
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!"); 
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furisouly*"); 
    }
}

// trait with an associated function: 
trait Animal {
    fn baby_name() -> String; 
}

struct Dog; 

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt; 

// trait that requires the functionality from Display: 
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); 
        let len = output.len(); 
        println!("{}", "*".repeat(len + 4)); 
        println!("*{}*", "*".repeat(len + 2)); 
        println!("* {output} *"); 
        println!("*{}*", "*".repeat(len + 2)); 
        println!("{}", "*".repeat(len + 4)); 
    }
}

// implement Display on Point: 
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y) 
    }
} 

// create a Wrapper type around Vec<String>:  
struct Wrapper(Vec<String>); 

// implemention of Display on Wrapper 
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // uses self.0 to access the inner Vec<T> 
        write!(f, "[{}]", self.0.join(", ")) 
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, 
        Point { x: 3, y: 3 }  
    ); 

    // calling fly on an instance of Human 
    let person = Human; 
    Pilot::fly(&person);    // this is your captain speaking. 
    Wizard::fly(&person);   // up  
    person.fly(); // *waving arms furisouly* 

    // using fully qualified syntax to specify the fn 
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot 
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy 

    // use newtype pattern on Display and Vec<T>: 
    let w = Wrapper(vec![String::from("hello"), String::from("world")]); 
    println!("w = {w}"); 
}
