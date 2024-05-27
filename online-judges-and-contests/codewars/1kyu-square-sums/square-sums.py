from math import isqrt

def is_perfect_square(x):
    return isqrt(x) ** 2 == x

def create_graph(n):
    graph = {i: [] for i in range(1, n+1)}
    for i in range(1, n+1):
        for j in range(i+1, n+1):
            if is_perfect_square(i + j):
                graph[i].append(j)
                graph[j].append(i)
    return graph

def hamiltonian_path(graph, n, path, visited):
    if len(path) == n:
        return path
    last_node = path[-1]
    for neighbor in graph[last_node]:
        if neighbor not in visited:
            visited.add(neighbor)
            path.append(neighbor)
            result = hamiltonian_path(graph, n, path, visited)
            if result:
                return result
            path.pop()
            visited.remove(neighbor)
    return None

def square_sums(n):
    graph = create_graph(n)
    for start in range(1, n+1):
        path = [start]
        visited = {start}
        result = hamiltonian_path(graph, n, path, visited)
        if result:
            return result
    return False