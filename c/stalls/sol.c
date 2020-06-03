#include "util.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef DEBUG
#define DEBUG_PRINT(x) printf x
#else
#define DEBUG_PRINT(x)                                                         \
  do {                                                                         \
  } while (0)
#endif

/*
 * INPUT FORMAT
 * ------------
 *  N: <size of stalls>
 *  Stalls: <stalls exist or not>
 *  <EOF if necessary>
 *
 *  Example:
 *  N: 3
 *  Stalls: 1 1 1
 *  <EOF if necessary>
 *
 *  Example:
 *  N: 6
 *  Stalls: 1 1 0 0 1 1
 *  <EOF if necessary>
 *
 */
int main(int argc, char **argv) {
  long arr_len;
  const long max_arr_size = 5000;
  const long line_size = 1000;
  char line[max_arr_size];

  // ASSUME AN EVIL PERSON WANTS TO BE EVIL AND CONTROL THE PROGRAM.
  // Get the input array size
  printf("N: ");
  if (fgets(line, line_size, stdin) == NULL) {
    err_wrap("fgets failed");
  }
  arr_len = strtol(line, NULL, 0);
  // Make sure the array size isn't bogus so we don't get overflowed
  if (arr_len <= 0 || arr_len > max_arr_size) {
    err_wrap("Array size <= 0 or >= ARR_SIZE");
  }
  DEBUG_PRINT(("Array will be size: %ld\n", arr_len));
  // Put this on the heap since we don't really know how big it should be
  printf("Stalls: ");
  int *stalls = (int *)calloc(arr_len, sizeof(int));
  init_stalls(stalls, arr_len, line, line_size);
  free(stalls);
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
 *  Why? -> 1 0 X 0 0 1 0 0 0 1 0 X 0 0 1 0
 *  Where X denotes where a cow was added
 *
 *  Example 2:
 *  ---------
 *
 */
int calculate_social_distance(int *stalls, long stalls_len) { 
    return 0;
}
