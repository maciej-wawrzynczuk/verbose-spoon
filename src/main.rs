use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let c: Config = Default::default();

    let files = files(Path::new(&c.local_doc_base));

    for e in files {
        println!("{}", e.display());
    }
    Ok(())
}

struct Config {
    pub local_doc_base: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            local_doc_base: "./test_doc".into(),
        }
    }
}

fn files(p: &Path) -> Vec<PathBuf> {
    WalkDir::new(p)
        .into_iter()
        .filter_map(|e| e.ok()) // TODO log errors
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cfg_defaults() {
        let c: Config = Default::default();
        assert_eq!(c.local_doc_base, "./test_doc");
    }

    #[test]
    fn walk() {
        let c: Config = Default::default();
        let test_v = PathBuf::from("./test_doc/file");
        let files = files(Path::new(&c.local_doc_base));
        assert!(files.contains(&test_v));
    }
}
