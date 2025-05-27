pub mod v1;

#[derive(Default)]
pub struct Counter {
    value: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increase(&mut self) {
        self.value += 1;
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

#[cfg(test)]
mod specs_for_counter {
    use super::Counter;

    #[tokio::test]
    async fn sut_increments_counter_3_times_leaves_it_at_3() {
        // Arrange
        let mut counter = Counter::new();

        // Act
        counter.increase();
        counter.increase();
        counter.increase();

        // Assert
        assert_eq!(3, counter.value())
    }

    #[tokio::test]
    async fn sut_runs_concurrently_safe() {
        // Arrange
        let count = 1000;
        let mut counter = Counter::new();

        // Act
        for _ in 0..count {
            counter.increase();
        }

        // Assert
    }
}
