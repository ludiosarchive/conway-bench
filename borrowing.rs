fn mutate_me_2(grid: &mut [uint, ..10]) {
	grid[0] += 1;
}

fn mutate_me(grid: &mut [uint, ..10]) {
	mutate_me_2(grid);
	grid[0] += 1;
}

fn main() {
	let mut grid = [0, ..10];
	mutate_me(&mut grid);
	println!("{}", grid[0]);
}
