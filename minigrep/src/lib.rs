pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ung";
        let contents = "\
Rust:
The rise of the jungle.
Pick three.";

        assert_eq!(vec!["The rise of the jungle."], search(query, contents));
    }
}
