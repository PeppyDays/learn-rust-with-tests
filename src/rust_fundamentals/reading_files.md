# Reading Files

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/blog).

## Overall Requirements

In this chapter, we'll explore how to read files, extract data from them, and put that data to good use.

Imagine you're collaborating with a friend to build blog software. Authors write their posts in markdown files, including metadata at the top. When the web server starts up, it reads a folder containing these files to create post objects, which then serve as the data source for the blog.

Our task is to create a package that transforms a folder of blog post files into a collection of post objects.

Here's what a typical blog post file looks like:

```plaintext
Title: Hello, TDD world!
Description: First post on our wonderful blog
Tags: tdd, go
---
Hello world!

The body of posts starts after the `---`
```

We'll convert this file into a `Post` struct like this:

```rust
pub struct Post {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
}
```

## Iterative, Test-driven Development

We'll work iteratively, taking simple and safe steps toward our goal.

While this means breaking our work into smaller pieces, we need to avoid the temptation of a "bottom-up" approach.

It's easy to let our imagination run wild at the start and create abstractions that we'll only validate when everything comes together—like building an elaborate blog post parser before we know if it's needed.

This approach isn't truly iterative and misses the quick feedback loops that make TDD so valuable.

Kent Beck says:

> Optimism is an occupational hazard of programming. Feedback is the treatment.

Instead, we should aim to deliver real value as quickly as possible by focusing on the "happy path." Once we've built something that works end-to-end, adding more features becomes much easier.

## Thinking About the Kind of Test

Before we begin, let's review our key principles:

- Write the test we'd want to see as users
- Consider how consumers would actually want to use our code
- Focus on the "what" and "why"—not the "how"

Our library should provide a simple function that takes a folder path and returns a collection of posts.

```rust,ignore
posts = Post::from_directory("path/to/posts");
```

To test this, we'd need a test folder with example posts. While this approach works, it comes with some trade-offs:

- Each test might require creating new files to test specific behaviors
- Some behaviors (like file loading failures) become harder to test
- Tests run slower due to file system access

We're also coupling our tests to the actual file system implementation.

## The First Requirement: Reading Files and Check File Count

### Write the Test First

Let's start small but meaningful. If we can prove our code reads all files in a directory, that's a solid foundation. We'll verify this by checking that the number of posts matches the number of files we create.

Create a new project and add these testing dependencies:

```toml
[dev-dependencies]
fake = "4.3.0"
rstest = "0.25.0"
tempfile = "3.20.0"
```

