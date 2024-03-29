from typing import Union

def input_grid() -> list[list[int]]:
	print("enter a three by three grid with each row on its own line,",
		"with the numbers seperated by spaces",
		"\nNumbers can be an integer from 0-8 inclusive")
	return [[int(x) for x in input().split()],
			[int(x) for x in input().split()],
			[int(x) for x in input().split()]]

def manhatten_distance(point1: tuple[int, int], point2: tuple[int, int]) -> int:
		return abs(point1[0] - point2[0]) + abs(point1[1] - point2[1])

class Board:
	grid: list[list[int]]
	empty: tuple[int, int]
	
	def __init__(self, grid, empty = None):
		self.grid = grid

		if empty:
			self.empty = empty
		else:
			for i, arr in enumerate(grid):
				for j, val in enumerate(arr):
					if val == 0:
						self.empty = (i, j)

		self._validate_grid()

	def generate_neighbors(self, last_emp: tuple[int, int] = None) -> list["Board"]:
		out = []
		neighbors = []
		if self.empty[0]:
			neighbors.append((self.empty[0]-1, self.empty[1]))
		if self.empty[1]:
			neighbors.append((self.empty[0], self.empty[1]-1))
		if self.empty[0] < len(self.grid)-1:
			neighbors.append((self.empty[0]+1, self.empty[1]))
		if self.empty[1] < len(self.grid)-1:
			neighbors.append((self.empty[0], self.empty[1]+1))

		if last_emp is not None:
			neighbors.remove(last_emp)

		for point in neighbors:
			b = self.copy()
			b._swap(self.empty, point)
			out.append(b)

		return out

	def _swap(self, p1, p2):
		self.grid[p1[0]][p1[1]], self.grid[p2[0]][p2[1]] = self.grid[p2[0]][p2[1]], self.grid[p1[0]][p1[1]] 
		if self.empty in (p1, p2):
			self.empty = p1 if p1 != self.empty else p2

	def _validate_grid(self):
		vals = set()
		n = 0
		for arr in self.grid:
			for val in arr:
				n+=1
				vals.add(val)

		if vals != {1,2,3,4,5,6,7,8,0} or n != 9:
			raise ValueError(f"all nine values in grid must be integers in the range [0,8]")

	def copy(self):
		new_grid = []

		for arr in self.grid:
			new_grid.append([])
			for x in arr:
				new_grid[-1].append(x)

		return Board(new_grid, self.empty)

class State:
	bi: Board
	m: int
	bi_1: Union["State", None]

	def __init__(self, bi, m = 0, bi_1 = None):
		self.bi = bi
		self.m = m
		self.bi_1 = bi_1

	def generate_neighbors(self) -> set[Board]:
		if self.bi_1:
			return self.bi.generate_neighbors(self.bi_1.bi.empty)
		return self.bi.generate_neighbors(None)

	def goal_is_met(self):
		goal1 = [[1,2,3],
				 [4,5,6],
				 [7,8,0],]

		return self.bi.grid == goal1

	def total_manhatten_distance(self) -> int:
		summ = 0
		for i, arr in enumerate(self.bi.grid):
			for j, val in enumerate(arr):
				if val == 0:
					continue
				md = manhatten_distance((i, j), ((val-1)//3, (val-1)%3))
				summ += md
		return summ

	def __lt__(self, other):
		return self.m + self.total_manhatten_distance() < other.m + other.total_manhatten_distance()

	def board_path(self) -> list[Board]:
		if not self.bi_1:
			return [self.bi]
		return self.bi_1.board_path() + [self.bi]


if __name__ == "__main__":
	from queue import PriorityQueue

	pq = PriorityQueue()

	grid = input_grid()

	initial_board = Board(grid)
	initial_state = State(initial_board)
	current_state = initial_state

	cont = True
	
	while not current_state.goal_is_met():

		for board in current_state.generate_neighbors():
			state = State(board, current_state.m+1, current_state)
			
			if state.m % 30 == 0:
				print(f"{state.m=} would you like to continue (y/n)? it may be infinite!")
				if input().lower() == 'n':
					cont = False
					break
			
			pq.put(state)

		if not cont:
			break

		current_state = pq.get()

	for i, board in enumerate(current_state.board_path()):
		print("move: " + str(i), *board.grid, "", sep = "\n")
