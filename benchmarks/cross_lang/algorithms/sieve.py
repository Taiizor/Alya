def count_primes(limit):
    flags = [1] * (limit + 1)
    p = 2
    while p * p <= limit:
        if flags[p] == 1:
            multiple = p * p
            while multiple <= limit:
                flags[multiple] = 0
                multiple += p
        p += 1

    count = 0
    for j in range(2, limit + 1):
        if flags[j] == 1:
            count += 1
    return count

if __name__ == "__main__":
    primes = count_primes(50000)
    print(primes)
