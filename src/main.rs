mod simple_csv;

use std::{fs::{self}, io::{self, Write}};
use curl::easy::Easy;
use scraper::{Html, Selector};

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

/// Reads the user's yes/no answer to a question. Returns `Y` or `N` as a string.
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

fn read_input_number(min: usize, max: usize) -> usize {
    let mut input: String = String::new();
    let mut number: usize;

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from keyboard");

        number = input.trim().parse().unwrap_or(0);

        if number >= min && number <= max {
            break;

        } else {
            println!("Wrong input. Please enter a number between {} and {}.", min, max);
            input.clear();
        }
    }

    return number;
}

fn download_site_data() -> String {
    print!("Enter URL: ");
    io::stdout().flush().unwrap();
    
    let mut site_link: String = String::new();
    io::stdin()
            .read_line(&mut site_link)
            .expect("Failed to read from keyboard");

    println!("Downloading from {}", site_link);
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&site_link.trim()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let body = String::from_utf8(data).expect("body is not valid UTF8!");

    return body;
}

fn read_entries_from_html(body: String, table_number: usize) -> Vec<ScheduleEntry>{
    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut entries: Vec<ScheduleEntry> = Vec::new();
    for (index, table) in document.select(&table_selector).enumerate() {
        if index + 1 != table_number {
            continue;
        }

        for row in table.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            
            if !cells.is_empty() && cells.len() == 8 {
                let mut freq = cells[2].text().collect::<Vec<_>>().join("");
                
                if freq != "sapt. 1" && freq != "sapt. 2" {
                    freq = "".to_string();
                }

                let entry = ScheduleEntry{
                    day: cells[0].text().collect::<Vec<_>>().join(""),
                    period: cells[1].text().collect::<Vec<_>>().join(""),
                    frequency: freq,
                    room: cells[3].text().collect::<Vec<_>>().join(""),
                    group: cells[4].text().collect::<Vec<_>>().join(""),
                    class_type: cells[5].text().collect::<Vec<_>>().join(""),
                    discipline: cells[6].text().collect::<Vec<_>>().join(""),
                    teacher: cells[7].text().collect::<Vec<_>>().join(""),
                };
                
                entries.push(entry);
            }
        }
    }

    return entries;
}

/// something
fn main() {
    println!("Result will be saved in output.csv");
    println!("Enter the last digit of your group (1-7)");
    
    let group_number: usize = read_input_number(1, 7);
    let mut entries: Vec<ScheduleEntry> = read_entries_from_html(download_site_data(), group_number);
    
    println!("Are you part of the first semigroup? (Y/N)");
    let group = read_input_yes_no();

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
