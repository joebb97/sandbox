import random
import time

def read_guess(guess, actual, prints=True):
    if guess < actual:
        if prints:
            print 'Too low'
        return False
    elif guess > actual:
        if prints:
            print 'Too high'
        return False
    else:
        if prints:
            print "You guessed right! The number was " + str(actual)
        return True


def play_game_human():
    print "I'm picking a number between 0 and 100, try to guess it"
    actual = random.randint(0, 100)
    guess = int(raw_input("Guess: "))
    while not read_guess(guess, actual):
        guess = int(raw_input("Guess: "))


def play_game_computer():
    # print "I'm picking a number between 0 and 100, try to guess it"
    end = 100000000
    actual = random.randint(0, end)
    high = end + 1
    low = 0
    guesses = []
    guess = (high + low) // 2
    guesses.append(guess)
    start = time.time()
    while not read_guess(guess, actual, False):
        if guess < actual:
            low = guess
        elif guess > actual:
            high = guess
        guess = (high + low) // 2
        guesses.append(guess)
    end = time.time()
    print "Guessing took: " + str(end - start)
    print guesses


if __name__ == "__main__":
    play_game_computer()
