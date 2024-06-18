pub mod lexer;

#[cfg(test)]
mod lexer_tests {
    use std::error::Error;

    use crate::lexer::{TokenType::*, TokenV2};

    use super::*;

    #[test]
    fn words_only() -> Result<(), Box<dyn Error>> {
        let line = "echo ls";

        let tokens = lexer::lex_v2(line.as_bytes())?;

        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Word("ls")));
        Ok(())
    }

    #[test]
    fn words_more_than_one_space() -> Result<(), Box<dyn Error>> {
        let line = "echo       ls";

        let tokens = lexer::lex_v2(line.as_bytes())?;

        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Word("ls")));
        Ok(())
    }

    #[test]
    fn words_trailing_leading_spaces() -> Result<(), Box<dyn Error>> {
        let line = "    echo ls   ";

        let tokens = lexer::lex_v2(line.as_bytes())?;

        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Word("ls")));
        Ok(())
    }

    #[test]
    fn pipes_only() -> Result<(), Box<dyn Error>> {
        let line = "| |     |   |";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        for i in 0..4 {
            assert!(matches!(tokens[i], TokenV2::Pipe));
        }
        Ok(())
    }

    #[test]
    fn words_and_pipes() -> Result<(), Box<dyn Error>> {
        let line = "cat Cargo.toml | grep rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;

        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[2], TokenV2::Pipe));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_pipes() -> Result<(), Box<dyn Error>> {
        let line = "echo|echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Pipe));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn redirections_only() -> Result<(), Box<dyn Error>> {
        let line = "< >     <   >";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Less));
        assert!(matches!(tokens[1], TokenV2::Great));
        assert!(matches!(tokens[2], TokenV2::Less));
        assert!(matches!(tokens[3], TokenV2::Great));
        Ok(())
    }

    #[test]
    fn words_and_redirections() -> Result<(), Box<dyn Error>> {
        let line = "cat  < Cargo.toml grep > rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::Less));
        assert!(matches!(tokens[2], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::Great));
        assert!(matches!(tokens[5], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_redirections() -> Result<(), Box<dyn Error>> {
        let line = "echo<echo>echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Less));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        assert!(matches!(tokens[3], TokenV2::Great));
        assert!(matches!(tokens[4], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn semicolons_only() -> Result<(), Box<dyn Error>> {
        let line = "; ;     ;   ;";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Semicolon));
        assert!(matches!(tokens[1], TokenV2::Semicolon));
        assert!(matches!(tokens[2], TokenV2::Semicolon));
        assert!(matches!(tokens[3], TokenV2::Semicolon));
        Ok(())
    }

    #[test]
    fn words_and_semicolons() -> Result<(), Box<dyn Error>> {
        let line = "cat  ; Cargo.toml grep ; rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::Semicolon));
        assert!(matches!(tokens[2], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::Semicolon));
        assert!(matches!(tokens[5], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_semicolons() -> Result<(), Box<dyn Error>> {
        let line = "echo;echo;echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Semicolon));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        assert!(matches!(tokens[3], TokenV2::Semicolon));
        assert!(matches!(tokens[4], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn or_only() {
        let line = "|| ||     ||   ||";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, Or);
        assert_eq!(tokens[1].ttype, Or);
        assert_eq!(tokens[2].ttype, Or);
        assert_eq!(tokens[3].ttype, Or);
    }

    #[test]
    fn words_and_or() {
        let line = "cat  || Cargo.toml grep || rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Or);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].ttype, Or);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, Word);
    }

    #[test]
    fn sticky_or() {
        let line = "echo||echo||echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Or);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, Or);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, Word);
    }

    #[test]
    fn ampersand_only() {
        let line = "& &     &   &";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, Ampersand);
        assert_eq!(tokens[1].ttype, Ampersand);
        assert_eq!(tokens[2].ttype, Ampersand);
        assert_eq!(tokens[3].ttype, Ampersand);
    }

    #[test]
    fn words_and_ampersand() {
        let line = "cat  & Cargo.toml grep & rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Ampersand);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].ttype, Ampersand);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, Word);
    }

    #[test]
    fn sticky_ampersand() {
        let line = "echo&echo&echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Ampersand);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, Ampersand);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, Word);
    }

    #[test]
    fn and_only() {
        let line = "&& &&     &&   &&";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, And);
        assert_eq!(tokens[1].ttype, And);
        assert_eq!(tokens[2].ttype, And);
        assert_eq!(tokens[3].ttype, And);
    }

    #[test]
    fn words_and_and() {
        let line = "cat  && Cargo.toml grep && rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, And);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].ttype, And);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, Word);
    }

    #[test]
    fn sticky_and() {
        let line = "echo&&echo&&echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, And);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, And);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, Word);
    }

    #[test]
    fn heredoc_and_append_only() {
        let line = "<< >>     <<   >>";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, LessLess);
        assert_eq!(tokens[1].ttype, GreatGreat);
        assert_eq!(tokens[2].ttype, LessLess);
        assert_eq!(tokens[3].ttype, GreatGreat);
    }

    #[test]
    fn double_quotes() {
        let line = "test \" Samini \"";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "\" Samini \"");
        assert_eq!(tokens[1].ttype, Word);
    }

    #[test]
    fn double_quotes_complex() {
        let line = "test\" Samini \"test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test\" Samini \"test");
        assert_eq!(tokens[0].ttype, Word);
    }

    #[test]
    fn double_quotes_error() {
        let line = "test\" Samini test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "Unclosed quotes");
        assert_eq!(tokens[0].ttype, Error);
    }

    #[test]
    fn single_quotes() {
        let line = "test ' Samini '";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "' Samini '");
        assert_eq!(tokens[1].ttype, Word);
    }

    #[test]
    fn single_quotes_complex() {
        let line = "test' Samini 'test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test' Samini 'test");
        assert_eq!(tokens[0].ttype, Word);
    }

    #[test]
    fn single_quotes_error() {
        let line = "test' Samini test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "Unclosed quotes");
        assert_eq!(tokens[0].ttype, Error);
    }

    #[test]
    fn parentheses() {
        let line = "(ls -l) && (echo test)";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, LeftParen);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, Word);
        assert_eq!(tokens[2].literal, "-l");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, RightParen);
        assert_eq!(tokens[4].ttype, And);
        assert_eq!(tokens[5].ttype, LeftParen);
        assert_eq!(tokens[6].literal, "echo");
        assert_eq!(tokens[6].ttype, Word);
        assert_eq!(tokens[7].literal, "test");
        assert_eq!(tokens[7].ttype, Word);
        assert_eq!(tokens[8].ttype, RightParen);
    }
}
