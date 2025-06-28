/*
    string slice is a reference to part of a String 
        let you reference a contiguous sequence of elements in a collection 
        it DOES NOT have ownership 

    &str is a slice pointing to that specific point of the binary 
        string literal is also &str (immutable) 
*/

fn main() {
    let mut s = String::from("hello world"); 
    let word = first_word(&s); // immutable borrow occurs 
    // s.clear(); will occur an error as mutable borrow occurs 
    // Recall: if we have an immutable ref, we cannot also take a mutable ref 
    println!("The first word is: {word}");

    // improved version: same fn on both types 
    let str = "hello world";
    let word1 = first_word_improve(&s); 
    let word2 = first_word_improve(&str); 
    println!("The first word in String is: {word1}"); 
    println!("The first word in liter is: {word2}"); 

    // more general slice type: &[i32] array 
    let a = [1, 2, 3, 4, 5]; 
    let slice = &a[1..3]; 
    assert_eq!(slice, &[2, 3]); 
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; 
        }
    }

    &s[..] 
}

// following will not catch the error: word isn't connected to the state of s 
// it is only meaning ful in the context of the &String (no guarantee) 
fn first_word_u(s: &String) -> usize {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; 
        }
    }

    s.len()
}

// improved: to use the same function on both String and literal 
fn first_word_improve(s: &str) -> &str {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; 
        }
    }

    &s[..] 
}