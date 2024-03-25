fn main() {

    let number = 30;
    let min_number = 20;

    if number < min_number {
        println!("Lower that min");
    } else {
        println!("Higher that min");
    }

    println!("Hello, world!");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

}
