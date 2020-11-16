#include <iostream>

extern "C" {
void test() { std::cout << "Hello strange world"; }
}
