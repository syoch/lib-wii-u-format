from math import *


def is_prime(x):
    if x < 2:
        return False
    elif x == 2:
        return True
    elif x % 2 == 0:
        return False
    elif x % 3 == 0:
        return False

    for i in range(5, x, 2):
        if x % i == 0:
            return False
    return True


def get_next_prime(x):
    if x % 2 == 0:
        x += 1

    while not is_prime(x):
        x += 2
    return x

# Ported from this C++ code:
# https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a#3-5-拡張-euclid-の互除法による逆元計算
# Thanks to drken


def modinv(a, m):
    b = m
    u = 1
    v = 0
    while b:
        t = a // b
        a -= t * b
        u -= t * v

        (a, b, u, v) = (b, a, v, u)
    u %= m
    if u < 0:
        u += m
    return u


def find_factor_from_sqrt(x):
    a = int(sqrt(x)) + 1
    for i in range(a, (a + x) // 2):
        if x % i == 0:
            return i
    return None
