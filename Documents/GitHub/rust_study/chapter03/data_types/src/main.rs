fn main() {
    /*
        scalar type represents a single value: 
            1. integer is a number without a fraction component (i or u)
            2. floating-point numbers are numbers with decimal points (f) 
            3. Booleans have two possible values, true or false (bool) 
            4. character is the language's most primitive alphabetic type (char)
    */

    let x = 2.0; // f64 (double precision) - default 
    let y: f32 = 3.0; // f32 (single precision) 

    let sim = 5 + 10; 
    let diff = 95.5 - 4.3; 
    let prod = 4 * 30; 
    let quot = 56.7 / 32.2; 
    let trun = -5 / 3; 
    let rem = 43 % 5; 

    let t = true; 
    let f: bool = false; 

    let c = 'z'; 
    let z: char = 'ℤ'; // char represents a Unicode Scalar Value (more than ASCII) 
    let heart_eyed_cat = '😻'; 

    /*
        compound types can group multiple values into one type: 
            1. tuple() groups together values with a variety of types; fixed length 
            2. array[] must have the same type; have a fixed length 
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    // can use pattern matching to destructure values 
    let (x, y, z) = tup; 
    println!("The value of y is: {y}"); 
    // can access an element directly by using a period (.) 
    let x = (500, 6.4, 1); 
    let five_hundred = x.0; 
    let six_point_four = x.1; 
    let one = x.2; 

    // arrays are useful when you want your data allocated on the "stack" 
    let a = [1, 2, 3, 4, 5]; 
    // access elements using indexing 
    let first = a[0]; 
    let second = a[1]; 
    // can initialize an array to contain the same value for each 
    let a = [3; 5]; // a = [3, 3, 3, 3, 3]; 

}
