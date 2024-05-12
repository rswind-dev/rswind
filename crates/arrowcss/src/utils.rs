pub fn decode_arbitrary_value(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '_' {
                    chars.next();
                    output.push('_');
                    continue;
                }
            }
        }
        output.push(if c == '_' { ' ' } else { c });
    }

    output
}
