def f():
    i = 1
    iR = 1.0
    acc = 0.0
    while i != 1000000:
      iR = float(i)
      acc = acc + 1.0 / (iR * iR)
      i = i + 1
    print(acc)

f()
