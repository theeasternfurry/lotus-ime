fn main() {
    let inputs = vec![
        "viet65",
        "nam"
    ];

    let mut result = String::new();
    for input in inputs {
        vi::transform_buffer(&vi::VNI, input.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "việt nam "
}
