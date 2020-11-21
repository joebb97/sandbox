cmake_minimum_required(VERSION 3.5)
project(bits_c)

set(CMAKE_C_STANDARD 99)

set(SOURCE_FILES main.c)
add_executable(bits_c ${SOURCE_FILES})

if(NOT EXISTS ../thingy.txt)
    MESSAGE(STATUS "I LOVE CMAKE")
endif()
