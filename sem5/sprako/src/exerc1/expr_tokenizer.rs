use std::io::{self, Read};

use lexer_rs::{ExprLexer, Token};

pub fn tokenizer() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lexer = ExprLexer::new(&input);
    let tokens = lexer.get_all_tokens();

    println!("# token stream without hidden channel tokens:");
    for t in &tokens {
        if t.channel() == Token::HIDDEN_CHANNEL {
            continue;
        }

        let name = lexer
            .vocabulary()
            .symbolic_name(t.token_type())
            .unwrap_or("UNKNOWN");

        print!(r#"{}("{}") "#, name, t.text());
    }
    println!();

    println!("# token stream including hidden channel tokens:");
    for t in &tokens {
        let name = lexer
            .vocabulary()
            .symbolic_name(t.token_type())
            .unwrap_or("UNKNOWN");

        print!(r#"[]("{}") "#, name, replace_whitespace(t.text()));
    }
    println!();

    Ok(())
}

fn replace_whitespace(s: &str) -> String {
    s.replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
