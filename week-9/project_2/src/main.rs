use std::io::{self, Write};
use std::fs::File;

fn main() {
    let mut name: Vec<String> = Vec::new();
    let mut matric: Vec<String> = Vec::new();
    let mut depar: Vec<String> = Vec::new();
    let mut level: Vec<i32> = Vec::new();

    let mut file = File::create("sibas.txt").unwrap();
    writeln!(file, "PAU SMIS").unwrap();
    writeln!(file, "{:<40}{:<40}{:<40}{:<40}", "Name", "Matric", "Department", "Level").unwrap();

    loop {
        let name1 = get_input("Enter your name or type 'done' when you are done: ");
        if name1 == "done" {
            break;
        }
        name.push(name1.clone());

        let mat = get_input("Enter your matric no: ");
        matric.push(mat.clone());

        let department = get_input("Enter your department: ");
        depar.push(department.clone());

        let lev: i32 = get_input("Enter your level: ").parse().unwrap_or(0);
        level.push(lev);

        let row = format!("{:<40}{:<40}{:<40}{:<40}\n", name1, mat, department, lev);
        file.write_all(row.as_bytes()).unwrap();
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is printed before input  
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}