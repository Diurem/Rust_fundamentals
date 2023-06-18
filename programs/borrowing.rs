fn main() {
    let mut value = String::from("Hello");
    let reference = &value;

    println!("Reference: {}", reference);

    value.push_str(", World!");
    println!("Modified value: {}", value);
}
