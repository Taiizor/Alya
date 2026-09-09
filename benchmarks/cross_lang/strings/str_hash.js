function fnv1a(s) {
    const slen = s.length;
    let h = 2166136261;
    for (let i = 0; i < slen; i++) {
        const c = s.charCodeAt(i);
        h ^= c;
        h = Math.imul(h, 16777619) & 0x7FFFFFFF;
    }
    return h;
}

const sample = "The quick brown fox jumps over the lazy dog! Alya is fast and modern.";
let h = 0;
for (let i = 0; i < 50000; i++) {
    h = fnv1a(sample);
}
console.log(h);
