fn main() {
	println!("Hello world from Rust!");

	let array: [i32; 5] = [1, 2, 3, 4, 5];

	//let x = array[6];
	for i in 0..5 {
		p	rintln!("array[{}] = {}", i, array[i]);
	}
}
