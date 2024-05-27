from math import isqrt
from collections import defaultdict, deque

def create_graph(n):
    graph = {i: [] for i in range(1, n + 1)}
    perfect_squares = set(isqrt(i) ** 2 for i in range(2 * n + 1))  # Precompute perfect squares up to 2*n

    for i in range(1, n + 1):
        for j in range(i + 1, n + 1):
            if (i + j) in perfect_squares:
                graph[i].append(j)
                graph[j].append(i)

    # Sort the adjacency lists based on the length (connectivity) of the nodes
    for key in graph:
        graph[key].sort(key=lambda x: len(graph[x]), reverse=True)
    
    return graph

def hamiltonian_path(graph, n, path, visited, best_path):
    if len(path) == n:
        return path
    last_node = path[-1]
    for neighbor in graph[last_node]:
        if neighbor not in visited:
            visited.add(neighbor)
            path.append(neighbor)
            result = hamiltonian_path(graph, n, path, visited, best_path)
            if result:
                return result
            path.pop()
            visited.remove(neighbor)
    return None

def square_sums(n):
    graph = create_graph(n)
    # Start with the node that has the highest connectivity
    start_nodes = sorted(graph.keys(), key=lambda x: len(graph[x]), reverse=True)
    best_path = None

    for start in start_nodes:
        path = [start]
        visited = {start}
        result = hamiltonian_path(graph, n, path, visited, best_path)
        if result:
            return result
    return False