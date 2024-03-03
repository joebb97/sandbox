#include <mutex>
#include <condition_variable>
#include <thread>
#include <functional>

class ZeroEvenOdd {
private:
    int n;
    int to_print;
    bool zeros_turn;
    std::mutex m;
    std::condition_variable print_even;
    std::condition_variable print_odd;
    std::condition_variable print_zero;
public:
    ZeroEvenOdd(int n) {
        this->n = n;
        this->to_print = 1;
        this->zeros_turn = true;
    }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        std::unique_lock<std::mutex> lck(this->m);
        while (!zeros_turn) {
            this->print_zero.wait(lck);
        }
        printNumber(0);
        this->zeros_turn = false;
        if (this->to_print % 2) {
            this->print_even.notify_one();
        } else {
            this->print_odd.notify_one();
        }
    }

    void even(function<void(int)> printNumber) {
        std::unique_lock<std::mutex> lck(this->m);
        while (this->to_print % 2 != 0) {
            this->print_even.wait(lck);
        }
        printNumber(this->to_print++);
        this->zeros_turn = true;
        this->print_zero.notify_one();
    }

    void odd(function<void(int)> printNumber) {
        std::unique_lock<std::mutex> lck(this->m);
        while (this->to_print % 2 == 0) {
            this->print_odd.wait(lck);
        }
        printNumber(this->to_print++);
        this->zeros_turn = true;
        this->print_zero.notify_one();
    }
};
