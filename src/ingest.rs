use std::path::{PathBuf, Path};
use color_eyre::eyre::Result;
use tokio::fs::read;
use walkdir::WalkDir;
use crate::data::Document;

pub fn files(p: &Path) -> Vec<PathBuf> {
    WalkDir::new(p)
        .into_iter()
        .filter_map(|e| e.ok()) // TODO log errors
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

pub async fn read_file(p: &Path) -> Result<Document> {
    let raw = read(p).await?;
    let c = String::from_utf8(raw)?;

    Ok(Document {
        content: c,
        file_location: p.to_owned()
    })
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

    #[tokio::test]
    async fn content() {
        let test_v = PathBuf::from("./test_doc/file");
        let c = read_file(&test_v).await.unwrap();
        assert_eq!(c.content, "foo bar\n".to_string());
        assert_eq!(c.file_location, test_v);
    }
}
