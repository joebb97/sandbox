#include "util.h"
#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

/*
 * Prints custom error mesage with and without errno.
 * Used for input validation.
 * Crashes the program.
 */
void err_wrap(const char *err_msg) {
  if (errno) {
    perror(err_msg);
  } else {
    fprintf(stderr, "%s", err_msg);
  }
  exit(EXIT_FAILURE);
}

/*
 * Function: init_stalls
 * ---------------------
 *  Converts unformatted character input into an array of integers.
 *  (HOPEFULLY) prevents overflow by stopping once stalls_len numbers have been
 * processed. Assumes an evil person is providing input
 *
 *  stalls: An array of size stalls_len to hold the input to the problem
 *  line: A cstring to transfer input from stdin into the stalls array
 *
 *  Example: Turns input in the form of
 *  123, 1 1 12,\n\x \t 88 99 123
 *  into the stalls array.
 *  [123, 1, 1, 12, 88, 99, 123]
 */
void init_stalls(int *stalls, long stalls_len, char *line, long line_size) {
  const int strtol_base = 10; // Assumes input is specified in base 10
  int sidx = 0, done = 0, same_number = 0;
  // Stop if we hit EOF
  while (fgets(line, line_size, stdin) != NULL) {
    char *it = line;
    // Find each contiguous string of digits in a line and
    // add them to the stalls array. it == iterator over the line.
    // Shouldn't overflow since fgets always puts a NULL terminator
    while (*it != '\0') {
      char it_is_digit = isdigit(*it);
      if (it_is_digit && !same_number) {
        // strtol handles the cases where you pass
        // "123<other non digit characters>" correctly, it'll return 123.
        // It'll also set &next to be the last character it left off at.
        char *next;
        stalls[sidx++] = strtol(it, &next, strtol_base);
        it = next;
        same_number = 1;
        if (sidx == stalls_len) {
          done = 1;
          break;
        }
        continue;
      } else if (!it_is_digit) {
        // We've hit a new number that we need to consume.
        same_number = 0;
      }
      ++it;
    }
    if (done)
      // We're here if we've consumed stalls_len numbers.
      break;
  }
}

/*
 * Function: calculate_social_distance
 * ------------------------------------
 *
 *  Calculate the MAX D after adding two cows to the stalls
 *
 *  A 1 denotes a cow occupies a stall, a 0 denotes the stall is empty
 *
 *  Example 1:
 *  ---------
 *  N: 14
 *  1 0 0 0 1 0 0 1 0 0 0 0 1 0
 *  D: 2
 *  Why? -> 1 0 0 0 1 0 0 0 1 0 X 0 0 1 0
 *  Where X denotes where a cow was added
 *
 *  Example 2:
 *  ---------
 *  N: 12
 *  1 0 0 1 1 1 1 1 1 1 1 1
 *  D: 0
 *
 */
int calculate_social_distance(int *stalls, long stalls_len) { 
    int best = 0;
    int left_idx = 0, right_idx = 0;
    for (int i = 0; i < stalls_len; ++i) {
        if (stalls[i]) {
            right_idx = i;
            int diff = right_idx - left_idx - 1;
            if (diff > best) {
                if (!stalls[left_idx] || !stalls[right_idx]) {
                    best = diff;
                } else {
                    if (diff % 2) {
                        best = (diff / 2);
                    } else {
                        best = (diff / 2) - 1;
                    }
                }
            }
            left_idx = i;
        }
    }
    right_idx = stalls_len - 1;
    int diff = right_idx - left_idx - 1;
    if (!stalls[left_idx] || !stalls[right_idx]) {
        if (diff > best) {
            best = diff;
        }
    }
    return best;
}
