import ast


def compare(left, right):
    for l, r in zip(left, right):
        if type(l) is int and type(r) is int:
            if l < r:
                return -1
            elif l > r:
                return 1
        elif type(l) is list and type(r) is list:
            res = compare(l, r)
            if res != 0:
                return res
        elif type(l) is list:
            r_list = [r]
            res = compare(l, r_list)
            if res != 0:
                return res
        elif type(r) is list:
            l_list = [l]
            res = compare(l_list, r)
            if res != 0:
                return res

    if len(left) < len(right):
        return -1
    elif len(left) > len(right):
        return 1

    return 0


f_input = open("input.txt", "r")
lines = f_input.readlines()
indexes = []

for i in range(0, len(lines), 3):
    left = ast.literal_eval(lines[i])
    right = ast.literal_eval(lines[i+1])

    isRight = compare(left, right) == -1
    if isRight:
        indexes.append(i//3 + 1)

print(sum(indexes))
