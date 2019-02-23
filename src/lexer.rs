use regex::Regex;

#[derive(Debug, Clone)]
pub enum Tokens {
    Symbol(String),
    OP,
    CP,
    Number(u32)
}

#[derive(Debug, Clone)]
enum ParsingState {
    Init, Number(u32), Symbol(String)
}

pub fn parse_fsm(input: &String) -> Vec<Tokens> {
    let mut current_state = ParsingState::Init;
    let mut i = 0;
    let mut tokens = vec![];
    let numbers = Regex::new(r"[0-9]").unwrap();
    let letters_and_numbers = Regex::new(r"[a-zA-Z0-9]").unwrap();
    loop {
        if i >= input.len() {
            return tokens;
        }
        let current_char = &input[i..i+1];
        match &current_state.clone() {
            ParsingState::Init => {
                if current_char == " " {
                } else if current_char == "(" {
                    tokens.push(Tokens::OP);
                } else if current_char == ")" {
                    tokens.push(Tokens::CP);
                } else if numbers.is_match(&current_char) {
                    current_state = ParsingState::Number(current_char.parse::<u32>().unwrap());
                } else {
                    current_state = ParsingState::Symbol(String::from(current_char));
                }
                i+=1;
            },
            ParsingState::Number(s) => {
                if !numbers.is_match(&current_char) {
                    tokens.push(Tokens::Number(s.clone()));
                    current_state = ParsingState::Init;
                } else {
                    let ii = current_char.parse::<u32>().unwrap();
                    current_state = ParsingState::Number(s*10 + ii);
                    i+=1;
                }
            },
            ParsingState::Symbol(s) => {
                if !letters_and_numbers.is_match(current_char) {
                    tokens.push(Tokens::Symbol(s.clone()));
                    current_state = ParsingState::Init;
                } else {
                    let mut new_string = s.clone();
                    new_string.push_str(current_char);
                    current_state = ParsingState::Symbol(new_string);
                    i+=1;
                }
            }
        }
    }
}