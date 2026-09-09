function count_primes(limit) {
    const flags = new Array(limit + 1).fill(1);
    for (let p = 2; p * p <= limit; p++) {
        if (flags[p] === 1) {
            for (let multiple = p * p; multiple <= limit; multiple += p) {
                flags[multiple] = 0;
            }
        }
    }

    let count = 0;
    for (let j = 2; j <= limit; j++) {
        if (flags[j] === 1) {
            count++;
        }
    }
    return count;
}

const primes = count_primes(50000);
console.log(primes);
