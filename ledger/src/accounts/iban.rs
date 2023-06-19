use super::BankAccount;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum CustomError {
    ConversionError(ParseIntError),
    OtherError,
}

#[derive(Debug)]
pub struct InvalidIbanError;

pub fn is_iban_valid(account: &BankAccount) -> Result<bool, InvalidIbanError> {
    if account.country_code.len() != 2 || 
        account.bank_code.len() != 4 ||
        account.branch_code.len() != 4 || 
        account.account_number.len() != 12 ||
        (account.control == 0 || account.control > 99) {
        return Err(InvalidIbanError);
    }

    let number = convert_into_97_10_u128(&account.iban_into_str());

    Ok(
        number % 97 == 1
    )
}

fn convert_into_97_10_u128(iban_str: &str) -> u128 {
    let iban = &(iban_str[4..].to_owned() + &iban_str[..4])
        .trim()
        .replace(" ", "");
    let converted_iban: String = iban.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let code = c as u8 - 55;
            code.to_string()
        } else {
            c.to_string()
        }
    }).collect();

    converted_iban.parse::<u128>().unwrap()
}

impl BankAccount {
    pub fn iban_into_str(&self) -> String {
        format!("{}{:02} {} {} {}", self.country_code, self.control, self.bank_code, self.branch_code,
            format_account_number(self.account_number.as_str()))
    }
}

fn format_account_number(account_number: &str) -> String {
    format!("{} {} {}", &account_number[..4], &account_number[4..8], &account_number[8..])
}

#[cfg(test)]
mod tests {
    use crate::accounts::BankAccount;
    use super::{is_iban_valid, convert_into_97_10_u128};

    const VALID_IBAN : &str = "ES91 2100 0418 4502 0005 1332";

    #[test]
    fn should_create_account_from_iban_str() {
        let result : BankAccount = VALID_IBAN.into();

        assert_eq!(result.country_code, "ES");
        assert_eq!(result.control, 91);
        assert_eq!(result.bank_code, "2100");
        assert_eq!(result.branch_code, "0418");
        assert_eq!(result.account_number, "450200051332");
    }

    #[test]
    fn iban_should_be_valid() {
        let account = VALID_IBAN.into();

        assert!(is_iban_valid(&account).unwrap());
    }

    #[test]
    fn should_correctly_convert_into_97_10_u128() {
        let result = convert_into_97_10_u128(VALID_IBAN);

        assert_eq!(result, 21000418450200051332142891u128);
    }
}
