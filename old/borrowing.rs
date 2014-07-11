// https://github.com/rust-lang/rust/issues/13859
struct S { s: [int, ..10] }

fn mutate_me_2(grid: &mut S) {
	grid.s[0] += 1;
}

fn mutate_me(grid: &mut S) {
	mutate_me_2(grid);
	grid.s[0] += 1;
}

fn main() {
	let mut grid = S { s: [0, ..10] };
	mutate_me(&mut grid);
	println!("{}", grid.s[0]);
}
