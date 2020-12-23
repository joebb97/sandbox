#include <iostream>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <chrono>

std::mutex mut;
std::condition_variable cv;
std::string print_str = "ping";

void ping(int n) {
    for (int i = 0; i < n; ++i) {
        std::unique_lock<std::mutex> lk(mut);
        while (print_str != "ping") {
            cv.wait(lk);
        }
        std::cout << print_str << "\n";
        print_str = "pong";
        lk.unlock();
        cv.notify_one();
    }
}

void pong(int n) {
    for (int i = 0; i < n; ++i) {
        std::unique_lock<std::mutex> lk(mut);
        while (print_str != "pong") {
            cv.wait(lk);
        }
        std::cout << print_str << "\n";
        print_str = "ping";
        lk.unlock();
        cv.notify_one();
    }
}

int main() {
    int n = 5;
    std::thread ping_thread(ping, n);
    std::thread pong_thread(pong, n);
    ping_thread.join();
    pong_thread.join();
}
