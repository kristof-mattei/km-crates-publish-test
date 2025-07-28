use std::borrow::Cow;

pub fn reverse<I: Into<String>>(input: I) -> String {
    let s: String = input.into();

    s.chars().rev().collect::<String>()
}

pub fn reverse2<I: Into<String>>(input: I) -> String {
    let s: String = input.into();

    s.chars().rev().collect::<String>()
}

#[must_use]
pub fn to_bytes(input: Cow<'_, str>) -> Vec<u8> {
    input.into_owned().into_bytes()
}

#[cfg(test)]
mod tests {
    use crate::{reverse, to_bytes};

    #[test]
    fn it_works() {
        assert_eq!("fotsirk", reverse("kristof"));
    }

    #[test]
    fn into_bytes() {
        assert_eq!(
            to_bytes("Hello, World".into()),
            &[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100]
        );
    }
}
