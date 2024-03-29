This is the first project for my AI class
The assignment was to write a program that solved an 8-block (“sliding tile”) puzzle.

We used the A* search algorithm which defines a function f(n) = g(n) + h(n) 
where g(n) is the number of moves since the starting board and
h(n) is a heuristic function that estimates the number of moves remaining to reach the goal state.

This makes f(n) a function that helps you find the least cost path from initial state to goal state.

In our case the heuristic h(n) = total manhatten distance of a given board
