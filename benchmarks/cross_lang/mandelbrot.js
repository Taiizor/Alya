function mandelbrot_point(cr, ci, max_iter) {
    let zr = 0.0;
    let zi = 0.0;
    let iter = 0;
    while (iter < max_iter) {
        let zr2 = zr * zr;
        let zi2 = zi * zi;
        if (zr2 + zi2 > 4.0) {
            return iter;
        }
        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        iter++;
    }
    return max_iter;
}

function compute_grid(width, height, max_iter) {
    let total = 0;
    for (let y = 0; y < height; y++) {
        let ci = -1.5 + (3.0 * y) / height;
        for (let x = 0; x < width; x++) {
            let cr = -2.0 + (3.0 * x) / width;
            let iters = mandelbrot_point(cr, ci, max_iter);
            total += iters;
        }
    }
    return total;
}

const total = compute_grid(200, 100, 200);
console.log(total);
