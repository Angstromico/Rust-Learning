pub fn run() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1; // Shadowing the previous variable x
    println!("The value of x after shadowing is: {}", x);
    {
        let x = x * 2; // Shadowing again in a new scope
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x after the inner scope is: {}", x);

    //Change to string with shadowing
    let x = "10"; // Shadowing the previous variable x with a string
    println!("The value of x as a string is: {}", x);
}