fn determine_position(role: &str, experience_years: u32) -> &str {
    if role == "Office Administrator" {
        if experience_years <= 2 {
            return "APS 1-2";
        } else if experience_years <= 5 {
            return "APS 3-5";
        } else {
            return "APS 5-8";
        }
    } else if role == "Academic" {
        if experience_years <= 2 {
            return "Intern";
        } else if experience_years <= 5 {
            return "PhD Candidate";
        } else {
            return "Senior Lecturer";
        }
    } else if role == "Lawyer" {
        if experience_years <= 2 {
            return "Paralegal";
        } else if experience_years <= 5 {
            return "Associate";
        } else {
            return "Senior Associate 1-2";
        }
    } else if role == "Teacher" {
        if experience_years <= 2 {
            return "Placement";
        } else if experience_years <= 5 {
            return "Senior Teacher";
        } else {
            return "Principal";
        }
    }
    
    "Unknown position"
}

fn main() {
    let member_data = vec![
        ("Lawyer", 6),
        ("Office Administrator", 3),
        ("Academic", 5),
        ("Teacher", 1),
    ];

    for (role, experience_years) in member_data {
        let position = determine_position(role, experience_years);
        println!("Role: {}, Experience: {} years - Position: {}", role, experience_years, position);
    }
}