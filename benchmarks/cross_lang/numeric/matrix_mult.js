const n = 120;
const a = new Int32Array(n * n);
const b = new Int32Array(n * n);
const c = new Int32Array(n * n);

for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
        a[i * n + j] = (i * 3 + j) % 10;
        b[i * n + j] = (i * 7 + j) % 10;
        c[i * n + j] = 0;
    }
}

for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
        let sum = 0;
        for (let k = 0; k < n; k++) {
            sum += a[i * n + k] * b[k * n + j];
        }
        c[i * n + j] = sum;
    }
}

let total = 0;
const total_cells = n * n;
for (let idx = 0; idx < total_cells; idx++) {
    total += c[idx];
}

console.log(total);
