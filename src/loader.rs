use std::fs;

use anyhow::bail;

/// Don't allow any nesting deeper than this. BF sources shouldn't be too complicated
/// and anything more than this probably means some errors in the source.
const MAX_NESTING_LEVEL: usize = 10_000;

fn parse_source_code(file_content: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut code = Vec::<u8>::with_capacity(file_content.len());
    let mut current_nesting_level: usize = 0;
    for b in file_content {
        match b {
            b'<' | b'>' | b'+' | b'-' | b'.' | b',' => {
                // Regular BF source character. Append to code.
                code.push(b);
            }
            b'[' => {
                current_nesting_level += 1;
                if current_nesting_level > MAX_NESTING_LEVEL {
                    bail!("Loop nesting too deep.");
                }
                code.push(b);
            }
            b']' => {
                if current_nesting_level == 0 {
                    bail!("Unmatched closing loop statement");
                }
                current_nesting_level -= 1;
                code.push(b);
            }
            _ => {
                // Any characters that are not valid BF source characters
                // are ignored. This allows the user to write comments
                // or add whitespace anywhere they like.
            }
        }
    }

    if code.len() == 0 {
        bail!("Source empty");
    }

    if current_nesting_level != 0 {
        bail!("Uneven count of opening and closing loop brackets");
    }

    Ok(code)
}

pub fn load_and_verify(file_name: &String) -> anyhow::Result<Vec<u8>> {
    let file_content = fs::read(file_name)?;
    parse_source_code(file_content)
}

#[cfg(test)]
mod tests {
    use crate::loader::parse_source_code;

    #[test]
    fn it_rejects_empty_source() {
        let code = "";
        let result = parse_source_code(Vec::from(code));
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_source_without_control_sequences() {
        let code = "hello :)";
        let result = parse_source_code(Vec::from(code));
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_too_many_closing_parens() {
        let code = "[]]";
        let result = parse_source_code(Vec::from(code));
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_too_many_opening_parens() {
        let code = "[[]";
        let result = parse_source_code(Vec::from(code));
        assert!(result.is_err());
    }

    #[test]
    fn it_accepts_valid_bf_code() {
        let source = "+++[+-<>][.,]---";
        let result = parse_source_code(Vec::from(source));
        let code = result.unwrap();
        assert_eq!(b"+++[+-<>][.,]---", code.as_slice());
    }

    #[test]
    fn it_accepts_valid_bf_code_with_comments() {
        let source = "+++ [+-<>] comments are ignored [.,]---";
        let result = parse_source_code(Vec::from(source));
        let code = result.unwrap();
        assert_eq!(b"+++[+-<>][.,]---", code.as_slice());
    }
}
