use std::io::prelude::*;
use std::io::{Result, stdin};

#[derive(Debug, Clone, PartialEq)]
pub struct Client(String, String, i32);

impl Client {
    fn new(name: String, ip: String, port: i32) -> Self {
        Client(name, ip, port)
    }

    fn register(&self) -> Result<()> {
        let user_input = stdin();
        let mut user = String::new();

        println!("Please register yourself. Type '|' to escape.");
        println!("Please enter your username:");

        user.clear();
        user_input.read_line(&mut user).expect("failed to readline");

        Ok(())
    }
}
