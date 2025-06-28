/*
    fn type is a function pointer which allows to use functions as args 
        to other functions (not to be confused with the Fn closure trait) 
        
        Ex. do_twice takes two params: function pointer to any fn that 
        takes an i32 param and returns an i32, and one i32 val 

        unlike closures, fn is a type rather than a trait, thus specify 
        fn as the param type directly (not generic type param) 

        implements all three of the closure triatsm, thus, can always pass
        a function pointer as an arg for a function that expects a closure 
        (best to write using a generic type and one of the closure traits) 

        one example to only accept fn and not closures is when interfacing 
        with external code that does not have closures: C functions 
        (ex. to use either a closure defined inline or a named function) 

    closures cannot be return directly as they are represented by traits; 
        becasue closures do not have a concrete type that is returnable 
        and not allowed to use the function pointer fn as a return type 

        Ex. fn returns_closure() -> dyn Fn(i32) -> i32 { |x| x + 1 } will 
        not compile because it tries to return a closure directly 

        error refs the Sized trait, Rust does not know how much space it 
        will need to store the closure; solution is to use a trait object 



*/

fn add_one(x: i32) -> i32 {
    x + 1
} 

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // calls the function f twice, passing arg 
    // then adds the two function call results 
    f(arg) + f(arg) 
}

// use a trait object to return a closure: 
// fn returns_closure() -> dyn Fn(i32) -> i32 { 
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1) 
}

fn main() {
    let answer = do_twice(add_one, 5); 
    println!("The answer is: {answer}"); // 12 

    // turn a vec tof nums into a vec of strings 
    // using a closure: 
    let list_of_nums = vec![1, 2, 3]; 
    let list_of_strings: Vec<String> = 
        list_of_nums.iter().map(|i| i.to_string()).collect(); 

    // using a function as the arg to map: 
    let list_of_nums = vec![1, 2, 3]; 
    let list_of_strings: Vec<String> = 
        // must use the fully qualified syntax 
        list_of_nums.iter().map(ToString::to_string).collect(); 
}
