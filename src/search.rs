
use std::process::Command;
use std::io::{self, Write};

fn search(base: &str, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://{}/search/{}", base, query);
    let response = reqwest::blocking::get(&url)?.text()?;
    
    let regex = regex::Regex::new(r#"<a href=".*?/(tv|movie)/watch-.*?-(\d+)".*?title="([^"]*)".*?class="fdi-item">([^<]*)</span>"#)?;

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
        return Err("No results found".into());
    }

    Ok(())
}

#[test]
fn test_search() {
    let base = "example.com"; // Replace with actual base URL
    let query = "test_query"; // Replace with actual query

    // Execute search function as a subprocess
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg(base)
        .arg(query)
        .output()
        .expect("Failed to execute search");

    // Check if the subprocess was successful
    if !output.status.success() {
        panic!("Search command failed: {:?}", output);
    }

    // Manually inspect the printed output (without assertions)
    io::stdout().write_all(&output.stdout).unwrap();
}
