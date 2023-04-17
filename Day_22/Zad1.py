def traverse_grid(grid, index, direction, count, row_edges, column_edges):
    for i in range(count):
        new_index = (index[0] + direction[0], index[1] + direction[1])
        row_start, row_end = row_edges[index[0]]
        column_start, column_end = column_edges[index[1]]
        row_dif = row_end - row_start + 1
        column_dif = column_end - column_start + 1
        modified_row = column_start + \
            ((new_index[0] - column_start) % column_dif)
        modified_col = row_start + \
            ((new_index[1] - row_start) % row_dif)
        new_index = (modified_row, modified_col)
        if grid[new_index[0]][new_index[1]] == '#':
            break
        index = new_index
    return index


f = open('./Day_22/input.txt')
lines = f.readlines()
directions = [(0, 1), (-1, 0), (0, -1), (1, 0)]
grid = [line[:-1] for line in lines[:-1]]
instructions = lines[-1]
direction = 0
index = (0, grid[0].index('.'))

row_edges = {}
column_edges = {}
for count, row in enumerate(grid):
    l = row.find('.')
    r = row.find('#')
    start = None
    if l == -1 or r == -1:
        start = max(l, r)
    else:
        start = min(l, r)

    l = row.rfind('.')
    r = row.rfind('#')
    end = max(l, r)
    row_edges[count] = (start, end)

    for ind, col in enumerate(row):
        if col == " ":
            continue
        if ind not in column_edges:
            column_edges[ind] = (count, count)
        else:
            column_edges[ind] = (min(count, column_edges[ind][0]), max(
                count, column_edges[ind][1]))

num = ""
for count, c in enumerate(instructions[:-1]):
    if c.isnumeric():
        num += c
        if count == len(instructions[:-1]) - 1:
            index = traverse_grid(grid, index, directions[direction], int(
                num), row_edges, column_edges)
            num == ""

    else:
        index = traverse_grid(grid, index, directions[direction], int(
            num), row_edges, column_edges)
        num = ""
        if c == "L":
            direction = (direction + 1) % 4
        else:
            direction = (direction - 1) % 4

print((index[0] + 1) * 1000 + (index[1] + 1)*4 + direction)
