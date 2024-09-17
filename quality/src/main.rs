/*

Script de maintenance du contenu Dojo-101

Dépendances, code et compilation du script vérifié via github Action: Oui

Exécution du script lors des workflows : Non (Lancement et contrôles manuels) 

*/


use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use std::collections::HashSet;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE, ACCEPT_ENCODING};

fn main() {

    // 0. Lancement de répertoire Dojo-101

    let current_dir = std::env::current_dir().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    println!("\n[*] Dojo 101 path: {:?}", parent_dir);

    // 1. Affiche le nombre de fichiers, les fichiers qui ne sont pas au format markdown, et les 5 plus vieux fichiers

    let mut files = vec![];
    let mut non_markdown_files = vec![];

    for entry in WalkDir::new(parent_dir).min_depth(2).max_depth(3) {
        let entry = entry.unwrap();
        let path = entry.path();
       
        let mut dojo101_file = true; // Vérifie si le fichier est dans un dossier à ignorer
        for ancestor in path.ancestors() {
            let dir_name = ancestor.file_name().unwrap_or_default().to_str().unwrap();
            if dir_name == "quality" || dir_name.starts_with('.') {
                dojo101_file = false;
                break;
            }
        }

        if path.is_file() && dojo101_file {
            files.push(path.to_path_buf());
            if path.extension().unwrap_or_default() != "md" {
                non_markdown_files.push(path.to_path_buf());
            }
        }
    }
    
    println!("\n[*] Dojo-101 content files : {}", files.len());
    println!("\n[*] Non-markdown files: {:?}", non_markdown_files);

    files.sort_by_key(|path| {
        fs::metadata(path).unwrap().modified().unwrap()
    });

    println!("\n[*] 5 oldest files: {:?}", &files[..5.min(files.len())]);

    // 2. Vérifie qu'il n'y a pas de dossier dans les sous dossiers

    for entry in WalkDir::new(parent_dir).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if path.file_name().unwrap() == "quality" || dir_name.starts_with('.') { continue; }
            for sub_entry in WalkDir::new(path).min_depth(2).max_depth(2) {
                let sub_entry = sub_entry.unwrap();
                if sub_entry.path().is_dir() {
                    println!("\n[!] Sub-Directory contains sub-directory: {:?}", sub_entry.path());
                }
            }
        }
    }

    // 3. Extrait la liste des URLs contenues dans les liens markdown (sous forme "[ref](url)")

    let url_regex = Regex::new(r"\[([^\]]+)\]\((https?://[^\s\)]+)\)").unwrap();
    let mut urls = HashSet::new();

    for path in files.iter().filter(|p| p.extension().unwrap_or_default() == "md") {
        let content = fs::read_to_string(path).unwrap();
        for cap in url_regex.captures_iter(&content) {
            urls.insert(cap[2].to_string());
        }
    }
    //println!("\n[*] Unique URLs in markdown files: {:?}", urls);

    // 4. Vérifie la validité des URLs

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:130.0) Gecko/20100101 Firefox/130.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));

    for url in &urls {
        match client.get(url).headers(headers.clone()).send() {
            Ok(response) => {
                if response.status().is_success() {
                    //println!("\n[*] URL is valid: {}", url);
                } else {
                    println!("\n[!] URL is not valid: {} (status: {})", url, response.status());
                }
            }
            Err(err) => {
                println!("\n[!] Failed to check URL: {} (error: {})", url, err);
            }
        }
    }
}
