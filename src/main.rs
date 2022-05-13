use colored::*;
use lindera::tokenizer::Token;
use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;
use std::fmt::*;
use std::fs::*;
use std::io::*;
use std::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WordCount {
    text: String,
    count: u32,
}

// struct TokenWrap<'a> {
//     token: Token<'a>,
// }

// impl<'a> TokenWrap<'a> {
//     fn new(token: &Token<'a>) -> TokenWrap<'a> {
//         TokenWrap { token: *token }
//     }
// }

// impl Debug for TokenWrap<'_> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         f.debug_struct("TokenWrap")
//             .field(("text"), &self.token.text)
//             .field("detail", &self.token.detail)
//             .finish()
//     }
// }

#[derive(Debug)]
struct TokenCount<'a> {
    token: TokenWrap<'a>,
    count: u32,
}

// impl PartialEq for TokenWrap<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.token.text == other.token.text && self.token.detail == other.token.detail
//     }
// }
// impl Eq for TokenWrap<'_> {}

fn main() -> LinderaResult<()> {
    let tokenizer = Tokenizer::new()?;
    let contents = get_raw_contents();

    let tokens = tokenizer.tokenize(contents.as_str())?;
    let mut sorted_word_count = words_count(&tokens);
    sorted_word_count.sort_by(|a, b| b.count.cmp(&a.count));
    println!("{:?}", sorted_word_count);

    let mut sorted_token_count = tokens_count(&tokens);
    sorted_token_count.sort_by(|a, b| b.count.cmp(&a.count));
    println!("{:?}", sorted_token_count);

    for token in tokens {
        match token.detail[0].as_str() {
            "接頭詞" => print!("{}", token.text.red()),
            "助詞" => print!("{}", token.text.blue()),
            "名詞" => print!("{}", token.text.yellow()),
            "動詞" => print!("{}", token.text.white()),
            "助動詞" => print!("{}", token.text.cyan()),
            "連体詞" => print!("{}", token.text.green()),
            _ => print!("{}", token.text.blue()),
        }
    }
    Ok(())
}

fn get_raw_contents() -> String {
    let args = Cli::from_args();
    let mut contents = String::new();
    match args.path {
        Some(file_path) => {
            let mut f = File::open(file_path).expect("file not found");
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");
        }
        None => {
            stdin()
                .read_line(&mut contents)
                .expect("Failed to read line.");
        }
    }
    contents
}

fn words_count(tokens: &Vec<Token<'_>>) -> Vec<WordCount> {
    let mut word_counts: Vec<WordCount> = Vec::new();
    for token in tokens {
        match word_counts.iter_mut().find(|e| e.text == token.text) {
            Some(x) => {
                x.count += 1;
            }
            None => {
                word_counts.push(WordCount {
                    text: token.text.to_string(),
                    count: 0,
                });
            }
        }
    }
    return word_counts;
}

fn tokens_count(tokens: &Vec<Token<'static>>) -> Vec<TokenCount<'static>> {
    let mut token_counts: Vec<TokenCount<'static>> = Vec::new();
    for token in tokens {
        match token_counts
            .iter_mut()
            .find(|e| e.token == TokenWrap::new(&token))
        {
            Some(x) => {
                x.count += 1;
            }
            None => {
                token_counts.push(TokenCount {
                    token: TokenWrap::new(&token),
                    count: 0,
                });
            }
        }
    }
    return token_counts;
}
