/*
    interior mutability is a design pattern that allows to mutate 
        data even when there are immutable refs to that data, which
        normally disallowed by the borrowing rules; thus, the pattern 
        uses unsafe code to mutate data to bend the rules, indicates 
        to the compiler that we are checking the rules manually 

        can use only when we can ensure that the borrowing rules 
        will be followed at runtume, even though the compilter 
        cannot guarantee that 

    RefCell<T> type follows the interior mutability pattern 
        represents single ownership over the data it holds; 
        unlike Box<T>, borrowing rules are enforced at runtime, 
        if RefCell<T> break these rules, program will panic! 

        useful when you are sure that the code follows the rules 
        but the compiler is unable to guarantee that; 
        like Rc<T>, only for use in single-threaded scenarios 

    recall: reasons to choose Box<T>, Rc<T>, or RefCell<T> 
        1. Rc<T> enables multiple owners of the same data 
        2. Box<T> allows immut or mut borrows checked at compile; 
            Rc<T> allows only immut borrows checked at compile; 
            RefCell<T> allows immut or mut borrows checked at runtime 
        3. can mutate the val inside the RefCell<T> even when immut 
        
    mutating the val inside an immut is the interior mutability pattern 
        mock objects are specific types of test doulbes that record 
        what happens during a test to assert the correct actions 

    Ex. library that tracks a val against a max and sends messages 
        based on how close to the maximum val the current val it: 
        need a mock object that, instead of sending an email or text 
        when we call send, will only keep track of the messages 

        attempt fails because we cannot modify the MockMessenger 
        as the send method takes an immut ref to self; also cannot 
        use &mut self instead, because then the signature of send 
        would not match the signature in the Messenger trait 

        solution is to store the sent_messages within a RefCell<T> 
        call borrow_mut on the RefCell<Vec<String>> to get a mut ref 
        to the value inside it; then can call push on the mut ref 
        to the vector to keep track of the messages sent 

    with RefCell<T>, use the borrow and borrow_mut methods (safe API) 
        borrow returns the Ref<T>, and borrow_mut returns the RefMut<T>; 
        RefCell<T> keeps track of how many Ref<T> and RefMut<T> are 
        currently active and lets us have many immutable borrows or 
        one mut borrow at any point in time (compile-time borrowing) 
*/

// lib to keep track of how close a val is to a max 
pub trait Messenger {
    // takes immut ref to self and the text 
    fn send(&self, msg: &str); 
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, 
    value: usize, 
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where 
    T: Messenger, 
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker{
            messenger, 
            value: 0, 
            max, 
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value; 

        let percentage = self.value as f64 / self.max as f64; 

        if percentage >= 1.0 {
            self.messenger.send("Error: You are over your quota!"); 
        } else if percentage >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!"); 
        } else if percentage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!"); 
        }
    }
}

// mock object that only keep track of the messages: 
#[cfg(test)] 
mod test {
    use super::*; 
    use std::cell::RefCell; 

    struct MockMessenger {
        // sent_messages: Vec<String>, 
        sent_messages: RefCell<Vec<String>>,  
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![], 
                sent_messages: RefCell::new(vec![]), 
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message)); 
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test] 
    fn it_sends_an_over_75_percent() {
        let mock_messenger = MockMessenger::new(); 
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100); 

        limit_tracker.set_value(80); 

        // assert_eq!(mock_messenger.sent_messages.len(), 1); 
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); 
    }
}