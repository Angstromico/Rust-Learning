pub fn run() {
 //Variable types can be inferred by the compiler, but we can also specify them explicitly.
    let constant_variable: i32 = 100;
   let inferred_variable = 200; // The compiler infers the type as i32.
    println!("Constant variable: {}", constant_variable);
    println!("Inferred variable: {}", inferred_variable);

    //We can also use type aliases to create a new name for an existing type.
    type Kilometers = i32;
    let distance: Kilometers = 5;
    println!("Distance in kilometers: {}", distance);
}