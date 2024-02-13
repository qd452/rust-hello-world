use ethers::types::U256;
use rust_decimal::{prelude::Zero, Decimal};
use std::collections::{BTreeMap, HashMap};
use std::{str::FromStr, vec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_amount = U256::from_dec_str("1000000000").unwrap();
    println!("Base amount: {:?}", base_amount.to_string());
    let decimal_amount = base_amount / (U256::from(10).pow(U256::from(18)));
    println!("Decimal amount: {:?}", decimal_amount.to_string());
    let real_decimal_amount = Decimal::from_str(&base_amount.to_string()).unwrap()
        / Decimal::from_str("1000000000000000000").unwrap();
    println!("Real decimal amount: {:?}", real_decimal_amount.to_string());

    // let v = "0.1111";
    // let v = "2.0423884741793798E-7";
    // let v = "2346.911588496553019816917207548126";
    let v = "0E-32";
    let a = Decimal::from_str(v)
        .unwrap_or_else(|_| Decimal::from_scientific(v).unwrap_or(Decimal::zero()));
    println!("a: {:?}", a);

    let a = Decimal::new(1_000_000, 0);
    let b = Decimal::new(30_000_000, 0);
    let c = a / b;
    println!("c: {:?}", c.to_string());

    let v = vec![1, 2, 3];
    // let rslt = v.iter().map(|x| x * 2);
    let rslt: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("rslt: {:?}", rslt);
    for i in rslt {
        println!("i: {:?}", i);
    }

    let v23: Vec<i32> = v.iter().map(|v| *v).filter(|x| *x > 1).collect();
    println!("v23: {:?}", v23);

    let vs: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
    let vs2: Vec<String> = vs
        .iter()
        .filter(|x| *x == "1")
        .map(|x| x.to_string())
        .collect();
    println!("vs2: {:?}", vs2);

    // tuple as hashmap key, and hashmap value is a vector
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    map.insert((1, 2), vec![1, 2, 3]);
    map.insert((2, 3), vec![4, 5, 6]);
    map.insert((3, 4), vec![7, 8, 9]);
    println!("map: {:?}", map);

    // BtreeMap
    let mut bmap: BTreeMap<i32, i32> = BTreeMap::new();
    bmap.insert(1, 2);
    bmap.insert(2, 3);
    bmap.insert(3, 4);

    for (k, v) in bmap.iter() {
        println!("k: {:?}, v: {:?}", k, v);
    }

    Ok(())
}
