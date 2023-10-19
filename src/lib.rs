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
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, WORD);
    }

    #[test]
    fn words_more_than_one_space() {
        let line = "echo       ls";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, WORD);
    }

    #[test]
    fn words_trailing_leading_spaces() {
        let line = "    echo ls   ";

        let tokens = lexer::lex(line);

        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].literal, "ls");
        assert_eq!(tokens[1].ttype, WORD);
    }

    #[test]
    fn pipes_only() {
        let line = "| |     |   |";

        let tokens = lexer::lex(line);
        for i in 0..4 {
            assert_eq!(tokens[i].ttype, PIPE);
        }
    }

    #[test]
    fn words_and_pipes() {
        let line = "cat Cargo.toml | grep rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].literal, "Cargo.toml");
        assert_eq!(tokens[1].ttype, WORD);
        assert_eq!(tokens[2].ttype, PIPE);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].literal, "rusty");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn sticky_pipes() {
        let line = "echo|echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, PIPE);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
    }

    #[test]
    fn redirections_only() {
        let line = "< >     <   >";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, LESS);
        assert_eq!(tokens[1].ttype, GREAT);
        assert_eq!(tokens[2].ttype, LESS);
        assert_eq!(tokens[3].ttype, GREAT);
    }

    #[test]
    fn words_and_redirections() {
        let line = "cat  < Cargo.toml grep > rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, LESS);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].ttype, GREAT);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, WORD);
    }

    #[test]
    fn sticky_redirections() {
        let line = "echo<echo>echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, LESS);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].ttype, GREAT);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn semicolons_only() {
        let line = "; ;     ;   ;";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, SEMICOLON);
        assert_eq!(tokens[1].ttype, SEMICOLON);
        assert_eq!(tokens[2].ttype, SEMICOLON);
        assert_eq!(tokens[3].ttype, SEMICOLON);
    }

    #[test]
    fn words_and_semicolons() {
        let line = "cat  ; Cargo.toml grep ; rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, SEMICOLON);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].ttype, SEMICOLON);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, WORD);
    }

    #[test]
    fn sticky_semicolons() {
        let line = "echo;echo;echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, SEMICOLON);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].ttype, SEMICOLON);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn or_only() {
        let line = "|| ||     ||   ||";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, OR);
        assert_eq!(tokens[1].ttype, OR);
        assert_eq!(tokens[2].ttype, OR);
        assert_eq!(tokens[3].ttype, OR);
    }

    #[test]
    fn words_and_or() {
        let line = "cat  || Cargo.toml grep || rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, OR);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].ttype, OR);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, WORD);
    }

    #[test]
    fn sticky_or() {
        let line = "echo||echo||echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, OR);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].ttype, OR);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn ampersand_only() {
        let line = "& &     &   &";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, AMPERSAND);
        assert_eq!(tokens[1].ttype, AMPERSAND);
        assert_eq!(tokens[2].ttype, AMPERSAND);
        assert_eq!(tokens[3].ttype, AMPERSAND);
    }

    #[test]
    fn words_and_ampersand() {
        let line = "cat  & Cargo.toml grep & rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, AMPERSAND);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].ttype, AMPERSAND);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, WORD);
    }

    #[test]
    fn sticky_ampersand() {
        let line = "echo&echo&echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, AMPERSAND);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].ttype, AMPERSAND);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn and_only() {
        let line = "&& &&     &&   &&";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, AND);
        assert_eq!(tokens[1].ttype, AND);
        assert_eq!(tokens[2].ttype, AND);
        assert_eq!(tokens[3].ttype, AND);
    }

    #[test]
    fn words_and_and() {
        let line = "cat  && Cargo.toml grep && rusty";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "cat");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, AND);
        assert_eq!(tokens[2].literal, "Cargo.toml");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].literal, "grep");
        assert_eq!(tokens[3].ttype, WORD);
        assert_eq!(tokens[4].ttype, AND);
        assert_eq!(tokens[5].literal, "rusty");
        assert_eq!(tokens[5].ttype, WORD);
    }

    #[test]
    fn sticky_and() {
        let line = "echo&&echo&&echo";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "echo");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].ttype, AND);
        assert_eq!(tokens[2].literal, "echo");
        assert_eq!(tokens[2].ttype, WORD);
        assert_eq!(tokens[3].ttype, AND);
        assert_eq!(tokens[4].literal, "echo");
        assert_eq!(tokens[4].ttype, WORD);
    }

    #[test]
    fn heredoc_and_append_only() {
        let line = "<< >>     <<   >>";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].ttype, LESSLESS);
        assert_eq!(tokens[1].ttype, GREATGREAT);
        assert_eq!(tokens[2].ttype, LESSLESS);
        assert_eq!(tokens[3].ttype, GREATGREAT);
    }

    #[test]
    fn double_quotes() {
        let line = "test \" Samini \"";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test");
        assert_eq!(tokens[0].ttype, WORD);
        assert_eq!(tokens[1].literal, "\" Samini \"");
        assert_eq!(tokens[1].ttype, WORD);
    }

    #[test]
    fn double_quotes_complex() {
        let line = "test\" Samini \"test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "test\" Samini \"test");
        assert_eq!(tokens[0].ttype, WORD);
    }

    #[test]
    fn double_quotes_error() {
        let line = "test\" Samini test";

        let tokens = lexer::lex(line);
        assert_eq!(tokens[0].literal, "Unclosed doublequotes");
        assert_eq!(tokens[0].ttype, ERROR);
    }
}
