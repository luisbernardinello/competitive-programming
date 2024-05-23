MOD = 99824435

def height(n, m):
    fact = precalc_fact()
    max_height = 0
    product = 1
    max_mod = m % MOD
    for i in range(1, n + 1):
        product = calc_product(product, max_mod, fact, i)
        max_height = new_max_height(max_height, product)
    return max_height

def precalc_fact():
    fact = [0] * 80001
    fact[0], fact[1] = 0, 1

    for i in range(2, 80001):
        fact[i] = (MOD - MOD//i) * fact[MOD%i] % MOD

    return fact

def calc_product(product, max_mod, fact, i):
    return (product * (max_mod - i + 1) % MOD * fact[i]) % MOD

def new_max_height(max_height, product):
    max_height = (max_height + product) % MOD
    if max_height < 0:
        max_height += MOD
    return max_height
