pub fn render(markdown: &str) -> String {
    markdown.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_plain_text() {
        let markdown = "Hello\n";
        let html = render(markdown);
        assert_eq!(html, "Hello\n");
    }
}
