"""
 * Numeric string, decimal digits 0....9
 * 123, 920, any length
 * 1 -> a
 * 2 -> b
 * ...
 * 9 -> i
 10 -> j
 11 -> k, aa
 * Inout- < numeric string
 * compute: # of ways to translate to an alphanetic string
 * "123" -> abc, lc, aw-> 3 ways -> output = 3
 *
"""

mapping = {str(i - 96): chr(i) for i in range(97, 123)}


def numeric_string(input_str):
    if len(input_str) == 0:
        return 0
    elif len(input_str) == 1:
        return 1
    elif len(input_str) == 2:
        if input_str in mapping:
            # "[12]", "[1][2]" -> 2
            return 2
        else:
            # "[28]", "[2][8]" -> 1 (since "[28]" doesn't work)
            return 1
    else:
        # "[123]", "[1][23]", "[12][3]", "[1][2][3]" -> 3
        # "[127]", "[1][27]", "[12][7]", "[1][2][7]" -> 2
        # "[999]", "[9][99]", "[99][9]", "[9][9][9]" -> 1
        # print(input_str[0:len(input_str) - 1], " ", input_str[1:])
        first_slice = numeric_string(input_str[0:len(input_str) - 1])
        last_slice = numeric_string(input_str[1:])
        ret = first_slice + last_slice - 1
        if (first_slice > 2 or last_slice > 2) and first_slice != last_slice:
            ret -= 1
        # print("returned:", ret)
        return ret


if __name__ == '__main__':
    assert(numeric_string("123") == 3)
    assert(numeric_string("127") == 2)
    assert(numeric_string("999") == 1)
    assert(numeric_string("4123") == 3)
    assert(numeric_string("1234") == 3)
    assert(numeric_string("1278") == 2)
    assert(numeric_string("9999") == 1)
    assert(numeric_string("1212") == 5)
    assert(numeric_string("55555") == 1)
