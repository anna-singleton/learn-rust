#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect {
            w: 8,
            h: 7,
        };

        let smaller = Rect {
            w: 5,
            h: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rect {
            w: 8,
            h: 7,
        };

        let smaller = Rect {
            w: 5,
            h: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn eqs() {
        assert_eq!(4, 2+2);
        assert_ne!(21, 9+10);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub fn greeting(name: &str) -> String {
    let mut s = String::from("Hello");
    s.push_str(name);
    s
}
