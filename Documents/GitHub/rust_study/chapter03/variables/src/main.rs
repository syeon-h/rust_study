fn main() {
    let x = 5; 
    println!("The value of x is: {x}"); 
    // x = 6; will not work as rust variables are immutable by default 

    /*
        constants have few differences compare to immutable variables: 
            1. cannot use mut, thus, they are always immutable 
            2. the types of the value must be annoated on the declaration 
            3. can be declared in any scope, including the global 
            4. may be set only to a constant expression, 
                not the result that could only be computed at runtime 
    */  

    const THREE_HOURS_IN_SECS: u32 = 60 * 60 * 3; 

    let mut mutx = 5; 
    println!("The value of x is: {mutx}"); 
    mutx = 6; 
    println!("The value of x is: {mutx}"); 

    // shadowing: the sceond var overshadows the first, taking any uses until either
    //      it itself is shadowed or the scope ends  
    let x = x + 1; 
    {
        let x = x * 2; 
        // The value of x in the inner scope is (5 + 1) * 2 = 12  
        println!("The value of x in the inner scope is: {x}");
    }
    // The value of x is (5 + 1) = 6 
    println!("The value of x is: {x}"); 

    // shadowing can change the type of the value but reuse the same name 
    // let mut spaces = "    "; and spaces = spaces.len(); will get a compile-time error
    let spaces = "    "; 
    let spaces = spaces.len(); 
    println!("The number of spaces is: {spaces}"); 

}