[tempfile](https://github.com/Stebalien/tempfile) is a crate that allows us to create temporary files and directories that are automatically cleaned up when they go out of scope. This is useful for testing without leaving behind any files. We will use it to create a temporary directory for our tests.

After that, add the following test to `tests/blog.rs`:

```rust,ignore
use std::fs::File;

use tempfile::tempdir;

#[test]
fn sut_loads_posts_from_files_correctly() {
    // Arrange
    let directory = tempdir().unwrap();
    let path = directory.path();
    let file_1 = File::create(path.join("post_1.md")).unwrap();
    let file_2 = File::create(path.join("post_2.md")).unwrap();

    // Act
    let posts = Post::from_directory(path);

    // Assert
    assert_eq!(2, posts.len());
}
```

Good TDD takes a consumer-driven approach—we test what users care about, not internal details. By placing our tests in the `tests` directory, we can only access public APIs, just like real users would.

We're using `tempfile::tempdir` to create temporary directories that automatically clean themselves up—perfect for our file-based tests.

Finally, we've defined how consumers will use our API and verified it creates the expected number of posts.

### Try to Run the Test

```bash
  --> tests/v1.rs:14:17
   |
14 |     let posts = Post::from_directory(path);
   |                 ^^^^ use of undeclared type `Post`
```

### Write the Minimal Amount of Code

Add just enough code to `src/lib.rs` to make the test compile:

```rust
use std::path::Path;

pub struct Post;

impl Post {
    pub fn from_directory(path: &Path) -> Vec<Post> {
        vec![]
    }
}
```

Perfect! The test now fails as expected:

```bash
thread 'sut_loads_posts_from_files_correctly' panicked at tests/v1.rs:19:5:
assertion `left == right` failed
  left: 2
  right: 0
```

### Write Enough Code to Make It Pass

We could "[slime](https://deniseyu.github.io/leveling-up-tdd/)" our way to a passing test:

```rust,ignore
impl Post {
    pub fn from_directory(path: &Path) -> Vec<Post> {
        vec![Post, Post]
    }
}
```

But, as Denise Yu wrote:

> Sliming is useful for giving a “skeleton” to your object. Designing an interface and executing logic are two concerns, and sliming tests strategically lets you focus on one at a time.

But we already have our structure in place. What should we do instead?

Since we've limited our scope, we simply need to read the directory and create a post for each file found. File parsing can wait for later.

```rust
use std::fs::read_dir;
use std::path::Path;

pub struct Post;

impl Post {
    pub fn from_directory(path: &Path) -> Vec<Post> {
        let mut posts = vec![];
        let directory = read_dir(path).unwrap();
        for _ in directory {
            posts.push(Post);
        }
        posts
    }
}
```

`std::fs::read_dir` gives us an iterator over directory entries. For now, we'll use it to count files and create a `Post` for each one. We'll do more with these entries later.

Reality intrudes—errors can happen! But since we're focused on making the test pass first, we'll handle errors properly later.

The rest is straightforward: iterate through entries, create a Post for each, and return the collection.

### Refactor

Nothing needs refactoring yet, so let's tackle the next requirement.

## The Second Requirement: Error Handling

Earlier, we put error handling aside to focus on the happy path. But file operations can fail in many ways—directories might not exist, files might be unreadable. Let's update our API to handle these cases properly (starting with our tests, of course).

```rust
#[test]
fn sut_loads_posts_from_files_correctly() {
    // Arrange
    let directory = tempdir().unwrap();
    let path = directory.path();
    let file_1 = File::create(path.join("post_1.md")).unwrap();
    let file_2 = File::create(path.join("post_2.md")).unwrap();

    // Act
    let posts = Post::from_directory(path).unwrap();

    // Assert
    assert_eq!(2, posts.len());
}
```

By adding `unwrap()` to our `Post::from_directory` call, we're signaling that it should return a `Result<Vec<Post>, Error>` instead of just `Vec<Post>`. The test setup ensures the method succeeds.

Run the test—it'll complain that `unwrap()` doesn't exist on our type. The fix is simple:

```rust,ignore
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
```

The test passes! Your inner TDD purist might object that we didn't see a failing test for error propagation. To do that, we'd need a test with an invalid directory.

While testing error handling is often important, here we're just propagating errors without any special handling. Writing a test for this wouldn't add much value.

Next, let's make our Post type actually useful by adding real data to it.

## The Third Requirement: Reading Post Title

### Write the Test First

Let's start with the title field—the first line in our blog post format.

We'll update our test files to match the specified format and verify that titles are parsed correctly.

```rust
#[test]
fn sut_loads_posts_from_files_correctly() {
    // Arrange
    let directory = tempdir().unwrap();
    let path = directory.path();
    let mut file_1 = File::create(path.join("post_1.md")).unwrap();
    file_1.write_all("Title: Post 1".as_bytes()).unwrap();
    let mut file_2 = File::create(path.join("post_2.md")).unwrap();
    file_2.write_all("Title: Post 2".as_bytes()).unwrap();

    // Act
    let posts = Post::from_directory(path).unwrap();

    // Assert
    assert_eq!(2, posts.len());
    assert_eq!("Post 1", posts[0].title);
    assert_eq!("Post 2", posts[1].title);
}
```

### Try to Run the Test

```bash
error[E0609]: no field `title` on type `blog::v2::Post`
  --> tests/v2.rs:24:35
   |
24 |     assert_eq!("Post 2", posts[1].title);
   |                                   ^^^^^ unknown field
```

### Write the Minimal Amount of Code

Add the title field to make our test compile:

```rust,ignore
pub struct Post {
    pub title: String,
}

impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        let mut posts = vec![];
        let directory = read_dir(path)?;
        for _ in directory {
            posts.push(Post { title: "".into() });
        }
        Ok(posts)
    }
}
```

Run the test again for a nice, clear failure:

```bash
thread 'sut_loads_posts_from_files_correctly' panicked at tests/v2.rs:23:5:
assertion `left == right` failed
  left: "Post 1"
  right: ""
```

### Write Enough Code to Make It Pass

Now we need to open each file and extract its title.

```rust,ignore
impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        let mut posts = vec![];
        let directory = read_dir(path)?;
        for file in directory {
            let file = file?;

            let mut data = String::new();
            File::open(file.path())?.read_to_string(&mut data)?;

            let title = data.strip_prefix("Title: ").unwrap();
            let post = Post {
                title: title.to_string(),
            };

            posts.push(post);
        }
        Ok(posts)
    }
}
```

Remember, we're not aiming for elegant code yet—just working software.

Even this small step required significant code and error handling decisions. This is a good time to discuss approaches with your team.

Our iterative approach quickly revealed gaps in our requirements understanding—valuable feedback!

We read the file contents and, for now, use simple string manipulation to extract the title—no fancy parsing needed yet.

### Refactor

Let's separate file reading from content parsing to make our code clearer and easier to work with.

First, let's pull the file reading logic into its own function:

```rust,ignore
impl Post {
    pub fn from_directory(path: &Path) -> Result<Vec<Post>, std::io::Error> {
        let mut posts = vec![];
        let directory = read_dir(path)?;
        for file in directory {
            let file = file?;
            let data = Self::load_file(&file.path())?;

            let title = data.strip_prefix("Title: ").unwrap();
            let post = Post {
                title: title.to_string(),
            };

            posts.push(post);
        }
        Ok(posts)
    }

    fn load_file(path: &PathBuf) -> Result<String, std::io::Error> {
        let mut data = String::new();
        File::open(path)?.read_to_string(&mut data)?;
        Ok(data)
    }
}
```

Next, let's extract the title parsing. Rust's idiomatic approach is to implement the `From` trait for type conversions:

```rust,ignore
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
```

Now all our parsing logic lives in `From<String> for Post`. With file handling out of the way, we can focus purely on extracting post data.

Let's improve our tests. Instead of manually creating files, we'll use `rstest` for fixtures and `fake` for realistic random data:

```rust,ignore
fn arrange_post_files(posts: Vec<Post>, directory: &Path) {
    for (n, post) in posts.iter().enumerate() {
        let title = format!("Title: {}", post.title);

        let mut file = File::create(directory.join(format!("post_{}.md", n))).unwrap();
        file.write_all(title.as_bytes()).unwrap();
    }
}

#[rstest::fixture]
fn post() -> Post {
    let title = Faker.fake::<String>();
    Post { title }
}

#[rstest::fixture]
fn posts(#[default(5)] n: usize) -> Vec<Post> {
    (0..n).map(|_| post()).collect()
}
```

With `rstest::fixture`, we get reusable test fixtures that generate random posts. Our `arrange_post_files` helper creates the actual files. Here's how we use them:

```rust,ignore
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
```

Our test is now more robust—it handles any number of posts with randomly generated data, making it more realistic and less brittle.

We'll need to derive `Clone` and `PartialEq` for `Post` to enable comparisons in our tests.

## The Fourth Requirement: Reading Post Description

### Write the Test First

Let's add support for the description field. By now, the TDD rhythm should feel natural.

```rust,ignore
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
```

Our test helpers now include descriptions, allowing us to verify they're parsed correctly.

### Try to Run the Test

```bash
error[E0609]: no field `description` on type `&blog::v3::Post`3(test)
  --> tests/v3.rs:30:59
   |
30 |         let description = format!("Description: {}", post.description);
   |                                                           ^^^^^^^^^^^ unknown field
   |
   = note: available field is: `title`

error[E0560]: struct `blog::v3::Post` has no field named `description`
  --> tests/v3.rs:42:19
   |
42 |     Post { title, description }
   |                   ^^^^^^^^^^^ `blog::v3::Post` does not have this field
   |
   = note: all struct fields are already assigned
```

### Write the Minimal Amount of Code

Add the new field to `Post`.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub title: String,
    pub description: String,
}

