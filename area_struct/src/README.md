* This program can be done in so many ways, but using structs gives it proper clarity and more meaning.
* `println!("rect1 is {:?}", rect1);` putting the `:?` specifier inside the bracket tells `println!` that we are using an output formatter called `Debug`.
* The `Debug` trait enables us to print the struct in such a way that it's helpful for the developers to see it's value while debugging the code. 
