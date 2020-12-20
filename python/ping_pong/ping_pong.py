import queue
import threading


def ping(ping_queue, pong_queue, num_times):
    for _ in range(num_times):
        print(ping_queue.get())
        pong_queue.put("pong")


def pong(ping_queue, pong_queue, num_times):
    for _ in range(num_times):
        ping_queue.put("ping")
        print(pong_queue.get())


if __name__ == '__main__':
    ping_queue = queue.Queue()
    pong_queue = queue.Queue()
    num_times = 5
    threads = [threading.Thread(target=ping, args=(ping_queue, pong_queue, num_times)),
               threading.Thread(target=pong, args=(ping_queue, pong_queue, num_times))]
    for thread in threads:
        thread.start()

    for thread in threads:
        thread.join()
