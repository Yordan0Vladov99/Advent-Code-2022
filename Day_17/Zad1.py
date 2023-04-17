f = open('./Day_17/input.txt')
wind = f.readline()

type_a = [['#', '#', '#', '#']]

type_b = [['.', '#', '.'],
          ['#', '#', '#'],
          ['.', '#', '.']]

type_c = [['.', '.', '#'],
          ['.', '.', '#'],
          ['#', '#', '#']]

type_d = [['#'],
          ['#'],
          ['#'],
          ['#'],]

type_e = [["#", "#"],
          ["#", "#"]]

rocks = [type_a, type_b, type_c, type_d, type_e]

grid = [['.' for _ in range(7)] for _ in range(2022 * 4)]
counter = 0
height = len(grid)
win_length = len(wind)

for i in range(2022):
    rock = rocks[i % 5]
    left_edge = 2
    right_edge = left_edge + len(rock[0]) - 1
    rock_length = len(rock)
    row = len(grid) - 1
    isStopped = False
    for j in range(height - 4, len(grid)):
        canGoLeft = True
        canGoRight = True
        for r in range(rock_length):
            for c in range(len(rock[r])):
                if rock[r][c] == '#' and grid[j-rock_length + r + 1][left_edge + c] == '#':
                    isStopped = True
                    row = j - 1
                    break
            if isStopped:
                break
        if isStopped:
            break
        gust = wind[counter % win_length]
        counter += 1
        for r in range(rock_length):
            for c in range(len(rock[r])):
                if rock[r][c] == '#' and left_edge + c > 0 and grid[j-rock_length + r + 1][left_edge + c - 1] == '#':
                    canGoLeft = False
                if rock[r][c] == '#' and left_edge + c < 6 and grid[j-rock_length + r + 1][left_edge + c + 1] == '#':
                    canGoRight = False

        if gust == '<' and left_edge > 0 and canGoLeft:
            left_edge -= 1
            right_edge -= 1
        if gust == '>' and right_edge < 6 and canGoRight:
            left_edge += 1
            right_edge += 1
    if row - rock_length < height - 1:
        height = row - rock_length + 1

    for r in range(rock_length):
        for c in range(len(rock[r])):
            if rock[r][c] == '#':
                grid[row - rock_length + r + 1][left_edge + c] = "#"
print(len(grid) - height)
