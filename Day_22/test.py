new_index = (5, 12)
row_start, row_end = (0, 11)
column_start, column_end = (8, 11)
row_dif = row_end - row_start + 1
column_dif = column_end - column_start + 1
modified_row = column_start + \
    ((new_index[0] - column_start) % column_dif)
modified_col = row_start + \
    ((new_index[1] - row_start) % row_dif)

print(modified_row, modified_col)
