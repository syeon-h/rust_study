use std::hash::Hash;

fn main() {
    /*
        Vec<T> stores more than one value in a single data structure 
            puts all the values next to each other in memory 
            can only store values of the same type 
            unlike tuple, can grow in size (extendable)    

        the borrow checker enforces the ownserhip and borrowing rules 
            recall: cannot have mutable and immutable ref in the same scope
            because vectors put the values next to each other in memory, 
            adding a new element might require allocating new memory 

        use enum to store multiple types 
            Ex. to get values from a row in a spreadshieet 
        
        like any other struct, a vector is freed when it goes out of scope 
    */
    // creating a new vector: 
    let v: Vec<i32> = Vec::new();    // empty vector 
    let v = vec![1, 2, 3]; // with initial values 

    // updating a vector: 
    let mut v = Vec::new(); 
    v.push(1); 
    v.push(2);
    v.push(3);  

    // reading elements of vectors: 
    let third: &i32 = &v[2];                    // indexing 
    println!("The third element is {third}"); 
    let third: Option<&i32> = v.get(2);  // get method 
    match third {
        Some(third) => println!("The third element is {third}"), 
        None => println!("There is no third element"),  
    }
    // attempting to access the element out of index: 
    let does_not_exist = &v[100];                    // program crashes 
    let does_not_exist = v.get(100);   // returns None 

    // iterating over the values in a vector: 
    for i in &v {
        println!("{i}"); 
    }
    
    // iterating over mutable ref to elements: 
    let mut v = vec![100, 32, 57]; 
    for i in &mut v {
        *i += 50; // use the * dereference to change the value 
    }

    /*
        String is implemented as a collection of bytes (standard lib) 
            growable, mutable, owned, UTF-8 encoded string type 
            recall: str is the string slice type (core language) 
        
        can include any properly UTF-8 encoded data in them 
            much broader range compare to ASCII 

        the + operator uses the add method, whose signautre looks 
            fn add(self, s: &str) -> String {
            s2 has an & meaning we are adding a ref to the first 
            because we can only add a &str to a String 
            and add takes ownership of self (s1) 
        
        format! mactro uses ref so the call does not take ownership 

        accessing parts of a String using indexing will occur an error 
            becuase it is a wrapper over a Vec<u8> 
            index into the string's btyes will not always correlate 
            to a valid Unicode scalar value 
            Ex. each value in Здравствуйте in UTF-8 takes two bytes 
        

    */
    // creating a new string: 
    let mut s = String::new();                // empty string 
    let s = String::from("initial contents"); // from a string literal 

    // updating a string: 
    let mut s = String::from("foo"); 
    s.push_str("bar"); // takes a string slice 

    let mut s = String::from("lo"); 
    s.push('l'); // takes a single character 

    // concatenation with the + operator: 
    let s1 = String::from("Hello"); 
    let s2 = String::from("world"); 
    let s3 = s1 + ", " + &s2; // recall: s1 has been removed here 

    // concatenation with the format! macro: 
    let s1 = String::from("hello"); 
    let s2 = String::from("world"); 
    let s3 = format!("{s1}, {s2}"); // format! returns a String 

    // slicing strings: 
    let hello = "Здравствуйте"; 
    let s = &hello[0..4]; // contains the first four bytes 
    println!("{s}");            // Зд as each takes two bytes 
    // &hello[0..1] slices only part of a character and not work 

    // iterating over strings: chars method 
    for c in "Зд".chars() { 
        println!("{c}"); // print two chars 
    }

    // iterating over strings: bytes method 
    for b in "Зд".bytes() {
        println!("{b}"); // print four bytes 
    }

    /*
        HashMap<K, V> stores a mapping of keys to values (hashing function) 
            useful when looking up data not by using an index but a key 
            Ex. keep track of each team's score in a game 
            it is not included in the prelude and have less support 
        
        for types that implement the Copy trait (i32), the values are copied 
            for owned values like String, the values will be moved 
        
        HashMap uses a SipHash function by default 
            that can provide resistance to DoS attacks involving hash tables 
            this is not the fastest hashing algorithm but better for the securty 
    */
    use std::collections::HashMap; 
    
    // creating a new hash map: 
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Yellow"), 50); 

    // accessing values in a hash map: get method 
    // copied to get an Option<i32> rather than an Option<&i32> 
    // unwrap_or to set score to zero if no entry for the key 
    let team_name = String::from("Blue"); 
    let score = scores.get(&team_name).copied().unwrap_or(0); 

    // interate over each key-value pair: 
    for (key, value) in &scores {
        println!("{key}:{value}"); 
    }

    // updating a hash map: overwrting a value 
    scores.insert(String::from("Blue"), 25); 
    
    // updating a hash map: if a key is not present 
    // entry method takes the key to be checked as a parameter 
    // and return an enum of a value that might or might not exist 
    // or_insert method returns a mutable ref to the value if key exists 
    // if not, it inserts the parameter as the new value and returns a mut ref 
    scores.entry(String::from("Blue")).or_insert(50); // exist 
    scores.entry(String::from("Red")).or_insert(50);  // not exist 

    // updating a value based on the old value: 
    let text = "hello world wonderful world"; 
    let mut map = HashMap::new(); 
    
    for word in text.split_whitespace() {
        // recall: or_insert returns a mutable ref (&mut V) 
        let count = map.entry(word).or_insert(0); 
        *count += 1; // asterisk(*) to dereference count 
    }
}
