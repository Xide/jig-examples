//! How the example greets someone.

/// The greeting for `name`, or for a stranger when the name is blank.
pub fn greeting(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        "Hello, stranger!".to_owned()
    } else {
        format!("Hello, {name}!")
    }
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greets_someone_by_name() {
        assert_eq!(greeting("Ada"), "Hello, Ada!");
    }

    #[test]
    fn greets_a_stranger_without_a_name() {
        assert_eq!(greeting("  "), "Hello, stranger!");
    }
}
