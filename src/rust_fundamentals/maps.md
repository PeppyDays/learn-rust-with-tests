# Maps

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/maps).

In [Arrays and Slices](./arrays_and_slices.md), you saw how to store values in order. Now, we will look at a way to store items by a key and look them up quickly.

## The First Requirement: Dictionary Search

Maps allow you to store items in a manner similar to a dictionary. You can think of the key as the word and the value as the definition. And what better way is there to learn about Maps than to build our own dictionary?

First, assuming we already have some words with their definitions in the dictionary, if we search for a word, it should return the definition of it.

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_dictionary_search {
    use super::Dictionary;

    #[test]
    fn sut_returns_value_if_key_exists_correctly() {
        // Arrange
        let dictionary = Dictionary::from([("test".to_string(), "value".to_string())]);

        // Act
        let actual = dictionary.search("test");

        // Assert
        assert_eq!(actual, "value");
    }
}
```

Now we added a test with a bit larger scope than as we did. With the test, we are trying to test three things:

- The dictionary is a struct or new type, anything but we don't want to use map type directly
- The dictionary is created correctly with seed entries
- The search method returns the correct value

If we're not confident about this, we can create some tests for creating a dictionary first with small scope. After that, add search functionality tests.

I'm pretty sure we're okay with this amount of scope. So let's keep going with it.

### Try to Run the Test

By running the test, we will see an error message complaining about the missing `Dictionary` object and the `search` method.

### Write the Minimal Amount of Code

```rust
use std::collections::HashMap;

pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn search(&self, key: &str) -> &str {
        ""
    }
}

impl<const N: usize> From<[(String, String); N]> for Dictionary {
    fn from(entries: [(String, String); N]) -> Self {
        Dictionary(HashMap::new())
    }
}
```

We implemented the `Dictionary` struct with [new type pattern](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) introduced in the [Errors](./errors.md) chapter.

We also implemented the `From` trait to create a dictionary from an array of tuples. The `From` trait is a standard library trait that allows you to convert one type into another. We can assume that constructing a new `Dictionary` with given entries is a conversion from the entries to `Dictionary`! `HashMap` does the same thing like [this](<https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-From%3C%5B(K,+V);+N%5D%3E-for-HashMap%3CK,+V%3E>).

Our test should now fail with a clear error message.

```bash
    thread 'specs_for_dictionary_search::sut_returns_value_if_key_exists_correctly' panicked at src/lib.rs:35:9:
    assertion `left == right` failed
      left: ""
     right: "value"
```

### Write Enough Code to Make the Test Pass

```rust
use std::collections::HashMap;

pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn search(&self, key: &str) -> &str {
        match self.0.get(key) {
            Some(value) => value,
            None => panic!("not defined!"),
        }
    }
}

impl<const N: usize> From<[(String, String); N]> for Dictionary {
    fn from(entries: [(String, String); N]) -> Self {
        Dictionary(HashMap::from(entries))
    }
}
```

Now the test should pass as follows.

```bash
 Nextest run ID f9955f9e-72f6-4269-94dc-437ad314e04a with nextest profile: default
    Starting 1 test across 1 binary
        PASS [   0.003s] maps specs_for_dictionary_search::sut_returns_value_if_key_exists_correctly
```

### Refactor

Nothing to refactor here, but `panic!` macro in `search` method is not a good idea. But, should we fix this right now? No! It just means we don't have enough specification written in tests to handle that kind of situation. So let's fix the specification for that situation, and add a test for that.

## The Second Requirement: Dictionary Search with Non-existing Entry

The basic search was very easy to implement, but what will happen if we supply a word that's not in our dictionary? We actually panic the application. Obviously, this is not a good idea, and we should return an error instead.

### Write the Test First

Let's write a test (and actually modify the previous one)!

```rust
#[cfg(test)]
mod specs_for_dictionary_search {
    use super::Dictionary;

    #[test]
    fn sut_returns_ok_with_value_if_key_exists_correctly() {
        // Arrange
        let dictionary = Dictionary::from([("test".to_string(), "value".to_string())]);

        // Act
        let actual = dictionary.search("test").unwrap();

        // Assert
        assert_eq!(actual, "value");
    }

    #[test]
    fn sut_returns_not_found_error_if_key_does_not_exists() {
        // Arrange
        let dictionary = Dictionary::new();

        // Act
        let actual = dictionary.search("test").unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::NotFound(_)));
        assert_eq!(actual.to_string(), "the key 'test' was not found");
    }
}
```

You can see several changes in the test:

- New constructor `new` is added to create an empty dictionary
- The `search` method now returns a `Result` type instead of `&str`
  - The previous test is now changed to use `unwrap` method to get the value
  - The new test is added to check the error case
  - The error is `DictionaryError` enum

As I mentioned, it is a bit bigger changer than as we did. If you feel it's too large and uncomfortable, you can always break it down into smaller scopes and follow TDD workflow.

### Try to Run the Test

This does not compile.

```bash
error[E0599]: no method named `unwrap` found for reference `&str` in the current scope
  --> src/lib.rs:38:48
   |
