#ifndef THREADS_H
#define THREADS_H

#include <iostream>       // std::cout
#include <thread>         // std::thread
#include <mutex>          // std::mutex

void lock_mutex(std::mutex);

void print_block(int, char);

int main_impl();


#endif
