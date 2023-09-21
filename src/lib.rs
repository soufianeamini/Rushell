mod lexer {
    #[derive(PartialEq, Debug)]
    pub enum TokenType {
        WORD,
        PIPE,
    }

    pub struct Token {
        pub literal: String,
        pub ttype: TokenType,
    }

    pub fn lex(line: &str) -> Vec<Token> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenType;

    use super::*;

    #[test]
    fn words_only() {
        let line = "echo ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].literal, "echo");
        assert_eq!(tokens[1].ttype, TokenType::WORD);
    }
}
