# Maps

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/maps).

In [Arrays and Slices](./arrays_and_slices.md), you learned to store values in ordered sequences. Now we'll explore storing items by key for efficient lookup.

## The First Requirement: Dictionary Search

Maps store items like a dictionary - think of keys as words and values as definitions. What better way to learn maps than building our own dictionary?

First, assuming we have words with definitions in our dictionary, searching for a word should return its corresponding definition.

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

This test has broader scope than previous examples. We're validating three things:
- The dictionary is a struct or new type - we don't want to use the map type directly
- The dictionary creates correctly with initial entries
- The search method returns the correct value

If this scope feels uncomfortable, we could create smaller tests for dictionary creation first, then add search functionality afterward.

However, this scope feels manageable, so let's proceed.

### Try to Run the Test

Running the test shows an error about the missing `Dictionary` object and `search` method.

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

We implemented the `Dictionary` struct using the [new type pattern](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) introduced in the [Errors](./errors.md) chapter.

We also implemented the `From` trait to create a dictionary from an array of tuples. The `From` trait enables type conversion. We can consider constructing a new `Dictionary` with given entries as converting from entries to `Dictionary`. `HashMap` implements this same pattern [here](<https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-From%3C%5B(K,+V);+N%5D%3E-for-HashMap%3CK,+V%3E>).

Our test should now fail with a clear error message:

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

The test should now pass:

```bash
 Nextest run ID f9955f9e-72f6-4269-94dc-437ad314e04a with nextest profile: default
    Starting 1 test across 1 binary
        PASS [   0.003s] maps specs_for_dictionary_search::sut_returns_value_if_key_exists_correctly
```

### Refactor

Nothing to refactor here, but the `panic!` macro in the `search` method isn't ideal. Should we fix this now? No! It means we lack test specifications for that situation. Let's write specifications for that scenario by adding a test.

## The Second Requirement: Dictionary Search with Non-existing Entry

Basic search was straightforward, but what happens when we supply a word that's not in our dictionary? Currently, we panic the application. This is clearly not ideal - we should return an error instead.

### Write the Test First

Let's write a test (and modify the previous one):

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

Several changes are visible:
- A new constructor `new` is added to create an empty dictionary
- The `search` method now returns a `Result` type instead of `&str`
  - The previous test now uses `unwrap` to get the value
  - A new test checks the error case
  - The error is a `DictionaryError` enum

As mentioned, this is a larger change than previous iterations. If it feels too large or uncomfortable, you can break it into smaller scopes and follow the TDD workflow.

### Try to Run the Test

This doesn't compile:

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

Our test should now fail with a clearer error message:

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

To make tests pass, we implemented the `search` method to return a `Result` type. The `Result` type is an enum that can be either `Ok` or `Err`. We also created a new enum called `DictionaryError` to represent error cases. The `NotFound` variant takes a string representing the key that wasn't found.

### Refactor

This looks good, but if you prefer functional-style programming, you can chain methods to create the `Result` in the `search` method.

After getting a value from the internal `HashMap` with `self.0.get(key)`, you receive an `Option<&String>` type. We converted from `Option<&String>` to `Result<&str, DictionaryError>` using pattern matching by splitting the `Option` type into two branches.

We can achieve the same result with two chained methods:
- `ok_or_else` method to convert `Option<T>` to `Result<T, E>`
- `map` method to convert `&String` to `&str`

Let's modify the `search` method to use these methods:

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

You can choose whichever approach you prefer. This demonstrates multiple ways to accomplish the same goal in Rust. The important thing is understanding the concepts and using them effectively.

## The Third Requirement: Dictionary Add Entry

We have a great way to search the dictionary. However, we can't add new words to our dictionary.

### Write the Test First

In this test, we're utilizing our `search` function to make dictionary validation easier:

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

The test should now fail with a clear error message:

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

What happens if we try to add an entry that already exists? There are many ways to handle this - overwriting or raising an error. Here, we'll raise an error if the entry already exists.

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

For this test, we modified the previous test to check that `add` returns `Ok` using `unwrap()`. We also added a new test for the error case.

### Try to Run the Test

The compiler fails because we're not returning a value from the `add` method:

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

Now we get a different error message:

```bash
    thread 'specs_for_dictionary_add::sut_raises_already_exists_error_if_entry_already_exists' panicked at src/lib.rs:72:14:
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

Here, we check if the key already exists using `HashMap`'s `contains_key` method. If it does, we return an error with `DictionaryError::AlreadyExists`. Otherwise, we insert the new entry.

### Refactor

With the current `add` method, we access the internal `HashMap` twice. We can reduce this to once using the `entry` method. The `entry` method returns an `Entry` enum with `Occupied` and `Vacant` variants. If occupied, we return an error. If vacant, we insert the new value:

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

Let's update an entry in the dictionary. We need to handle two cases: whether the key exists. If it exists, we update the value. If it doesn't exist, we raise an error.

### Write the Test First

We're familiar with this situation, so let's write both tests together:

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

The compiler error occurs because we don't have the `update` method. The implementation is similar to the `add` method:

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

Simple! Now we can run the test and see it pass:

```bash
 Nextest run ID eb8c9ea0-1c2b-45de-af7e-2784ea520e38 with nextest profile: default
    Starting 2 tests across 1 binary (14 tests skipped)
        PASS [   0.003s] maps specs_for_dictionary_update::sut_returns_not_found_error_if_key_does_not_exists
        PASS [   0.003s] maps specs_for_dictionary_update::sut_returns_ok_and_the_value_is_updated_correctly
────────────
     Summary [   0.004s] 2 tests run: 2 passed, 14 skipped
```

### Refactor

Nothing to refactor here. The code is already clean and readable.

## The Sixth Requirement: Dictionary Remove Entry

You should now be able to implement this yourself. The `delete` method specification is:
- If the key exists, remove the entry and return `Ok(())`
- If the key doesn't exist, return `Err(DictionaryError::NotFound(key))`

You can check the code in the repository referenced at the beginning of this chapter.

## Wrapping Up

In this section, we covered significant ground. We built a full CRUD (Create, Read, Update, and Delete) API for our dictionary. Throughout the process, we learned how to:
- Create a struct with the new type pattern
- Create a `HashMap`
- Search for items in a `HashMap`
- Add new items to a `HashMap`
- Update items in a `HashMap`
- Work with errors in more detail