impl From<String> for Post {
    fn from(data: String) -> Self {
        let title_line = data.lines().next().unwrap();
        let title = title_line.strip_prefix("Title: ").unwrap().to_string();
        Post {
            title,
            description: String::new(),
        }
    }
}
```

The tests compile but fail—exactly what we want.

```bash
thread 'sut_loads_posts_from_files_correctly' panicked at tests/v3.rs:23:9:
assertion failed: posts.contains(&actual)
```

### Write Enough Code to Make It Pass

```rust,ignore
impl From<String> for Post {
    fn from(data: String) -> Self {
        let mut lines = data.lines();
        let title_line = lines.next().unwrap();
        let title = title_line.strip_prefix("Title: ").unwrap().to_string();
        let description_line = lines.next().unwrap();
        let description = description_line
            .strip_prefix("Description: ")
            .unwrap()
            .to_string();
        Post { title, description }
    }
}
```

We simply use `lines()` to iterate through the string and `next()` to grab the title and description lines.

### Refactor

Let's extract those hard-coded strings into constants and streamline our code by combining the line reading and prefix stripping:

```rust,ignore
const TITLE_PREFIX: &str = "Title: ";
const DESCRIPTION_PREFIX: &str = "Description: ";

impl From<String> for Post {
    fn from(data: String) -> Self {
        let mut lines = data.lines();
        let title = lines
            .next()
            .unwrap()
            .strip_prefix(TITLE_PREFIX)
            .unwrap()
            .to_string();
        let description = lines
            .next()
            .unwrap()
            .strip_prefix(DESCRIPTION_PREFIX)
            .unwrap()
            .to_string();
        Post { title, description }
    }
}
```

Looking at this code, I see an opportunity to refactor `from_directory` in a more functional style:

```rust,ignore
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
```

You might love or hate this style—that's fine! The beauty of refactoring with tests is the freedom to experiment. We can try different approaches and always roll back if needed. TDD gives us the safety net to explore and find the best solution.

## The Fifth Requirement: Reading Post Tags

Next up: extracting tags. If you're coding along, try implementing this yourself first! You should have a good feel for the TDD rhythm by now.

To save space, I'll skip the TDD steps and show you the test with tags support:

```rust,ignore
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
```

Remember, copying and pasting won't help you learn! But here's my implementation for reference:

```rust,ignore
const TITLE_PREFIX: &str = "Title: ";
const DESCRIPTION_PREFIX: &str = "Description: ";
const TAGS_PREFIX: &str = "Tags: ";

