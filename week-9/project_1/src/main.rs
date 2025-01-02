use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Open/Create the file `drinks.txt` in write mode  
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("drinks.txt")
        .expect("Cannot open file");

    // Writing the drinks categories to the file  
    write_to_file(&mut file, "Lager", &["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]);
    write_to_file(&mut file, "Stout", &["Legend", "Turbo King", "Williams"]);
    write_to_file(&mut file, "Non-Alcoholic", &["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]);

    println!("Drinks categories have been saved to drinks.txt");
}

// Function to write category and its drinks to the file  
fn write_to_file(file: &mut std::fs::File, category: &str, drinks: &[&str]) {
    writeln!(file, "{}:", category).expect("Write failed");
    for drink in drinks {
        writeln!(file, "  - {}", drink).expect("Write failed");
    }
    // Add a newline for better readability  
    writeln!(file).expect("Write failed");
}