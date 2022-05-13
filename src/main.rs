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

#[derive(Debug)]
struct TokenCount {
    text: String,
    detail: Vec<String>,
    count: u32,
}

impl TokenCount {
    fn new(token: &Token) -> TokenCount {
        TokenCount {
            text: token.text.to_string(),
            detail: token.detail.clone(),
            count: 1,
        }
    }
}

fn main() -> LinderaResult<()> {
    let tokenizer = Tokenizer::new()?;
    let contents = get_raw_contents();
    let tokens = tokenizer.tokenize(contents.as_str())?;
    let ignore_words: Vec<&str> = vec!["的", "レベル", "こと", "毎"];

    // let mut sorted_word_count = words_count(&tokens);
    // sorted_word_count.sort_by(|a, b| b.count.cmp(&a.count));
    // println!("{:?}", sorted_word_count);

    let mut sorted_token_count = tokens_count(&tokens);
    sorted_token_count.sort_by(|a, b| b.count.cmp(&a.count));
    let mut frequent_noun: Vec<String> = Vec::new();
    let mut count_nouns = 3;
    for token_count in sorted_token_count.iter() {
        println!("{:?}", token_count);
        if token_count.detail[0] == "名詞"
            && count_nouns > 0
            && !ignore_words.contains(&token_count.text.as_str())
        {
            frequent_noun.push(token_count.text.clone());
            count_nouns -= 1;
        }
    }

    for token in tokens {
        match token.detail[0].as_str() {
            "接頭詞" => print!("{}", token.text.blue()),
            "助詞" => print!("{}", token.text.blue()),
            "名詞" => {
                if frequent_noun.contains(&token.text.to_string()) {
                    if token.text == frequent_noun[0] {
                        print!("{}", token.text.bright_yellow());
                    } else if token.text == frequent_noun[1] {
                        print!("{}", token.text.bright_red());
                    } else {
                        print!("{}", token.text.bright_green());
                    }
                } else {
                    print!("{}", token.text.white())
                }
            }
            "動詞" => print!("{}", token.text.white()),
            //"助動詞" => print!("{}", token.text.cyan()),
            //"連体詞" => print!("{}", token.text.green()),
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

fn tokens_count(tokens: &Vec<Token>) -> Vec<TokenCount> {
    let mut token_counts: Vec<TokenCount> = Vec::new();
    for token in tokens {
        match token_counts.iter_mut().find(|e| {
            e.text == TokenCount::new(&token).text && e.detail == TokenCount::new(&token).detail
        }) {
            Some(x) => {
                x.count += 1;
            }
            None => {
                token_counts.push(TokenCount::new(token));
            }
        }
    }
    return token_counts;
}
