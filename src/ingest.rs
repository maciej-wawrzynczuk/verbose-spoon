use std::path::{PathBuf, Path};
use walkdir::WalkDir;

pub fn files(p: &Path) -> Vec<PathBuf> {
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
    use crate::Config;
    use std::path::{Path, PathBuf};

    #[test]
    fn walk() {
        let c: Config = Default::default();
        let test_v = PathBuf::from("./test_doc/file");
        let files = files(Path::new(&c.local_doc_base));
        assert!(files.contains(&test_v));
    }
}
