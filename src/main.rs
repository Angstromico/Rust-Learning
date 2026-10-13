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
}
