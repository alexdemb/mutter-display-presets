use std::io::Write;

const USAGE_TEXT: &str = "Usage:
    mutter-display-presets [action] [flags...]

ACTIONS:
    help - display usage info

FLAGS:
";

pub fn print_usage<W: Write>(writer: &mut W) -> std::io::Result<()> {
    writer.write_all(USAGE_TEXT.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_usage_generates_expected_output() {
        let mut buffer: Vec<u8> = Vec::new();

        let result = print_usage(&mut buffer);

        assert!(!result.is_err());
        assert_eq!(buffer.as_slice(), USAGE_TEXT.as_bytes());
    }

}