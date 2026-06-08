use std::io::Result;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub async fn main_menu() -> Result<MenuChoice> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        println!("1. Register");
        println!("2. Exit");
        println!("Enter choice (1 or 2): ");

        let mut input = String::new();

        reader
            .read_line(&mut input)
            .await
            .expect("failed to readline");

        match input.trim() {
            "1" => return Ok(MenuChoice::Register),
            "2" => return Ok(MenuChoice::Exit),
            _ => println!("Unknown input. Please try again"),
        }
    }
}

pub async fn connected_menu() -> Result<ClientCommand> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        println!("1. List all users");
        println!("2. Send broadcast message");
        println!("3. Initiate direct chat (handshake)");
        println!("4. Logout and disconnect");
        println!("Enter a number (1-4): ");

        let mut input = String::new();

        reader
            .read_line(&mut input)
            .await
            .expect("failed to readline");

        match input.trim() {
            "1" => return Ok(ClientCommand::ListUsers),
            "2" => return Ok(ClientCommand::BroadcastMessage),
            "3" => return Ok(ClientCommand::InitiateChat),
            "4" => return Ok(ClientCommand::Logout),
            _ => println!("Invalid command. Please try again."),
        }
    }
}

pub async fn read_broadcast_message() -> Result<String> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        println!("Enter your message (or '|' to abort): ");

        let mut msg = String::new();

        reader
            .read_line(&mut msg)
            .await
            .expect("failed to readline");

        let msg = msg.trim().to_string();

        if msg == "|" {
            return Ok(String::new());
        } else if msg.is_empty() {
            println!("Message cannot be empty. Please try again.");
        } else {
            return Ok(msg);
        }
    }
}

pub async fn read_target_username() -> Result<String> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        println!("Enter target user's username (or '|' to abort): ");

        let mut username = String::new();

        reader
            .read_line(&mut username)
            .await
            .expect("failed to readline");

        let username = username.trim().to_string();

        if username == "|" {
            return Ok(String::new());
        } else if username.is_empty() {
            println!("Username cannot be empty. Please try again.");
        } else {
            return Ok(username);
        }
    }
}

pub async fn read_direct_message() -> Result<String> {
    let mut reader = BufReader::new(io::stdin());

    loop {
        println!("Enter your message (or '|' to end chat): ");

        let mut msg = String::new();

        reader
            .read_line(&mut msg)
            .await
            .expect("failed to readline");

        let msg = msg.trim().to_string();

        if msg.is_empty() {
            println!("Message cannot be empty. Please try again.");
        } else {
            return Ok(msg);
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuChoice {
    Register,
    Exit,
}

#[derive(Debug, Clone)]
pub enum ClientCommand {
    ListUsers,
    BroadcastMessage,
    InitiateChat,
    Logout,
}

pub fn get_cmd_desc(cmd: &ClientCommand) -> String {
    match cmd {
        ClientCommand::ListUsers => String::from("Listing all users..."),
        ClientCommand::BroadcastMessage => String::from("Sending broadcast message..."),
        ClientCommand::InitiateChat => String::from("Initiating direct chat..."),
        ClientCommand::Logout => String::from("Logging out..."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cmd_desc() {
        assert_eq!(
            get_cmd_desc(&ClientCommand::ListUsers),
            "Listing all users..."
        );
        assert_eq!(
            get_cmd_desc(&ClientCommand::BroadcastMessage),
            "Sending broadcast message..."
        );
        assert_eq!(
            get_cmd_desc(&ClientCommand::InitiateChat),
            "Initiating direct chat..."
        );
        assert_eq!(get_cmd_desc(&ClientCommand::Logout), "Logging out...");
    }

    #[test]
    fn test_menu_choice_debug() {
        assert_eq!(format!("{:?}", MenuChoice::Register), "Register");
    }

    #[test]
    fn test_client_command_debug() {
        assert_eq!(
            format!("{:?}", ClientCommand::BroadcastMessage),
            "BroadcastMessage"
        );
    }
}
