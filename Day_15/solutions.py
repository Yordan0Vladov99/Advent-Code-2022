import itertools


class sensor:
    def __init__(self, pos: tuple[int, int], beacon: tuple[int, int]) -> None:
        self.pos = pos
        self.beacon = beacon
        self.dist = abs(self.beacon[0] - self.pos[0]) + abs(
            self.beacon[1] - self.pos[1]
        )


def overlaps(range1: tuple[int, int], range2: tuple[int, int]) -> bool:
    return (
        range2[0] <= range1[0] <= range2[1]
        or range2[0] <= range1[0] <= range2[1]
        or range1[0] <= range2[0] <= range1[1]
        or range1[0] <= range2[0] <= range1[1]
    )


class exc_range:
    def __init__(self) -> None:
        self.rngs = []

    def add(self, start, stop) -> None:
        res: list[tuple[int, int]] = []
        additional: list[tuple[int, int]] = []
        for rngs, rnge in self.rngs:
            if overlaps((rngs, rnge), (start, stop)):
                additional.append((min(start, rngs), max(stop, rnge)))
            else:
                res.append((rngs, rnge))

        if len(additional) >= 1:
            mins = min(map(lambda tup: tup[0], additional))
            maxe = max(map(lambda tup: tup[1], additional))
            res.append((mins, maxe))
        else:
            res.append((start, stop))

        self.rngs = res

    def rem(self, point) -> None:
        res: list[tuple[int, int]] = []
        for rngs, rnge in self.rngs:
            if rngs < point < rnge:
                res.append((rngs, point - 1))
                res.append((point + 1, rnge))
            elif point == rngs:
                res.append((point + 1, rnge))
            elif point == rnge:
                res.append((rngs, point - 1))
            else:
                res.append((rngs, rnge))
        self.rngs = res


def parse(string: str):
    res: list[sensor] = []
    for line in string.splitlines():
        first, second = line.split(": ")
        x, y = first[12:].split(", ")
        x = int(x)
        y = int(y[2:])

        bx, by = second[23:].split(", ")
        bx = int(bx)
        by = int(by[2:])
        res.append(sensor((x, y), (bx, by)))

    return res


def unavailable_positions(y: int, sensors: list[sensor]) -> list[tuple[int, int]]:
    res: exc_range = exc_range()
    occupied: list[int] = []

    for sensor in sensors:
        distance = sensor.dist - abs(y - sensor.pos[1])
        if distance <= 0:
            continue
        if sensor.beacon[1] == y:
            occupied.append(sensor.beacon[0])
        if sensor.pos[1] == y:
            occupied.append(sensor.pos[0])
        start = sensor.pos[0] - distance
        stop = sensor.pos[0] + distance
        res.add(start, stop)

    for pos in occupied:
        res.rem(pos)

    return res.rngs


def main(input):
    sensors = parse(input)
    count = 0
    for start, stop in unavailable_positions(2000000, sensors):
        count += stop - start + 1
    return count


if __name__ == "__main__":
    with open("./Day_15/input.txt") as f:
        input = f.read()
    print(main(input))
