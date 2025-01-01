fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Alantic University";
    let addr:&str = "km 52 Lekki-Epe Expressway, Ibeju-lekki, lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println("Department: {}, \nSchool: {}",department,school);
}