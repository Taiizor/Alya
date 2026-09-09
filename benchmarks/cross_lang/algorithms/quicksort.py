import sys

# Increase recursion depth for quicksort
sys.setrecursionlimit(100000)

def partition(arr, low, high):
    pivot = arr[high]
    i = low - 1
    for j in range(low, high):
        if arr[j] <= pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1

def quicksort(arr, low, high):
    if low < high:
        pi = partition(arr, low, high)
        quicksort(arr, low, pi - 1)
        quicksort(arr, pi + 1, high)

n = 50000
arr = [0] * n
seed = 42
for i in range(n):
    seed = (seed * 1103515245 + 12345) % 1000000
    if seed < 0:
        seed = -seed
    arr[i] = seed

quicksort(arr, 0, n - 1)

checksum = arr[0] + arr[25000] + arr[n - 1]
print(checksum)
