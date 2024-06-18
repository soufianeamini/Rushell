pub mod lexer;

#[cfg(test)]
mod lexer_tests {
    use std::error::Error;

    use crate::lexer::Token;

    use super::*;

    #[test]
    fn words_only() -> Result<(), Box<dyn Error>> {
        let line = "echo ls";

        let tokens = lexer::lex(line.as_bytes())?;

        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Word("ls")));
        Ok(())
    }

    #[test]
    fn words_more_than_one_space() -> Result<(), Box<dyn Error>> {
        let line = "echo       ls";

        let tokens = lexer::lex(line.as_bytes())?;

        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Word("ls")));
        Ok(())
    }

    #[test]
    fn words_trailing_leading_spaces() -> Result<(), Box<dyn Error>> {
        let line = "    echo ls   ";

        let tokens = lexer::lex(line.as_bytes())?;

        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Word("ls")));
        Ok(())
    }

    #[test]
    fn pipes_only() -> Result<(), Box<dyn Error>> {
        let line = "| |     |   |";

        let tokens = lexer::lex(line.as_bytes())?;
        for i in 0..4 {
            assert!(matches!(tokens[i], Token::Pipe));
        }
        Ok(())
    }

    #[test]
    fn words_and_pipes() -> Result<(), Box<dyn Error>> {
        let line = "cat Cargo.toml | grep rusty";

        let tokens = lexer::lex(line.as_bytes())?;

        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[2], Token::Pipe));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_pipes() -> Result<(), Box<dyn Error>> {
        let line = "echo|echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Pipe));
        assert!(matches!(tokens[2], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn redirections_only() -> Result<(), Box<dyn Error>> {
        let line = "< >     <   >";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Less));
        assert!(matches!(tokens[1], Token::Great));
        assert!(matches!(tokens[2], Token::Less));
        assert!(matches!(tokens[3], Token::Great));
        Ok(())
    }

    #[test]
    fn words_and_redirections() -> Result<(), Box<dyn Error>> {
        let line = "cat  < Cargo.toml grep > rusty";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::Less));
        assert!(matches!(tokens[2], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::Great));
        assert!(matches!(tokens[5], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_redirections() -> Result<(), Box<dyn Error>> {
        let line = "echo<echo>echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Less));
        assert!(matches!(tokens[2], Token::Word("echo")));
        assert!(matches!(tokens[3], Token::Great));
        assert!(matches!(tokens[4], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn semicolons_only() -> Result<(), Box<dyn Error>> {
        let line = "; ;     ;   ;";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Semicolon));
        assert!(matches!(tokens[1], Token::Semicolon));
        assert!(matches!(tokens[2], Token::Semicolon));
        assert!(matches!(tokens[3], Token::Semicolon));
        Ok(())
    }

    #[test]
    fn words_and_semicolons() -> Result<(), Box<dyn Error>> {
        let line = "cat  ; Cargo.toml grep ; rusty";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::Semicolon));
        assert!(matches!(tokens[2], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::Semicolon));
        assert!(matches!(tokens[5], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_semicolons() -> Result<(), Box<dyn Error>> {
        let line = "echo;echo;echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Semicolon));
        assert!(matches!(tokens[2], Token::Word("echo")));
        assert!(matches!(tokens[3], Token::Semicolon));
        assert!(matches!(tokens[4], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn or_only() -> Result<(), Box<dyn Error>> {
        let line = "|| ||     ||   ||";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Or));
        assert!(matches!(tokens[1], Token::Or));
        assert!(matches!(tokens[2], Token::Or));
        assert!(matches!(tokens[3], Token::Or));
        Ok(())
    }

    #[test]
    fn words_and_or() -> Result<(), Box<dyn Error>> {
        let line = "cat  || Cargo.toml grep || rusty";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::Or));
        assert!(matches!(tokens[2], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::Or));
        assert!(matches!(tokens[5], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_or() -> Result<(), Box<dyn Error>> {
        let line = "echo||echo||echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Or));
        assert!(matches!(tokens[2], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Or));
        assert!(matches!(tokens[2], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn ampersand_only() -> Result<(), Box<dyn Error>> {
        let line = "& &     &   &";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Ampersand));
        assert!(matches!(tokens[1], Token::Ampersand));
        assert!(matches!(tokens[2], Token::Ampersand));
        assert!(matches!(tokens[3], Token::Ampersand));
        Ok(())
    }

    #[test]
    fn words_and_ampersand() -> Result<(), Box<dyn Error>> {
        let line = "cat  & Cargo.toml grep & rusty";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::Ampersand));
        assert!(matches!(tokens[2], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::Ampersand));
        assert!(matches!(tokens[5], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_ampersand() -> Result<(), Box<dyn Error>> {
        let line = "echo&echo&echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::Ampersand));
        assert!(matches!(tokens[2], Token::Word("echo")));
        assert!(matches!(tokens[3], Token::Ampersand));
        assert!(matches!(tokens[4], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn and_only() -> Result<(), Box<dyn Error>> {
        let line = "&& &&     &&   &&";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::And));
        assert!(matches!(tokens[1], Token::And));
        assert!(matches!(tokens[2], Token::And));
        assert!(matches!(tokens[3], Token::And));
        Ok(())
    }

    #[test]
    fn words_and_and() -> Result<(), Box<dyn Error>> {
        let line = "cat  && Cargo.toml grep && rusty";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("cat")));
        assert!(matches!(tokens[1], Token::And));
        assert!(matches!(tokens[2], Token::Word("Cargo.toml")));
        assert!(matches!(tokens[3], Token::Word("grep")));
        assert!(matches!(tokens[4], Token::And));
        assert!(matches!(tokens[5], Token::Word("rusty")));
        Ok(())
    }

    #[test]
    fn sticky_and() -> Result<(), Box<dyn Error>> {
        let line = "echo&&echo&&echo";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("echo")));
        assert!(matches!(tokens[1], Token::And));
        assert!(matches!(tokens[2], Token::Word("echo")));
        assert!(matches!(tokens[3], Token::And));
        assert!(matches!(tokens[4], Token::Word("echo")));
        Ok(())
    }

    #[test]
    fn heredoc_and_append_only() -> Result<(), Box<dyn Error>> {
        let line = "<< >>     <<   >>";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::LessLess));
        assert!(matches!(tokens[1], Token::GreatGreat));
        assert!(matches!(tokens[2], Token::LessLess));
        assert!(matches!(tokens[3], Token::GreatGreat));
        Ok(())
    }

    #[test]
    fn double_quotes() -> Result<(), Box<dyn Error>> {
        let line = "test \" Samini \"";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("test")));
        assert!(matches!(tokens[1], Token::Word("\" Samini \"")));
        Ok(())
    }

    #[test]
    fn double_quotes_complex() -> Result<(), Box<dyn Error>> {
        let line = "test\" Samini \"test";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("test\" Samini \"test")));
        Ok(())
    }

    #[test]
    fn double_quotes_error() -> Result<(), Box<dyn Error>> {
        let line = "test\" Samini test";

        let tokens = lexer::lex(line.as_bytes());
        match tokens {
            Ok(_) => panic!(),
            Err(_) => (),
        }
        Ok(())
    }

    #[test]
    fn single_quotes() -> Result<(), Box<dyn Error>> {
        let line = "test ' Samini '";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("test")));
        assert!(matches!(tokens[1], Token::Word("' Samini '")));
        Ok(())
    }

    #[test]
    fn single_quotes_complex() -> Result<(), Box<dyn Error>> {
        let line = "test' Samini 'test";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::Word("test' Samini 'test")));
        Ok(())
    }

    #[test]
    fn single_quotes_error() -> Result<(), Box<dyn Error>> {
        let line = "test' Samini test";

        let tokens = lexer::lex(line.as_bytes());
        match tokens {
            Ok(_) => panic!(),
            Err(_) => (),
        }
        Ok(())
    }

    #[test]
    fn parentheses() -> Result<(), Box<dyn Error>> {
        let line = "(ls -l) && ( echo test )";

        let tokens = lexer::lex(line.as_bytes())?;
        assert!(matches!(tokens[0], Token::LeftParen));
        assert!(matches!(tokens[1], Token::Word("ls")));
        assert!(matches!(tokens[2], Token::Word("-l")));
        assert!(matches!(tokens[3], Token::RightParen));
        assert!(matches!(tokens[4], Token::And));
        assert!(matches!(tokens[5], Token::LeftParen));
        assert!(matches!(tokens[6], Token::Word("echo")));
        assert!(matches!(tokens[7], Token::Word("test")));
        assert!(matches!(tokens[8], Token::RightParen));
        Ok(())
    }
}
