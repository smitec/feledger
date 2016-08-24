extern crate feledger;

use feledger::{parse_ledger_file};
use feledger::data::{ Transaction, Date, Account, Currency, Value, Entry };  

#[test]
fn test_file_parse() {
    let result = parse_ledger_file("./tests/testfile.feledger");
    let expected = Transaction{
        date: Date{ year: 2016, month: 8, day: 24},
        comment: "A test transaction in a file".to_string(),
        entries: vec!(
            Entry{
                account: Account{
                    label: "expenses:time".to_string()
                },
                value: Value{
                    amount: 100f32,
                    currency: Currency{ symbol: "$".to_string() }
                }
            },
            Entry{
                account: Account{
                    label: "assets:joy".to_string()
                },
                value: Value{
                    amount: -100f32,
                    currency: Currency{ symbol: "$".to_string() }
                }
            })
    };
    match result {
        Ok(t) => assert_eq!(t, vec!(expected)),
        Err(t) => assert_eq!(t, "")
    }
}
