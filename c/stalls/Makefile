CXX=gcc
CXXFLAGS=-Wall -Werror
SOURCES=$(wildcard *.c)
OBJS=$(SOURCES:%.c=%.o)
EXEC=sol

all: $(EXEC)

release: CXXFLAGS += -O3 -DNDEBUG
release: clean all

debug: CXXFLAGS += -g3 -DDEBUG
debug: clean all

profile: CXXFLAGS += -pg
profile: clean all


$(EXEC): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $@ 

%.o: %.c
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(EXEC) $(OBJS) *.dSYM

.PHONY: clean
