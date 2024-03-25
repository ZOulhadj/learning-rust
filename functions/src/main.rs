fn function_that_returns() -> u32 {
    500
}

fn another_function() {
    println!("This is from another function");
}

fn main() {
    println!("Hello, world!");

    another_function();
    let value = function_that_returns();

    println!("Here is a value {value}");
}
