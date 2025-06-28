/*
    1. the compiler first looks in the crate root file (src/lib.rs or src/main.rs) 
    2. in the crate root file, you can declare new modules with mod 
    3. in any file, other than the crate root, you can declare submodules 
    4. can refer to code in the module from anywhere else using the path 
    5. code within a module is private by default 
        to make a module public, declare it with pub mod 
    6. within a scope, the use keyword creates shortcuts to items 
*/

use crate::garden::vegetables::Asparagus; 

pub mod garden; 

fn main() {
    let plant = Asparagus {}; 
    println!("I'm growing {plant:?}!"); 
}
