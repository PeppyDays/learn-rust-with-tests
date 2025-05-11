pub mod v1;

use std::io::Write;

pub fn greet(writer: &mut dyn Write, name: &str) {
    let greeting = format!("Hello, {}!", name);
    writer.write_all(greeting.as_bytes()).unwrap();
}

#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_writes_greeting_to_bytes_buffer_correctly() {
        // Arrange
        let mut buffer: Vec<u8> = Vec::new();

        // Act
        greet(&mut buffer, "Chris");

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "Hello, Chris!";
        assert_eq!(expected, actual);
    }
}
