fn main() {
    let num: i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of num is: {}", num);
}

fn mutate_num_to_zero(param_num: i32) {
    param_num = param_num; // This line won't actually change the value  
    println!("param_num value is: {}", param_num);
}
