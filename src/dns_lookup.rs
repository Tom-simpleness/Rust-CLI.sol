use anyhow::{Result, anyhow};
use reqwest;
use url::Url;

pub async fn lookup_dns_records(url: &str) -> Result<(String, usize)> {
    // Parse the URL and extract the domain
    let parsed_url = Url::parse(url).map_err(|e| anyhow!("Invalid URL: {}", e))?;
    let domain = match parsed_url.host_str() {
        Some(host) => host.to_string(),
        None => return Err(anyhow!("Failed to extract domain from URL")),
    };

    // Build the URL for the API request
    let api_url = format!("https://dns.google/resolve?name={}&type=ANY", domain);

    // Send the request
    let response = reqwest::get(&api_url).await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch DNS records. HTTP Status: {}", response.status()));
    }

    // Get the response body
    let response_body = response.text().await?;
    
    // Check if the response body is empty
    if response_body.is_empty() {
        return Err(anyhow!("Empty response body received from DNS API"));
    }

    // Parse the response body as JSON
    let json_response: serde_json::Value = serde_json::from_str(&response_body)
        .map_err(|e| anyhow!("Error decoding response body: {}", e))?;

    // Extract the number of DNS records from the "Answer" field
    let record_count = json_response["Answer"]
        .as_array()
        .map(|answers| answers.len())
        .unwrap_or(0);

    // Return the domain and the number of DNS records
    Ok((domain, record_count))
}
