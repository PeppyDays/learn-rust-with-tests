pub fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}

#[cfg(test)]
mod specs_for_sum {
    use super::sum;

    #[test]
    fn sut_returns_15_if_input_array_is_1_to_5() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 15;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_6_if_input_array_is_1_to_3() {
        // Arrange
        let numbers = [1, 2, 3];

        // Act
        let actual = sum(&numbers);

        // Assert
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
