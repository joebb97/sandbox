import itertools


def is_palindrome(s):
    # return s == s[::-1]
    if not s:
        return False

    low, high = 0, len(s) - 1
    while low < high:
        if s[low] != s[high]:
            return False
        low += 1
        high -= 1
    return True


def is_palindrome_alpha(s):
    low, high = 0, len(s) - 1
    while low < high:
        if not s[low].isalpha():
            low += 1
        elif not s[high].isalpha():
            high -= 1
        else:
            if s[low].lower() != s[high].lower():
                return False
            low += 1
            high -= 1

    return True


def is_palindrome_for(s):
    # return s == s[::-1]
    if not s:
        return False

    for i in range(0, len(s) // 2):
        # back = len(s) - 1 - i
        back = ~i

        if s[i] != s[back]:
            return False

    return True


def all_palindromes(s):
    ret = set()
    for length in range(1, len(s) + 1):
        for start in range(0, len(s) - length + 1):
            sub_str = s[start:start+length]
            if is_palindrome(sub_str):
                ret.add(sub_str)
    print(ret)


def all_palindromes_comb(s):
    return ({s[i:j]
             for (i, j) in itertools.combinations(range(0, len(s) + 1), 2)
             if is_palindrome(s[i:j])})


def all_substrings(s):
    for length in range(1, len(s) + 1):
        for start in range(0, len(s) - length + 1):
            print(s[start:start+length])


def all_substrings_comb(s):
    return {s[i:j]
            for (i, j) in itertools.combinations(range(0, len(s) + 1), 2)}
