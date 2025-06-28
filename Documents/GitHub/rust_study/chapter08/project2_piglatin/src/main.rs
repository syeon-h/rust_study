use std::io; 

fn main() {
    let word1 = String::from("first"); 
    let word2 = String::from("apple"); 

    let latin1 = pig_latin(&word1); 
    let latin2 = pig_latin(&word2); 

    println!("Convert result of {word1} is: {latin1}"); 
    println!("Convert result of {word2} is: {latin2}"); 

    println!("Enter a word to convert: ");
    let mut input = String::new(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); 

    let output = pig_latin(&input); 

    println!("Convert result of {input} is: {output}"); 
}

fn pig_latin(str: &str) -> String {
    let vowel = ["a", "e", "i", "o", "u"]; 
    let first = &str[0..1];

    if vowel.contains(&first) {
        let new_word = String::from(str); 
        return new_word + "-hay" 
    } else {
        let new_word = String::from(&str[1..]); 
        return new_word + "-" + first + "ay" 
    }
}
