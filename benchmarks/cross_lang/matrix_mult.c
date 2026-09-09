#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n = 120;
    int *a = (int *)malloc(sizeof(int) * n * n);
    int *b = (int *)malloc(sizeof(int) * n * n);
    int *c = (int *)malloc(sizeof(int) * n * n);

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            a[i * n + j] = (i * 3 + j) % 10;
            b[i * n + j] = (i * 7 + j) % 10;
            c[i * n + j] = 0;
        }
    }

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            int sum = 0;
            for (int k = 0; k < n; k++) {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }

    int total = 0;
    int total_cells = n * n;
    for (int idx = 0; idx < total_cells; idx++) {
        total += c[idx];
    }

    printf("%d\n", total);

    free(a);
    free(b);
    free(c);
    return 0;
}
