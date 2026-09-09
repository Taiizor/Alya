#include <stdio.h>
#include <stdlib.h>

typedef struct TreeNode {
    struct TreeNode *left;
    struct TreeNode *right;
    long long item;
} TreeNode;

static TreeNode *make_tree(long long item, int depth) {
    TreeNode *node = (TreeNode *)malloc(sizeof(TreeNode));
    node->item = item;
    if (depth == 0) {
        node->left = NULL;
        node->right = NULL;
    } else {
        node->left = make_tree(2 * item - 1, depth - 1);
        node->right = make_tree(2 * item, depth - 1);
    }
    return node;
}

static long long check_tree(TreeNode *node) {
    if (node->left == NULL) {
        return node->item;
    }
    return node->item + check_tree(node->left) - check_tree(node->right);
}

static void free_tree(TreeNode *node) {
    if (node == NULL) return;
    free_tree(node->left);
    free_tree(node->right);
    free(node);
}

int main(void) {
    int max_depth = 14;
    TreeNode *stretch = make_tree(0, max_depth + 1);
    long long total = check_tree(stretch);
    free_tree(stretch);

    TreeNode *long_lived = make_tree(0, max_depth);

    for (int depth = 4; depth <= max_depth; depth += 2) {
        int iterations = 1 << (max_depth - depth + 4);
        long long check = 0;
        for (int i = 1; i <= iterations; i++) {
            TreeNode *t1 = make_tree(i, depth);
            TreeNode *t2 = make_tree(-i, depth);
            check += check_tree(t1) + check_tree(t2);
            free_tree(t1);
            free_tree(t2);
        }
        total += check;
    }

    total += check_tree(long_lived);
    free_tree(long_lived);

    printf("%lld\n", total);
    return 0;
}
