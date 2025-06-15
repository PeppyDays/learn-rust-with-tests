use std::fs::File;
use std::io::Write;
use std::path::Path;

use fake::Fake;
use fake::Faker;
use tempfile::tempdir;

use blog::v3::Post;

#[rstest::rstest]
fn sut_loads_posts_from_files_correctly(posts: Vec<Post>) {
    // Arrange
    let directory = tempdir().unwrap();
    arrange_post_files(posts.clone(), directory.path());

    // Act
    let actuals = Post::from_directory(directory.path()).unwrap();

    // Assert
    assert_eq!(actuals.len(), posts.len());
    for actual in actuals {
        assert!(posts.contains(&actual));
    }
}

fn arrange_post_files(posts: Vec<Post>, directory: &Path) {
    for (n, post) in posts.iter().enumerate() {
        let title = format!("Title: {}", post.title);
        let description = format!("Description: {}", post.description);
        let data = format!("{}\n{}\n", title, description);

        let mut file = File::create(directory.join(format!("post_{}.md", n))).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}

#[rstest::fixture]
fn post() -> Post {
    let title = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    Post { title, description }
}

#[rstest::fixture]
fn posts(#[default(5)] n: usize) -> Vec<Post> {
    (0..n).map(|_| post()).collect()
}
