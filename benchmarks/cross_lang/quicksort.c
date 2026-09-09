#include <stdio.h>
#include <stdlib.h>

static int partition(int *arr, int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    int temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;
    return i + 1;
}

static void quicksort(int *arr, int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

int main(void) {
    int n = 50000;
    int *arr = (int *)malloc(n * sizeof(int));
    long long seed = 42;
    for (int i = 0; i < n; i++) {
        seed = (seed * 1103515245 + 12345) % 1000000;
        if (seed < 0) seed = -seed;
        arr[i] = (int)seed;
    }

    quicksort(arr, 0, n - 1);

    long long checksum = (long long)arr[0] + arr[25000] + arr[n - 1];
    printf("%lld\n", checksum);
    free(arr);
    return 0;
}
