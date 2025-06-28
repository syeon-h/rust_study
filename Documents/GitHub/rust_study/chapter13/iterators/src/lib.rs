/*
    all iterators implement an Iterator trait and only requires 
        implementors to define the next method, which returns 
        one item at a time wrapped in Some or None 
        pub trait Iterator {
            type Item; 
            
            fn next(&mut self) -> Option<Self::Item>; 
        }

    calling the next method charges internal state that the 
        iterator uses to keep track of where it is in the sequence; 
        thus, this code consumes (uses up) the iterator 
        (vals got from the calls are immutable references) 
        methods that call next are called consuming apaters 

    sum method takes ownershup of the iterator and iterates 
        through the items by repeatedly calling next 
*/

#[test] 
fn iterator_demo() {
    let v1 = vec![1, 2, 3]; 
    // mutable because it is calling the next   
    let mut v1_iter = v1.iter(); 
    
    assert_eq!(v1_iter.next(), Some(&1)); 
    assert_eq!(v1_iter.next(), Some(&2)); 
    assert_eq!(v1_iter.next(), Some(&3)); 
    assert_eq!(v1_iter.next(), None); 
}

// methods that consume the iterator (sum) 
#[test] 
fn iterator_sum() {
    let v1 = vec![1, 2, 3]; 
    let v1_iter = v1.iter(); 
    let total: i32 = v1_iter.sum(); 
    // not allowed to use v1_iter from here  
    // as sum takes ownership of the iterator 

    assert_eq!(total, 6); 
}

// using closures that capture their environment 
#[derive(PartialEq, Debug)] 
struct Shoe {
    size: u32, 
    style: String, 
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // inter_iter to create an iterator that takes ownership 
    // filter returns a vec containing only of the shoe_size 
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() 
}

#[cfg(test)] 
mod tests {
    use super::*; 

    #[test] 
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10, 
                style: String::from("sneaker"), 
            }, 
            Shoe {
                size: 13, 
                style: String::from("sandal"), 
            }, 
            Shoe {
                size: 10, 
                style: String::from("boot"), 
            }, 
        ]; 

        let in_my_size = shoes_in_size(shoes, 10); 

        assert_eq!(
            in_my_size, 
            vec![
                Shoe {
                    size: 10, 
                    style: String::from("sneaker"), 
                }, 
                Shoe {
                    size: 10, 
                    style: String::from("boot"), 
                }, 
            ]
        ); 
    }
}