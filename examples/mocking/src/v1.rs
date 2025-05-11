use std::io::Write;

pub fn countdown(out: &mut dyn Write) {
    out.write_all(b"3").unwrap();
}

#[cfg(test)]
mod specs_for_countdown {
    use super::countdown;

    #[test]
    fn sut_writes_3() {
        // Arrangk
        let mut buffer = Vec::new();

        // Act
        countdown(&mut buffer);

        // Assert
        assert_eq!(String::from_utf8(buffer).unwrap(), "3");
    }
}
