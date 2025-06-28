/*
    inheritance is a machanism whereby an object can inherit elements 
        from another, thus gaining the prarent object's data and behaviour 

        1. reuse of code: can implement particular behaviour for one type 
            and inheritance enables to reuse for a different type 
        2. polymorphism: to enable a child type to be used in the same 
            places as the parent type; can substitute multiple objects for
            each other at runtime if they share certain characteristics 
        
        Rust does not have inheritance, however, takes the different 
        approach of using trait objects instead 

        has recently fallen out of favour as a programming design solution 
        because it's often at risk of sharing more code than necessary 

    trait object points to both instance of a type implementing the 
        specified trait and a table used to look up trait methods 
        
        create by specifying some sort of pointer then the dyn keyword

        Rust's type system will ensure at compile time that any value 
        used in that context will implement the trait object's trait 

        recall: Rust refrain from calling structs and enums objects to 
        distinguish them from other languages' ojbects: the data in the 
        struct fields and the behaviour in impl blocks are separated 

    Ex. graphical user interface (GUI) tool that iterates through a list
        of items, calling a draw method on each: this crate might include 
        Button or TextField but cannot know and define all the types other 
        programmers might want to create at the time of writing 

        Screen struct could have defined the Screen struct using a generic 
        tpye T and a trait bound: this restricts us to a Screen instance 
        that has a list of components all of one type (all Button type); 
        if you will only ever have homogeneous collections, is preferable 



*/

// definition of the Draw trait 
pub trait Draw {
    fn draw(&self); 
}

// definition of the screen sturct 
pub struct Screen { 
    // holding a vector of trait objects 
    pub components: Vec<Box<dyn Draw>> 
}

impl Screen {
    // calls the draw method on each component 
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); 
        }
    }
}

// button struct that implements the draw trait 
pub struct Button {
    pub width: u32, 
    pub height: u32, 
    pub label: String, 
}

impl Draw for Button {
    fn draw(&self) {
        // will use diffeernt code to define
        // how to draw the particualr type 
    }
}