impl From<String> for Post {
    fn from(data: String) -> Self {
        fn extract<'a>(lines: &mut impl Iterator<Item = &'a str>, prefix: &str) -> String {
            lines
                .next()
                .and_then(|line| line.strip_prefix(prefix))
                .unwrap()
                .to_string()
        }

        let mut lines = data.lines();
        let title = extract(&mut lines, TITLE_PREFIX);
        let description = extract(&mut lines, DESCRIPTION_PREFIX);
        let tags = extract(&mut lines, TAGS_PREFIX)
            .split(", ")
            .map(|tag| tag.to_string())
            .collect();
        Post {
            title,
            description,
            tags,
        }
    }
}
```

Pretty straightforward—we split tags on commas and extracted the common prefix-stripping logic into a helper function to keep things DRY.

## The Sixth Requirement: Reading Post Body

Our final happy-path feature is extracting the post body. Here's a reminder of our file format:

```plaintext
Title: Hello, TDD world!
Description: First post on our wonderful blog
Tags: tdd, go
---
Hello world!

The body of posts starts after the `---`
```

We've handled the first three lines. Now we need to skip the separator line and capture everything else as the body.

### Write the Test First

Let's update our test data with the separator and a multi-line body to ensure we capture everything:

```rust,ignore
#[rstest::fixture]
fn post() -> Post {
    let title = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let tags = (0..Faker.fake::<u8>() % 10 + 1)
        .map(|_| Faker.fake::<String>())
        .collect();
    let body = Paragraph(3..10).fake::<String>();
    Post {
        title,
        description,
        tags,
        body,
    }
}

