pub fn run() {
    let immutable_variable = 20;
    let mut mutable_variable = 30;
    println!("Initial mutable variable: {}", mutable_variable);
    mutable_variable = 40; // This is allowed.
    println!("Immutable variable: {}", immutable_variable);
    println!("Mutable variable: {}", mutable_variable);
    // immutable_variable = 50; // This causes a compile-time error because immutable_variable is not mutable.
}
