pub mod lexer {
    #[derive(PartialEq, Debug)]
    pub enum TokenType {
        WORD,
        PIPE,
    }

    #[derive(Debug)]
    pub struct Token {
        pub literal: String,
        pub ttype: TokenType,
    }

    fn push_word(value: &mut String, tokens: &mut Vec<Token>) {
        if !value.is_empty() {
            tokens.push(Token {
                literal: value.clone(),
                ttype: TokenType::WORD,
            })
        }

        value.clear()
    }

    pub fn lex(line: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut it = line.chars().into_iter().peekable();
        let mut value = String::new();

        while let Some(char) = it.next() {
            match char {
                ' ' => push_word(&mut value, &mut tokens),
                _ => value.push(char),
            }
        }

        push_word(&mut value, &mut tokens);
        tokens
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::TokenType;

    use super::*;

    #[test]
    fn words_only() {
        let line = "echo ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, TokenType::WORD);
    }

    #[test]
    fn words_more_than_one_space() {
        let line = "echo       ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, TokenType::WORD);
    }

    #[test]
    fn words_trailing_leading_spaces() {
        let line = "    echo ls   ";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, TokenType::WORD);
    }
}
