/*
    by using a trait bound with an impl block that uses generic params,  
        can implement mnethods conditionally for types
        that implement the specified traits 
        Ex. Pair<T> always implements the new function to return 
            a new instance of Pair<T> 

    blanket implementation: implementations of a trait on any type  
        that satisfies the trait bounds 
        Ex. standard lib implements the ToString trait on any type 
            that implements the Display trait 
            impl<T: Display> ToString for T {} 
        thus, conditionally implement a trait for any type that 
        implmements another triat 

*/
use std::fmt::Display; 

struct Pair<T> {
    x: T, 
    y: T, 
}

impl<T> Pair<T> {
    // return a new instance of Pair<T> 
    fn new(x: T, y: T) -> Self {
        // Self is a type alias for the type of the impl block 
        Self{ x, y } 
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // Pair<T> only implements the cmp_display method if 
    // inner type T implements the ParticalOrd + Dispaly triat 
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x); 
        } else {
            println!("The largest member is y = {}", self.y); 
        }
    }
}

fn largest<T>(list: &[T]) -> T 
    // trait bounds with where clauses:  
    where T: PartialOrd + Copy 
    // restricts the types valid to only those that
    // implement PartialOrd and Copy trait 
{
    let mut largest = list[0]; 

    for item in list {
        if *item > largest {
            largest = *item; 
        }
    }

    largest 
}

fn main() {} 