fn arrange_post_files(posts: Vec<Post>, directory: &Path) {
    for (n, post) in posts.iter().enumerate() {
        let title = format!("Title: {}", post.title);
        let description = format!("Description: {}", post.description);
        let tags = format!("Tags: {}", post.tags.join(", "));
        let content = format!("{}\n{}\n{}\n---\n{}", title, description, tags, post.body);

        let mut file = File::create(directory.join(format!("post_{}.md", n))).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
```

### Try to Run the Test

```bash
error[E0609]: no field `body` on type `&blog::v4::Post`bin), v5(test)
  --> tests/v5.rs:34:85
   |
34 |         let content = format!("{}\n{}\n{}\n---\n{}", title, description, tags, post.body);
   |                                                                                     ^^^^ unknown field
   |
   = note: available fields are: `title`, `description`, `tags`
```

Just as expected!

### Write the Minimal Amount of Code

```rust,ignore
#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
}

impl From<String> for Post {
    fn from(data: String) -> Self {
        fn extract<'a>(lines: &mut impl Iterator<Item = &'a str>, prefix: &str) -> String {
            lines
                .next()
                .and_then(|line| line.strip_prefix(prefix))
                .unwrap()
                .to_string()
        }

        let mut lines = data.lines();
        let title = extract(&mut lines, TITLE_PREFIX);
        let description = extract(&mut lines, DESCRIPTION_PREFIX);
        let tags = extract(&mut lines, TAGS_PREFIX)
            .split(", ")
            .map(|tag| tag.to_string())
            .collect();
        Post {
            title,
            description,
            tags,
            body: String::new(),
        }
    }
}
```

### Write Enough Code to Make It Pass

We'll skip the `---` separator and collect all remaining lines into the body:

```rust,ignore
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
```

### Refactor

Looks good to me—no refactoring needed.

## Iterating Further

We've built our "steel thread"—a working path through the system—but there's still work to do before it's production-ready.

Still to handle:

- Invalid file formats
- Non-markdown files
- Different metadata field ordering—should we support this?

But here's the key: we have working software with a clear interface. Those missing features are just more iterations away. Best of all, we can add them without changing our design—just the implementation.

By staying focused on our goal, we made the important decisions early and validated them against real behavior, instead of getting lost in details that don't affect the core design.

## Wrapping Up

Our tests serve as living documentation—users can read them to understand how our library works. As maintainers, we know these tests are valuable because they focus on what consumers care about, not implementation details. This means our tests will help, not hinder, future refactoring.

Good engineering practices like dependency injection keep our code testable and reusable.

When building libraries (even internal ones), take a top-down, consumer-driven approach. This prevents over-engineering and ensures your tests actually matter.

Our iterative approach kept each step manageable, and the constant feedback helped us discover requirement gaps early—much sooner than we might have with a less structured approach.
