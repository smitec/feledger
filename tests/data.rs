extern crate feledger;

use feledger::data::{Date, Currency, Entry, Transaction, Value, Account};
use std::collections::HashMap;

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
            currency: Currency {
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
            currency: Currency {
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
        Ok(_) => {assert!(true)},
        Err(_) => {assert!(false)}
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
            currency: Currency {
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
            currency: Currency {
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
        Ok(_) => {assert!(false)},
        Err(_) => {assert!(true)}
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
            currency: Currency {
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
            currency: Currency {
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
            currency: Currency {
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
            currency: Currency {
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
        Ok(_) => {assert!(true)},
        Err(_) => {assert!(false)}
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
            currency: Currency {
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
            currency: Currency {
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
            currency: Currency {
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
            currency: Currency {
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
        Ok(_) => {assert!(false)},
        Err(_) => {assert!(true)}
    }
}

#[test]
fn apply_one_transaction() {
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
            currency: Currency {
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
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    let a = Account { label: "left".to_string() };
    let b = Account { label: "right".to_string() };

    entries.push(lhs);
    entries.push(rhs);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    let mut hm : HashMap<&Account, Value> = HashMap::new();
    let mut expected : HashMap<&Account, Value> = HashMap::new();

    transaction.apply_to(&mut hm).unwrap();

    expected.insert(&a, Value {
        currency: Currency {
            symbol: "$".to_string() 
        }, amount:100f32 });
    expected.insert(&b, Value {
        currency: Currency {
            symbol: "$".to_string() 
        }, amount:-100f32 });

    assert!(hm == expected);
}

#[test]
fn apply_two_transaction() {
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
            currency: Currency  {
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
            currency: Currency {
                symbol: "$".to_string()
            }
        }
    };

    let mut entries = Vec::new();

    let a = Account { label: "left".to_string() };
    let b = Account { label: "right".to_string() };

    entries.push(lhs);
    entries.push(rhs);

    let transaction = Transaction {
        date: date,
        comment: comment,
        entries: entries
    };

    let mut hm : HashMap<&Account, Value> = HashMap::new();
    let mut expected : HashMap<&Account, Value> = HashMap::new();

    transaction.apply_to(&mut hm).unwrap();
    transaction.apply_to(&mut hm).unwrap();

    expected.insert(&a, Value {
        currency: Currency {
            symbol: "$".to_string() 
        }, amount:200f32 });
    expected.insert(&b, Value {
        currency: Currency {
            symbol: "$".to_string() 
        }, amount:-200f32 });

    assert!(hm == expected);
}
