CXX=g++
CXXFLAGS=-std=c++14 -Wconversion -Wall -Wextra -pedantic #took out -Werror
SOURCES=$(wildcard src/*.cpp)
temp=$(SOURCES:%.cpp=%.o)
OBJS=$(addprefix obj/, $(notdir $(temp)))
EXEC=bin/threads

release: CXXFLAGS += -O3 -DNDEBUG
release: clean all

debug: CXXFLAGS += -g3 -DDEBUG
debug: clean all

profile: CXXFLAGS += -pg
profile: clean all


all: $(EXEC)


$(EXEC): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $@

$(EXECD): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $@

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
