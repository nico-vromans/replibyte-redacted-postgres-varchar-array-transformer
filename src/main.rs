use std::io::Read;

fn main() {
    // Constants used for transformation
    const CHAR_COUNT: usize = 3;
    const FILL_WIDTH: usize = 10;
    const CHAR_MASK: &str = "*";

    // Read input value from stdin
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // Remove curly braces and split by comma
    let trimmed = input.trim().trim_start_matches('{').trim_end_matches('}');
    let values: Vec<String> = trimmed.split(',').map(String::from).collect();

    // Transform each value with the original RepliByte RedactedTransformer logic
    let transformed_values: Vec<String> = values
        .iter()
        .map(|s| match s.len() {
            len if len > CHAR_COUNT => {
                format!(
                    "{}{}",
                    s.chars().take(CHAR_COUNT).collect::<String>(),
                    CHAR_MASK.repeat(FILL_WIDTH - CHAR_COUNT)
                )
            }
            _ => s.clone(),
        })
        .collect();

    // Write transformed value to stdout
    println!("{{{}}}", transformed_values.join(","));
}
