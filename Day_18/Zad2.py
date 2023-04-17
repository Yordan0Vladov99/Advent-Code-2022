import numpy as np
import sys


sys.setrecursionlimit(10000)

f = open('./Day_18/test_input.txt')
lines = f.readlines()


def traverse_grid(x, y, z, grid, used):
    used[x][y][z] = True

    if x > 0 and not used[x - 1][y][z] and grid[x - 1][y][z] == 0:
        traverse_grid(x - 1, y, z, grid, used)

    if x < 21 and not used[x + 1][y][z] and grid[x + 1][y][z] == 0:
        traverse_grid(x + 1, y, z, grid, used)

    if y > 0 and not used[x][y - 1][z] and grid[x][y-1][z] == 0:
        traverse_grid(x, y - 1, z, grid, used)

    if y < 21 and not used[x][y + 1][z] and grid[x][y+1][z] == 0:
        traverse_grid(x, y + 1, z, grid, used)

    if z > 0 and not used[x][y][z-1] and grid[x][y][z-1] == 0:
        traverse_grid(x, y, z - 1, grid, used)

    if z < 21 and not used[x][y][z+1] and grid[x][y][z+1] == 0:
        traverse_grid(x, y, z + 1, grid, used)


grid = np.zeros((22, 22, 22))
used = np.zeros((22, 22, 22), dtype=bool)
cubes = []
for line in lines:
    coords = [int(x) for x in line[:-1].split(',')]
    x = coords[0]
    y = coords[1]
    z = coords[2]
    grid[x][y][z] = 1
    cubes.append((x, y, z))
sides = len(lines) * 6

traverse_grid(0, 0, 0, grid, used)

for x in range(22):
    for y in range(22):
        for z in range(22):
            if grid[x][y][z] == 0 and not used[x][y][z]:
                grid[x][y][z] = 1
for cube in cubes:
    x = cube[0]
    y = cube[1]
    z = cube[2]

    if x > 0 and grid[x - 1][y][z] == 1:
        sides -= 1
    if x < 21 and grid[x + 1][y][z] == 1:
        sides -= 1

    if y > 0 and grid[x][y - 1][z] == 1:
        sides -= 1
    if y < 21 and grid[x][y + 1][z] == 1:
        sides -= 1

    if z > 0 and grid[x][y][z - 1] == 1:
        sides -= 1
    if z < 21 and grid[x][y][z+1] == 1:
        sides -= 1

print(sides)
