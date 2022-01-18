from functools import reduce


def longestCommonPrefix(strs):
    idx = 0
    accum = ""
    while True:
        c = None
        for string in strs:
            if not c and string:
                c = string[idx]
            elif idx >= len(string) or string[idx] != c:
                return accum

        accum += c
        idx += 1


def red_func(res, cur):
    c = cur[0]
    all_match = all(map(lambda char: char == c, cur))
    return res + c if all_match else res

# return reduce(red_func, zip(*strs), "")


def longestCommonPrefixFunction(strs):
    return reduce(
        lambda result, cur:
            result + cur[0]
            if all(map(lambda char: char == cur[0], cur)) else result,
        zip(*strs),
        ""
    )


if __name__ == "__main__":
    dafunc = longestCommonPrefixFunction  # shall be within you
    assert dafunc([""]) == ""
    assert dafunc(["ab", "a"]) == "a"
    assert dafunc(["ab", ""]) == ""
    assert dafunc(["asnthb", "asnth"]) == "asnth"
    assert dafunc(["flower", "flow", "flowerpot"]) == "flow"
