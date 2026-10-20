use std::{env, fs, path::Path};

fn visit(path: &Path, total: &mut u64, files: &mut u64) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => { eprintln!("Skipping {}: {}", path.display(), error); return; }
    };
    for entry in entries.flatten() {
        let item = entry.path();
        if item.is_dir() { visit(&item, total, files); }
        else if let Ok(meta) = item.metadata() {
            if meta.is_file() { *total += meta.len(); *files += 1; }
        }
    }
}
fn main() {
    let folder = env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut bytes = 0;
    let mut files = 0;
    visit(Path::new(&folder), &mut bytes, &mut files);
    println!("Files counted: {files}");
    println!("Total size: {:.2} MiB", bytes as f64 / 1_048_576.0);
}
