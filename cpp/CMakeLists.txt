cmake_minimum_required(VERSION 3.31)
project(cpp)

set(CMAKE_CXX_STANDARD 14)

#include_directories(.)
#include_directories()
include_directories(${CMAKE_SOURCE_DIR}/../../
        ${CMAKE_SOURCE_DIR}/../target/cxxbridge/)

add_executable(cpp
        hello.cc
        hello.h)
