def get_equation(monkey, monkeys, cache):
    eq = monkeys[monkey]
    if monkey in cache:
        return cache[monkey]

    if monkey == "humn" or eq.isnumeric():
        cache[monkey] = float(eq)
        return float(eq)
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


f = open('./Day_21/modified_input.txt')
lines = f.readlines()
monkeys = {}

cache = {}

for line in lines:
    monkey = line.split(':')[0]
    eq = line.split(': ')[1][:-1]
    monkeys[monkey] = eq


l = 3000000000000
r = 4000000000000
a = get_equation("qggp", monkeys, cache)
while l <= r:
    m = l + (r - l)/2
    monkeys["humn"] = str(m)
    cache = {}
    b = get_equation("tcmj", monkeys, cache)
    if a == b:
        print(m)
        break
    elif a > b:
        r = m
    else:

        l = m

print(int(a), int(b))
print(len(str(int(a))), len(str(int(b))))
