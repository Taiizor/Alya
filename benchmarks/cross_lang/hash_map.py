def main():
    m = {}
    n = 20000

    for i in range(n):
        m["key_" + str(i)] = i * 3

    for i in range(n):
        if i % 2 == 0:
            m["key_" + str(i)] = i * 5

    total = 0
    for i in range(n):
        total += m["key_" + str(i)]

    print(total)

if __name__ == '__main__':
    main()
