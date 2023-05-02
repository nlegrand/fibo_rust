use std::io;

fn main() {
    loop {
	println!("Please type the nth Fibn number to calculate:");
	let mut nth = String::new();

	io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");
	let nth: i64 = nth.trim().parse().expect("Please type a number");
	if nth > 91 {
	    println!("Sorry, {nth} would overflow the i64 type, 91 is the limit");
	    continue;
	}
	let mut ap: i64 = 0;
	let mut p: i64 = 1;
	println!("fibo 1: {p}");
	for n in 1..nth {
	    let buf: i64 = ap + p;
	    ap = p;
	    p = buf;
	    println!("fibo {}: {buf}", n + 1)
	}
    }
}
