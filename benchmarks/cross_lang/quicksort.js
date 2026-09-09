function partition(arr, low, high) {
    const pivot = arr[high];
    let i = low - 1;
    for (let j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            const temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    const temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;
    return i + 1;
}

function quicksort(arr, low, high) {
    if (low < high) {
        const pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

const n = 50000;
const arr = new Int32Array(n);
let seed = 42;
for (let i = 0; i < n; i++) {
    seed = (seed * 1103515245 + 12345) % 1000000;
    if (seed < 0) seed = -seed;
    arr[i] = seed;
}

quicksort(arr, 0, n - 1);

const checksum = arr[0] + arr[25000] + arr[n - 1];
console.log(checksum);
