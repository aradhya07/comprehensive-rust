fn fib(n: u32) -> u32 {
	if n <= 2 {
		// The base case.
		return 1;
	} else {
		// The recursive case.
		return fib(n-2) + fib(n-1);
	}
}
    
fn main() {
	let n = 3;
	println!("fib(n) = {}", fib(n));

	let x = 10;
	let size = if x < 20 { "small" } else { "large" };
	println!("number size: {}", size);
}