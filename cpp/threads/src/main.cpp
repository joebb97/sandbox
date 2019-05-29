#include "main.h"

std::mutex mtx;
void lock_mutex(std::mutex & asdf){
    asdf.lock();
}

void print_block(int n, char c) {
  // critical section (exclusive access to std::cout signaled by locking mtx):
    std::lock_guard<std::mutex> guard(mtx);
    for (int i=0; i<n; ++i) { 
        std::cout << c; 
    }
    std::cout << '\n';
}

int main_impl(){ 
    std::thread th2(print_block,50,'$');
    std::thread th1(print_block,50,'*');

    th1.join();
    th2.join();
    return 0;
}

int main(){
    main_impl();
}
