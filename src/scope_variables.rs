pub fn run() {
    let a = 10;
    
    {
        let b = 20;
        println!("Inside inner scope: a = {}, b = {}", a, b);
        let a = 30; // Is not possible to shadow a variable in different scopes, but we can shadow it in the same scope.
        println!("Inside inner scope after shadowing: a = {}", a);
    }
    println!("Outside inner scope: a = {}", a);
}