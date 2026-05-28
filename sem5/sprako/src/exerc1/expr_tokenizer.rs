use std::fs;
use std::io::{self, stdin};

use antlr4rust::{char_stream, lexer};

pub fn tokenizer() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    const FILE_PATH: &str = "./abfahrt-kn.txt";
    let fs = fs::read_to_string(FILE_PATH).expect("couldn't read file");
    let lexer = lexer::BaseLexer::new_base_lexer(char_stream::CharStream::get_text(fs), LexerRecog::);

    //stdin().read_to_string(&mut input)?;

    //let mut lexer = ExprLexer::new(&input);

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

        print!(r#"{}("{}") "#, name, replace_whitespace(t.text()));
    }
    println!();

    Ok(())
}

fn replace_whitespace(s: &str) -> String {
    s.replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
