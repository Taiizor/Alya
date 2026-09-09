#include <stdio.h>

int mandelbrot_point(double cr, double ci, int max_iter) {
    double zr = 0.0;
    double zi = 0.0;
    int iter = 0;
    while (iter < max_iter) {
        double zr2 = zr * zr;
        double zi2 = zi * zi;
        if (zr2 + zi2 > 4.0) {
            return iter;
        }
        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        iter++;
    }
    return max_iter;
}

long long compute_grid(int width, int height, int max_iter) {
    long long total = 0;
    for (int y = 0; y < height; y++) {
        double ci = -1.5 + (3.0 * y) / height;
        for (int x = 0; x < width; x++) {
            double cr = -2.0 + (3.0 * x) / width;
            int iters = mandelbrot_point(cr, ci, max_iter);
            total += iters;
        }
    }
    return total;
}

int main() {
    long long total = compute_grid(200, 100, 200);
    printf("%lld\n", total);
    return 0;
}
