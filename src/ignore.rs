pub fn path_contains_ignore(path: &String, ignores: &Vec<String>) -> bool {
    for ignore in ignores {
        if path.contains(ignore) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_contains_ignore() {
        let path = String::from("src/main.rs");
        let ignores = vec![String::from("src/")];
        assert_eq!(path_contains_ignore(&path, &ignores), true);

        let path = String::from("src/main.rs");
        let ignores = vec![String::from("main.rs")];
        assert_eq!(path_contains_ignore(&path, &ignores), true);
    }

    #[test]
    fn test_path_does_not_contain_ignore() {
        let path = String::from("src/main.rs");
        let ignores = vec![String::from("debug/")];
        assert_eq!(path_contains_ignore(&path, &ignores), false);
    }
}
