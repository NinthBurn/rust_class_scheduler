/// Parses a line from the CSV document, extracting all columns and returning them as a vector of strings.
/// Keeps double quotes used for escaping characters.
pub fn parse_line_escaped(line: &String) -> Vec<String> {
    let mut in_quotes = false;
    let mut column = String::new();
    let mut columns: Vec<String> = Vec::new();

    for (_, item) in line.chars().enumerate() {
        match item {
            '"' => {
                column.push(item);
                in_quotes = !in_quotes;
            }

            ',' => {
                if in_quotes {
                    column.push(item);
                } else {
                    columns.push(column.clone());
                    column.clear();
                }
            }

            _ => column.push(item),
        }
    }

    columns.push(column);

    return columns;
}

/// Removes double quotes used for escaping characters. Clears the content of the passed string.
fn remove_escape_chars(string: &mut String) -> String {
    let result_string;

    if string.len() > 2 {
        let last_char = string.pop().unwrap();

        if last_char == '"' {
            let mut it = string.chars();
            it.next();
            string.clone_from(&String::from(it.as_str()));
        } else {
            string.push(last_char);
        }
    }

    result_string = string.replace("\"\"", "\"");
    string.clear();

    return result_string;
}

/// Parses a line from the CSV document, extracting all columns and returning them as a vector of strings.
/// Removes double quotes used for escaping characters.
pub fn parse_line(line: &String) -> Vec<String> {
    let mut in_quotes = false;
    let mut column = String::new();
    let mut columns: Vec<String> = Vec::new();

    for (_, item) in line.chars().enumerate() {
        match item {
            '"' => {
                column.push(item);
                in_quotes = !in_quotes;
            }

            ',' => {
                if in_quotes {
                    column.push(item);
                } else {
                    columns.push(remove_escape_chars(&mut column));
                }
            }

            _ => column.push(item),
        }
    }

    columns.push(remove_escape_chars(&mut column));

    return columns;
}
