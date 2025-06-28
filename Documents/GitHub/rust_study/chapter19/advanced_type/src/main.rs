/*
    newtype pattern abstract away some implementation details of a type: 
        the new type can expose a public API that is different from the 
        API of the private inner type 

        Ex. Millimetres and Metres structs wrapped u32 vals in a newtype; 
        if a function has a param of Millimetres, it cannot be compiled 
        with a val of type Metres or a plain u32 

        can also hide internal implementation and be a lightweight way to 
        achieve encapsulation to hide implmentation details 

        Ex. People that wraps a HashMap<i32, String> stroing a person's ID
        associated with their name; code using People would only interact 
        with the public API provided (add a name string) and would not 
        need to know that it gets assigned an i32 ID to names internally 

    type alias gives an another name to existing type (type synonyms) 
        Ex. type Kilometres = i32; creates the alias Kilometres to i32, 
        and this Kilometres is not a separate, new type and treated the 
        same as values of type i32 

        however, type aliases do not get the type checking benefits 
        Ex. if Kilometres and i32 get miexd up somewhere, the compiler 
        will not give an error 

        main use case is to reduce repetition for a lengthy type 
        Ex. type Thunk = Box<dyn Fn() + Send + 'static>; can replace all 
        uses of the type with the shorter alias Thunk 

        commonly used with the Result<T, E> type for reducing repetition:
        std::io has a std::io::Error struct that represents all possible 
        I/O errors; many of the functions in std::io will be returning 
        Result<T, E> where the E is std::io::Error 
        Ex. type Result<T> = std::result::Result<T, std::io::Error>;  

    never type ! is an empty type that has no value; stands in the place
        of the return type when a fn will never return 
        Ex. fn bar() -> ! {} is read as the fn bar returns never 

        functions that return never are called diverging functions 

        Ex. recall that match arms must all return the same type: 
            let guess = match guess.trim().parse() {
                Ok(num) => num, 
                Err(_) => continue, // continue has a ! value 
            }; 

        useful with the panic! macro: panic! does not produce a value; 
        it ends program (ex. None => panic!("`None`` value")) 

        one final expression that has the type ! is a loop: the loop never
        ends, so ! is the value of the expression 

    dynamically sized (unsized) type allows to write code using values 
        whose size that can be known only at runtime 

        str is a DST, cannot know how long the string is until runtime 
        meaning a var of type str cannot be created, nor taken as an arg 
        Ex. let s1: str = "Hello there!"; and 
            let s2: str = "How's it going"; will not work 

        Rust needs to know how much memory to allocate, so make the types
        of s1 and s2 a &str rather than a str; slice data structure just
        stores the starting position and the length: &str is two vals 

        the golden rule of DSTs is that we must always put vals of DSTs 
        behind a pointer of some kind; can combine str with all kinds of 
        pointers such as Box<str> or Rc<str> 

    Sized trait is provided by Rust to determine whether or not a type's 
        size is known at compile automatically implemented for everything 
        whose size is known at compile time 

        Rust implicitly adds a bound on Sized to every generic function 
        Ex. fn generic<T>(t: T) {} is acutally treated as 
            fn generic<T: Sized>(t: T) {} 
        
        can use the special syntax ?Sized to relax the restriction: 
        Ex. fn generic<T: ?Sized>(t: &T) {} means T may or may not be 
        Sized and this notation overrides the default to be a known size 




*/

// type alias declaration for the std::io 
type Result<T> = std::result::Result<T, std::io::Error>; 

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>; 
    // fn flush(&mut self) -> Result<(), Error>; 
    fn flush(&mut self) -> Result<()>; 
}

fn main() {
    // type alias to i32:  
    type Kilometres = i32; 

    let x: i32 = 5; 
    let y: Kilometres = 5; 

    // can add Kilometres and i32 types 
    println!("x + y = {}", x + y); 

    // alias Thunk for the verbose type: 
    type Thunk = Box<dyn Fn() + Send + 'static>; 
    let f: Thunk = Box::new(|| println!("hi")); 
    
}
