use colored::Colorize;
use std::env;
use walkdir::WalkDir;

fn ignore(entry: &walkdir::DirEntry) -> bool {
    let ignored_dirs = [
        "node_modules",
        ".git",
        "target",
        "dist",
        "build",
        ".venv",
        "venv",
        ".idea",
        ".vscode",
        ".cache",
        "pycache",
        "bin",
        "next",
        "temp",
        "tmp",
        "vendor",
        ".next",
        ".nuxt",
        ".angular",
        ".mvn",
    ];
    entry.file_type().is_dir() && ignored_dirs.iter().any(|&x| x == entry.file_name())
}

fn print_tree(dir: &str, prefix: &str, depth: usize, max_depth: usize) {
    if depth > max_depth {
        println!("{}|-- ...", prefix);
        return;
    }

    let entries: Vec<_> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let conn = if is_last { "└── " } else { "├── " };
        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        let file_name = entry.file_name().to_string_lossy();
        if ignore(entry) {
            println!("{}{}{}", prefix, conn, file_name.red());
        } else if entry.file_type().is_dir() {
            println!("{}{}{}", prefix, conn, file_name.blue().bold());
            print_tree(
                entry.path().to_str().unwrap(),
                &new_prefix,
                depth + 1,
                max_depth,
            );
        } else {
            println!("{}{}{}", prefix, conn, file_name.green());
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let dir = if args.len() == 2 { &args[1] } else { "." };
    println!("\n{}:", dir);
    print_tree(dir, "", 0, 3);
    println!("\n")
}
