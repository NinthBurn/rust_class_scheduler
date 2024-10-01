mod simple_csv;

use std::{fs::{self, File}, io::{self, BufRead, Write}, path::Path};
use simple_csv::parse_line_escaped;

#[derive(Debug)]
struct ScheduleEntry {
    day: String,
    period: String,
    frequency: String,
    room: String,
    group: String,
    class_type: String,
    discipline: String,
    teacher: String,
}

impl ScheduleEntry {
    fn to_csv_row(&self) -> String {
        let mut result_string = String::new();
        
        result_string.push_str(&self.day);
        result_string.push(',');
        result_string.push_str(&self.period);
        result_string.push(',');
        result_string.push_str(&self.frequency);
        result_string.push(',');
        result_string.push_str(&self.room);
        result_string.push(',');
        result_string.push_str(&self.group);
        result_string.push(',');
        result_string.push_str(&self.class_type);
        result_string.push(',');
        result_string.push_str(&self.discipline);
        result_string.push(',');
        result_string.push_str(&self.teacher);

        return result_string;
    }
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Reads the whole schedule from a file. Returns a vector containing every single entry for a class.
fn read_entries_from_file() -> Vec<ScheduleEntry> {
    let mut entries: Vec<ScheduleEntry> = Vec::new();
    let file_name = String::from("./schedule.csv");

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

/// Reads the user's yes/no answer to a question. Returns ```Y``` or ```N``` as a string.
fn read_input_yes_no() -> String {
    let mut input: String = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from keyboard");

        input = input.to_uppercase().trim().to_string();
        
        if input == "Y" || input == "N"{
            break;

        } else {
            println!("Wrong input. Please try again.");
            input.clear();
        }
    }

    return input;
}

/// something
fn main() {
    println!("Schedule will be taken from schedule.csv");
    println!("Result will be saved in output.csv");
    println!("Are you part of the first semigroup? (Y/N)");
    
    let group = read_input_yes_no();
    
    let mut entries: Vec<ScheduleEntry> = read_entries_from_file();
    let mut classes: Vec<String> = Vec::new();
    let mut desired_classes: Vec<String> = Vec::new();
    
    match group.as_str() {
        "Y" => entries.retain(|entry| !entry.group.contains("/2")),
        _ => entries.retain(|entry| !entry.group.contains("/1")),
    }

    for entry in entries.iter() {
        if !classes.contains(&entry.discipline) {
            classes.push(entry.discipline.clone());
        }
    }

    println!("Please select the classes you are signed up for:");
    for class in classes.iter() {
        println!("{} (Y/N)", class);
        let input = read_input_yes_no();
        match input.as_str() {
            "Y" => desired_classes.push(class.clone()),
            _ => (),
        }
    }

    let mut output_file = fs::OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open("output.csv")
                            .unwrap();

    let header = String::from("Ziua,Orele,Frecventa,Sala,Formatia,Tipul,Disciplina,Cadrul didactic");
    if let Err(e) = writeln!(output_file, "{}", header) {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    for entry in entries.iter() {
        for class in desired_classes.iter(){
            if entry.discipline.contains(class) {
                if let Err(e) = writeln!(output_file, "{}", entry.to_csv_row()) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }
}
