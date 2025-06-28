/*
    macro refers to a family of features in Rust: 
        (1) declarative macros with macro_rules! (ex. print!)  
        (2)three kinds of procedural macros 

    macros are writing code that writes other code (metaprogramming) 
        print! and vec! macros expand to produce more code than the code 
        that actually written; useful for reducing the amount of code and 
        have some additional powers that functions do not 

        macros can take a variable number of params unlike a function sig
        that must declare the number and type of params the function has; 
        downside is that macro definitions are more complex 

        another diff is that macros must be defined or brought into scope 
        BEFORE they are getting called (can define and call fns anywhere) 

    declartive macro is also reffered to as macro_rules! macro, allows to 
        write something similar to a match expression; compare a val to 
        patterns that are asscoiated with particular code:  
            1. val is the literal Rust source code passed to the macro 
            2. patterns are compared with the structure of the source code 
            3. code associated with each pattern, when matched, replaces 

        uses macro_rules! construct to define a macro: #[macro_export] 
        annotation indicates that this macro should be available whenever 
        the crate in which the macro is defined is brought into scope 

        Ex. with vec![1, 2, 3];, the $x pattern matches three times with 
        the three expressions 1, 2, and 3; which generates 
            let mut temp_vec= Vec::new(); 
            temp_vec.push(1); 
            temp_vec.push(2); 
            temp_vec.push(3); 
            temp_vec

    procedural macro acts more like a function (is a type of procedure); 
        accepts some code as an input, operate on that code, and procedure 
        some code as an output rather than matching againt patterns 

        three kinds are: (1) custom derive (2) attribute-like and 
        (3) function-like, and all work in a similar fashion 

        when creating, the definitions must reside in their own crate with 
        a special crate type 

        ex. to define where some_attribute is a placeholder for using a 
        specific macro variety: 
            use proc_macro; 
            #[some_attribute] 
            pub fn some_name(input: TokenStream) -> TokenStream {} 
        TokenStream is defined by the proc_macro crate that is included 
        with Rust; this is the core of the macro: the source code that the 
        macro is operating on makes up the input TokenStream, and the code
        the macro produces is the output Toekn Stream 
        
        can have multiple kinds of procedural macros in the same crate 

*/

// simplified definition of the vec! macro: 
#[macro_export] 
// vec is the name of the macro (without !) 
macro_rules! vec { // body of the definition starts 
    // dollar sign to delcare a macro variable 
    // within $() is the pattern to be matched ($x:expr) 
    // with any Rust expression and gives the name $x 
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec=new(); 
            $(
                // generated for each part that matches 
                temp_vec.push($x); // $x is replaced with each 
            )* 
            temp_vec
        }
    }; 
}
