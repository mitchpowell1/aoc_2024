use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

use reqwest::Method;

const AOC_SESSION_COOKIE_ENV_VAR: &str = "AOC_SESSION_COOKIE";
const AOC_BASE_URL: &str = "https://adventofcode.com/2024/day";

pub fn fetch_input(day: u32) -> String {
    let input_path_str = format!("inputs/day_{day}");
    let input_path = Path::new(input_path_str.as_str());
    if let Ok(input) = read_to_string(input_path) {
        return input;
    }

    let aoc_session_token = env::var(AOC_SESSION_COOKIE_ENV_VAR).unwrap_or_else(|_| panic!("Expected {AOC_SESSION_COOKIE_ENV_VAR} environment variable to be set. Cannot fetch input."));

    let url = format!("{AOC_BASE_URL}/{day}/input");
    let url = reqwest::Url::from_str(url.as_str()).expect("Bad URL");
    let jar = reqwest::cookie::Jar::default();
    let session_cookie = format!("session={aoc_session_token};");
    jar.add_cookie_str(session_cookie.as_str(), &url);

    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_provider(jar.into())
        .build()
        .unwrap();

    let request = client
        .request(Method::GET, url)
        .build()
        .expect("Failed to build request to fetch input");

    let out = client
        .execute(request)
        .expect("Failed to execute input request")
        .text()
        .expect("Failed to grab text from input response");

    std::fs::write(input_path, out.as_bytes()).expect("Failed to write input to filesystem");

    out
}
