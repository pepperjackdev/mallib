#include <iostream>
#include "linear_algebra/matrix.hpp"

int main() {
    Matrix<int64_t> matrix(3, 3, {
        1, 2, 3,
        4, 5, 6,
        7, 8, 9
    });

    Matrix<int64_t> product = matrix.dot(matrix);
    std::cout << product.toString() << std::endl;
}