use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug)]
struct Password {
    min_letter: usize,
    max_letter: usize,
    letter: char,
    password: String
}

#[derive(Debug)]
enum PasswordError {
    InvalidString
}

impl Password {
    fn from_string(string: String) -> Result<Password, PasswordError> {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        match re.captures(&string) {
            Some(cap) => return Ok(Password{
                min_letter: cap[1].parse::<usize>().unwrap(),
                max_letter: cap[2].parse::<usize>().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                password: cap[4].to_string()
            }),
            _ => Err(PasswordError::InvalidString)
        }
    }

    fn validate(&self) -> bool {
        let n_in_password = self.password.chars().filter(|&c| c == self.letter).count();
        (self.min_letter <= n_in_password) & (n_in_password <= self.max_letter)
    }

    fn validate2(&self) -> bool {
        if self.password.len() < self.min_letter { return false };
        if self.password.len() < self.max_letter { return false };
        (self.password.chars().nth(self.min_letter - 1).unwrap() == self.letter) ^
        (self.password.chars().nth(self.max_letter - 1).unwrap() == self.letter)
    }
}

#[allow(dead_code)]
pub fn day02() {
    let mut passwords = Vec::<Password>::new();
    let file = File::open("inputs/day02").unwrap();

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(ln) =>  passwords.push(Password::from_string(ln).unwrap()),
            _ => println!("failed to read")
        }
    }

    println!("total: {}", passwords.len());
    let n_valid = passwords.iter().filter(|&p| p.validate()).count();
    println!("part1: {}", n_valid);

    let n_valid = passwords.iter().filter(|&p| p.validate2()).count();
    //for p in passwords.iter() {
    //    println!("{:?}, {}", p, p.validate2());
    //};
    println!("part2: {}", n_valid);
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

    #[test]
    fn test_from_string() {
        let sut = Password::from_string("1-3 b: cdefg".to_string()).unwrap();
        assert_eq!(sut.min_letter, 1);
        assert_eq!(sut.max_letter, 3);
        assert_eq!(sut.letter, 'b');
        assert_eq!(sut.password, "cdefg");
    }

    #[test]
    fn test_password_validate_2_good() {
        let sut = Password{
            min_letter: 1,
            max_letter: 3,
            letter: 'a',
            password: "abcde".to_string()
        };
        assert!(sut.validate2())
    }

    #[test]
    fn test_password_validate_2_bad() {
        let sut = Password{
            min_letter: 1,
            max_letter: 3,
            letter: 'b',
            password: "cdefg".to_string()
        };
        assert!(!sut.validate2())
    }

    #[test]
    fn test_password_validate_2_bad2() {
        let sut = Password{
            min_letter: 2,
            max_letter: 9,
            letter: 'c',
            password: "ccccccccc".to_string()
        };
        assert!(!sut.validate2())
    }

}