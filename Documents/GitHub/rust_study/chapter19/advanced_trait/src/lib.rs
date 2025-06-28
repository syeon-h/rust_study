/*
    associated type specifies placeholder types in trait definitions 
        trait method definitions can use these types in their signatures; 
        thus, can define a trait that uses some types without needing to 
        know exactly what those are until the trait is implemented 

        Ex. associated type Item stands in for the type of the values 
        that the type implementing the Iterator trait is iterating over; 
        implementors will specify the concrete type for Item 

        might seem like a similar to generics, but the diff is that when 
        using generics, we must annotate the types in each implementation 
        (ex. multiple implementations of Iterator for Counter) 

        however, with associated types, do not need to annotate types 
        becasue we cannot implement a trait on a type multiple times; can 
        only choose what the type of Item will be once, thus, do not have 
        to specify that we want an iterator of u32 vals everywhere  
        (ex. only one impl Iterator for Counter)  

    default generic type parameter eliminates the need for implementors 
        of the trait to specify a concrete type if the default type works; 
        specify with the <PlaceholderType=ConcreteType> syntax  

        useful with operator overloading, which to customize the behaviour 
        of an operator that listed in std::ops by implementing the traits 

        Ex. overload the + operator to add two Point instances together: 

*/

// definition of the Iterator trait
pub trait Iterator {
    // associated type (placeholder) 
    type Item; 

    // return values of type Option<Self::Item> 
    fn next(&mut self) -> Option<Self::Item>; 
}

// implementation that specifies the Item type is u32: 
impl Iterator for Counter {
    type Item = u32; 

    fn next(&mut self) -> Option<Self::Item> {
        // --snip-- 
    }
}

/*
    hypothetical definition using generics: 
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>; 
    }
*/