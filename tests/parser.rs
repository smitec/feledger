extern crate feledger;
extern crate nom;

use feledger::data::{Date};
use feledger::parser::{date};

#[test]
fn test_date_parse() {
    assert_eq!(nom::IResult::Done(&[][..], Date{ year: 2016, month: 8, day: 23}), date(b"2016/08/23"));
}
