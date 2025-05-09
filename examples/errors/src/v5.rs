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

    pub fn withdraw(&mut self, amount: BitCoin) -> Result<(), WalletError> {
        if amount > self.balance {
            return Err(WalletError::InsufficientFunds(String::from(
                "cannot withdraw, insufficient funds",
            )));
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn balance(&self) -> BitCoin {
        self.balance
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("{0}")]
    InsufficientFunds(String),
}

#[cfg(test)]
mod specs_for_wallet {
    use rstest::rstest;

    use super::BitCoin;
    use super::Wallet;
    use super::WalletError;

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
        _ = wallet.withdraw(10);
        let actual = wallet.balance();

        // Assert
        let expected = 10;
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(20, 10)]
    #[case(20, 20)]
    fn sut_returns_ok_if_withdrawing_less_than_or_equal_balance(
        #[case] balance: BitCoin,
        #[case] amount: BitCoin,
    ) {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(balance);

        // Act
        let actual = wallet.withdraw(amount);

        // Assert
        assert!(actual.is_ok());
    }

    #[test]
    fn sut_returns_error_if_withdrawing_more_than_balance() {
        // Arrange
        let mut wallet = Wallet::open();
        wallet.deposit(20);

        // Act
        let actual = wallet.withdraw(30).unwrap_err();

        // Assert
        assert!(matches!(actual, WalletError::InsufficientFunds(_)));
        assert_eq!(actual.to_string(), "cannot withdraw, insufficient funds");
    }
}
