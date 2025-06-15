use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
use std::path::Path;

const TITLE_PREFIX: &str = "Title: ";
const DESCRIPTION_PREFIX: &str = "Description: ";
const TAGS_PREFIX: &str = "Tags: ";

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
}

impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        read_dir(path)?
            .filter_map(Result::ok)
            .map(|entry| {
                let path = &entry.path();
                let post = Self::from(Self::load_file(path)?);
                Ok(post)
            })
            .collect()
    }

    fn load_file(path: &Path) -> Result<String, std::io::Error> {
        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl From<String> for Post {
    fn from(content: String) -> Self {
        fn extract<'a>(lines: &mut impl Iterator<Item = &'a str>, prefix: &str) -> String {
            lines
                .next()
                .and_then(|line| line.strip_prefix(prefix))
                .unwrap()
                .to_string()
        }

        let mut lines = content.lines();
        let title = extract(&mut lines, TITLE_PREFIX);
        let description = extract(&mut lines, DESCRIPTION_PREFIX);
        let tags = extract(&mut lines, TAGS_PREFIX)
            .split(", ")
            .map(|tag| tag.to_string())
            .collect();
        let _separator = lines.next().unwrap();
        let body = lines.collect::<Vec<_>>().join("\n");
        Post {
            title,
            description,
            tags,
            body,
        }
    }
}
