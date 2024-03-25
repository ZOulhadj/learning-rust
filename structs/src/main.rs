struct User {
    first: bool,
    name: String,
    age: u32
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    println!("Hello, world!");

    let user = User {
        first: true,
        name: String::from("Zakariya Oulhadj!"),
        age: 22
    };

    println!("User name is {}", user.name);


    let user_1 = User {
        first: false,
        ..user
    };

    let rect1 = Rectangle {
        width: 10,
        height: 10
    };

    println!("The area of rectangle is: {} square pixels", rect1.area());
    println!("{:#?}", rect1);
}
