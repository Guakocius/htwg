pub fn main_menu() {
    println!("Type \t r for register \t l for login");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");

    match input.trim() {
        "l" => {}
        "r" => {}
        _ => {
            println!("Unknown input. Please try again");
            main_menu()
        }
    }
}
