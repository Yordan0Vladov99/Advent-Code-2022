f = open('./Day_20/input.txt')
arr = [(y, int(x)) for y, x in enumerate(f.readlines())]
decrypted_arr = arr[:]
for i in range(len(arr)):
    # print(decrypted_arr)
    index = decrypted_arr.index(arr[i])

    steps = arr[i][1]
    inc = 1 if steps > 0 else -1

    for j in range(0, steps, inc):
        temp = decrypted_arr[index]
        decrypted_arr[index] = decrypted_arr[(
            index + inc) % len(decrypted_arr)]
        decrypted_arr[(index + inc) % len(decrypted_arr)] = temp
        index = (index + inc) % len(decrypted_arr)


# print(decrypted_arr)
zindex = 0
for ind, x in enumerate(decrypted_arr):
    if x[1] == 0:
        zindex = ind
        break


print(decrypted_arr[(zindex + 1000) % len(decrypted_arr)], decrypted_arr[(zindex + 2000) %
      len(decrypted_arr)], decrypted_arr[(zindex + 3000) % len(decrypted_arr)])

print(decrypted_arr[(zindex + 1000) % len(decrypted_arr)][1] + decrypted_arr[(zindex + 2000) %
      len(decrypted_arr)][1] + decrypted_arr[(zindex + 3000) % len(decrypted_arr)][1])
