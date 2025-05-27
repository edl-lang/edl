// core/src/lib.rs

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod runtime;

// Basic function to demonstrate linkage, will be expanded later
pub fn greet() -> String {
    "Hello from core!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(greet(), "Hello from core!");
    }
}