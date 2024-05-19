import threading

phi_cache = {}
cache_lock = threading.Lock()

def euler_totient(n):
    with cache_lock:
        if n in phi_cache:
            return phi_cache[n]
    
    original_n = n
    result = n
    i = 2
    while i * i <= n:
        if n % i == 0:
            while n % i == 0:
                n //= i
            result -= result // i
        i += 1
    if n > 1:
        result -= result // n
    
    with cache_lock:
        phi_cache[original_n] = result
    
    return result

def calculate_operation(input, divisor):
    four_times_divisor = divisor * 4
    if input < four_times_divisor:
        return input
    remainder = input % divisor
    three_times_divisor = divisor * 3
    return remainder + three_times_divisor

def tower_mod_pow(base, exp, mod):
    result = 1
    while exp != 0:
        if exp % 2 == 1:
            result = calculate_operation(result * base, mod)
        base = calculate_operation(base * base, mod)
        exp //= 2
    return result

def calculate_growth(base, height, modulus):
    if height == 1 or modulus == 1:
        return calculate_operation(base, modulus)
    
    next_modulus = euler_totient(modulus)
    next_growth = calculate_growth(base, height - 1, next_modulus)
    return tower_mod_pow(base, next_growth, modulus)

def tower(base, height, modulus):
    if modulus == 1:
        return 0
    if base == 1 or height == 0:
        return 1
    if height == 1:
        return base % modulus
    
    return calculate_growth(base, height, modulus) % modulus
