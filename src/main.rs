mod ingest;
use crate::ingest::files;
use std::path::Path;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cfg_defaults() {
        let c: Config = Default::default();
        assert_eq!(c.local_doc_base, "./test_doc");
    }
}
