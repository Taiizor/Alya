#include <stdio.h>
#include <stdlib.h>

long long count_primes(long long limit) {
    long long *flags = (long long *)malloc((limit + 1) * sizeof(long long));
    for (long long i = 0; i <= limit; i++) {
        flags[i] = 1;
    }

    for (long long p = 2; p * p <= limit; p++) {
        if (flags[p] == 1) {
            for (long long multiple = p * p; multiple <= limit; multiple += p) {
                flags[multiple] = 0;
            }
        }
    }

    long long count = 0;
    for (long long j = 2; j <= limit; j++) {
        if (flags[j] == 1) {
            count++;
        }
    }
    free(flags);
    return count;
}

int main() {
    long long primes = count_primes(50000);
    printf("%lld\n", primes);
    return 0;
}
