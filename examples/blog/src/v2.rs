use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub title: String,
}

impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        let mut posts = vec![];
        let directory = read_dir(path)?;
        for file in directory {
            let file = file?;
            let data = Self::load_file(&file.path())?;
            let post = Post::from(data);
            posts.push(post);
        }
        Ok(posts)
    }

    fn load_file(path: &Path) -> Result<String, std::io::Error> {
        let mut data = String::new();
        File::open(path)?.read_to_string(&mut data)?;
        Ok(data)
    }
}

impl From<String> for Post {
    fn from(data: String) -> Self {
        let title_line = data.lines().next().unwrap();
        let title = title_line.strip_prefix("Title: ").unwrap().to_string();
        Post { title }
    }
}
