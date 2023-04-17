import numpy as np


def print_grid(grid, min_height, height, min_width, width):
    print(min_height, height)
    print(min_width, width)
    for i in range(min_height, height+1):
        for j in range(min_width, width+1):
            print(grid[i][j], end=" ")
        print()


f = open("input.txt")
lines = f.readlines()
min_height = None
min_width = None
height = 0
width = 0
grains = 0
for line in lines:
    for pair in line[:-1].split(" -> "):
        p = pair.split(",")
        min_height = int(p[1]) if min_height is None else min(
            min_height, int(p[1]))
        min_width = int(p[0]) if min_width is None else min(
            min_width, int(p[0]))

        height = max(height, int(p[1]))
        width = max(width, int(p[0]))

grid = np.full((height + 1, width + 1), " ")

for line in lines:
    p = line.split(" -> ")[0].split(",")
    prev_height = int(p[1])
    prev_width = int(p[0])
    for pair in line[:-1].split(" -> ")[1:]:
        p = pair.split(",")
        cur_height = int(p[1])
        cur_width = int(p[0])

        if (cur_height == prev_height):
            for col in range(min(cur_width, prev_width), max(cur_width, prev_width)+1):
                grid[cur_height][col] = '#'
        else:
            for row in range(min(cur_height, prev_height), max(cur_height, prev_height) + 1):
                grid[row][cur_width] = '#'
        prev_height = cur_height
        prev_width = cur_width


overflow = False

while not overflow:
    overflow = True
    y = 500
    for x in range(0, height):
        if grid[x+1][y] in ["#", "o"]:
            if y == 0:
                break
            elif grid[x+1][y-1] not in ["#", "o"]:
                y -= 1
            elif y == width:
                break
            elif grid[x+1][y+1] not in ["#", "o"]:
                y += 1
            else:
                grid[x][y] = "o"
                grains += 1
                overflow = False
                break

#print_grid(grid, min_height, height, min_width, width)
print(grains)
