import ast
from functools import cmp_to_key


def cmp(left, right):
    l = ast.literal_eval(left)
    r = ast.literal_eval(right)
    return compare(l, r)


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
packets = []
for line in lines:
    if line != "\n":
        packets.append(line)

packets.append("[[2]]")
packets.append("[[6]]")

packets = sorted(packets, key=cmp_to_key(cmp))

start = 0
end = 0
for i in range(0, len(packets)):
    if packets[i] == "[[2]]":
        start = i + 1
    if packets[i] == "[[6]]":
        end = i+1

print(start * end)
