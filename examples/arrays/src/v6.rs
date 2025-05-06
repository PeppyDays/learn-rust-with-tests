pub fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    total
}

pub fn sum_all(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        sums.push(sum(numbers));
    }
    sums
}

pub fn sum_all_tails(numbers_to_sum: &[&[i32]]) -> Vec<i32> {
    let mut sums = Vec::with_capacity(numbers_to_sum.len());
    for numbers in numbers_to_sum {
        if numbers.is_empty() {
            sums.push(0);
        } else {
            sums.push(sum(&numbers[1..]));
        }
    }
    sums
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

#[cfg(test)]
mod specs_for_sum_all {
    use super::sum_all;

    #[test]
    fn sut_returns_two_summed_up_elements_if_two_arrays_are_given() {
        // Arrange
        let numbers_1 = [1, 2];
        let numbers_2 = [0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![3, 9];
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_two_summed_up_elements_if_two_vectors_are_given() {
        // Arrange
        let numbers_1 = vec![1, 2];
        let numbers_2 = vec![0, 9];

        // Act
        let actual = sum_all(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![3, 9];
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_sum_all_tails {
    use super::sum_all_tails;

    #[test]
    fn sut_returns_sum_of_each_collection_in_vector_correctly() {
        // Arrange
        let numbers_1 = vec![1, 2, 3];
        let numbers_2 = vec![0, 9, 10];

        // Act
        let actual = sum_all_tails(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![5, 19];
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_sets_summed_value_as_0_for_empty_collection() {
        // Arrange
        let numbers_1 = vec![];
        let numbers_2 = vec![3, 4, 5];

        // Act
        let actual = sum_all_tails(&[&numbers_1, &numbers_2]);

        // Assert
        let expected = vec![0, 9];
        assert_eq!(expected, actual);
    }
}
