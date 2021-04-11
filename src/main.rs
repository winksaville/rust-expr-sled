// Example code to use bincode as the serde:
//  https://stackoverflow.com/questions/58358179/using-sled-how-do-i-serialize-and-deserialize

use serde::{Deserialize, Serialize};
use bincode;

#[derive(Debug, Deserialize, Serialize)]
pub enum SymbolFilters {
    LotSize {
        min_qty: f64,
        max_qty: f64,
        step_size: f64,
    },

    MarketLotSize {
        min_qty: f64,
        max_qty: f64,
        step_size: f64,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    pub symbol: String,
    pub filters: Vec<SymbolFilters>,
}

impl Symbol {
    pub fn new(symbol: String, filters: Vec<SymbolFilters>) -> Self {
        Symbol { symbol, filters, }
    }
}
fn main() -> sled::Result<()> {

    // this directory will be created if it does not exist
    let path = "my_storage_directory";

    // Open the data base and create if it doesn't exist
    let db = sled::open(path)?;

    // Insert an element, setting to false you can test persistence.
    if true {
        let mut vsf = Vec::<SymbolFilters>::new();
        vsf.push(SymbolFilters::LotSize { min_qty: 1.0, max_qty: 100.0, step_size: 1.0 });
        vsf.push(SymbolFilters::MarketLotSize { min_qty: 20.0, max_qty: 200.0, step_size: 2.0 });
        let sym1 = Symbol::new("BNB".to_string(), vsf);
        db.insert(&sym1.symbol, bincode::serialize(&sym1).unwrap()).unwrap();
    }

    // Retrieve a "symbol" BNB
    let x = db.get("BNB")?.unwrap();
    let y: Symbol = bincode::deserialize(&x).unwrap();
    println!("y={:#?}", y);

    // Remove BNB which will currently leave DB empty
    // in this simple one element case.
    if false {
        db.remove("BNB")?;
    }

    Ok(())
}
