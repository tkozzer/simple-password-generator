mod test;

use rand::{seq::IteratorRandom, seq::SliceRandom, thread_rng};

pub struct PasswordGenerator {
    pub length: usize,
    pub use_lowercase: bool,
    pub use_uppercase: bool,
    pub use_digits: bool,
    pub use_special_chars: bool,
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        use_lowercase: bool,
        use_uppercase: bool,
        use_digits: bool,
        use_special_chars: bool,
    ) -> Self {
        Self {
            length,
            use_lowercase,
            use_uppercase,
            use_digits,
            use_special_chars,
        }
    }

    pub fn generate(&self) -> String {
        const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
        const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const DIGITS: &str = "0123456789";
        const SPECIAL: &str = "!@#$%^&*()_-+=<>?";

        let mut rng = thread_rng();

        let mut password_chars: Vec<char> = Vec::with_capacity(self.length);

        for _ in 0..5 {
            let mut group = Vec::with_capacity(4);
            group.push(LOWERCASE.chars().choose(&mut rng).unwrap());
            group.push(UPPERCASE.chars().choose(&mut rng).unwrap());
            group.push(DIGITS.chars().choose(&mut rng).unwrap());
            group.push(SPECIAL.chars().choose(&mut rng).unwrap());

            group.shuffle(&mut rng);
            password_chars.extend(group);
        }

        password_chars.iter().collect()
    }
}
