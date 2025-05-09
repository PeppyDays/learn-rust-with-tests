pub struct Wallet {
    balance: u64,
}

impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod specs_for_wallet {
    use super::Wallet;

    #[test]
    fn sut_deposits_correctly() {
        // Arrange
        let mut wallet = Wallet::open();

        // Act
        wallet.deposit(100);
        let actual = wallet.balance();

        // Assert
        assert_eq!(100, actual);
    }
}
