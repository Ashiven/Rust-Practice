fn main() {
    // declarative macros with macro_rules!
    // these compare values to patterns similarly to the match expression
    // instead of values source code is passed to the macro and matched against patterns
    // and gets replaced with the code associated with the matched pattern

    let v: Vec<u32> = vec![1, 2, 3];

    #[macro_export] // make this macro available when the crate is brought into scope
    macro_rules! vec {  // defining the macro and giving it the name vec
        ( $( $x:expr ), *) => {  // here we match against a pattern and emit a block of code if we have a match
            // $() is used to declare a variable containing the Rust code matching the pattern
            // within this variable we have $x:expr which matches any expression and gives it the name $x
            // the comma after the variable containing the expression signifies that a comma can come after it
            // for example, here $( $x:expr ) would match 1 and with the comma we would match 1,
            // the star signifies that we can match this 0 to n times
            // so we would match  1,  and then  2,  and then  3  for the entire thing preceding *
            // while $x would match the values 1 2 and 3
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x); // everything within $()* is generated per match for $x (1 2 and 3)
                )*
                temp_vec
            }
        }
    }
    // the code that would end up being generated is the following
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    }

    // procedural macros
    // this type of macro acts more similarly to a function
    // they accept some code as input, operate on it, and produce some code as output
    // rather than matching matching against patterns and replacing the code
    // the procedural macros kinds are: derive, attribute-like, and function-like
}
