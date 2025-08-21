use lotus_ime::input::{Input};

fn main() {
    let inputs = vec![
        "viet65",
        "nam"
    ];

    let config = Input::new();

    println!("{}", config.return_text()); // prints "việt nam "
}
