const m = new Map();
const n = 20000;

for (let i = 0; i < n; i++) {
    m.set("key_" + i, i * 3);
}

for (let i = 0; i < n; i++) {
    if (i % 2 === 0) {
        m.set("key_" + i, i * 5);
    }
}

let sum = 0;
for (let i = 0; i < n; i++) {
    sum += m.get("key_" + i);
}

console.log(sum);