38 |         let actual = dictionary.search("test").unwrap();
   |                                                ^^^^^^ method not found in `&str`

error[E0599]: no method named `unwrap_err` found for reference `&str` in the current scope
  --> src/lib.rs:50:48
   |
50 |         let actual = dictionary.search("test").unwrap_err();
   |                                                ^^^^^^^^^^ method not found in `&str`

error[E0433]: failed to resolve: use of undeclared type `DictionaryError`
  --> src/lib.rs:53:34
   |
53 |         assert!(matches!(actual, DictionaryError::NotFound(_)));
   |                                  ^^^^^^^^^^^^^^^
   |                                  |
   |                                  use of undeclared type `DictionaryError`
   |                                  help: a struct with a similar name exists: `Dictionary`

```

### Write the Minimal Amount of Code

```rust,ignore
#[derive(Default)]
pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        match self.0.get(key) {
            Some(value) => Ok(value),
            None => Err(DictionaryError::NotFound("".to_string())),
        }
    }
}
```

Our test should now fail with a much clearer error message.

```bash
    thread 'specs_for_dictionary_search::sut_returns_not_found_error_if_key_does_not_exists' panicked at src/lib.rs:61:9:
    assertion `left == right` failed
      left: ""
     right: "the key 'test' was not found"
```

### Write Enough Code to Make It Pass

```rust,ignore
#[derive(Default)]
pub struct Dictionary(HashMap<String, String>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        match self.0.get(key) {
            Some(value) => Ok(value),
            None => Err(DictionaryError::NotFound(key.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("the key '{0}' was not found")]
    NotFound(String),
}
```

To make the tests pass, we implemented the `search` method to return a `Result` type. The `Result` type is an enum that can be either `Ok` or `Err`. We also created a new enum called `DictionaryError` to represent the error case. The `NotFound` variant of the enum takes a string as an argument, which is the key that was not found.

### Refactor

It looks okay, but if you do like functional style programming, you can chain methods to make `Result` in `search` method.

After getting a value from the internal `HashMap` with `self.0.get(key)`, you can get `Option<&String>` type. We changed from `Option<&String>` to `Result<&str, DictionaryError>` with pattern matching by splitting the `Option` type into two branches.

We can do the same thing with two chaining methods:

- `ok_or_else` method to convert `Option<T>` to `Result<T, E>`
- `map` method to convert `&String` to `&str`

Let's change the `search` method to use these methods.

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }
}
```

You can do as you prefer. I just wanted to show you that there are several ways to do the same thing in Rust. The important thing is to understand the concepts and how to use them effectively.

## The Third Requirement: Dictionary Add Entry

We have a great way to search the dictionary. However, we have no way to add new words to our dictionary.

### Write the Test First

In this test, we are utilizing our `search` function to make the validation of the dictionary a little easier.

```rust,ignore
#[cfg(test)]
mod specs_for_dictionary_add {
    use super::Dictionary;

    #[test]
    fn sut_adds_entry_to_be_able_to_search_it_later() {
        // Arrange
        let mut dictionary = Dictionary::new();

        // Act
        dictionary.add("test".to_string(), "value".to_string());

        // Assert
        let actual = dictionary.search("test").unwrap();
        assert_eq!("value", actual);
    }
}
```

### Write the Minimal Amount of Code

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) {}
}
```

Now test should fail with a clear error message.

```bash
    thread 'specs_for_dictionary_add::sut_adds_entry_to_be_able_to_search_it_later' panicked at src/lib.rs:49:48:
    called `Result::unwrap()` on an `Err` value: NotFound("test")
```

### Write Enough Code to Make It Pass

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }
}
```

### Refactor

There isn't much to refactor in our implementation.

## The Fourth Requirement: Dictionary Add Entry Already Exists

What happens if we try to add an entry that already exists in the dictionary? There could be many ways to handle this situation, such as overwriting or raising an error. Here, we raises an error if the entry already exists.

### Write the Test First

```rust,ignore
#[cfg(test)]
mod specs_for_dictionary_add {
    use super::Dictionary;

    #[test]
    fn sut_returns_ok_and_able_to_search_the_entry() {
        // Arrange
        let mut dictionary = Dictionary::new();

        // Act
        dictionary
            .add("test".to_string(), "value".to_string())
            .unwrap();

        // Assert
        let actual = dictionary.search("test").unwrap();
        assert_eq!("value", actual);
    }

    #[test]
    fn sut_raises_already_exists_error_if_entry_already_exists() {
        // Arrange
        let mut dictionary = Dictionary::from([("test".to_string(), "value1".to_string())]);

        // Act
        let actual = dictionary
            .add("test".to_string(), "value2".to_string())
            .unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::AlreadyExists(_)));
        assert_eq!(actual.to_string(), "the key 'test' already exists");
    }
}
```

