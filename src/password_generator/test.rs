#[cfg(test)]
mod tests {
    use crate::PasswordGenerator;

    #[test]
    fn test_generate_password() {
        let generator = PasswordGenerator::new(20, true, true, true, true);
        let password = generator.generate();
        assert_eq!(password.len(), 20);
    }

    #[test]
    fn test_generate_password_contains_groups() {
        let generator = PasswordGenerator::new(20, true, true, true, true);
        let password = generator.generate();

        let lowercase_count = password.chars().filter(|&c| c.is_lowercase()).count();
        let uppercase_count = password.chars().filter(|&c| c.is_uppercase()).count();
        let digit_count = password.chars().filter(|&c| c.is_digit(10)).count();
        let special_count = password
            .chars()
            .filter(|&c| "!@#$%^&*()_-+=<>?".contains(c))
            .count();

        assert_eq!(lowercase_count, 5);
        assert_eq!(uppercase_count, 5);
        assert_eq!(digit_count, 5);
        assert_eq!(special_count, 5);
    }

    #[test]
    #[ignore = "unimplemented"]
    fn test_generate_password_length() {
        let generator = PasswordGenerator::new(12, true, true, true, true);
        let password = generator.generate();
        assert_eq!(password.len(), 12);
    }

    #[test]
    #[ignore = "unimplemented"]
    fn test_generate_password_disable_groups() {
        let generator = PasswordGenerator::new(20, true, false, true, false);
        let password = generator.generate();

        assert_eq!(password.len(), 20);

        // Check only for lowercase and digits
        let lowercase_count = password.chars().filter(|&c| c.is_lowercase()).count();
        let digit_count = password.chars().filter(|&c| c.is_digit(10)).count();
        let uppercase_count = password.chars().filter(|&c| c.is_uppercase()).count();
        let special_count = password
            .chars()
            .filter(|&c| "!@#$%^&*()_-+=<>?".contains(c))
            .count();

        assert!(lowercase_count > 0);
        assert!(digit_count > 0);
        assert_eq!(uppercase_count, 0);
        assert_eq!(special_count, 0);
    }

    #[test]
    fn test_generate_password_uniqueness() {
        let generator = PasswordGenerator::new(20, true, true, true, true);
        let password1 = generator.generate();
        let password2 = generator.generate();

        assert_ne!(password1, password2);
    }
}
