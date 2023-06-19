pub mod iban;
#[derive(Debug)]
pub struct BankAccount {
    country_code : String,
    control: u8,
    bank_code: String,
    branch_code: String,
    account_number: String
}

impl From<&str> for BankAccount {
    fn from(value: &str) -> Self {
        let clean_value = value.trim().replace(" ", "");

        BankAccount { 
            country_code: clean_value[..2].to_string(),
            control: clean_value[2..4].parse().unwrap(),
            bank_code: clean_value[4..8].to_string(),
            branch_code: clean_value[8..12].to_string(),
            account_number: clean_value[12..].to_string(),
        }
    }
}