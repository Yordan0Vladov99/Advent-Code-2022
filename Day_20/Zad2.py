class Node:
    def __init__(self, n):
        self.n = n
        self.prev = None
        self.next = None


f = open("./Day_20/input.txt")
arr = [Node(int(x) * 811589153) for x in f.readlines()]

for i in range(len(arr)):
    arr[i].prev = arr[(i - 1) % len(arr)]
    arr[i].next = arr[(i + 1) % len(arr)]

for _ in range(10):
    z = None
    m = len(arr) - 1

    for k in arr:
        if k.n == 0:
            z = k
            continue
        p = k
        if k.n > 0:
            for _ in range(k.n % m):
                p = p.next
            if k == p:
                continue
            k.next.prev = k.prev
            k.prev.next = k.next
            p.next.prev = k
            k.next = p.next
            p.next = k
            k.prev = p
        else:
            for _ in range(-k.n % m):
                p = p.prev
            if k == p:
                continue
            k.prev.next = k.next
            k.next.prev = k.prev
            p.prev.next = k
            k.prev = p.prev
            p.prev = k
            k.next = p

t = 0

for _ in range(3):
    for _ in range(1000):
        z = z.next
    print(z.n, end=" ")
    t += z.n
print()
print(t)
