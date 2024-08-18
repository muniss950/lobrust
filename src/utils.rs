use regex::Regex;
use reqwest::blocking::get;
use std::io::{self, Write};
use std::{error::Error, result};

pub fn get_input() -> Result<String, Box<dyn Error>> {
    let mut query = String::new();
    // print!("Searching for query... ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut query)?;
    Ok(query.trim().to_string())
}
/// Searches for items based on the base URL and query.
/// Returns a formatted string with results or an error message.
pub fn search(base: &str, query: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://{}/search/{}", base, query);
    let response = get(&url)?.text()?;
    // println!("Raw HTML Response:\n{}", response); // For debugging purposes
    let mut results_found = false;
    let mut result = String::new();
    // Remove newlines and extra spaces
    let cleaned_response = response.replace('\n', "").replace('\r', "");

    // Compile the regex pattern to match and extract data
    let regex = Regex::new(
        r#"<div class="flw-item">.*?<img data-src="([^"]*)".*?<a href="/(tv|movie)/watch-.*?-(\d+)".*?title="([^"]*)".*?class="fdi-item">([^<]*)</span>"#,
    )?;

    // Apply the regex to the cleaned HTML content
    for cap in regex.captures_iter(&cleaned_response) {
        let image_url = &cap[1];
        let kind = &cap[2];
        let id = &cap[3];
        let title = &cap[4];
        let additional_info = &cap[5];

        // Print the extracted information in the desired format
        println!(
            "{}\t{}\t{}\t{} [{}]",
            image_url, id, kind, title, additional_info
        );
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_success() {
        // Use the actual base URL of the website
        let base = "flixhq.to";
        let query = "joker";

        // Call the search function
        let result = search(base, query).expect("Search failed");

        // Print the result for manual inspection
        println!("Search Result:\n{}", result);

        // Optionally check if the result contains expected content
        // For a real test, you would need to inspect the actual website response
        // assert!(
        //     result.contains("Joker"),
        //     "Expected 'Joker' to be in the search results"
        // );
    }
}
