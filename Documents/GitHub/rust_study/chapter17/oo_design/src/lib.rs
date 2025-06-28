/*
    state pattern is an object-oriented design pattern
        define a set of states a value can have internally; 
        the states are represented by a set of state objects, and 
        the value's behaviour changed based on its state 

        pros is that, when the business requirements change, 
        does not need to change the code of the value holding the state 
        or the code that uses the val; update the code inside one of the 
        state objects to change its rules or add more state objects 

    state objects share functionality: each is responsible for its own 
        behaviour and for governing when it should change into another  

    Ex. blog post struct that has a field to hold its state  
        state object from the set draft, review, or published 

    Rust is capable of implementing the o-o state pattern to encapsulate 
        the different kinds of behaivour a post should have in each state 

        if we were to create an alternative implementation, might instead 
        use match expressions in the methods on Post or even in the main; 
        but that would have to look in several places to understand the 
        implications of a post being in the published state 

        state pattern is easy to extend to add more functionality; one 
        downside is that, because the state implement the trasitions, 
        some of the states are coupled to each other (ex. if another state
        between Pending and Published is added, have to change the code 
        in Pending to transition to the new state instead) 

        another downside is that it has duplicated some logic; 
        to eliminate, make default implementations for the request_review 
        and approve methods on the State trait that return self but 
        this would violate object safety, because the trait does not know 
        what the concrete self will be exactly 
*/

// public struct that holds some content 
pub struct Post {
    // hold the state object 
    state: Option<Box<dyn State>>, 
    content: String, 
}

impl Post {
    pub fn new() -> Post {
        Post {
            // new instance of the Draft struct 
            state: Some(Box::new(Draft {})), 
            content: String::new(), 
        }
    }

    // to add text to a post's content 
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text); 
    }

    // return the conent val depending on the state 
    pub fn content(&self) -> &str {
        // want a ref to the val rather than ownership 
        // call content on the &Box<dyn State> 
        self.state.as_ref().unwrap().content(self) 
    }

    pub fn request_review(&mut self) {
        // take method to take the Some vallue out of the State 
        // and leave a None in its place (cannot unpopulate) 
        if let Some(s) = self.state.take() {
            // call an internal request_review on the current state 
            self.state = Some(s.request_review()) 
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve()) 
        }
    }
}

// trait that defines the behaviour 
// objects are Draft, Pending, Published 
trait State {
    // all types need to implement request_reivew 
    // self: Box<Self> takes ownership; invaliding the old state 
    fn request_review(self: Box<Self>) -> Box<dyn State>; 

    // will set state to the val that the current state 
    // says it should have when the state is approved 
    fn approve(self: Box<Self>) -> Box<dyn State>; 

    // default implementation for the current method 
    // thus, do not need to implement on the Draft and Pending 
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// 
struct Draft {} 

impl State for Draft {
    // returns a new, boxed instance of the Pending  
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending {})
    }

    // have no effect on Draft 
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
} 

struct Pending {} 

impl State for Pending {
    // returns itself; because already in Pending (review) 
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    // returns a new, boxed instance of the Published 
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {} 

impl State for Published {
    // returns itself, because the state should not change 
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    // override the content method and return the val 
    // taking ref to a post and returning a ref to part of the post 
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}