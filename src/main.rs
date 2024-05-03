use std::fs;

use ignore::path_contains_ignore;

mod ignore;

fn get_all_files_in_directory_tree(dir: &str, ignores: &Vec<String>) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut files = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_dir() {
            let mut nested_files = get_all_files_in_directory_tree(&path_str, &ignores);
            files.append(&mut nested_files);
            continue;
        }

        if path_contains_ignore(&path_str, &ignores) {
            continue;
        }
        files.push(path_str);
    }
    files
}

fn main() {
    let current_dir = std::env::current_dir().unwrap().clone();
    let current_directory = match current_dir.to_str() {
        Some(dir) => dir,
        None => panic!("Could not get current directory"),
    };

    let gitignore_path = format!("{}/.gitignore", current_directory);
    let mut gitignore_text: Vec<String> = match fs::read_to_string(gitignore_path) {
        Ok(text) => text
            .lines()
            .filter(|line| line.len() > 0)
            .map(|line| line.to_string())
            .collect(),
        Err(_) => Vec::new(),
    };
    gitignore_text.append(&mut vec![String::from(".git")]);

    let files = get_all_files_in_directory_tree(current_directory, &gitignore_text);
    for file in files {
        let file_text = fs::read_to_string(&file).unwrap();
        let file = file.replacen(&current_directory, "", 1);
        let file = file.replacen("/", "", 1);
        println!("{}:\n{}", file, file_text);
    }
}
