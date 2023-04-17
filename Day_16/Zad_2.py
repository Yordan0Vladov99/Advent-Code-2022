from collections import deque

f = open('./Day_16/input.txt')
lines = f.readlines()
tunnels = {}
nonempty = []
dists = {}
for line in lines:
    line_spl = line.split(" ")
    name = line_spl[1]
    flow_rate = int(line_spl[4][5:-1])
    neighbours = [x[:-1] for x in line_spl[9:]]
    tunnels[name] = (flow_rate, neighbours)

for tunnel in tunnels:
    if tunnel != "AA" and not tunnels[tunnel][0]:
        continue

    if tunnel != "AA":
        nonempty.append(tunnel)

    dists[tunnel] = {tunnel: 0, "AA": 0}
    visited = {tunnel}
    queue = deque([(0, tunnel)])
    while queue:
        distance, position = queue.popleft()

        for neighbour in tunnels[position][1]:
            if neighbour in visited:
                continue
            visited.add(neighbour)
            if tunnels[neighbour][0]:
                dists[tunnel][neighbour] = distance + 1
            queue.append((distance + 1, neighbour))
    del dists[tunnel][tunnel]
    if tunnel != "AA":
        del dists[tunnel]["AA"]

indices = {}

for index, element in enumerate(nonempty):
    indices[element] = index

cache = {}


def dfs(time, valve, bitmask):
    if (time, valve, bitmask) in cache:
        return cache[(time, valve, bitmask)]

    maxval = 0
    for neighbour in dists[valve]:
        bit = 1 << indices[neighbour]
        if bitmask & bit:
            continue
        remtime = time - dists[valve][neighbour] - 1
        if remtime <= 0:
            continue
        maxval = max(maxval, dfs(remtime, neighbour, bitmask |
                     bit) + tunnels[neighbour][0] * remtime)
    cache[(time, valve, bitmask)] = maxval
    return maxval


b = (1 << len(nonempty)) - 1
m = 0

for i in range((b + 1) // 2):
    m = max(m, dfs(26, "AA", i) + dfs(26, "AA", b ^ i))

print(m)
