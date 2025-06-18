//! Crate core du langage EDL : contient l'AST, le lexer, le parser et le runtime.
//! Fournit des fonctions utilitaires pour parser et exécuter du code EDL.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod runtime;

// Réexporte les éléments principaux pour un accès plus simple depuis l'extérieur
pub use ast::*;
pub use lexer::*;
pub use parser::*;
pub use runtime::*;

/// Parse du code source EDL et retourne l'AST (liste de statements)
pub fn parse_source(src: &str) -> Result<Vec<ast::Stmt>, parser::ParseError> {
    let mut parser = parser::Parser::new(src);
    parser.parse()
}

/// Parse et exécute du code source EDL dans un nouvel environnement.
/// Retourne la dernière valeur évaluée ou une erreur.
pub fn eval_source(src: &str) -> Result<runtime::Value, runtime::RuntimeError> {
    let stmts = parse_source(src).map_err(|e| runtime::RuntimeError::Message(format!("{:?}", e)))?;
    let mut interp = runtime::Interpreter::new();
    let mut last = runtime::Value::Null;
    for stmt in stmts {
        last = interp.eval_stmt(&stmt)?;
    }
    Ok(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_eval_let() {
        let src = "let x = 2 + 2; x;";
        let result = eval_source(src).unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    fn parse_error() {
        let src = "let = ;";
        let err = parse_source(src);
        assert!(err.is_err());
    }
}