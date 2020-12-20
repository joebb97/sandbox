import asyncio

async def ping(ping_event, pong_event, num_times):
    for _ in range(num_times):
        await ping_event.wait()
        print("ping")
        ping_event.clear()
        pong_event.set()

async def pong(ping_event, pong_event, num_times):
    for _ in range(num_times):
        ping_event.set()
        await pong_event.wait()
        print("pong")
        pong_event.clear()

async def main():
    ping_event, pong_event = asyncio.Event(), asyncio.Event()
    num_times = 5
    ping_task = asyncio.create_task(ping(ping_event, pong_event, num_times))
    pong_task = asyncio.create_task(pong(ping_event, pong_event, num_times))
    await ping_task
    await pong_task

print_str = "ping"
async def ping_cond(cond, num_times):
    global print_str
    for _ in range(num_times):
        async with cond:
            while print_str != "ping":
                await cond.wait()
            print(print_str)
            print_str = "pong"
            cond.notify()

async def pong_cond(cond, num_times):
    global print_str
    for _ in range(num_times):
        async with cond:
            while print_str != "pong":
                await cond.wait()
            print(print_str)
            print_str = "ping"
            cond.notify()

async def main_cond():
    num_times = 5
    cond = asyncio.Condition()
    ping_task = asyncio.create_task(ping_cond(cond, num_times))
    pong_task = asyncio.create_task(pong_cond(cond, num_times))
    await ping_task
    await pong_task

if __name__ == '__main__':
    # asyncio.run(main())
    asyncio.run(main_cond())
