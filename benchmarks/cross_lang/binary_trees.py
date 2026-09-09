import sys

# Increase recursion depth if needed
sys.setrecursionlimit(50000)

class TreeNode:
    __slots__ = ('left', 'right', 'item')
    def __init__(self, left, right, item):
        self.left = left
        self.right = right
        self.item = item

def make_tree(item, depth):
    if depth == 0:
        return TreeNode(None, None, item)
    return TreeNode(make_tree(2 * item - 1, depth - 1), make_tree(2 * item, depth - 1), item)

def check_tree(node):
    if node.left is None:
        return node.item
    return node.item + check_tree(node.left) - check_tree(node.right)

def main():
    max_depth = 14
    stretch = make_tree(0, max_depth + 1)
    total = check_tree(stretch)

    long_lived = make_tree(0, max_depth)

    for depth in range(4, max_depth + 1, 2):
        iterations = 1 << (max_depth - depth + 4)
        check = 0
        for i in range(1, iterations + 1):
            t1 = make_tree(i, depth)
            t2 = make_tree(-i, depth)
            check += check_tree(t1) + check_tree(t2)
        total += check

    total += check_tree(long_lived)
    print(total)

if __name__ == '__main__':
    main()
