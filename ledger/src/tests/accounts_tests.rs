use super::*;
use core::panic;

struct MockedRepo;

impl AccountRepository for MockedRepo {
    fn get_next_account_number(&self, _branch: u16) -> u32 {
        1001
    }
}

#[test]
fn should_return_invalid_when_malformed_str() {
    if let AccountNumber::Invalid(e) = AccountNumber::from("00") {
        println!("{e}");
    } else {
        panic!("should have failed");
    }
}

#[test]
fn should_convert_to_valid_account_number() {
    if let AccountNumber::Bban { branch, number } = AccountNumber::from("1000123456") {
        assert_eq!(branch, 1000);
        assert_eq!(number, 123456);
    } else {
        panic!("should have return valid account");
    }
}

#[test]
fn should_convert_account_number_to_string() {
    let s = String::from(AccountNumber::Bban {
        branch: 1000,
        number: 123,
    });

    assert_eq!(s, "1000000123");
}

#[test]
fn should_create_new_account_number() {
    let repository = Box::new(MockedRepo);
    let service = AccountService::new(repository);
    if let AccountNumber::Bban { branch, number } = service.create_account_number(100) {
        println!("{branch:04},{number:06}");
    } else {
        panic!("should have created account number successfuly");
    }
}
