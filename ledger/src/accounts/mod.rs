pub enum AccountNumber {
    Bban { branch: u16, number: u32 },
    Invalid(String),
}

impl From<&str> for AccountNumber {
    fn from(value: &str) -> AccountNumber {
        if value.len() == 10 {
            let branch = value[..4].parse().ok();
            let number = value[4..].parse().ok();

            if let (Some(branch), Some(number)) = (branch, number) {
                return AccountNumber::Bban { branch, number };
            }
        }

        AccountNumber::Invalid(format!("Invalid account value {value:?}"))
    }
}

impl From<AccountNumber> for String {
    fn from(val: AccountNumber) -> String {
        if let AccountNumber::Bban { branch, number } = val {
            return format!("{:04}{:06}", branch, number);
        }

        String::from("")
    }
}

#[path = "../tests/accounts_tests.rs"]
#[cfg(test)]
mod tests;
