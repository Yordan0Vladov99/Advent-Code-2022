import ast

f_input = open("test.txt", "r")
lines = f_input.readlines()
left = ast.literal_eval(lines[0])
right = ast.literal_eval(lines[1])

print(left[0][0])
print(right[0][0])
