struct Commissioner {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Sample data  
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut merged_data: Vec<Commissioner> = Vec::new();

    // Merging the datasets  
    for i in 0..commissioners.len() {
        let commissioner = Commissioner {
            name: commissioners[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        };
        merged_data.push(commissioner);
    }

    // Display merged data  
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEO. ZONE");
    for (i, comm) in merged_data.iter().enumerate() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            comm.name,
            comm.ministry,
            comm.geopolitical_zone  
        );
    }
}