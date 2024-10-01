# Class scheduler written in Rust
Rust program to generate my schedule for university
Takes data from ```schedule.csv```, having the following structure/columns:

 DAY | TIME | FREQUENCY | ROOM | GROUP | TYPE | SUBJECT | TEACHER 
-----|------|------|------|------|------|------|------|

Having a different number of fields will break it, so please don't 🙃

Usage: ```schedule.csv``` must be in the same folder as the executable; run the program and select your desired classes. The result will be saved in ```output.csv```.
