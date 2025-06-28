/*
    iterator perform some task on a sequence of items in turn 
        responsible for (1) the logic of iterating over each 
        and (2) determining when the sequence has finished 

    in rust, iterators are lazy, meaning they have no affect until 
        methods that consume the iterator are called 

    map takes a closure to call on each item and returns 
        a new iterator that produces the modified items 

    collect consumes the iterator and collects the resulting vals 
        into a collection data type 
*/

fn main() {
    // creating an iterator: 
    // iterator is stored in the v1_iter var 
    let v1 = vec![1, 2, 3]; 
    let v1_iter = v1.iter(); 

    // separate the iterator from the use of it: 
    for val in v1_iter { 
        println!("Got: {val}"); 
    }

    // methods that produce other iterator (map, collect)  
    let v1: Vec<i32> = vec![1, 2, 3]; 
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); 
    assert_eq!(v2, vec![2, 3, 4]); 
    
}
