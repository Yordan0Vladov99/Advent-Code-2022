import numpy as np

f = open('./Day_18/test_input.txt')
lines = f.readlines()


grid = np.zeros((22, 22, 22))
cubes = []
for line in lines:
    coords = [int(x) for x in line[:-1].split(',')]
    x = coords[0]
    y = coords[1]
    z = coords[2]
    grid[x][y][z] = 1
    cubes.append((x, y, z))
sides = len(lines) * 6

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
