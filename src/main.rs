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

fn read_entries_from_file() -> Vec<ScheduleEntry> {
    let mut entries: Vec<ScheduleEntry> = Vec::new();
    let file_name = String::from("./schedule.csv");

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            let parse_result = parse_line_escaped(&line);
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

fn read_desired_classes_from_file() -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let file_name = String::from("./classes.txt");

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.flatten() {
            entries.push(line);
        }
    }

    return entries;
}

/// something
fn main() {
    println!("Make sure you have written the name of your classes in classes.txt as they are in the document");
    println!("Schedule will be taken from schedule.csv");
    println!("Result will be saved in output.csv");
    println!("Press any key to continue...");
    
    let mut group = String::new();

    io::stdin()
        .read_line(&mut group)
        .expect("Failed to read from keyboard");
    
    let entries = read_entries_from_file();
    let classes = read_desired_classes_from_file();
    
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

    println!("{:?} {:?}", entries, classes);
    
    for entry in entries.iter() {
        for class in classes.iter(){
            if entry.discipline.contains(class) {
                if let Err(e) = writeln!(output_file, "{}", entry.to_csv_row()) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }
}
