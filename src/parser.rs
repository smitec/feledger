//! Parser component of FeLedger
//! This file holds the types and functions used to read ledger files
use std::str::{ FromStr, from_utf8 };
use data::{ Date, Currency, Value, Account, Entry };
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

named!(transactional_value<Value>, chain!(
        sym: symbol ~ 
        val: signed_value,
        || Value{ amount: val, currency: Currency{ symbol: from_utf8(sym).unwrap().to_string() }}
        ));


#[test]
fn test_gets_whole_value() {
    let expected = Value{ amount: -100.1f32, currency: Currency{ symbol: "$".to_string() }};
    let result = transactional_value(b"$-100.1");
    assert_eq!(nom::IResult::Done(&[][..], expected), result);
}


named!(sub_account, recognize!(chain!(tag!(":") ~ alpha, || {})));

named!(account<Account>, map!(recognize!(
       chain!(
        alpha ~
        many0!(sub_account),
        || {}
        )),
        |v| Account{ label: from_utf8(v).unwrap().to_string()} 
));

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


named!(single_entry<Entry>, chain!(
    tag!(" ") ~ // leading space
    many1!(tag!(" ")) ~ // at least 2 spaces total
    label: account ~
    tag!(" ") ~
    many1!(tag!(" ")) ~ // 2 spaces between account and value
    val: transactional_value,
    || Entry{ account: label, value: val}
    ));

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
