mod delete_rules;

use std::{env};
use std::path::{PathBuf, Path};
use walkdir::WalkDir;
use delete_rules::cleanup_rules::{CleanupRule, RULES};
use colored::Colorize;


fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!("Arguments: {:?}", args);\
    if args.len() < 2 {
        eprintln!("{}", "No mode specified. Use 'help' for the list of functions.".red());
        return;
    }
    let mode = &args[1];
    if mode == "help" {
        print_help();
        return;
    }
    if args.len() < 3 {
        eprintln!("{}", "No path specified. Use 'help' for the list of functions.".red());
        return;
    }
    let path = &args[2];
    let recursive = args.get(3).is_some_and(|arg| arg == "--recursive");
    let scan_path = Path::new(path);
    match mode.as_str() {
        "scan" if recursive => recursive_scan(scan_path),
        "clean" if recursive => recursive_clean(scan_path),
        "scan" => scan(scan_path),
        "clean" => clean(scan_path),
        "help" => print_help(),
        _ => eprintln!("Unknown mode: {}. Use 'help' for the list of functions.", mode.red()),
    }
}

fn print_help() {
    println!("{}", "janalyze - A tool to analyze and clean project directories".yellow());
    println!("{}", "-------------------------------------------------------------------------------------------".green());
    println!("{}", "|  janalyze <mode> <path> [--recursive]                                                   |".green());
    println!("{}", "-------------------------------------------------------------------------------------------".green());
    println!("{}", "Modes:".yellow());
    println!{"-------------------------------------------------------------------------------------------"};
    println!("|  scan <path>    | - Scan the specified directory for project types and cleanup targets. |");
    println!("|  clean <path>   | - Clean the specified directory by removing detected cleanup targets. |");
    println!("|  help           | - Display this help message.                                          |");
    println!("-------------------------------------------------------------------------------------------");
    println!("{}", "Options:".yellow());
    println!("-------------------------------------------------------------------------------------------");
    println!("|  --recursive - Recursively scan or clean all subdirectories.                            |");
    println!("-------------------------------------------------------------------------------------------");
}


fn scan(path: &Path) {
    let mut total_size: u64 = 0;

    println!("Scanning directory: {}", path.display());   
    println!("{}", check_project_type(path)
        .map_or("No project type detected".yellow(), 
        |rule| format!("Detected project type: {} \nFound anchor: <{}>", rule.name, rule.anchor).green()));
    
    let targets = find_cleanup_targets(path, check_project_type(path).unwrap_or(&CleanupRule {
        name: "Unknown",
        anchor: "",
        trash_targets: &[],
    }));

    for target in &targets {
        let size = calculate_dir_size(target);
        total_size += size;
    }

    if !targets.is_empty() {
        let output = targets
            .iter()
            .map(|target| {
                let size = calculate_dir_size(target);
                format!("Found target {} - {}", target.display(), format_bytes(size))
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", output.green());
        println!("{}", format!("Project potential cleanup size: {}", format_bytes(total_size as u64)).yellow());
    }
    else {
        println!("{}", "No cleanup targets found.".yellow());
    }
}

fn recursive_scan(path: &Path) {
    let mut total_size: u64 = 0;
    let mut it = WalkDir::new(path).into_iter();
    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry.file_type().is_dir() && check_project_type(entry_path).is_some() {
                if let Some(rule) = check_project_type(entry_path) {
                    let targets = find_cleanup_targets(entry_path, rule);
                    for target in &targets {
                        total_size += calculate_dir_size(target);
                    }
                }
                scan(entry_path);
                it.skip_current_dir();
            }
        }
    }
        println!("{}", "------------------------------------------------------------".green());
        println!("{}", format!("Total potential cleanup size: {}", format_bytes(total_size)).green());
        println!("{}", "------------------------------------------------------------".green());
}

fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|metadata| metadata.len())
        .sum()
}

fn check_project_type(path: &Path) -> Option<&'static CleanupRule> {
    for rule in RULES {
        let anchor_path = path.join(rule.anchor);
        if anchor_path.exists() { 
            return Some(rule);
        }
    }
    None
}
    

fn find_cleanup_targets(path: &Path, rule: &CleanupRule) -> Vec<PathBuf> {
    let mut targets = Vec::new();
    for target in rule.trash_targets {
        let target_path = path.join(target);
        if target_path.exists() {
            targets.push(target_path);
        }
    }
    targets
}

fn clean(path: &Path) { 
    let mut total_size: u64 = 0;
    if let Some(rule) = check_project_type(path) {
        let targets = find_cleanup_targets(path, rule);
        for target in &targets {
            let size = calculate_dir_size(target);
            total_size += size;
            println!("{}", format!("Found target: {}", target.display()).yellow());
            println!("{}", format!("Cleaned target: {} - {}", target.display(), format_bytes(size)).green());
            std::fs::remove_dir_all(target).ok();
        }
    } else {
        println!("No cleanup rules found for this directory.");
    }
    if total_size > 0 {
        println!("{}", format!("Total cleaned size: {}", format_bytes(total_size)).green())
    }
}

fn recursive_clean(path: &Path) {
    let mut total_size: u64 = 0;
    let mut it = WalkDir::new(path).into_iter();
    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry.file_type().is_dir() && check_project_type(entry_path).is_some() {
                    if let Some(rule) = check_project_type(entry_path) {
                        let targets = find_cleanup_targets(entry_path, rule);
                        for target in &targets {
                            total_size += calculate_dir_size(target);
                        }
                    }
                    clean(entry_path);
                    it.skip_current_dir();
            }
        }
    }
        println!("{}", "------------------------------------------------------------".green());
        println!("{}", format!("Total cleaned size: {}", format_bytes(total_size)).green());
        println!("{}", "------------------------------------------------------------".green());
}
        

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
