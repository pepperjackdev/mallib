#include <iostream>
#include "linear_algebra/matrix.hpp"

int main() {
    Matrix<int> matrix(3, 3);
    std::cout << matrix.toString() << std::endl;
}