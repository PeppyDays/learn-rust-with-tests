pub type BitCoin = u64;

pub struct Wallet {
    balance: BitCoin,
}

impl Wallet {
    pub fn open() -> Self {
        Self { balance: 0 }
    }

    pub fn deposit(&mut self, amount: BitCoin) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: BitCoin) {
        self.balance -= amount;
    }

    pub fn balance(&self) -> BitCoin {
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
        let amount = 100;

        // Act
        wallet.deposit(amount);
        let actual = wallet.balance();

        // Assert
        assert_eq!(amount, actual);
    }

    #[test]
    fn sut_withdraws_correctly() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        wallet.withdraw(10);
        let actual = wallet.balance();

        // Assert
        let expected = 10;
        assert_eq!(expected, actual);
    }
}
