use std::fs::File;
use std::io::Write;
use std::path::Path;

use fake::Fake;
use fake::Faker;
use tempfile::tempdir;

use blog::v4::Post;

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
        dbg!(&actual);
        assert!(posts.contains(&actual));
    }
}

fn arrange_post_files(posts: Vec<Post>, directory: &Path) {
    for (n, post) in posts.iter().enumerate() {
        let title = format!("Title: {}", post.title);
        let description = format!("Description: {}", post.description);
        let tags = format!("Tags: {}", post.tags.join(", "));
        let data = format!("{}\n{}\n{}\n", title, description, tags);

        let mut file = File::create(directory.join(format!("post_{}.md", n))).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}

#[rstest::fixture]
fn post() -> Post {
    let title = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let tags = (0..Faker.fake::<u8>() % 10 + 1)
        .map(|_| Faker.fake::<String>())
        .collect();
    Post {
        title,
        description,
        tags,
    }
}

#[rstest::fixture]
fn posts(#[default(5)] n: usize) -> Vec<Post> {
    (0..n).map(|_| post()).collect()
}
