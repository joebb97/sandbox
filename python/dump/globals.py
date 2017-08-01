myGlobal = 0

def myFunc():
    global myGlobal
    myGlobal += 1

def main():
    print 'started'
    while myGlobal < 10:
        myFunc()
    print 'finished'

if __name__ == '__main__':
    main()
