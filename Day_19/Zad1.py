import math

f = open('./Day_19/input.txt')
lines = f.readlines()


def dfs(bp, max_cost, cache, time, robots, amount):
    if time == 0:
        return amount[3]

    key = tuple([time, *robots, *amount])

    if key in cache:
        return cache[key]

    maxval = amount[3] + robots[3] * time

    for bot_type, recipe in enumerate(bp):
        if bot_type != 3 and robots[bot_type] >= max_cost[bot_type]:
            continue

        wait = 0
        for recipe_amount, recipe_type in recipe:
            if robots[recipe_type] == 0:
                break
            wait = max(wait, math.ceil(
                (recipe_amount - amount[recipe_type]) / robots[recipe_type]))
        else:
            rem_time = time - wait - 1
            if rem_time <= 0:
                continue
            _robots = robots[:]
            _amount = [x + y * (wait + 1) for x, y in zip(amount, robots)]
            for r_amount, r_type in recipe:
                _amount[r_type] -= r_amount
            _robots[bot_type] += 1
            for i in range(3):
                _amount[i] = min(_amount[i], max_cost[i] * rem_time)
            maxval = max(maxval, dfs(bp, max_cost, cache,
                         rem_time, _robots, _amount))
    cache[key] = maxval
    return maxval


value_sum = 0

for line in lines:

    line_list = line.split(' ')
    blueprint_id = int(line_list[1][:-1])

    ore_robot_cost = [(int(line_list[6]), 0)]
    clay_robot_cost = [(int(line_list[12]), 0)]
    obsidian_robots_cost = [(int(line_list[18]), 0), (int(line_list[21]), 1)]
    geode_robots_cost = [(int(line_list[27]), 0), (int(line_list[30]), 2)]
    max_cost = [max(ore_robot_cost[0][0], clay_robot_cost[0][0], obsidian_robots_cost[0]
                    [0], geode_robots_cost[0][0]), obsidian_robots_cost[1][0], geode_robots_cost[1][0]]
    bp = [ore_robot_cost, clay_robot_cost,
          obsidian_robots_cost, geode_robots_cost]
    v = dfs(bp, max_cost, {}, 24, [1, 0, 0, 0], [0, 0, 0, 0])
    value_sum += v * blueprint_id

print(value_sum)
