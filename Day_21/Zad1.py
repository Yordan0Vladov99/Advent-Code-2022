def get_equation(monkey, monkeys, cache):
    eq = monkeys[monkey]
    if monkey in cache:
        return cache[monkey]

    if eq.isnumeric():
        cache[monkey] = int(eq)
        return int(eq)
    args = eq.split(' ')
    lmonkey = get_equation(args[0], monkeys, cache)
    rmonkey = get_equation(args[2], monkeys, cache)
    operation = args[1]
    if operation == "+":
        cache[monkey] = lmonkey + rmonkey
        return lmonkey + rmonkey
    elif operation == "-":
        cache[monkey] = lmonkey - rmonkey
        return lmonkey - rmonkey
    elif operation == "*":
        cache[monkey] = lmonkey * rmonkey
        return lmonkey * rmonkey
    else:
        cache[monkey] = lmonkey / rmonkey
        return lmonkey / rmonkey


f = open('./Day_21/input.txt')
lines = f.readlines()
monkeys = {}

cache = {}

for line in lines:
    monkey = line.split(':')[0]
    eq = line.split(': ')[1][:-1]
    monkeys[monkey] = eq

print(get_equation("root", monkeys, cache))
