def regex_divisible_b(n):
    if n == 1:
        return '^(0|1)+$'
    exp = 0
    while n % 2 == 0:
        n /= 2
        exp += 1
    m = [['' for j in range(n)] for i in range(n)]
    for i in range(n):
        m[i][i * 2 % n] = '0'
        m[i][(i * 2 + 1) % n] = '1'
    for i in range(n):
        m[i][n - 1] = ''
        m[n - 1][i] = ''
    if n % 2 == 1:
        m[int((n - 1) / 2)][n - 2] = '01*0'
    else:
        m[int(n / 2 - 1)][n - 2] = '11*0'

    buf_i = ['' for i in range(n)]
    buf_o = ['' for i in range(n)]
    for k in range(1, n - 1):
        for i in range(n):
            buf_i[i] = ''
            buf_o[i] = ''
        for i in range(n):
            if m[i][k] != '':
                buf_i[i] = m[i][k]
                m[i][k] = ''
            if m[k][i] != '':
                buf_o[i] = m[k][i]
                m[k][i] = ''
        for i in range(n):
            if buf_i[i] != '':
                for j in range(n):
                    if buf_o[j] != '':
                        if buf_i[k] == '':
                            if m[i][j] == '':
                                m[i][j] = '(' + buf_i[i] + ')(' + buf_o[j] + ')'
                            else:
                                m[i][j] = '(' + m[i][j] + ')|(' + buf_i[i] + ')(' + buf_o[j] + ')'
                        elif i != k:
                            if m[i][j] == '':
                                m[i][j] = '(' + buf_i[i] + ')(' + buf_i[k] + ')*(' + buf_o[j] + ')'
                            else:
                                m[i][j] = '(' + m[i][j] + ')|(' + '(' + buf_i[i] + ')(' + buf_i[k] + ')*(' + buf_o[j] + '))'
    if exp == 0:
        return '^(' + m[0][0] + ')+$'
    elif n == 1:
        return '^' + '(0|1)*' + '0' * (exp + 1) + '$'
    else:
        return '^' + '(' + m[0][0] + ')*' + '0' * (exp + 1) + '$'

