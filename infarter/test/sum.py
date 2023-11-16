def el(i):
    return 1.0 / (i*i)

sum = 0.0
i = 1

while i < 100000:
    sum += el(float(i))
    i += 1

print(sum)
