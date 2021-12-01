### Stacks and Queues are nice, but they are just too basic for a senior System Engineer

#### Let's level up and get to know a little bit more advanced data structure called `Tree` by stating some **_obvious facts_**:
1. A tree is just an `collection` of nodes where:
    - Each node has a `value` and/or a `reference` to its children
2. A childless node is called a `leaf`
3. The first node is called `the root`
4. A `branch` is a link between a parent node and a child node
    1. A tree whose nodes have at most 2 branches/children is called a `binary tree` 
5. A child node can very well be the `root node of a sub-tree` of the tree

### Tree Algorithms - Traversing a tree
Unlike `linear data structures` (Array, Linked List, Queues, Stacks, etc) which have `only one logical way` to **traverse** them, trees can be traversed in different ways. Let's consider the first traversal strategy.

#### In-order travesal

![Simple Tree](https://media.geeksforgeeks.org/wp-content/cdn-uploads/2009/06/tree12.gif)

Applying `In-order traversal algorithm` to the `binary tree` above gives us:
- 4, 2, 5, 1, 3
 
![Big Tree](https://contribute.geeksforgeeks.org/wp-content/uploads/binary-tree-to-DLL.png)

Applying it to the the more complex `binary tree` above gives us:
- 8 - 4 - 9 - 2 - 10 - 5 - 11 - 1 - 6 - 13 - 3 - 14 - 7

Re-arrange the following instructions to implement this algorithm in Python:

1. 
```
traverse(tree.getRightChild())
```
2. 
```
traverse(tree.getLeftChild())
```
3. 
```
if tree:
```

4. 
```
print(tree.getRootVal())
```

5. 
```
def traverse(tree):
```

In case you need some explanation:
- def = define a function
- getRightChild() = return the child node on the right
- getLeftChild() = return the child node on the left
- getRootVal() = get value of the root node