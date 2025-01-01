use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2  
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude  
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height  
}

fn main() {
    println!("Select an equation to perform calculations:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => {
            let (height, base1, base2) = read_trapezium_inputs();
            let area = area_of_trapezium(height, base1, base2);
            println!("Area of Trapezium: {}", area);
        },
        2 => {
            let (diagonal1, diagonal2) = read_rhombus_inputs();
            let area = area_of_rhombus(diagonal1, diagonal2);
            println!("Area of Rhombus: {}", area);
        },
        3 => {
            let (base, altitude) = read_parallelogram_inputs();
            let area = area_of_parallelogram(base, altitude);
            println!("Area of Parallelogram: {}", area);
        },
        4 => {
            let side = read_cube_input();
            let area = area_of_cube(side);
            println!("Area of Cube: {}", area);
        },
        5 => {
            let (radius, height) = read_cylinder_inputs();
            let volume = volume_of_cylinder(radius, height);
            println!("Volume of Cylinder: {}", volume);
        },
        _ => println!("Invalid choice"),
    }
}

fn read_trapezium_inputs() -> (f64, f64, f64) {
    let height = read_input("Enter the height: ");
    let base1 = read_input("Enter base1: ");
    let base2 = read_input("Enter base2: ");
    (height, base1, base2)
}

fn read_rhombus_inputs() -> (f64, f64) {
    let diagonal1 = read_input("Enter diagonal1: ");
    let diagonal2 = read_input("Enter diagonal2: ");
    (diagonal1, diagonal2)
}

fn read_parallelogram_inputs() -> (f64, f64) {
    let base = read_input("Enter base: ");
    let altitude = read_input("Enter altitude: ");
    (base, altitude)
}

fn read_cube_input() -> f64 {
    read_input("Enter the length of the side: ")
}

fn read_cylinder_inputs() -> (f64, f64) {
    let radius = read_input("Enter the radius: ");
    let height = read_input("Enter the height: ");
    (radius, height)
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}
