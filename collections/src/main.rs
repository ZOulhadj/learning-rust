use std::io;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(20);
    v1.push(30);
    v1.push(40);
    v1.push(50);

    let v2 = vec![1, 2, 3];

    let value = &v1[2];

    for i in &mut v1 {
        *i += 100;

        println!("{i}");
    }
}
