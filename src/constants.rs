pub fn run() {
    const MAX_POINTS: u32 = 100_000; // Constants are always immutable and must have a type annotation.
    println!("Maximum points: {}", MAX_POINTS);

    //Different with unmutable variables, constants are not allowed to be shadowed.
    //const MAX_POINTS: u32 = 200_000; // This would cause a compile-time error.

    //The scope of a constant is the entire crate, so it can be accessed from anywhere in the crate.
}