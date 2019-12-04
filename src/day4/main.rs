use std::error::Error;
use std::io::BufRead;

fn is_correct_length(s: &str) -> bool {
    s.len() == 6
}

fn is_increasing(s: &str) -> bool {
    s.chars()
        .fold(Ok(0), |acc: Result<u32, Box<dyn Error>>, c| {
            let digit = c.to_digit(10).ok_or("cannot convert do digit")?;
            if acc? <= digit {
                Ok(digit)
            } else {
                Err(Box::new(simple_error::SimpleError::new("increased digit")))
            }
        })
        .is_ok()
}

fn is_in_range(i: i64, lower: i64, upper: i64) -> bool {
    i >= lower && i <= upper
}

fn contains_pair(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    if chars[0] == chars[1] && chars[1] != chars[2] {
        return true;
    }
    if chars[chars.len() - 2] == chars[chars.len() - 1] && chars[chars.len() - 3] != chars[chars.len() - 2] {
        return true;
    }
    chars
        .windows(4)
        .any(|arr| {
            match arr {
                [a, x, y, z] => x == y && a != x && y != z,
                _ => false
            }
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    let mut input = String::new();
    stdin_locked.read_line(&mut input)?;
    let lower: i64 = input.trim().parse()?;
    let mut input = String::new();
    stdin_locked.read_line(&mut input)?;
    let upper: i64 = input.trim().parse()?;

    let count =
        (lower..upper + 1)
            .fold(0, |acc, i| {
                let s = i.to_string();
                if is_correct_length(&s)
                    && is_in_range(i, lower, upper)
                    && contains_pair(&s)
                    && is_increasing(&s) {
                    acc + 1
                } else {
                    acc
                }
            });
    println!("{}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert_eq!(true, is_increasing("12345"));
        assert_eq!(true, is_increasing("11111"));
        assert_eq!(false, is_increasing("1234567898"));
    }

    #[test]
    fn test_increasing_non_digit() {
        assert_eq!(false, is_increasing("hello"));
    }

    #[test]
    fn correct_length() {
        assert_eq!(true, is_correct_length("abcdef"));
        assert_eq!(true, is_correct_length("123456"));
        assert_eq!(false, is_correct_length("12345"));
        assert_eq!(false, is_correct_length("1234567"));
    }

    #[test]
    fn in_range() {
        assert_eq!(true, is_in_range(5, 0, 10));
        assert_eq!(true, is_in_range(0, 0, 10));
        assert_eq!(true, is_in_range(10, 0, 10));
        assert_eq!(false, is_in_range(11, 0, 10));
    }

    #[test]
    fn pairs() {
        assert_eq!(true, contains_pair("aabc"));
        assert_eq!(true, contains_pair("abbc"));
        assert_eq!(true, contains_pair("abcc"));
        assert_eq!(true, contains_pair("abbbbc"));
        assert_eq!(false, contains_pair("abc"));
    }
}