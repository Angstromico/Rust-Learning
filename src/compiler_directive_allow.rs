#![allow(unused_variables)] // This directive allows unused variables in this file.
#![allow(dead_code)]
pub fn run() {
    let x: i32 = 5;
    let y: i32 = 10; // This variable is unused, but the compiler will not warn about it due to the directive above.
    //no_sum()
}

pub fn no_sum()  {
    let z: i32 = 15; // This variable is also unused, but the directive applies here as well.
}