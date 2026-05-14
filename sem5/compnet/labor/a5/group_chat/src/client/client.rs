use std::io::{Result, Error, ErrorKind, stdin};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub username: String,
    pub ip: String,
    pub port: String,
}

impl Client {
    pub fn new() -> Self {
        //Client(name, ip, port)
        Option::expect(Self::register().0, "Registering failed. Please try again")
    }

    fn register() -> (Option<Self>, Result<()>) {
        let user_input = stdin();
        let mut username = String::new();
        let mut ip = String::new();
        let mut port = String::new();

        println!("Please register yourself. Type '|' to escape.");

        ["username", "IP address", "UDP port"]
            .into_iter()
            .zip([&mut username, &mut ip, &mut port])
            .fuse()
            .for_each(|(k, mut v)| {
                println!("Please enter your {}:", k);
                user_input.read_line(&mut v).expect("failed to readline");
            });
        println!("{}", port);
        if !Regex::new(r"[a-zA-Z0-9_-]{3,20}")
            .unwrap()
            .is_match(&username)
        {
            println!(
                "Username must follow this RegEx convention: [a-zA-Z0-9_-]{{3,20}}"
            );
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-1"))
            )
        } else if !Regex::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b")
            .unwrap()
            .is_match(&ip) {
                println!("IP address must follow this RegEx convention:
                    \\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){{3}}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\b");
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-2")) 
            )
        } else if port.trim().parse::<u32>().unwrap() < 1 && port.trim().parse::<u32>().unwrap() > 65535 {
                println!("UDP port number must follow this RegEx convention:
                    (6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{{2}}|6[0-4][0-9]{{3}}|[1-5][0-9]{{4}}|[0-9]{{1,4}})");
         
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-3")) 
            )
        } else {
            (
                Some(Client {
                username: username,
                ip: ip,
                port: port
            }),
                Ok(())
            )
        }
    }
}
