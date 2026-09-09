#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TABLE_SIZE 65536
#define MASK (TABLE_SIZE - 1)

typedef struct {
    char key[32];
    int val;
    int occupied;
} Entry;

static unsigned int djb2(const char *s) {
    unsigned int hash = 5381;
    while (*s) {
        hash = ((hash << 5) + hash) + (unsigned char)(*s++);
    }
    return hash;
}

static void map_set(Entry *table, const char *key, int val) {
    unsigned int idx = djb2(key) & MASK;
    while (table[idx].occupied) {
        if (strcmp(table[idx].key, key) == 0) {
            table[idx].val = val;
            return;
        }
        idx = (idx + 1) & MASK;
    }
    table[idx].occupied = 1;
    strncpy(table[idx].key, key, 31);
    table[idx].key[31] = '\0';
    table[idx].val = val;
}

static int map_get(Entry *table, const char *key) {
    unsigned int idx = djb2(key) & MASK;
    while (table[idx].occupied) {
        if (strcmp(table[idx].key, key) == 0) {
            return table[idx].val;
        }
        idx = (idx + 1) & MASK;
    }
    return 0;
}

int main(void) {
    Entry *table = (Entry *)calloc(TABLE_SIZE, sizeof(Entry));
    int n = 20000;
    char buf[32];

    for (int i = 0; i < n; i++) {
        snprintf(buf, sizeof(buf), "key_%d", i);
        map_set(table, buf, i * 3);
    }

    for (int i = 0; i < n; i++) {
        if (i % 2 == 0) {
            snprintf(buf, sizeof(buf), "key_%d", i);
            map_set(table, buf, i * 5);
        }
    }

    int sum = 0;
    for (int i = 0; i < n; i++) {
        snprintf(buf, sizeof(buf), "key_%d", i);
        sum += map_get(table, buf);
    }

    printf("%d\n", sum);
    free(table);
    return 0;
}
