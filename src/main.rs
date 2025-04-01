/// The main function that prints a greeting message.
fn main() {
    println!("{}", greet());
}

/// Returns a greeting message.
///
/// # Examples
///
/// ```
/// assert_eq!(greet(), "Hello, world!");
/// ```
fn greet() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `greet` function to ensure it returns the correct greeting message.
    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}
