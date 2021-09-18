use std::io::{self};

fn dividable_into_evens(number: i32) -> bool {
    (number > 2) && (number % 2 == 0)
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).unwrap();

    let number: i32 = buffer.trim_end().parse().unwrap();

    if dividable_into_evens(number) {
        println!("YES")
    } else {
        println!("NO")
    }
}

#[cfg(test)]
mod tests {
    use crate::dividable_into_evens;

    #[test]
    fn test_1 () {
        assert_eq!(dividable_into_evens(2), false);
    }

    #[test]
    fn test_2 () {
        assert_eq!(dividable_into_evens(8), true);
    }

    #[test]
    fn test_3 () {
        assert_eq!(dividable_into_evens(6), true);
    }

    #[test]
    fn test_4 () {
        assert_eq!(dividable_into_evens(7), false);
    }
}
