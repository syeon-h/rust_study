use add_one; 

fn main() {
    let num = 10; 
    // using the add_one library crate 
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num)); 
}

/*
    specify the package in the workspace to run the binary crate 
        from the add directory by using -p argument 
        Ex. cargo run -p adder 
*/