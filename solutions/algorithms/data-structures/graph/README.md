# Graph


A graph is a fundamental data structure comprising two main components:

1. A finite set of vertices, also known as nodes.
2. A finite set of ordered pairs (u, v) known as edges, which represent connections between vertices.


# Graphs are typically represented in two main ways:


## Adjacency Matrix

An adjacency matrix is a 2D array of size V x V, where V represents the number of vertices in the graph.

- Each cell M[i][j] of the matrix holds a value:
-   M[i][j] = 1 if there exists an edge between vertices i and j.
-   M[i][j] = 0 otherwise.
-   Complexity:
    -   Space: `O(V ** 2)`
    -   Checking for the existence of an edge: `O(1)`

![](https://www.geeksforgeeks.org/wp-content/uploads/adjacency_matrix_representation.png)

## Adjacency List

An adjacency list is an array of linked lists, where each list represents the vertices adjacent to a particular vertex.

-   Complexity:
    -   Space: `O(V + E)`, where E represents the number of edges in the graph.

![](https://www.geeksforgeeks.org/wp-content/uploads/adjacency_list_representation.png)

# Breadth First Traversal (BFS)

Breadth First Traversal is a graph traversal algorithm used to explore nodes layer by layer. It employs a queue data structure to keep track of nodes.

1. Mark the current node as visited and enqueue it.
2. Process each adjacent node of the current node and enqueue them if they have not been visited.
3. Dequeue the current node and repeat steps 1 and 2 for its adjacent nodes.
4. Continue until the queue is empty.

![](https://he-s3.s3.amazonaws.com/media/uploads/fdec3c2.jpg)

# Depth First Traversal (DFS)

DFS is another graph traversal algorithm that explores as far as possible along each branch before backtracking. It utilizes a stack data structure to keep track of nodes.

1. Mark the current node as visited and push it onto the stack.
2. Process each adjacent node of the current node recursively if they have not been visited.
3. Pop the current node from the stack and repeat step 2 for its adjacent nodes.
4. Continue until the stack is empty.

![](https://he-s3.s3.amazonaws.com/media/uploads/9fa1119.jpg)
