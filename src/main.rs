use std::io;

#[allow(unused_variables)]
fn main() {
	// println!("Enter n (2≤n≤104):");
	let mut s = safe_read();
	let n: u8 = s.parse().expect("NaN");
	assert_bounds(n, 2, 104);

	let mut ai: Vec<u8> = vec!();
	// #[allow(unused_variables)] for this `i`
	for i in 0..n {
		// println!("Enter ai #{}/{} (1≤ai≤104):", i + 1, n);
		s = safe_read();
		ai.push(assert_bounds(s.parse().expect("NaN"), 1, 104));
	}

	let mut imax = 0;
	let mut imin = 0;
	for (j, &value) in ai.iter().enumerate() {
		if value > ai[imax] {
			imax = j;
		}
		if value < ai[imin] {
			imin = j;
		}
	}
	// println!("ai: {:?}", ai);
	// println!("max: {}", imax);
	// println!("min: {}", imin);
	let v: u8 = ((imax as i8 - imin as i8).abs() - 1) as u8;
	println!("{}", v);
}

fn safe_read() -> String {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Err(_) => { panic!("Could not read input") }
		Ok(_) => ()
	}
	input.pop();
	input
}

fn assert_bounds(int: u8, more_or_equal: u8, less_or_equal: u8) -> u8 {
	if int < more_or_equal || int > less_or_equal {
		panic!("Int out of bounds");
	}
	int
}
