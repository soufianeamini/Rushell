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

    #[test]
    fn pipes_only() {
        let line = "| |     |   |";

        let tokens = lexer::lex(line);
        for i in 0..4 {
            assert_eq!(tokens[i].ttype, TokenType::PIPE);
        }
    }

    #[test]
    fn words_and_pipes() {
        let line = "cat Cargo.toml | grep rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].literal, "Cargo.toml");
        assert_eq!(tokens[1].ttype, TokenType::WORD);
        assert_eq!(tokens[2].ttype, TokenType::PIPE);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, TokenType::WORD);
        assert_eq!(tokens[4].literal, "rusty");
        assert_eq!(tokens[4].ttype, TokenType::WORD);
    }

    #[test]
    fn sticky_pipes() {
        let line = "echo|echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, TokenType::WORD);
        assert_eq!(tokens[1].ttype, TokenType::PIPE);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, TokenType::WORD);
    }
}
