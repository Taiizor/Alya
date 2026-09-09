function TreeNode(left, right, item) {
    this.left = left;
    this.right = right;
    this.item = item;
}

function makeTree(item, depth) {
    if (depth === 0) {
        return new TreeNode(null, null, item);
    }
    return new TreeNode(makeTree(2 * item - 1, depth - 1), makeTree(2 * item, depth - 1), item);
}

function checkTree(node) {
    if (node.left === null) {
        return node.item;
    }
    return node.item + checkTree(node.left) - checkTree(node.right);
}

const maxDepth = 14;
const stretch = makeTree(0, maxDepth + 1);
let total = checkTree(stretch);

const longLived = makeTree(0, maxDepth);

for (let depth = 4; depth <= maxDepth; depth += 2) {
    const iterations = 1 << (maxDepth - depth + 4);
    let check = 0;
    for (let i = 1; i <= iterations; i++) {
        const t1 = makeTree(i, depth);
        const t2 = makeTree(-i, depth);
        check += checkTree(t1) + checkTree(t2);
    }
    total += check;
}

total += checkTree(longLived);
console.log(total);
