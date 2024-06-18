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
    fn or_only() -> Result<(), Box<dyn Error>> {
        let line = "|| ||     ||   ||";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Or));
        assert!(matches!(tokens[1], TokenV2::Or));
        assert!(matches!(tokens[2], TokenV2::Or));
        assert!(matches!(tokens[3], TokenV2::Or));
        Ok(())
    }

    #[test]
    fn words_and_or() -> Result<(), Box<dyn Error>> {
        let line = "cat  || Cargo.toml grep || rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::Or));
        assert!(matches!(tokens[2], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::Or));
        assert!(matches!(tokens[5], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_or() -> Result<(), Box<dyn Error>> {
        let line = "echo||echo||echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Or));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Or));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn ampersand_only() -> Result<(), Box<dyn Error>> {
        let line = "& &     &   &";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Ampersand));
        assert!(matches!(tokens[1], TokenV2::Ampersand));
        assert!(matches!(tokens[2], TokenV2::Ampersand));
        assert!(matches!(tokens[3], TokenV2::Ampersand));
        Ok(())
    }

    #[test]
    fn words_and_ampersand() -> Result<(), Box<dyn Error>> {
        let line = "cat  & Cargo.toml grep & rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::Ampersand));
        assert!(matches!(tokens[2], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::Ampersand));
        assert!(matches!(tokens[5], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_ampersand() -> Result<(), Box<dyn Error>> {
        let line = "echo&echo&echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::Ampersand));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        assert!(matches!(tokens[3], TokenV2::Ampersand));
        assert!(matches!(tokens[4], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn and_only() -> Result<(), Box<dyn Error>> {
        let line = "&& &&     &&   &&";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::And));
        assert!(matches!(tokens[1], TokenV2::And));
        assert!(matches!(tokens[2], TokenV2::And));
        assert!(matches!(tokens[3], TokenV2::And));
        Ok(())
    }

    #[test]
    fn words_and_and() -> Result<(), Box<dyn Error>> {
        let line = "cat  && Cargo.toml grep && rusty";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("cat")));
        assert!(matches!(tokens[1], TokenV2::And));
        assert!(matches!(tokens[2], TokenV2::Word("Cargo.toml")));
        assert!(matches!(tokens[3], TokenV2::Word("grep")));
        assert!(matches!(tokens[4], TokenV2::And));
        assert!(matches!(tokens[5], TokenV2::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_and() -> Result<(), Box<dyn Error>> {
        let line = "echo&&echo&&echo";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("echo")));
        assert!(matches!(tokens[1], TokenV2::And));
        assert!(matches!(tokens[2], TokenV2::Word("echo")));
        assert!(matches!(tokens[3], TokenV2::And));
        assert!(matches!(tokens[4], TokenV2::Word("echo")));
        Ok(())
    }

    #[test]
    fn heredoc_and_append_only() -> Result<(), Box<dyn Error>> {
        let line = "<< >>     <<   >>";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::LessLess));
        assert!(matches!(tokens[1], TokenV2::GreatGreat));
        assert!(matches!(tokens[2], TokenV2::LessLess));
        assert!(matches!(tokens[3], TokenV2::GreatGreat));
        Ok(())
    }

    #[test]
    fn double_quotes() -> Result<(), Box<dyn Error>> {
        let line = "test \" Samini \"";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("test")));
        assert!(matches!(tokens[1], TokenV2::Word("\" Samini \"")));
        Ok(())
    }

    #[test]
    fn double_quotes_complex() -> Result<(), Box<dyn Error>> {
        let line = "test\" Samini \"test";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("test\" Samini \"test")));
        Ok(())
    }

    #[test]
    fn double_quotes_error() -> Result<(), Box<dyn Error>> {
        let line = "test\" Samini test";

        let tokens = lexer::lex_v2(line.as_bytes());
        match tokens {
            Ok(_) => panic!(),
            Err(_) => (),
        }
        Ok(())
    }

    #[test]
    fn single_quotes() -> Result<(), Box<dyn Error>> {
        let line = "test ' Samini '";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("test")));
        assert!(matches!(tokens[1], TokenV2::Word("' Samini '")));
        Ok(())
    }

    #[test]
    fn single_quotes_complex() -> Result<(), Box<dyn Error>> {
        let line = "test' Samini 'test";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::Word("test' Samini 'test")));
        Ok(())
    }

    #[test]
    fn single_quotes_error() -> Result<(), Box<dyn Error>> {
        let line = "test' Samini test";

        let tokens = lexer::lex_v2(line.as_bytes());
        match tokens {
            Ok(_) => panic!(),
            Err(_) => (),
        }
        Ok(())
    }

    #[test]
    fn parentheses() -> Result<(), Box<dyn Error>> {
        let line = "(ls -l) && ( echo test )";

        let tokens = lexer::lex_v2(line.as_bytes())?;
        assert!(matches!(tokens[0], TokenV2::LeftParen));
        assert!(matches!(tokens[1], TokenV2::Word("ls")));
        assert!(matches!(tokens[2], TokenV2::Word("-l")));
        assert!(matches!(tokens[3], TokenV2::RightParen));
        assert!(matches!(tokens[4], TokenV2::And));
        assert!(matches!(tokens[5], TokenV2::LeftParen));
        assert!(matches!(tokens[6], TokenV2::Word("echo")));
        assert!(matches!(tokens[7], TokenV2::Word("test")));
        assert!(matches!(tokens[8], TokenV2::RightParen));
        Ok(())
    }
}
