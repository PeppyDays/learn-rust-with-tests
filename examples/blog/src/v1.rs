use std::fs::read_dir;
use std::path::Path;

pub struct Post;

impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        let mut posts = vec![];
        let directory = read_dir(path)?;
        for _ in directory {
            posts.push(Post);
        }
        Ok(posts)
    }
}
