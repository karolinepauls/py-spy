import time


def f(arg):
    if arg == 50:
        time.sleep(100)
        return

    a = 1
    b = {}
    c = globals()
    f(arg + 1)


if __name__ == "__main__":
    f(1)
