f = open("./Day_15/input.txt")
lines = f.readlines()


def manhattan_distance(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


max_x = None
min_x = None
max_y = None
min_y = None
closest = {}

for line in lines:
    sensor = line.split(": ")[0].split("at ")[1]
    sensor_x = int(sensor.split(", ")[0].split("=")[1])
    sensor_y = int(sensor.split(", ")[1].split("=")[1])

    beacon = line.split(": ")[1].split("at ")[1]
    beacon_x = int(beacon.split(", ")[0].split("=")[1])
    beacon_y = int(beacon.split(", ")[1].split("=")[1])

    #print("sensor:", sensor_x, sensor_y, " beacon:", beacon_x, beacon_y)

    closest[(sensor_x, sensor_y)] = (beacon_x, beacon_y)

    max_x = max(sensor_x, beacon_x) if max_x is None else max(
        max_x, sensor_x, beacon_x)
    min_x = min(sensor_x, beacon_x) if min_x is None else min(
        min_x, sensor_x, beacon_x)

    max_y = max(sensor_y, beacon_y) if max_y is None else max(
        max_y, sensor_y, beacon_y)
    min_y = max(sensor_y, beacon_y) if min_y is None else min(
        min_y, sensor_y, beacon_y)

impossible_beacons = 0
for i in range(min(min_y, min_x), max(max_y, max_x)+1):
    pos = (i, 2_000_000)
    for sensor, beacon in closest.items():
        if pos != beacon and manhattan_distance(sensor, pos) <= manhattan_distance(sensor, beacon):
            impossible_beacons += 1
            break


print(impossible_beacons)
