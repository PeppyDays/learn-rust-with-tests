use std::io::Write;

const COUNTDOWN_START: usize = 3;
const FINAL_WORD: &str = "Go!";

pub fn countdown(out: &mut dyn Write) {
    for i in (1..=COUNTDOWN_START).rev() {
        out.write_all(format!("{}\n", i).as_bytes()).unwrap();
    }
    out.write_all(FINAL_WORD.as_bytes()).unwrap();
}

#[cfg(test)]
mod specs_for_countdown {
    use super::countdown;

    #[test]
    fn sut_writes_3_2_1_go() {
        // Arrange
        let mut buffer = Vec::new();

        // Act
        countdown(&mut buffer);

        // Assert
        let actual = String::from_utf8(buffer).unwrap();
        let expected = "3\n2\n1\nGo!";
        assert_eq!(expected, actual);
    }
}
