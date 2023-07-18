pub trait AccountRepository {
    fn get_next_account_number(&self, branch: u16) -> u32;
}

pub struct AccountService {
    repository: Box<dyn AccountRepository>,
}

pub enum AccountNumber {
    Bban { branch: u16, number: u32 },
    Invalid(String),
}

pub struct AccountNumberError;

impl AccountService {
    fn new(repository: Box<dyn AccountRepository>) -> AccountService {
        AccountService { repository }
    }

    fn create_account_number(&self, branch: u16) -> AccountNumber {
        AccountNumber::Invalid("failed to create account".to_owned())
    }
}

fn new_account_number_for_branch(branch: u16) -> Result<u32, AccountNumberError> {
    todo!()
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
