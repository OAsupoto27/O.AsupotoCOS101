fn main() {
    // Vector of candidates with (name, years of experience)
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 10),
        ("Charlie", 7),
        ("David", 12),
        ("Eve", 9),
    ];

    let mut highest_experience = 0;
    let mut top_candidate = "";

    // Iterate through candidates to find the one with the highest experience  
    for (name, experience) in candidates {
        if experience > highest_experience {
            highest_experience = experience;
            top_candidate = name;
        }
    }

    // Display the candidate with the highest experience  
    println!("The candidate with the highest years of programming experience is: {}, with {} years.", top_candidate, highest_experience);
}