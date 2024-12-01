pub fn split_on_new_lines(content: &str) -> Vec<String> {
    content.split('\n').map(|line| line.to_string()).collect()
}
