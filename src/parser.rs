//! Parser component of FeLedger
//! This file holds the types and functions used to read ledger files
use std::str::{ FromStr, from_utf8 };
use data::{ Date, Currency, Value, Account, Entry, Transaction };
use nom::{self, is_digit, alpha};


// Date parser in the format YYYY/MM/DD with no flexibility
// With some guidance from: https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/
macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut failed = false;
      for &idx in $input {
        if !$submac!(idx, $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        nom::IResult::Error(nom::Err::Position(nom::ErrorKind::Custom(20),$input))
      } else {
        nom::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    check!($input, call!($f));
  );
);


pub fn str_to_i32(s: &[u8]) -> i32 {
    FromStr::from_str(from_utf8(s).unwrap()).unwrap()
}

named!(pub take_4_digits, flat_map!(take!(4), check!(is_digit)));
named!(pub take_2_digits, flat_map!(take!(2), check!(is_digit)));

named!(year <&[u8], i32>, map!(call!(take_4_digits), str_to_i32));
named!(month <&[u8], i32>, map!(call!(take_2_digits), str_to_i32));
named!(day <&[u8], i32>, map!(call!(take_2_digits), str_to_i32));

named!(pub date <&[u8], Date>, chain!(
        y: year      ~
           tag!("/") ~
        m: month     ~
           tag!("/") ~
        d: day
        ,
        || { Date{ year: y, month: m, day: d } }
        ));

// Transaction parser in the format (|spacing)account:subaccount...(>2 spaces)[symbol][number]
// For example `  expenses:catfood    $10.00

named!(value<f32>, map!(
    alt_complete!(
	    recognize!(chain!(take_while!(nom::is_digit) ~ tag!(".") ~ take_while!(nom::is_digit), || {})) |
	    take_while!(nom::is_digit)
	    ),
    |v| FromStr::from_str(from_utf8(v).unwrap()).unwrap()
	)
);

named!(sign<f32>, map!(opt!(tag!("-")), |v| {
    match v {
        Some(_) => return -1f32,
        None => return 1f32
    }
}));

named!(signed_value<f32>, chain!(pre: sign ~ val: value, || { pre*val }));

#[test]
fn test_negative() {
	assert_eq!(nom::IResult::Done(&[][..], -100.1f32), signed_value(b"-100.1"));
}

named!(symbol, take_until_either!("-0123456789."));

named!(transactional_value<Value>, chain!(
        sym: symbol ~ 
        val: signed_value,
        || Value{ amount: val, currency: Currency{ symbol: from_utf8(sym).unwrap().to_string() }}
        ));

named!(sub_account, recognize!(chain!(tag!(":") ~ alpha, || {})));

named!(account<Account>, map!(recognize!(
       chain!(
        alpha ~
        many0!(sub_account),
        || {}
        )),
        |v| Account{ label: from_utf8(v).unwrap().to_string()} 
));

named!(single_entry<Entry>, chain!(
    tag!(" ") ~ // leading space
    many1!(tag!(" ")) ~ // at least 2 spaces total
    label: account ~
    tag!(" ") ~
    many1!(tag!(" ")) ~ // 2 spaces between account and value
    val: transactional_value,
    || Entry{ account: label, value: val}
    ));


// Multi line parser

named!(entry_line<Entry>, chain!(
        t: single_entry ~
        take_until_and_consume!("\n"),
        || t
        ));

named!(multiple_entries< Vec<Entry> >, many1!(entry_line));

// Comment and date line

named!(header<Transaction>, chain!(
        d: date ~
        many1!(tag!(" ")) ~
        comment: take_until!("\n") ~
        tag!("\n"),
        || Transaction{ date: d, comment: from_utf8(comment).unwrap().to_string(), entries: Vec::new() }
        ));

named!(transaction<Transaction>, chain!(
        mut t: header ~
        entries: multiple_entries ~
        many1!(tag!("\n")), // Separating new line
        || {
            t.entries = entries;
            return t;
        }));

named!(pub parse_file< Vec<Transaction> >, chain!(
        many0!(tag!("\n")) ~
        t: many1!(transaction),
        || t
        ));

// Tests
#[test]
fn test_value_parse() {
	assert_eq!(nom::IResult::Done(&[][..], 100f32), value(b"100.0"));
}

#[test]
fn test_value_parse_no_decimal() {
	assert_eq!(nom::IResult::Done(&[][..], 100f32), value(b"100"));
}

#[test]
fn test_value_parse_with_decimal() {
	assert_eq!(nom::IResult::Done(&[][..], 100.1f32), value(b"100.1"));
}

#[test]
fn test_gets_symbol() {
    let expected : &[u8] = b"$";
    let result = symbol(b"$-100.1");
    match result {
        nom::IResult::Done(_, t) => assert_eq!(t, expected),
        nom::IResult::Error(_) => assert!(false),
        nom::IResult::Incomplete(_) => assert!(false)
    };
}

#[test]
fn test_gets_whole_value() {
    let expected = Value{ amount: -100.1f32, currency: Currency{ symbol: "$".to_string() }};
    let result = transactional_value(b"$-100.1");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}


#[test]
fn test_gets_account() {
    let expected = Account{ label: "test".to_string() };
    let result = account(b"test");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}

#[test]
fn test_gets_account_with_sub() {
    let expected = Account{ label: "test:pie".to_string() };
    let result = account(b"test:pie");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}

#[test]
fn test_gets_whole_line() {
    let expected = Entry{ 
        account: Account{label: "test:pie".to_string()},
        value: Value{
            amount: 100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let result = single_entry(b"  test:pie  $100.1");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}

#[test]
fn test_gets_whole_line_with_ending() {
    let expected = Entry{ 
        account: Account{label: "test:pie".to_string()},
        value: Value{
            amount: 100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let result = entry_line(b"  test:pie  $100.1  \n");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}

#[test]
fn test_multi_line() {
    let expected = Entry{ 
        account: Account{label: "test:pie".to_string()},
        value: Value{
            amount: 100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let expected2 = Entry{ 
        account: Account{label: "test:cat".to_string()},
        value: Value{
            amount: -100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let expected_vec : Vec<Entry> = vec!(expected, expected2);
    let result = multiple_entries(b"  test:pie  $100.1  \n  test:cat  $-100.1  \n");
    assert_eq!(nom::IResult::Done(&[][..], expected_vec), result);
}
#[test]
fn test_header_line() {
    let expected = Transaction{
        date: Date{ year: 2016, month:1, day: 10},
        comment: "This is a test entry".to_string(),
        entries: Vec::new()
    };
    let result = header(b"2016/01/10 This is a test entry\n");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}

#[test]
fn test_full_transaction() {
    let expected1 = Entry{ 
        account: Account{label: "test:pie".to_string()},
        value: Value{
            amount: 100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let expected2 = Entry{ 
        account: Account{label: "test:cat".to_string()},
        value: Value{
            amount: -100.1f32,
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };
    let expected = Transaction{
        date: Date{ year: 2016, month:1, day: 10},
        comment: "This is a test entry".to_string(),
        entries: vec!(expected1, expected2)
    };
    let result = transaction(b"2016/01/10 This is a test entry\n  test:pie  $100.1\n  test:cat  $-100.1\n\n");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}
