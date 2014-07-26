#include <stdio.h>
#include <stdlib.h>

#define COLS 177
#define ROWS 60

#define GRID_TYPE unsigned short int

static void update_ghost(GRID_TYPE grid[COLS][ROWS], GRID_TYPE ghost_grid[COLS+2][ROWS+2]) {
	/*
	A virtual grid that includes wrapped edges, so that we don't have to
	do funky modulo arithmetic.
	*/

	int n, x, y;

	/* Copy bottom of grid to top of ghost_grid */
	for(n=0; n <= COLS-1; n++) {
		ghost_grid[n+1][0] = grid[n][ROWS-1];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for(n=0; n <= COLS-1; n++) {
		ghost_grid[n+1][ROWS+2-1] = grid[n][0];
	}

	/* Copy the rest of grid to ghost_grid */
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			ghost_grid[x+1][y+1] = grid[x][y];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for(y=0; y <= ROWS+2-1; y++) {
		ghost_grid[0][y] = ghost_grid[COLS+2-2][y];
		ghost_grid[COLS+2-2][y] = ghost_grid[1][y];
	}
}

static int count_neighbors(int x, int y, GRID_TYPE ghost_grid[COLS+2][ROWS+2]) {
	return \
		ghost_grid[(x-1)+1][(y-1)+1] + ghost_grid[(x)+1][(y-1)+1] + ghost_grid[(x+1)+1][(y-1)+1] + \
		ghost_grid[(x-1)+1][(y)+1  ]                              + ghost_grid[(x+1)+1][(y)+1  ] + \
		ghost_grid[(x-1)+1][(y+1)+1] + ghost_grid[(x)+1][(y+1)+1] + ghost_grid[(x+1)+1][(y+1)+1];
}

static int get_random(int n) {
	int val;
	while (n <= (val = rand() / (RAND_MAX/n)));
	return val;
}

static void pretty_print(GRID_TYPE grid[COLS][ROWS]) {
	int x, y;

	char out[(COLS+1)*ROWS];
	int out_i = 0;

	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			if(grid[x][y] == 0) {
				out[out_i] = ' ';
				out_i += 1;
			} else {
				out[out_i] = '#';
				out_i += 1;
			}
		}
		out[out_i] = '\n';
		out_i += 1;
	}
	puts(out);
}

static void next_gen(GRID_TYPE grid[COLS][ROWS], GRID_TYPE ghost_grid[COLS+2][ROWS+2]) {
	int neighbors, x, y;
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			neighbors = count_neighbors(x, y, ghost_grid);
			//printf("N %d, ", neighbors);
			if(neighbors < 2 || neighbors > 3) {
				grid[x][y] = 0;
			} else if(neighbors == 3) {
				grid[x][y] = 1;
			}
		}
	}
	update_ghost(grid, ghost_grid);
}


static struct timespec wait = {0, 160 * 1000 * 1000};

int main() {
	GRID_TYPE grid[COLS][ROWS];
	GRID_TYPE ghost_grid[COLS+2][ROWS+2];
	int character;
	int x;
	int y;

	/* Generate a random grid */
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			grid[x][y] = get_random(2);
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	int iterations = 10000;

	while(iterations--) {
		puts("\n\n\n");
		pretty_print(grid);
		//nanosleep(&wait, NULL);

		next_gen(grid, ghost_grid);
	}
}
