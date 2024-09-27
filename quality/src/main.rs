/*
Script de maintenance du contenu Dojo-101.
Avant toute modificaiton, lire le workflow github correspondant.
*/

use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use std::collections::HashSet;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE, ACCEPT_ENCODING};

fn main() {
    let parent_dir = get_parent_directory();
    println!("\n[*] Dojo 101 path: {:?}", parent_dir);

    let (files, non_markdown_files) = get_files(&parent_dir);
    println!("\n[*] Dojo-101 content files : {}", files.len());
    println!("\n[*] Non-markdown files: {:?}", non_markdown_files);

    println!("\n[*] check subdirectories...");
    check_subdirectories(&parent_dir);

    let urls = extract_urls(&files);
    println!("\n[*] Number of unique URLs in markdown files: {}", urls.len());
    println!("\n[*] check urls...");
    check_urls(&urls);
}



fn get_parent_directory() -> std::path::PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.parent().unwrap().to_path_buf()
}

fn get_files(parent_dir: &std::path::Path) -> (Vec<std::path::PathBuf>, Vec<std::path::PathBuf>) {
    let mut files = vec![];
    let mut non_markdown_files = vec![];

    for entry in WalkDir::new(parent_dir).min_depth(1).max_depth(3) {
        let entry = entry.unwrap();
        let path = entry.path();

        if is_dojo101_file(path) {
            files.push(path.to_path_buf());
            if path.extension().unwrap_or_default() != "md" {
                non_markdown_files.push(path.to_path_buf());
            }
        }
    }
    (files, non_markdown_files)
}

fn is_dojo101_file(path: &std::path::Path) -> bool {
    for ancestor in path.ancestors() {
        let dir_name = ancestor.file_name().unwrap_or_default().to_str().unwrap();
        if dir_name == "quality" || dir_name == "images" || dir_name.starts_with('.') {
            return false;
        }
    }
    path.is_file()
}

fn check_subdirectories(parent_dir: &std::path::Path) {
    for entry in WalkDir::new(parent_dir).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name == "quality" || dir_name.starts_with('.') { continue; }
            for sub_entry in WalkDir::new(path).min_depth(2).max_depth(2) {
                let sub_entry = sub_entry.unwrap();
                if sub_entry.path().is_dir() {
                    println!("\n[!] Sub-Directory contains sub-directory: {:?}", sub_entry.path());
                }
            }
        }
    }
}

fn extract_urls(files: &[std::path::PathBuf]) -> HashSet<String> {
    let url_regex = Regex::new(r"\[([^\]]+)\]\((https?://[^\s\)]+)\)").unwrap();
    let mut urls = HashSet::new();

    for path in files.iter().filter(|p| p.extension().unwrap_or_default() == "md") {
        let content = fs::read_to_string(path).unwrap();
        for cap in url_regex.captures_iter(&content) {
            urls.insert(cap[2].to_string());
        }
    }
    urls
}

fn check_urls(urls: &HashSet<String>) {
    let client = Client::new();
    let headers = build_headers();

    for url in urls {
        match client.get(url).headers(headers.clone()).send() {
            Ok(response) => {
                if !response.status().is_success() {
                    println!("\n[!] URL is not valid: {} (status: {})", url, response.status());
                }
            }
            Err(err) => {
                println!("\n[!] Failed to check URL: {} (error: {})", url, err);
            }
        }
    }
}

fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:130.0) Gecko/20100101 Firefox/130.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers
}
