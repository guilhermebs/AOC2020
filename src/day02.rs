
struct Password {
    min_letter: usize,
    max_letter: usize,
    letter: char,
    password: String
}

impl Password {
    fn validate(&self) -> bool {
        let n_in_password = self.password.chars().filter(|&c| c == self.letter).count();
        (self.min_letter <= n_in_password) & (n_in_password <= self.max_letter)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_good() {
        let sut = Password{
            min_letter: 1,
            max_letter: 3,
            letter: 'a',
            password: "abcde".to_string()
        };
        assert!(sut.validate())
    }

    #[test]
    fn test_password_bad() {
        let sut = Password{
            min_letter: 1,
            max_letter: 3,
            letter: 'b',
            password: "cdefg".to_string()
        };
        assert!(!sut.validate())
    }

}