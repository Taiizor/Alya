#include <stdio.h>
#include <string.h>

long long fnv1a(const char *s) {
    size_t slen = strlen(s);
    long long h = 2166136261LL;
    for (size_t i = 0; i < slen; i++) {
        long long c = (unsigned char)s[i];
        h ^= c;
        h = (h * 16777619LL) & 0x7FFFFFFFLL;
    }
    return h;
}

int main() {
    const char *sample = "The quick brown fox jumps over the lazy dog! Alya is fast and modern.";
    long long h = 0;
    for (int i = 0; i < 50000; i++) {
        h = fnv1a(sample);
    }
    printf("%lld\n", h);
    return 0;
}
