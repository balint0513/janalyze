mod delete_rules;

use std::{env};
use std::path::{PathBuf, Path};
use walkdir::WalkDir;
use delete_rules::cleanup_rules::{CleanupRule, RULES};
use colored::Colorize;
use std::collections::HashMap;

struct ProjectAnalysis {
    rule: &'static CleanupRule,
    targets_with_sizes: Vec<(PathBuf, u64)>,
    total_size: u64,
}


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
    let mut size_cache = HashMap::new();

    println!("Scanning directory: {}", path.display());   
    let Some(analysis) = analyze_project(path, &mut size_cache) else {
        println!("{}", "No project type detected".yellow());
        println!("{}", "No cleanup targets found".yellow());
        return;
    };
    print_scan_result(path, &analysis);
}

fn recursive_scan(path: &Path) {
    let mut total_size: u64 = 0;
    let mut size_cache = HashMap::new();
    let mut it = WalkDir::new(path).into_iter();
    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry.file_type().is_dir() {
                if let Some(analysis) = analyze_project(entry_path, &mut size_cache) {
                    print_scan_result(entry_path, &analysis);
                    total_size += analysis.total_size;
                    it.skip_current_dir();
     
                }
            }
        }
    }
        println!("{}", "------------------------------------------------------------".green());
        println!("{}", format!("Total potential cleanup size: {}", format_bytes(total_size)).green());
        println!("{}", "------------------------------------------------------------".green());
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
    let mut size_cache = HashMap::new();

    let Some(analysis) = analyze_project(path, &mut size_cache) else {
        println!("No cleanup rules found for this directory.");
        return;
    };

    let cleaned_size = clean_project(path, &analysis);
    if cleaned_size > 0 {
        println!("{}", format!("Cleaned size: {}", format_bytes(cleaned_size)).green());
    }
}

fn recursive_clean(path: &Path) {
    let mut total_size: u64 = 0;
    let mut size_cache = HashMap::new();
    let mut it = WalkDir::new(path).into_iter();
    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry.file_type().is_dir() {
                if let Some(analysis) = analyze_project(entry_path, &mut size_cache) {
                    total_size += clean_project(entry_path, &analysis);
                    it.skip_current_dir();
                    }
                }
            }
        }
    println!("{}", "------------------------------------------------------------".green());
    println!("{}", format!("Total cleaned size: {}", format_bytes(total_size)).green());
    println!("{}", "------------------------------------------------------------".green());
}

fn analyze_project(path: &Path, size_cache: &mut HashMap<PathBuf, u64>) -> Option<ProjectAnalysis> {
    let rule = check_project_type(path)?;
    let targets = find_cleanup_targets(path, rule);
    
    let mut total_size: u64 = 0;
    let mut targets_with_sizes = Vec::with_capacity(targets.len());

    for target in targets {
        let size = calculate_dir_size_cached(&target, size_cache);
        total_size += size;
        targets_with_sizes.push((target, size));
    }
    Some(ProjectAnalysis {
        rule,
        targets_with_sizes,
        total_size,
    })
}


fn print_scan_result(path: &Path, analysis: &ProjectAnalysis) {
    println! (
        "{}", 
        format!(
            "Detected project type: {} \n Found anchor: <{}>", 
            analysis.rule.name, analysis.rule.anchor)
        .green()
        );

    if analysis.targets_with_sizes.is_empty(){
        println!("{}", "No cleanup targets found".yellow());
        return;
    }

    let output = analysis
        .targets_with_sizes
        .iter()
        .map(|(target, size)| format!("Found target {} - {}", target.display(), format_bytes(*size)))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output.green());
    println!(
        "{}", 
        format!("Project potential cleanup size: ({}): {}",
        path.display(),
        format_bytes(analysis.total_size)
        )
        .green()
        );
}

fn clean_project(_path: &Path, analysis: &ProjectAnalysis) -> u64 {
    for (target,size) in &analysis.targets_with_sizes {
        println!("{}", format!("Found target {}", target.display()).yellow());
        println!(
            "{}",
            format!("Cleaning target {} - {}", target.display(), format_bytes(*size)).green()
        );
        std::fs::remove_dir_all(target).ok();
    }
    analysis.total_size
}


fn calculate_dir_size_cached(path: &Path, cache: &mut HashMap<PathBuf, u64>) -> u64 {
    let key = path.to_path_buf();
    if let Some(size) = cache.get(&key) {
        return *size;
    }

    let size: u64 = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.metadata().ok())
        .map(|metadata| metadata.len())
        .sum();
    
    cache.insert(key, size);
    size
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
