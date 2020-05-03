#ifndef _STALLS_SOL_H
#define _STALLS_SOL_H

void err_wrap(const char *err_msg); 

void init_stalls(int * stalls, long stalls_len,
                 char * line, long line_size);

#endif
