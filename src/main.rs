use std::{env, fs, io, path::{Path, PathBuf}};

fn category(ext: &str) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        "jpg"|"jpeg"|"png"|"gif"|"webp"|"svg" => "Images",
        "pdf"|"doc"|"docx"|"txt"|"md"|"ppt"|"pptx"|"xls"|"xlsx" => "Documents",
        "mp3"|"wav"|"flac"|"m4a" => "Audio",
        "mp4"|"mov"|"mkv"|"avi"|"webm" => "Video",
        "zip"|"rar"|"7z"|"tar"|"gz" => "Archives",
        "rs"|"py"|"js"|"ts"|"java"|"cpp"|"c"|"cs"|"go"|"kt"|"php" => "Code",
        _ => "Other",
    }
}

fn safe_destination(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !first.exists() { return first; }
    let p = Path::new(name);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
    for n in 1.. {
        let candidate = if ext.is_empty() { format!("{}_{}", stem, n) } else { format!("{}_{}.{}", stem, n, ext) };
        let path = dir.join(candidate);
        if !path.exists() { return path; }
    }
    unreachable!()
}

fn run(root: &Path, dry: bool, include_hidden: bool) -> io::Result<()> {
    let mut moved = 0;
    for item in fs::read_dir(root)? {
        let entry = item?;
        let path = entry.path();
        if !path.is_file() { continue; }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
        if !include_hidden && name.starts_with('.') { continue; }
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let target_dir = root.join(category(ext));
        let target = safe_destination(&target_dir, name);
        println!("{} -> {}", path.display(), target.display());
        if !dry { fs::create_dir_all(&target_dir)?; fs::rename(&path, &target)?; }
        moved += 1;
    }
    println!("{} file(s) {}", moved, if dry {"would be organized"} else {"organized"});
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() { eprintln!("Usage: cargo run -- <folder> [--dry-run] [--include-hidden]"); return; }
    let dry = args.iter().any(|a| a == "--dry-run");
    let include_hidden = args.iter().any(|a| a == "--include-hidden");
    let folder = args.iter().find(|a| *a != "--dry-run" && *a != "--include-hidden").unwrap();
    if let Err(e) = run(Path::new(folder), dry, include_hidden) { eprintln!("Error: {}", e); std::process::exit(1); }
}
