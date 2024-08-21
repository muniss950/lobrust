use regex::Regex;
use reqwest::blocking::get;
use std::collections::HashMap;
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
pub fn search(base: &str, query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let url = format!("https://{}/search/{}", base, query);
    let response = get(&url)?.text()?;
    // println!("Raw HTML Response:\n{}", response); // For debugging purposes
    let mut result = Vec::new();
    // Remove newlines and extra spaces
    let cleaned_response = response.replace('\n', "").replace('\r', "");

    // Compile the regex pattern to match and extract data
    let regex = Regex::new(
        r#"<div class="flw-item">.*?<img data-src="([^"]*)".*?<a href="/(tv|movie)/watch-.*?-(\d+)".*?title="([^"]*)".*?class="fdi-item">([^<]*)</span>"#,
    )?;

    // Apply the regex to the cleaned HTML content
    for cap in regex.captures_iter(&cleaned_response) {
        // Print the extracted information in the desired format
        let mut map = HashMap::new();

        // Insert values into the map
        map.insert("image_url".to_string(), cap[1].to_string());
        map.insert("media_type".to_string(), cap[2].to_string());
        map.insert("media_id".to_string(), cap[3].to_string());
        map.insert("title".to_string(), cap[4].to_string());
        map.insert("additional_info".to_string(), cap[5].to_string());

        // Push the map into the results vector
        result.push(map);
        // println!(
        //     "{}\t{}\t{}\t{} [{}]",
        //     image_url, id, kind, title, additional_info
        // );
    }

    Ok(result)
}

fn get_embed(
    base: &str,
    media_type: &str,
    media_id: &str,
    provider: &str,
) -> Result<String, Box<dyn Error>> {
    let episode_id: String;

    if media_type == "movie" {
        // Construct the movie page URL
        let movie_page_url = format!("https://{}/ajax/movie/episodes/{}", base, media_id);
        let movie_page = get(&movie_page_url)?.text()?;

        // Clean the movie page response
        let cleaned_movie_page = movie_page.replace('\n', "").replace('\r', "");

        // Regex to match the movie URL based on the provider
        let movie_url_regex = Regex::new(&format!(r#"href="([^"]*)"[^>]*title="{}""#, provider))?;
        if let Some(cap) = movie_url_regex.captures(&cleaned_movie_page) {
            let movie_url = cap
                .get(1)
                .ok_or("Could not extract movie URL")?
                .as_str()
                .to_string();

            // Regex to extract the episode ID from the movie URL
            let episode_id_regex = Regex::new(r".*-([0-9]+)\.([0-9]+)$")?;
            episode_id = episode_id_regex
                .captures(&movie_url)
                .and_then(|cap| cap.get(2).map(|m| m.as_str().to_string()))
                .ok_or("Could not extract episode ID")?;
        } else {
            return Err("Could not find movie URL with the specified provider".into());
        }
    } else {
        return Err("Unsupported media type".into());
    }

    // Fetch the embed link using the episode ID
    let embed_url = format!("https://{}/ajax/sources/{}", base, episode_id);
    let embed_page = get(&embed_url)?.text()?;

    // Clean the embed page response
    let cleaned_embed_page = embed_page.replace('\n', "").replace('\r', "");

    // Regex to match the embed link
    let embed_link_regex = Regex::new(r#""link":"([^"]*)""#)?;
    if let Some(cap) = embed_link_regex.captures(&cleaned_embed_page) {
        let embed_link = cap
            .get(1)
            .ok_or("Could not extract embed link")?
            .as_str()
            .replace(r"\u002F", "/"); // Unescape the URL

        Ok(embed_link)
    } else {
        Err("Could not find embed link".into())
    }
}

async fn json_from_id(source_id: &str) -> Result<String, Box<dyn Error>> {
    // Replace with the actual URL you are using to decrypt the ID
    let url = format!(
        "https://lobster-decryption.netlify.app/decrypt?id={}",
        source_id
    );

    // Make a request to the decryption service
    let response = reqwest::get(&url).await?.text().await?;

    Ok(response)
}

async fn get_json(embed_link: &str) -> Result<(), Box<dyn Error>> {
    // Regex to parse the embed link
    let parse_embed_regex = Regex::new(r"^(.*)/embed-(4|6)/(.*)\?z=$")?;

    let captures = parse_embed_regex
        .captures(embed_link)
        .ok_or("Failed to parse embed link")?;

    let _provider_link = captures.get(1).map_or("", |m| m.as_str());
    let _embed_type = captures.get(2).map_or("", |m| m.as_str());
    let source_id = captures.get(3).map_or("", |m| m.as_str());

    // Get JSON data from source ID
    let json_data = json_from_id(source_id).await?;

    if !json_data.is_empty() {
        extract_from_json(&json_data, Some("1080p"), "English", false)?; // Replace arguments with your actual needs
    } else {
        println!("Error: Could not get JSON data");
        return Err(Box::from("Could not get JSON data"));
    }

    Ok(())
}
// Placeholder for the extract_from_json function

fn extract_from_json(
    json_data: &str,
    quality: Option<&str>,
    subs_language: &str,
    json_output: bool,
) -> Result<(), Box<dyn Error>> {
    // Extract video link
    let video_regex = Regex::new(r#""file":"(.*?\.m3u8)""#)?;
    let mut video_link = video_regex
        .captures(json_data)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .ok_or("No video link found")?;

    if let Some(q) = quality {
        video_link = video_link.replace("/playlist.m3u8", &format!("/{}/index.m3u8", q));
    }

    // Output JSON if requested
    if json_output {
        println!("{}", json_data);
        return Ok(());
    }

    // Extract subtitles
    let subs_regex = Regex::new(&format!(
        r#""file":"([^"]*)","label":"{}[," ]"#,
        subs_language
    ))?;
    let mut subs_links = Vec::new();

    for cap in subs_regex.captures_iter(json_data) {
        if let Some(sub) = cap.get(1) {
            subs_links.push(sub.as_str().replace(":", "\\:"));
        }
    }

    if subs_links.is_empty() {
        println!("No subtitles found");
    } else {
        let subs_arg = if subs_links.len() > 1 {
            "--sub-files"
        } else {
            "--sub-file"
        };
        let subs_links_str = subs_links.join(",");
        println!("{} {}", subs_arg, subs_links_str);
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn test_search_success() {
        // Use the actual base URL of the website
        let base = "flixhq.to";
        let query = "joker";

        // Call the search function
        let result = search(&base, &query).expect("Search failed");

        // Print the result for manual inspection
        // println!("Search Result:\n{:?}", result) ;

        let provider = "UpCloud";
        let embed_link = get_embed(
            base,
            &result[0]["media_type"],
            &result[0]["media_id"],
            &provider,
        )
        .expect("lol");
        // println!("{:?}",x);
        let json_data = get_json(&embed_link).expect("lol");
        // println!("{:?}", json_data);
        let extracted =
            extract_from_json(&json_data, Some("1080p"), "English", false).expect("lol");

        // Optionally check if the result contains expected content
        // For a real test, you would need to inspect the actual website response
        // assert!(
        //     result.contains("Joker"),
        //     "Expected 'Joker' to be in the search results"
        // );
    }
}
