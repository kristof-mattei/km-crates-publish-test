pub fn reverse(input: impl Into<String>) -> String {
    let s: String = input.into();

    s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn it_works() {
        assert_eq!("fotsirk", reverse("kristof"));
    }
}
