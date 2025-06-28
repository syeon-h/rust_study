/*
    generics create definitions for items like function signatures or structs, 
        which then can be sued with many different concrete data types 

    recall: enum Option<T> {Some(T), None,} 
        enums can hold generic data types in their variants 
        Some holds one value of type T 
        None variant that does not hold any value 

    enums can use multiple generic types as well 
        Ex. enum Result<T, E> {Ok(T), Err(E),} 

    methods can be implemented on structs and enums and 
        use generic types in their definitions too 

    generic type parameters in a struct definition are not always the same 
        as those you can use in that same struct's method signatures 

    generic type parameters do not have a runtime cost 
        Rust performs monomorphization using generics at compile time,  
        the process of turning generic into specific by filling in the concrete types 
*/

// finds the largest i32 in a slice 
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0]; 

    for item in list {
        if item > largest {
            largest = item; 
        }
    }

    largest 
}

// finds the largest char in a slice 
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0]; 

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest 
}

// generic function over some type T 
// return a reference to a value of the same type T 
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // std::cmp::PartialOrd restricts the types valid 
    // to only those that implement ParticalOrd  
    let mut largest = &list[0]; 

    for item in list {
        if item > largest {
            largest = item; 
        }
    }

    largest 
}

// struct that holds x and y values of type T: 
// x and y must be the same type 
struct Point<T> {
    x: T, 
    y: T, 
}

// method on the Point<T> struct:  
// return a reference to the x field type T 
impl<T> Point<T> {
    // declared T just after impl
    // so T to specify on the type Point<T> 
    fn x(&self) -> &T {
        &self.x
    }
}

// can also specify constraints on generic types 
// method only on Point<f32>: 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// struct that could have different types:  
struct PointTwo<X1, Y1> {
    x: X1, 
    y: Y1,  
} 

// generic types X1 and Y1 for PointTwo struct 
// X2 and Y2 for mixup method signature (to make clearer) 
impl<X1, Y1> PointTwo<X1, Y1> {
    // creates a new PointTwo with X1 and Y2 
    fn mixup<X2, Y2>(self, other: PointTwo<X2, Y2>) -> PointTwo<X1, Y2> {
        PointTwo {
            x: self.x, 
            y: other.y, 
        }
    }
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65]; 
    let char_list = vec!['y', 'm', 'a', 'q']; 

    let result = largest_i32(&number_list); 
    println!("The largest number is {result}"); 

    let result = largest_char(&char_list); 
    println!("The largest char is {result}"); 

    let result = largest(&number_list); 
    println!("The largest number is {result}"); 
    let result = largest(&char_list); 
    println!("The largest char is {result}"); 

    let integer = Point { x: 5, y: 10 }; 
    println!("p.x= {}", integer.x()); 
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point {x: 5, y: 4.0 }; 

    let integer_and_float = PointTwo {x: 5, y: 4.0}; 

    let p1 = PointTwo { x: 5, y: 10.4 }; 
    let p2 = PointTwo { x: "Hello", y: 'c' }; 
    let p3 = p1.mixup(p2); 
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); 

}
