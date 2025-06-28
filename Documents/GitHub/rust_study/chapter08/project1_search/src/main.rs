use std::collections::HashMap; 
use rand::Rng; 

fn main() {
    let mut rng = rand::thread_rng(); 
    let array:[i32; 20] = rand::random(); 
    let vector = array.to_vec(); 

    println!("{:?}", vector); 
    println!("The median is: {}", median(&vector)); 
    println!("The mode is: {}", mode(&vector)); 
}

fn median(v: &Vec<i32>) -> i32 {
    let mut list = v.to_owned(); 
    let med = list.len()/2;  

    list.sort(); 
    return list[med]; 
}

fn mode(v: &Vec<i32>) -> i32 {
    let list = v.to_owned(); 
    let mut storage = HashMap::new(); 

    for i in list {
        let count = storage.entry(i).or_insert(0); 
        *count += 1; 
    }

    let mut mode = -1; 
    let mut count = -1;  

    for (k, v) in storage {
        if v > count {
            count = v; 
            mode = k; 
        }
    }

    return mode; 
}