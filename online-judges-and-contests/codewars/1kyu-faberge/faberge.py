MOD = 998244353

def height(n, m):
    inv_factorials = [0] * 80001
    inv_factorials[0] = 0
    inv_factorials[1] = 1

    for i in range(2, 80001):
        inv_factorials[i] = (MOD - MOD // i) * inv_factorials[MOD % i] % MOD

    max_height = 0
    product = 1
    m_mod = m % MOD

    for i in range(1, n + 1):
        product = product * (m_mod - i + 1) % MOD * inv_factorials[i] % MOD
        max_height = (max_height + product) % MOD

    if max_height < 0:
        max_height += MOD

    return max_height