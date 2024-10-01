# Class scheduler written in Rust
Rust program to generate my schedule for university
Takes data from ```schedule.csv```, having the following structure/columns:

 DAY | TIME | FREQUENCY | ROOM | GROUP | TYPE | SUBJECT | TEACHER 
-----|------|------|------|------|------|------|------|

Having a different number of fields will break it, so please don't 🙃

Usage: ```schedule.csv``` must be in the same folder as the executable; run the program and select your desired classes. The result will be saved in ```output.csv```.

Usage example:

![terminal_input](https://github.com/user-attachments/assets/51b4ade0-c0fd-4493-be0d-d4e6fc585c82)

Results:

![result](https://github.com/user-attachments/assets/23581a1a-88fe-4c20-8078-44cc8f509290)