For this test, we modified the previous test to check the result of `add` is `Ok` with `unwrap()`. We also added a new test to check the error case.

### Try to Run the Test

The compiler will fail because we are not returning a value from the `add` method.

```bash
error[E0599]: no method named `unwrap_err` found for unit type `()` in the current scope
  --> src/lib.rs:68:14
   |
66 |           let actual = dictionary
   |  ______________________-
67 | |             .add("test".to_string(), "value2".to_string())
68 | |             .unwrap_err();
   | |             -^^^^^^^^^^ method not found in `()`
   | |_____________|
```

### Write the Minimal Amount of Code

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        self.0.insert(key, value);
        Ok(())
    }
}
```

Now we get a different error message.

```bash
    thread 'specs_for_dictionary_add::sut_raises_already_exists_error_if_entry_already_exists' panicked at src
/lib.rs:72:14:
    called `Result::unwrap_err()` on an `Ok` value: ()
```

### Write Enough Code to Make It Pass

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        if self.0.contains_key(&key) {
            return Err(DictionaryError::AlreadyExists(key));
        }
        self.0.insert(key, value);
        Ok(())
    }
}
```

Here, we check if the key already exists in the dictionary with `contains_key` of `HashMap`. If it does, we return an error with `DictionaryError::AlreadyExists`. Otherwise, we insert the new entry into the dictionary.

### Refactor

With the current `add` method, we access the internal `HashMap` twice. We can reduce it to once by using the `entry` method of `HashMap`. The `entry` method returns an `Entry` enum including `Occupied` and `Vacant`. If the entry is occupied, we can return an error. If it is vacant, we can insert the new value.

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(DictionaryError::AlreadyExists(key)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }
}
```

## The Fifth Requirement: Dictionary Update Entry

Let's update the entry in the dictionary. As we experienced, we have to care about two cases - the key exists or not. If it exists, we can update the value. If it does not exist, we should raise an error.

### Write the Test First

We're fully accustomed to this kind of situation, so let's write the two tests together.

```rust,ignore
#[cfg(test)]
mod specs_for_dictionary_update {
    use super::Dictionary;
    use super::DictionaryError;

    #[test]
    fn sut_returns_ok_and_the_value_is_updated_correctly() {
        // Arrange
        let mut dictionary = Dictionary::from([("test".to_string(), "value1".to_string())]);

        // Act
        dictionary
            .update("test".to_string(), "value2".to_string())
            .unwrap();

        // Assert
        let actual = dictionary.search("test").unwrap();
        assert_eq!("value2", actual);
    }

    #[test]
    fn sut_returns_not_found_error_if_key_does_not_exists() {
        // Arrange
        let mut dictionary = Dictionary::new();

        // Act
        let actual = dictionary
            .update("test".to_string(), "value".to_string())
            .unwrap_err();

        // Assert
        assert!(matches!(actual, DictionaryError::NotFound(_)));
        assert_eq!(actual.to_string(), "the key 'test' was not found");
    }
}
```

### Write Enough Code to Make It Pass

We can see that the compiler error happens because we don't have the `update` method in our `Dictionary` struct. The implementation of `update` method is similar to `add` method, so let's do this fast.

```rust,ignore
impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }

    pub fn search(&self, key: &str) -> Result<&str, DictionaryError> {
        self.0
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| DictionaryError::NotFound(key.to_string()))
    }

    pub fn add(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(DictionaryError::AlreadyExists(key)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn update(&mut self, key: String, value: String) -> Result<(), DictionaryError> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(_) => Err(DictionaryError::NotFound(key)),
        }
    }
}
```

Easy peasy! Now we can run the test and see it pass.

```bash
 Nextest run ID eb8c9ea0-1c2b-45de-af7e-2784ea520e38 with nextest profile: default
    Starting 2 tests across 1 binary (14 tests skipped)
        PASS [   0.003s] maps specs_for_dictionary_update::sut_returns_not_found_error_if_key_does_not_exists
        PASS [   0.003s] maps specs_for_dictionary_update::sut_returns_ok_and_the_value_is_updated_correctly
────────────
     Summary [   0.004s] 2 tests run: 2 passed, 14 skipped
```

### Refactor

Nothing to refactor here. The code is already clean and easy to read.

## The Sixth Requirement: Dictionary Remove Entry

Now I believe you can do this by yourself. The `delete` method specification is as follows:

- If the key exists, remove the entry and return `Ok(())`
- If the key does not exist, return `Err(DictionaryError::NotFound(key))`

You can check the code in the repository guided in the beginning of this chapter.

## Wrapping Up

In this section, we covered a lot. We made a full CRUD(Create, Read, Update and Delete) API for our dictionary. Throughout the process we learned how to:

- Create a struct with new type pattern
- Create `HashMap`
- Search for items in `HashMap`
- Add new items to `HashMap`
- Update items in `HashMap`
- Learned more about errors
