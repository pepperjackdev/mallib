#pragma once

#include <vector>
#include <string>
#include <format>

template <typename T>
class Matrix {
private:
    std::vector<T> data;
    size_t rows, cols;

public:
    Matrix(size_t rows, size_t cols): 
        rows(rows), cols(cols), data(rows * cols) {}

    T& operator()(size_t row, size_t col) {
        return data[cols * row + col];
    }

    const T& operator()(size_t row, size_t col) const {
        return data[cols * row + col];
    }

    std::string toString() const {
        std::string str = "[";
        for (size_t row = 0; row < rows; row++) {
            if (row > 0) str+=" ";
            str += "[";
            for (size_t col = 0; col < cols; col++) {
                str += std::format("{}", (*this)(row, col));
                if (col + 1 < cols) str += ", ";
            }
            str += "]";
            if (row + 1 < rows) str += ",\n";
        }
        str += "]";
        return str;
    }
};
