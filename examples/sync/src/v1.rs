#[derive(Default, Debug)]
pub struct Counter {
    value: usize,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }

    fn increase(&mut self) {
        self.value += 1;
    }

    fn value(&self) -> usize {
        self.value
    }
}

#[cfg(test)]
mod specs_for_counter {
    use super::Counter;

    #[test]
    fn sut_is_increased_3_times_and_leaves_it_at_3() {
        // Arrange
        let count = 3;
        let mut counter = Counter::new();

        // Act
        for _ in 0..count {
            counter.increase();
        }

        // Assert
        let actual = counter.value();
        assert_eq!(count, actual);
    }
}
