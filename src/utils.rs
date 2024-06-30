// use crate::cli::Args; 
use std::io;
use reqwest::blocking::get;
use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::io::Write;

pub fn get_input(){
    let mut query= String::new();
    io::stdin().read_line(&mut query) 
        .ok()
        .expect("errror: {error}");
    println!("Querying {}..",query);
}
pub fn prompt_to_continue(){
    let mut query= String::new();
    io::stdin().read_line(&mut query) 
        .ok()
        .expect("errror: {error}");
    println!("Querying {}..",query);
}




fn search(base: &str, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://{}/search/{}", base, query);
    let response = get(&url)?.text()?;
    
    let regex = Regex::new(r#"<a href=".*?/(tv|movie)/watch-.*?-(\d+)".*?title="([^"]*)".*?class="fdi-item">([^<]*)</span>"#)?;

    let mut results_found = false;

    for cap in regex.captures_iter(&response) {
        results_found = true;
        let id = &cap[2];
        let kind = &cap[1];
        let title = &cap[3];
        let additional_info = &cap[4];
        println!("{} ({}) [{}]\t{}", id, kind, title, additional_info);
    }

    if !results_found {
        eprintln!("Error: No results found");
        return Err(Box::from("No results found"));
    }

    Ok(())
}

// #[test]
// fn test_search() {
//     let base = "flixhq.to"; // Replace with actual base URL
//     let query = "joker"; // Replace with actual query
//
//     // Capture printed output
//     let output = Command::new("cargo")
//         .args(&["run", "--quiet"])
//         .arg("--")
//         .arg(base)
//         .arg(query)
//         .output()
//         .expect("Failed to execute search");
//
//     // Check if the output was successful
//     assert!(output.status.success());
//
//     // Manually inspect the printed output (without assertions)
//     std::io::stdout().write_all(&output.stdout).unwrap();
// }
