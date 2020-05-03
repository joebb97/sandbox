#include "util.h"
#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <ctype.h>

void err_wrap(const char *err_msg) {
  if (errno) {
    perror(err_msg);
  } else {
    fprintf(stderr, "%s", err_msg);
  }
  exit(EXIT_FAILURE);
}

void init_stalls(int * stalls, long stalls_len,
                 char * line, long line_size){
  int sidx = 0, done = 0, same_number = 0;
  while (fgets(line, line_size, stdin) != NULL) {
    char *it = line;
    while (*it != '\0') {
      char it_is_digit = isdigit(*it);
      if (it_is_digit && !same_number) {
        stalls[sidx++] = atoi(it);
        same_number = 1;
        if (sidx == stalls_len) {
          done = 1;
          break;
        }
      } else if (!it_is_digit){
        same_number = 0;
      }
      ++it;
    }
    if (done)
      break;
  }
}


