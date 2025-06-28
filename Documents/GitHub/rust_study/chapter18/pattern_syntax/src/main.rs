struct Point { 
    x: i32, 
    y: i32, 
}

enum Message { 
    Quit, 
    Move { x: i32, y: i32 }, 
    Write(String), 
    ChangeColour(i32, i32, i32), 
}

enum MessageHello {
    Hello { id: i32 },  
}

fn main() {
    // matching literals example: 
    let x = 1; 

    match x {
        1 => println!("one"), 
        2 => println!("two"), 
        3 => println!("three"), 
        _ => println!("anything"), 
    }

    // matching named variables example: 
    let x = Some(5); 
    let y = 10; 

    match x {
        Some(50) => println!("Got 50"), 
        // new shadow variable y binds inside a Some(5) 
        Some(y) => println!("Matched, y = {y}"), 
        _ => println!("Default case, x = {x:?}"), 
    } // shadow y = 5 gets out of scope 

    println!("at the end: x = {x:?}, y = {y}"); // y = 10 

    // multiple patterns example: 
    let x = 1; 
    
    match x {
        // | is the pattern OR operator 
        1 | 2 => println!("one or two"), 
        3 => println!("three"), 
        _ => println!("anything"), 
    }

    // matching ranges of values with ..= example: 
    // but only allowed with numeric or char values (compiler check) 
    let x = 5; 
    
    match x {
        // ..= syntax match to an inclusive range of vals 
        1..=5 => println!("one through five"), // 1 to 5 
        _=> println!("something else"), 
    }

    // destrucing structs example: 
    let p = Point { x: 0, y: 7 }; 
    let Point {x: a, y: b } = p; 
        // x and y break apart using a pttern with a let 
        // equivalent to let Point { x, y } = p; 
    assert_eq!(0, a); 
    assert_eq!(7, b);  
    
    // destructing mad matching literal values example: 
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"), // if y = 0 
        Point { x: 0, y } => println!("On the y axis at {y}"), // if x = 0 
        Point { x, y } => println!("On neither axis: ({x}, {y})"), 
    }

    // destructing enums example: 
    let msg = Message::ChangeColour(0, 160, 255); 

    match msg {
        // match with patterns that destructure each inner value 
        Message::Quit => { 
            println!("The Quit variant has no data to destructure."); 
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}"); 
        }
        Message::Write(text) => {
            println!("Text message: {text}"); 
        }
        Message::ChangeColour(r, g, b) => {
            println!("Change the colour to red {r}, green {g}, and blue {b}") 
        }
    }

    // destructuring structs and tuples example: 
    let ((feet, inches), Point { x , y }) = ((3, 10), Point { x: 3, y: -10 }); 
    
    /* 
        ignoring values in a pattern: 
    */
    // ignoring an entire value with _ example: 
    // need a certain type sig but the function body is not implemented yet 
    fn foo(_: i32, y: i32) { // _ as a wildcard pattern 
        println!("This code only uses the y parameter: {y}"); 
    }
    foo(3, 4); // 3 is not bind to any var 

    // ignoring parts of a value with a nested _ example: 
    let mut setting_val = Some(5); 
    let new_setting_val = Some(10); 

    match (setting_val, new_setting_val) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite an existing customized value"); 
        }
        _ => {
            setting_val = new_setting_val; 
        }
    }

    // ignoring remaining parts of a value with .. 
    // avoiding the need to list underscores for each ignored val 
    let origin = Point { x: 0, y: 0 }; 

    match origin {
        Point { x, .. } => println!("x is {x}"), 
        // using .. must be unambiguous and clear 
    } 

    // match guard example: an additional if condition, 
    // specified after the pattern in a match arm 
    let num = Some(4); 

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"), 
        Some(x) => println!("The number {x} is odd"), 
        None => (), 
    }

    // match guard to test with an outer var example: 
    let x = Some(5); 
    let y = 10; 

    match x {
        Some(50) => println!("Got 50"), 
        // y is the outer y rather than a new shadowed y 
        Some(n) if n == y => println!("Matched, n = {n}"), 
        _ => println!("Default case, x = {x:?}"), 
    } // n gets out of scope beyond this point 

    // @ bindings example: crete a var that holds a val,
    // at the same time testing it for a pattern match 
    let msg = MessageHello::Hello { id: 5 }; 

    match msg { 
        // test a val and save it in a variable within one pattern ``
        MessageHello::Hello { id: id_var @ 3..=7,  
        } => println!("Found an id in range: {id_var}"), 
        MessageHello::Hello { id: 10..=12 } => 
            println!("Found an id in another range"), 
        MessageHello::Hello { id } => println!("Found some other id: {id}"), 
    }
}
