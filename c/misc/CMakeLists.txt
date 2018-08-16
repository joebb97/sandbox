cmake_minimum_required(VERSION 2.8.9)
project(bits_c)

set(CMAKE_C_STANDARD 99)

include_directories(include)

file(GLOB SOURCES "src/*.c")

add_executable(bits_c ${SOURCES})
