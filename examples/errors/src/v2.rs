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
}
