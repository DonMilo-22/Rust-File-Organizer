use std::{collections::BTreeMap, env, fs, path::Path};

fn main() {
    let folder = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let entries = match fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("Cannot read {}: {}", folder, error);
            std::process::exit(1);
        }
    };
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() { continue; }
        let key = match Path::new(&path).extension().and_then(|ext| ext.to_str()) {
            Some(ext) => format!(".{}", ext.to_ascii_lowercase()),
            None => "[no extension]".to_string(),
        };
        *counts.entry(key).or_insert(0) += 1;
        total += 1;
    }
    println!("Files scanned: {total}");
    for (extension, count) in counts {
        println!("{extension:18} {count:>5}");
    }
}
