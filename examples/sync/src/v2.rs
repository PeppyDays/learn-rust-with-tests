use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct Counter {
    value: RwLock<usize>,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }

    fn increase(&self) {
        *self.value.write().unwrap() += 1;
    }

    fn value(&self) -> usize {
        *self.value.read().unwrap()
    }
}

#[cfg(test)]
mod specs_for_counter {
    use std::sync::Arc;

    use futures::future::join_all;

    use super::Counter;

    #[test]
    fn sut_is_increased_3_times_and_leaves_it_at_3() {
        // Arrange
        let count = 3;
        let counter = Counter::new();

        // Act
        for _ in 0..count {
            counter.increase();
        }

        // Assert
        let actual = counter.value();
        assert_eq!(count, actual);
    }

    #[tokio::test]
    async fn sut_runs_concurrently_safe() {
        // Arrange
        let count = 1000;
        let counter = Arc::new(Counter::new());

        // Act
        let handles = (0..count)
            .map(|_| {
                let counter = Arc::clone(&counter);
                tokio::spawn(async move {
                    counter.increase();
                })
            })
            .collect::<Vec<_>>();
        join_all(handles).await;

        // Assert
        let actual = counter.value();
        assert_eq!(actual, count);
    }
}
