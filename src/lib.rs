use std::fs::File;
use std::io::Write;

pub fn save_on_file(text: &str, path: &str) -> std::io::Result<()> {
    println!("Saving {} at {}", text, path);

    std::fs::create_dir_all(path)?;
    let mut f = File::create([path, "text.txt"].join("/"))?;
    f.write_all(text.as_bytes())?;
    let meta = f.metadata()?;
    println!("{}", meta.is_file());

    f.sync_all()?;
    Ok(())
}
