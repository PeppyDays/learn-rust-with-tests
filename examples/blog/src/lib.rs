use std::fs::DirEntry;
use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
use std::path::Path;

const TITLE_PREFIX: &str = "Title: ";
const DESCRIPTION_PREFIX: &str = "Description: ";
const TAGS_PREFIX: &str = "Tags: ";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub content: String,
}

impl Post {
    pub fn new(title: &str, description: &str, tags: Vec<&str>, content: &str) -> Self {
        Post {
            title: title.to_string(),
            description: description.to_string(),
            tags: tags.iter().map(|&tag| tag.to_string()).collect(),
            content: content.to_string(),
        }
    }

    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        read_dir(path)?
            .filter_map(Result::ok)
            .map(|entry| {
                let content = Post::load_file(entry)?;
                let post = Post::from(content);
                Ok(post)
            })
            .collect()
    }

    fn load_file(file: DirEntry) -> Result<String, std::io::Error> {
        let mut content = String::new();
        File::open(file.path())?.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl From<String> for Post {
    fn from(content: String) -> Self {
        let lines = content
            .lines()
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>();
        let title = lines[0].strip_prefix(TITLE_PREFIX).unwrap_or("");
        let description = lines[1].strip_prefix(DESCRIPTION_PREFIX).unwrap_or("");
        let tags = lines[2]
            .strip_prefix(TAGS_PREFIX)
            .unwrap_or("")
            .split(", ")
            .collect();
        let content = lines[4..].join("\n");
        Post::new(title, description, tags, &content)
    }
}
