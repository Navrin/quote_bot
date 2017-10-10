use std::fmt;
use std::collections::HashMap;

enum Token {
    Name(String),
    // Assignment,
    Value(String),
}

impl Token {
    // fn assign(self, val: &str) -> Token {
    //     match self {
    //         Token::Name(_) => Token::Name(val.to_string()),
    //         Token::Assignment => Token::Assignment,
    //         Token::Value(_) => Token::Value(val.to_string()),
    //     }
    // }

    fn inner(&self) -> String {
        match self {
            &Token::Name(ref v) => v.to_string(),
            // &Token::Assignment => String::new(),
            &Token::Value(ref v) => v.to_string(),
        }
    }

    fn push_char(&self, ch: char) -> Token {
        match self {
            &Token::Name(ref v) => Token::Name(format!("{}{}", v, ch)),
            // &Token::Assignment => Token::Assignment,
            &Token::Value(ref v) => Token::Value(format!("{}{}", v, ch)),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &Token::Name(ref v) => format!("__{}__ = value", v),
                // &Token::Assignment => "name __=__ value".to_string(),
                &Token::Value(ref v) => format!("name = __{}__", v),
            }
        )
    }
}

// /// Get whatever token is valid for the next word
// fn valid_tokens(token: &Token) -> Token {
//     match token {
//         &Token::Name(_) => Token::Assignment,
//         &Token::Assignment => Token::Value("value here".to_string()),
//         &Token::Value(_) => Token::Name("name here".to_string()),
//     }
// }

/// Transforms a line of text into a vector of tokens, to be converted into a hashmap.
fn walk_line(line: String) -> Result<Vec<Token>, String> {
    if line.len() <= 0 {
        return Ok(vec![]);
    }

    let mut tokens = vec![];
    let mut cur_token = Token::Name(String::new());
    let mut in_quotes = false;
    let mut quoted_char: Option<char> = None;


    for ch in line.chars() {
        if in_quotes && quoted_char.unwrap() == ch {
            in_quotes = false;
            quoted_char = None;

            continue;
        }
        if in_quotes {
            cur_token = cur_token.push_char(ch);
            continue;
        }

        if !in_quotes && (ch == '"' || ch == '\'') {
            in_quotes = true;
            quoted_char = Some(ch);
            continue;
        }


        if ch == '=' || ch == ':' {
            tokens.push(cur_token);
            cur_token = Token::Value(String::new());
            continue;
        }

        match cur_token {
            Token::Value(_) if ch == ' ' || ch == ',' => {
                if ch == ' ' && cur_token.inner().len() <= 0 {
                    continue;
                }

                tokens.push(cur_token);
                cur_token = Token::Name(String::new());
                continue;
            }
            _ => (),
        }

        cur_token = cur_token.push_char(ch);
    }

    tokens.push(cur_token);

    Ok(tokens)
}

/// Turns a `key = value` set in a string message into a HashMap<Key, Value>`
pub fn get_values(query: &str) -> Result<HashMap<String, String>, String> {
    let mut values = HashMap::new();
    let tokens = walk_line(query.to_string())?;
    let tokens = tokens.chunks(2);

    for pair in tokens {
        let key = match pair.get(0) {
            Some(v) => v,
            None => return Err("Key missing".to_string()),
        };

        let value = match pair.get(1) {
            Some(v) => v,
            None => return Err("Value missing".to_string()),
        };

        values.insert(
            key.inner().trim().to_string(),
            value.inner().trim().to_string(),
        );
    }

    Ok(values)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_pairs_correctly() {
        let ex = "hello = world abc = def";

        let map = get_values(ex).unwrap();

        assert_eq!(map["hello"], "world");
        assert_eq!(map["abc"], "def");
    }

    #[test]
    fn it_doesnt_care_about_whitespace() {
        let ex = "hello=world,abc=def";
        let map = get_values(ex).unwrap();

        assert_eq!(map["hello"], "world");
        assert_eq!(map["abc"], "def");
    }

    #[test]
    fn it_likes_quoted_things() {
        let ex = r#"hello="world this" abc='is a test"'"#;
        let map = get_values(ex).unwrap();

        assert_eq!(map["hello"], "world this");
        assert_eq!(map["abc"], "is a test\"");
    }

    #[test]
    fn it_works_with_other_chars() {
        let ex = "hello: world abc: def";
        let map = get_values(ex).unwrap();

        assert_eq!(map["hello"], "world");
        assert_eq!(map["abc"], "def");
    }

    #[test]
    fn it_fails_on_bad_messages() {
        let ex = "hello = = world";

        let map = get_values(ex);

        match map {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
