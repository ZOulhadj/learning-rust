// ~(const std::string my_string)~
fn take_ownership(my_string: String) -> String {
    my_string
}

// const std::string& my_string
fn use_reference(my_string: &String) -> u32 {
    println!("Using reference: {my_string}");

    32
}

// std::string& my_string
fn ref_and_modify(my_string: &mut String) {
    my_string.push_str(" ref and modified");
}

fn main() {
    let mut s = String::from("Hello, World!");

    s.push_str(" And appended!");

    println!("My string {s}");



    let s1 = String::from("My name is Zak");
    let mut s2 = s1.clone();

    s2.push_str(" and I am...");

    println!("s1 {s1}");
    println!("s2 {s2}");

    let mut s3 = String::from("Taking ownership");

    s3 = take_ownership(s3);
    println!("Value of s3 {s3}");

    let returned_val = use_reference(&s3);
    println!("Returned value {returned_val}");

    ref_and_modify(&mut s3);

    println!("Value of s3 {s3}");
}
