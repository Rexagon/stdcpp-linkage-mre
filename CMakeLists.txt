cmake_minimum_required(VERSION 3.10)

project(mylib)
add_library(mylib STATIC src/mylib.cpp)
install(TARGETS mylib DESTINATION .)
