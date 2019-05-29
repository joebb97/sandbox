CXX=g++
CXXFLAGS=-std=c++14 -Wconversion -Wall -Wextra -pedantic -I./include #took out -Werror
LDFLAGS=-lpthread
SOURCES=$(wildcard src/*.cpp)
temp=$(SOURCES:%.cpp=%.o)
OBJS=$(addprefix obj/, $(notdir $(temp)))
EXEC=bin/threads

all: $(EXEC)

release: CXXFLAGS += -O3 -DNDEBUG
release: clean all

debug: CXXFLAGS += -g3 -DDEBUG
debug: clean all

profile: CXXFLAGS += -pg
profile: clean all


$(EXEC): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $@ $(LDFLAGS)

obj/%.o: src/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@ 

### ADD HEADER DEPENDENCIES HERE
src/asdf.cpp: include/asdf.h


clean:
	rm -rf bin/* 
	rm -rf obj/*


a: all
c: clean
d: debug
r: release
p: profile
.PHONY: all clean release debug a c d r p
