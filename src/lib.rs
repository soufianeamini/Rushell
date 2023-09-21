pub mod lexer;

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
