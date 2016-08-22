//! Data formats used in FeLedger
use std::collections::HashMap;

// Date struct
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

// Account struct
#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Account {
    pub label: String,
}

// Currency enum. Either a prefixed or suffixed string to acompany an amount in
// a value.
#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Currency {
    pub symbol: String,
}

// Value Struct. Used for dollars btu also other items which may be recorded in
// the ledger.
#[derive(PartialEq,Debug,Clone)]
pub struct Value {
    pub amount: f32, // Should I be more general and use a Num or similar
    pub currency: Currency,
}

// Entry Struct, used for single entry in the ledger
#[derive(PartialEq,Debug,Clone)]
pub struct Entry {
    pub account: Account,
    pub value: Value,
}

// Transaction struct, a collection of entries under one transaction
#[derive(PartialEq,Debug,Clone)]
pub struct Transaction {
    pub date: Date,
    pub comment: String,
    pub entries: Vec<Entry>
}

impl Transaction {
    pub fn balance(&self) -> Result<(), &'static str> {
        //Loop through the entries and ensure the total balance is 0 in all present currencies 
        let mut currency_table : HashMap<&Currency, f32> = HashMap::new();

        for entry in &self.entries {
            let currency = currency_table.entry(&entry.value.currency).or_insert(0f32);
            *currency += entry.value.amount;
        }

        for (_, value) in currency_table {
            if value > 0f32 {
                return Err("Transaction does not balance!");
            }
        }

        return Ok(());
    }

    pub fn apply_to<'a>(&'a self, balance_table : &mut HashMap<&'a Account, Value>) -> Result<(), &'static str> {

        for entry in &self.entries {
            let account = balance_table.entry(&entry.account)
                .or_insert(Value { currency: entry.value.currency.clone(), amount: 0f32});
            if account.currency != entry.value.currency {
                return Err("Multiple Currencies in one account!");
            }

            (*account).amount += entry.value.amount;
        }

        return Ok(());
    }
}
