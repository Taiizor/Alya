def fnv1a(s):
    h = 2166136261
    for ch in s:
        c = ord(ch)
        h ^= c
        h = (h * 16777619) & 0x7FFFFFFF
    return h

if __name__ == "__main__":
    sample = "The quick brown fox jumps over the lazy dog! Alya is fast and modern."
    h = 0
    for _ in range(50000):
        h = fnv1a(sample)
    print(h)
