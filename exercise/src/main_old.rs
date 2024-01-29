fn interproduct(a: i32, b: i32, c: i32) -> i32 {
	return  a * b + b * c + c * a;
}

fn takes_u32(x: u32) {
	println!("u32: {x}");
}

fn takes_i32(x: i32) {
	println!("u32: {x}");
}

fn takes_i8(y: i8) {
	println!("i8: {y}");
}

fn main() {
	let x: i32;
	x = interproduct(200, 300, 700);
	println!("interproduct x = {:?}", &x);

	let greeting: &str = "Greetings";
	let planet: &str = "🪐";
	let mut sentence = String::new();
	sentence.push_str(greeting);
	sentence.push_str(", ");
	sentence.push_str(planet);
	println!("final sentence: {sentence}");
	println!("final sentence: {}", sentence);
	println!("{:?}", &sentence[0..5]);
	println!("{:?}", &sentence[11..15]);


	let z = 10;
	let y = 20;
    
	takes_u32(z);
	takes_i32(y);
	//takes_u32(y);
}