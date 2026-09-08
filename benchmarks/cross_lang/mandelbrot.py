def mandelbrot_point(cr, ci, max_iter):
    zr = 0.0
    zi = 0.0
    i = 0
    while i < max_iter:
        zr2 = zr * zr
        zi2 = zi * zi
        if zr2 + zi2 > 4.0:
            return i
        zi = 2.0 * zr * zi + ci
        zr = zr2 - zi2 + cr
        i += 1
    return max_iter

def compute_grid(width, height, max_iter):
    total = 0
    for y in range(height):
        ci = -1.5 + (3.0 * y) / height
        for x in range(width):
            cr = -2.0 + (3.0 * x) / width
            iters = mandelbrot_point(cr, ci, max_iter)
            total += iters
    return total

if __name__ == "__main__":
    total = compute_grid(200, 100, 200)
    print(total)
