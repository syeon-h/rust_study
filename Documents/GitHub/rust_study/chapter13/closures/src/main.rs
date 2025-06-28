/*
    closure is anonymous fn that saves in a var or pass args to other fns 
        can create the closure in one place and then call it elsewhere 
        to evaluate it in a different context 
        can capture values from the scope in which they are defined 

*/

/*
    closures can capture values from the environment 
        they are defined in for later use 

    Ex. t-shirt company gives away a shirts to someone on the mailing list 
        and people can optionally add their favourtie colour 

    passed a closure that calls self.most_stocked() on the current 
        Inventory instance, thus, standard lib did not need to know 
        anything about the Inventory or ShirtColour types, or the logic 
    
    closure capteres an immutable ref to the self Inventory instance 
        and passes it with the code we spcify to the method 
        which function are not able to capture 
*/

#[derive(Debug, PartialEq, Copy, Clone)] 
// type of colours available 
enum ShirtColour {
    Red, 
    Blue, 
}

// company's inventory 
struct Inventory {
    // shirt colour currently in stock 
    shirts: Vec<ShirtColour>, 
}

impl Inventory {
    // gets the optional colour preference and return one 
    fn giveaway(&self, user_pref: Option<ShirtColour>) -> ShirtColour {
        // closure that takes no params itself 
        // and the body calls self.most_stocked() 
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0; 
        let mut num_blue = 0; 

        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1, 
                ShirtColour::Blue => num_blue += 1, 
            }
        }
        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue 
        }
    }
}

#[derive(Debug)] 
struct Rectangle {
    width: u32, 
    height: u32, 
}

use std::thread; 
use std::time::Duration; 

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue], 
    }; 

    let user_pref1 = Some(ShirtColour::Red); 
    let giveaway1 = store.giveaway(user_pref1); 
    println!(
        "The user with preference {:?} gets {:?}", 
        user_pref1, giveaway1
    ); 

    let user_pref2 = None; 
    let giveaway2 = store.giveaway(user_pref2); 
    println!(
        "The user with preference {:?} gets {:?}", 
        user_pref2, giveaway2
    ); 

    /*
        closures do not usually require you to annoate the type 
            of the params or the return like fn functions do because 
            closures are typically short and relevant only within 
            a narrow context, so the compiler can infer the types 
        
        can add type annotations to increase explicitness and clarify 
        Ex. all valid definitions that will produce the same behaviour: 
            fn add_one_v1 (x: u32) -> u32 { x + 1 } // function definition 
            let add_one_v2 = |x: u32| -> u32 { x + 1 }; // fully annotated 
            let add_one_v3 = |x| { x + 1 }; // no annotations 
            let add_one_v4 = |x| x + 1; // brackets are optional 

        for closure defs, the compuler will infer one concrete type for 
            each of their params and for their return val 
        Ex. for let example_clousre = |x| x; will not allow: 
            let s = example_clousre(String::from("hello")); 
            let n = example_clousre(5); 
    */

    // adding optional type annotations of the param and return val: 
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly..."); 
        thread::sleep(Duration::from_secs(2)); 
        num
    }; 

    /*
        closures can capture vals from their environment in three ways: 
            borrowing immutably, borrowing mutably, and taking ownership 
    */

    // defining and calling a clousre that captures an immutable ref 
    let list = vec![1, 2, 3]; 
    println!("Before defining clolusre: {list:?}"); 

    let only_borrwos = || println!("From closure: {list:?}"); 

    println!("Before calling closure: {list:?}"); 
    only_borrwos(); 
    println!("After calling closure: {list:?}"); 

    // using move to force the closure to take ownership: 
    // useful when passing a closure to a new thread to move 
    // the data so that it is owned by the new thread 
    let list = vec![1, 2, 3]; 
    println!("Before defining clolusre: {list:?}"); 

    thread::spawn(move || println!("From thread: {list:?}")) 
        .join()
        .unwrap(); 

    /*
        closure body can do: move a captured val out of the closure, 
            mutate the captured value, neither move nor mutate the value, 
            capture nothing from the environment to begin with 
        
        the way a closure caputures and handles values from env affects 
            which Fn traits the closure implements: 
        1. FnOnce applies to closures that can be called once 
            all closures implement at least this trait 
        2. FnMut applies to that do not move captured values out 
            but that might mutate the captured values 
        3. Fn applies to that do not move captured values out 
            and that do not mutate captured values 

        impl<T> Option<T> {
            pub fn unwrap_or_else<F>(self, f: F) -> T 
            // f is the closure we privde when calling 
            where 
                F: FnOnce() -> T 
            {
                match self {
                    some(x) => x, 
                    None => f(), 
                }
            }
        }
    */

    // sort_by_key uses FnMut: calls multiple times 
    // once for each item in the slice 
    let mut list = [
        Rectangle { width: 10, height: 1 }, 
        Rectangle { width: 3, height: 5 }, 
        Rectangle { width: 7, height: 12 }, 
    ]; 

    // closure does not capture, mutate, or move out 
    list.sort_by_key(|r| r.width); 
    println!("{list:#?}"); 

    /*
        Ex. attempt to use FnOnce with sort_by_key: 
            count the number of times sort_by_key calls the closure 
            
        let mut list = [
            Rectangle { width: 10, height: 1 }, 
            Rectangle { width: 3, height: 5 }, 
            Rectangle { width: 7, height: 12 }, 
        ]; 

        let mut sort_operations = vec![]; 
        let value = String::from("closure called"); 

        list.sort_by_key(|r| {
            // closure captures value then moves value out  
            // by transferring ownsership to the sort_operations 
            // thus, this closure can be called once 
            // and only implements FnOnce 
            sort_operations.push(value); 
            r.width
        }); 
        println!("{list:#?}"); 

    */
}

