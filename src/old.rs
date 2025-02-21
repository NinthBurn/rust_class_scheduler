// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Reads the whole schedule from a CSV file. Returns a vector containing every single entry for a class.
fn read_entries_from_csv_file(file_name: String) -> Vec<ScheduleEntry> {
    let mut entries: Vec<ScheduleEntry> = Vec::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let parse_result = parse_line_escaped(&line);

            if parse_result.len() != 8 {
                panic!("CSV must have 8 fields");
            }

            let entry = ScheduleEntry{
                day: parse_result[0].clone(),
                period: parse_result[1].clone(),
                frequency: parse_result[2].clone(),
                room: parse_result[3].clone(),
                group: parse_result[4].clone(),
                class_type: parse_result[5].clone(),
                discipline: parse_result[6].clone(),
                teacher: parse_result[7].clone(),
            };

            entries.push(entry);
        }
    }

    return entries;
}