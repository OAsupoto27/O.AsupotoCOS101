use std::io::Read;

fn main() {
    println!("\nWhat is your user role?(A, B, C, D, E)");

    println!("\nA = Administrator");
    println!("B = Project Manager");
    println!("C = Employee");
    println!("D = Customer");
    println!("E = Vendor");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("invalid input");
    let input = input.trim();
    
    if input == "a" {
        println!("Welcome Administrator");
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("\n{}", contents);
    } 
    else if input == "b" {
        println!("Welcome Project Manager");
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents1 = String::new();
        file.read_to_string(&mut contents1).unwrap();
        println!("\n{}", contents1);
    } 
    else if input == "c" {
        println!("Welcome Employee");
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents2 = String::new();
        file.read_to_string(&mut contents2).unwrap();
        println!("\n{}", contents2);
    } 
    else if input == "d" {
        println!("Welcome Customer");
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents3 = String::new();
        file.read_to_string(&mut contents3).unwrap();
        println!("\n{}", contents3);
    } 
    else if input == "e" {
        println!("Welcome Vendor");
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents4 = String::new();
        file.read_to_string(&mut contents4).unwrap();
        println!("\n{}", contents4);
    } 
    else {
        println!("Invalid user role");
    }
      
}