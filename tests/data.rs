extern crate feledger;

use feledger::data::{Date, Currency, Entry, Transaction, Value, Account};

#[test]
fn it_works() {
}

#[test]
fn test_simple_balance() {
    let date = Date {
        year: 2016,
        month: 8,
        day: 13
    };

    let comment = "Test Transaction".to_string();

    let lhs = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };
    let rhs = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    entries.push(lhs);
    entries.push(rhs);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    match transaction.balance() {
        Ok(x) => {assert!(true)},
        Err(x) => {assert!(false)}
    }
}

#[test]
fn test_simple_bad_balance() {
    let date = Date {
        year: 2016,
        month: 8,
        day: 13
    };

    let comment = "Test Transaction".to_string();

    let lhs = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 101f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };
    let rhs = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    entries.push(lhs);
    entries.push(rhs);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    match transaction.balance() {
        Ok(x) => {assert!(false)},
        Err(x) => {assert!(true)}
    }
}

#[test]
fn test_simple_balance_multi_currency() {
    let date = Date {
        year: 2016,
        month: 8,
        day: 13
    };

    let comment = "Test Transaction".to_string();

    let lhs = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let rhs = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let lhs_a = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 100f32,
            currency: Currency::Prefix {
                symbol: "#".to_string()
            }
        }
    };

    let rhs_a = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "#".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    entries.push(lhs);
    entries.push(rhs);
    entries.push(lhs_a);
    entries.push(rhs_a);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    match transaction.balance() {
        Ok(x) => {assert!(true)},
        Err(x) => {assert!(false)}
    }
}

#[test]
fn test_simple_balance_multi_currency_fail() {
    let date = Date {
        year: 2016,
        month: 8,
        day: 13
    };

    let comment = "Test Transaction".to_string();

    let lhs = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let rhs = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "$".to_string()
            }
        }
    };

    let lhs_a = Entry {
        account: Account {
            label: "left".to_string()
        },
        value: Value {
            amount: 100f32,
            currency: Currency::Prefix {
                symbol: "#".to_string()
            }
        }
    };

    let rhs_a = Entry {
        account: Account {
            label: "right".to_string()
        },
        value: Value {
            amount: -100f32,
            currency: Currency::Prefix {
                symbol: "!".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    entries.push(lhs);
    entries.push(rhs);
    entries.push(lhs_a);
    entries.push(rhs_a);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    match transaction.balance() {
        Ok(x) => {assert!(false)},
        Err(x) => {assert!(true)}
    }
}
