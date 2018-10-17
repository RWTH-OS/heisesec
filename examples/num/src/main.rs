extern crate num;

use num::bigint::{BigInt, ToBigInt};

fn main() {
    println!("Hello, world!");

	let x = ToBigInt::to_bigint(&4).unwrap();
	let y = ToBigInt::to_bigint(&1).unwrap();

	match x.checked_div(&y) {
    	None => panic!("Unable to divide numbers"),
    	Some(ratio) => println!("Result {}", ratio)
	};
}
