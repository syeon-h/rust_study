// adding useful functionality with DERIVED TRAITS: 
// have to explicitly opt in (add the outer attribute) 
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

// defining an area method on the struct: 
impl Rectangle {
    // Self is an alias for the type that the impl is for 
    fn area(&self) -> u32 {
        self.width * self.height 
    }
    
    // can the same name as one of the fields 
    fn width(&self) -> bool {
        self.width > 0 
    }
    
    // associated function: function defined with an impl 
    // often for constructors that will return a new instance 
    fn square(size: u32) -> Self {
        Self {
            width: size, 
            height: size, 
        }
    }
}

// each struct is allowed to have multiple impl blocks 
impl Rectangle {
    // methods with more parameters: 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    } 
}

fn main() {
    // case#1: specified by separate variables 
    let width1 = 30; 
    let height1 = 50; 

    println!(
        "The area of the rectangle is {} square pixels.", 
        area(width1, height1)
    ); 

    // case#2: refactoring with tuples 
    let rect1 = (30, 50); 
    println!(
        "The area of the rectangle is {} square pixels.", 
        area_tup(rect1)
    ); 

    // case#3: refactoring with structs (meaning) 
    let rect2 = Rectangle {
        width: 30, 
        height: 50, 
    }; 
    println!(
        "The area of the rectangle is {} square pixels.", 
        area_struct(&rect2)
    ); 

    /*
        Rust does not try to guess what we want 
            println!("rect2 is {rect2}"); will not work 
            error[E0277]: `Rectangle` doesn't implement `std::fmt::Display` 
        
        putting the specifier :? tells println! 
            we want to use an output format called Debug 
        
        Debug trait enables us to print our struct in a way 
            that is useful for developers (values) 

        putting the specifier :#? will output the whole struct 

        dbg! mactro takes ownership (as opposed to println!) 
            prints the file + line number of where it occurs 
            along with the resultant value of that expressions, 
            and returns ownership of the value 
    */
    println!("rect2 is {rect2:?}"); 
    dbg!(&rect2); 

    let scale = 2; 
    let rect3 = Rectangle {
        width: dbg!(30 * scale), // [main.rs:59:16] 30 * scale = 60 
        height: 50, 
    }; 

    // using method syntax of Rectangle 
    println!(
        "The area of the rectangle is {} square pixels.", 
        rect2.area()
    ); 

    if rect2.width() {
        println!("The rectangle has a nonzero width: {}", rect2.width);  
    }

    let rect3 = Rectangle {
        width: 10, 
        height: 40, 
    }; 

    // to take another instance of Rectangle: 
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3)); 

    // to call the associated function: 
    let sq = Rectangle::square(3); 
}

// has two parameters; not clear that the parameters are related 
fn area(width: u32, height: u32) -> u32 {
    width * height 
}

// add a bit of structure, but tuples do not name their elements 
fn area_tup(dimensions: (u32, u32)) -> u32 {
    // might mixing up the width and height 
    dimensions.0 * dimensions.1
}

// fn is now defined with one parameter 
// (immutable borrow so main retains its ownership) 
fn area_struct(rectangle: &Rectangle) -> u32 {
    // conveys that the width and height are related to each other 
    // also gives descriptive names to the values 
    rectangle.width * rectangle.height 
}