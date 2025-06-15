use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use blog::Post;
use fake::Fake;
use fake::Faker;
use tempfile::tempdir;

#[rstest::rstest]
fn sut_loads_posts_from_files_correctly(posts: Vec<Post>) {
    // Arrange
    let directory = arrange(posts.clone());

    let actuals = Post::from_directory(&directory).unwrap();
    // Act

    // Assert
    assert_eq!(actuals.len(), posts.len());
    for actual in actuals {
        assert!(posts.contains(&actual));
    }
}

#[rstest::fixture]
fn post() -> Post {
    Post::new(
        &Faker.fake::<String>(),
        &Faker.fake::<String>(),
        vec![&Faker.fake::<String>(), &Faker.fake::<String>()],
        &Faker.fake::<String>(),
    )
}

#[rstest::fixture]
fn posts(#[default(5)] n: usize) -> Vec<Post> {
    (0..n).map(|_| post()).collect()
}

fn arrange(posts: Vec<Post>) -> PathBuf {
    let directory = tempdir().unwrap().keep();
    posts.iter().enumerate().for_each(|(n, post)| {
        let title = format!("Title: {}", post.title);
        let description = format!("Description: {}", post.description);
        let tags = format!("Tags: {}", post.tags.join(", "));
        let whole = format!(
            "{}\n{}\n{}\n---\n{}",
            title, description, tags, post.content,
        );

        let mut file = File::create(directory.join(format!("post_{}.md", n))).unwrap();
        file.write_all(whole.as_bytes()).unwrap();
    });
    directory
}

