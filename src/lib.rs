pub fn reverse(s: &str) -> String {
    s.chars().into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn edge_cases() {
        assert_eq!(reverse(""), "");
        assert_eq!(
            reverse("123456789012345678901234567890"),
            "098765432109876543210987654321"
        );
    }
}
