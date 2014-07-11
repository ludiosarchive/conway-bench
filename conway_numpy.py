import sys
import time
import random
from numpy import ndarray

COLS = 177
ROWS = 60
EMPTY_OUT = " " * ((COLS+1)*ROWS)

def update_ghost(grid, ghost_grid):
	"""
	A virtual grid that includes wrapped edges, so that we don't have to
	do funky modulo arithmetic.
	"""

	# Copy bottom of grid to top of ghost_grid
	for n in xrange(COLS):
		ghost_grid[n+1,0] = grid[n,ROWS-1]

	# Copy top of grid to bottom of ghost_grid
	for n in xrange(COLS):
		ghost_grid[n+1,ROWS+2-1] = grid[n,0]

	# Copy the rest of grid to ghost_grid
	for y in xrange(ROWS):
		for x in xrange(COLS):
			ghost_grid[x+1,y+1] = grid[x,y]

	# Wrap ghost_grid left and right columns
	for y in xrange(ROWS+2):
		ghost_grid[0,y] = ghost_grid[COLS+2-2,y]
		ghost_grid[COLS+2-2,y] = ghost_grid[1,y]

def count_neighbors(x, y, ghost_grid):
	return \
		ghost_grid[(x-1)+1,(y-1)+1] + ghost_grid[(x)+1,(y-1)+1] + ghost_grid[(x+1)+1,(y-1)+1] + \
		ghost_grid[(x-1)+1,(y  )+1]                             + ghost_grid[(x+1)+1,(y  )+1] + \
		ghost_grid[(x-1)+1,(y+1)+1] + ghost_grid[(x)+1,(y+1)+1] + ghost_grid[(x+1)+1,(y+1)+1]

def pretty_print(grid):
	out = bytearray(EMPTY_OUT)
	out_i = 0

	for y in xrange(ROWS):
		for x in xrange(COLS):
			if grid[x,y] == 0:
				out[out_i] = ' '
				out_i += 1
			else:
				out[out_i] = '#'
				out_i += 1
		out[out_i] = '\n'
		out_i += 1
	sys.stdout.write(out)

def next_gen(grid, ghost_grid):
	for y in xrange(ROWS):
		for x in xrange(COLS):
			neighbors = count_neighbors(x, y, ghost_grid)
			#print!("N %d, ", neighbors)
			if neighbors < 2 or neighbors > 3:
				grid[x,y] = 0
			elif neighbors == 3:
				grid[x,y] = 1
	update_ghost(grid, ghost_grid)

def run_once():
	grid = ndarray(shape=(COLS, ROWS), dtype=int)
	ghost_grid = ndarray(shape=(COLS + 2, ROWS + 2), dtype=int)

	# Generate a random grid
	for y in xrange(ROWS):
		for x in xrange(COLS):
			grid[x,y] = random.randint(0, 2)

	pretty_print(grid)
	update_ghost(grid, ghost_grid)

	for i in xrange(50000):
		sys.stdout.write("\n\n\n")
		pretty_print(grid)
		#nanosleep(&wait, NULL)

		next_gen(grid, ghost_grid)

def main():
	run_once()
	print >>sys.stderr, "Warmed up."
	start = time.time()
	run_once()
	end = time.time()
	print >>sys.stderr, "Ran in", (1000 * (end - start)), "ms"

if __name__ == '__main__':
	main()
