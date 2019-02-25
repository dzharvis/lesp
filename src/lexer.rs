use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
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
        let current_char = if i >= input.len() {
            None
        } else {
            Some(&input[i..i+1])
        };
        match &current_state.clone() {
            ParsingState::Init => {
                match current_char {
                    Some(" ") => (),
                    Some("(") => {tokens.push(Tokens::OP);},
                    Some(")") => {tokens.push(Tokens::CP);},
                    Some(ch) if numbers.is_match(&ch) => {current_state = ParsingState::Number(ch.parse::<u32>().unwrap());},
                    Some(ch) => {current_state = ParsingState::Symbol(String::from(ch));},
                    None => return tokens
                };
                i+=1;
            },
            ParsingState::Number(num) => {
                match current_char {
                    Some(ch) if !numbers.is_match(&ch) => {
                        tokens.push(Tokens::Number(num.clone()));
                        current_state = ParsingState::Init;
                    },
                    Some(ch) => {
                        let current_number = ch.parse::<u32>().unwrap();
                        current_state = ParsingState::Number(num*10 + current_number);
                        i+=1;
                    },
                    None => {
                        tokens.push(Tokens::Number(num.clone()));
                        return tokens;
                    }
                };
            },
            ParsingState::Symbol(sym) => {
                 match current_char {
                    Some(ch) if !letters_and_numbers.is_match(&ch) => {
                        tokens.push(Tokens::Symbol(sym.clone()));
                        current_state = ParsingState::Init;
                    },
                    Some(ch) => {
                        let mut new_string = sym.clone();
                        new_string.push_str(ch);
                        current_state = ParsingState::Symbol(new_string);
                        i+=1;
                    },
                    None => {
                        tokens.push(Tokens::Symbol(sym.clone()));
                        return tokens;
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsm() {
        assert_eq!(parse_fsm(&String::from("1")), vec![Tokens::Number(1)]);
        assert_eq!(parse_fsm(&String::from("2")), vec![Tokens::Number(2)]);
        assert_eq!(parse_fsm(&String::from("(+ 1 2)")), vec![Tokens::OP, Tokens::Symbol(String::from("+")), Tokens::Number(1), Tokens::Number(2), Tokens::CP]);
    }
}