pub mod lexer;

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::TokenType::*;

    use super::*;

    #[test]
    fn words_only() {
        let line = "echo ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, Word);
    }

    #[test]
    fn words_more_than_one_space() {
        let line = "echo       ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, Word);
    }

    #[test]
    fn words_trailing_leading_spaces() {
        let line = "    echo ls   ";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, Word);
    }

    #[test]
    fn pipes_only() {
        let line = "| |     |   |";

        let tokens = lexer::lex(line);
        for i in 0..4 {
            assert_eq!(tokens[i].ttype, Pipe);
        }
    }

    #[test]
    fn words_and_pipes() {
        let line = "cat Cargo.toml | grep rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].literal, "Cargo.toml");
        assert_eq!(tokens[1].ttype, Word);
        assert_eq!(tokens[2].ttype, Pipe);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].literal, "rusty");
        assert_eq!(tokens[4].ttype, Word);
    }

    #[test]
    fn sticky_pipes() {
        let line = "echo|echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Pipe);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
    }

    #[test]
    fn redirections_only() {
        let line = "< >     <   >";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, Less);
        assert_eq!(tokens[1].ttype, Great);
        assert_eq!(tokens[2].ttype, Less);
        assert_eq!(tokens[3].ttype, Great);
    }

    #[test]
    fn words_and_redirections() {
        let line = "cat  < Cargo.toml grep > rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Less);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].ttype, Great);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, Word);
    }

    #[test]
    fn sticky_redirections() {
        let line = "echo<echo>echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Less);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, Great);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, Word);
    }

    #[test]
    fn semicolons_only() {
        let line = "; ;     ;   ;";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, Semicolon);
        assert_eq!(tokens[1].ttype, Semicolon);
        assert_eq!(tokens[2].ttype, Semicolon);
        assert_eq!(tokens[3].ttype, Semicolon);
    }

    #[test]
    fn words_and_semicolons() {
        let line = "cat  ; Cargo.toml grep ; rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Semicolon);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, Word);
        assert_eq!(tokens[4].ttype, Semicolon);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, Word);
    }

    #[test]
    fn sticky_semicolons() {
        let line = "echo;echo;echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, Word);
        assert_eq!(tokens[1].ttype, Semicolon);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, Word);
        assert_eq!(tokens[3].ttype, Semicolon);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, Word);
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
