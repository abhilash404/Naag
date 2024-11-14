use naag::scanner::{Scanner, TokenType, LiteralValue};
use naag::environment::Environment;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_variable_definition() {
        let mut env = Environment::new();
        let source = "var x = 42;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");
        
        assert_eq!(tokens.len(), 4); // Includes EOF
        run(source, &mut env).expect("Failed to run");
        assert_eq!(env.get("x"), Some(&LiteralValue::IntValue(42)));
    }

    #[test]
    fn handle_basic_arithmetic() {
        let source = "var y = 5 + 3;";
        let mut env = Environment::new();
        run(source, &mut env).expect("Failed to run");
        assert_eq!(env.get("y"), Some(&LiteralValue::IntValue(8)));
    }

    #[test]
    fn handle_undefined_variable() {
        let mut env = Environment::new();
        let source = "print x;";
        assert!(run(source, &mut env).is_err(), "Should fail on undefined variable");
    }
}
