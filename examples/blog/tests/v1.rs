use std::fs::File;

use tempfile::tempdir;

use blog::v1::Post;

#[test]
fn sut_loads_posts_from_files_correctly() {
    // Arrange
    let directory = tempdir().unwrap();
    let path = directory.path();
    let _ = File::create(path.join("post_1.md")).unwrap();
    let _ = File::create(path.join("post_2.md")).unwrap();

    // Act
    let posts = Post::from_directory(path).unwrap();

    // Assert
    assert_eq!(2, posts.len())
}
