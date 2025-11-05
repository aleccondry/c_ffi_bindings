use anyhow::anyhow;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let c_src_path = PathBuf::from("c_src");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    compile_c(&c_src_path)?;
    write_headers(&c_src_path, &out_dir)?;
    
    Ok(())
}

fn compile_c(src_path: &PathBuf) -> Result<()> {
    let src_files = walk_dir(src_path)?
        .into_iter()
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("c"))
        .collect::<Vec<_>>();
    cc::Build::new().files(src_files).compile("out_lib");
    Ok(())
}

fn write_headers(src_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let header_files: Vec<_> = walk_dir(src_path)?
        .into_iter()
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("h"))
        .collect();
    for header_file in header_files {
        let file_stem = header_file
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid header file name"))?;
        bindgen::Builder::default()
            .header(
                header_file
                    .to_str()
                    .ok_or_else(|| anyhow!("Invalid header file path"))?,
            )
            .use_core()
            .generate()?
            .write_to_file(out_dir.join(format!("{file_stem}.rs")))?;
    }
    Ok(())
}

fn walk_dir(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(walk_dir(&path)?);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}
