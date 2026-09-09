def main():
    n = 120
    a = [(i * 3 + j) % 10 for i in range(n) for j in range(n)]
    b = [(i * 7 + j) % 10 for i in range(n) for j in range(n)]
    c = [0] * (n * n)

    for i in range(n):
        i_n = i * n
        for j in range(n):
            s = 0
            for k in range(n):
                s += a[i_n + k] * b[k * n + j]
            c[i_n + j] = s

    total = sum(c)
    print(total)

if __name__ == '__main__':
    main()
