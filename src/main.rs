fn main() {
    let pears_in_basket: i32 = 5;
    println!("Number of pears: {}", pears_in_basket);
    let oranges_in_basket: i32 = 3;
    println!("Number of oranges: {}", oranges_in_basket);
    let fruits_in_basket: i32 = pears_in_basket + oranges_in_basket;
    println!("Total number of fruits: {}", fruits_in_basket);
    println!("Oranges {} pears: {}", oranges_in_basket, pears_in_basket);
    //You can pass variables to println! using positional arguments directly.
    println!("Oranges {oranges_in_basket} pears: {pears_in_basket}");
    //You can use numeric positional arguments to specify the order of variables in the output.
    println!("Oranges {1} pears: {0}", pears_in_basket, oranges_in_basket);
    //You can also use named arguments to specify the order of variables in the output.
    println!("Oranges {oranges} pears: {pears}", oranges = oranges_in_basket, pears = pears_in_basket);

    //you can use underscores in variables to avoid warning when they are not used
    let _unused_variable = 10; 

    //Inmmutable and mutable variables
    let immutable_variable = 20;      
    let mut mutable_variable = 30;
    println!("Initial mutable variable: {}", mutable_variable);
    mutable_variable = 40; // This is allowed
    println!("Immutable variable: {}", immutable_variable);
    println!("Mutable variable: {}", mutable_variable);
    //immutable_variable = 50; // This will cause a compile-time error because immutable_variable is not mutable
}
