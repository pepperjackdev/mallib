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

        Matrix(size_t rows, size_t cols, const std::vector<T>& data): 
            rows(rows), cols(cols), data(data) {
            if (data.size() != rows * cols) {
                throw std::invalid_argument(std::format(
                    "Data does not match the shape ({},{})",
                    rows, cols
                ));
            }
        }

        T& operator()(size_t row, size_t col) {
            return data[cols * row + col];
        }

        const T& operator()(size_t row, size_t col) const {
            return data[cols * row + col];
        }

        Matrix<T> transpose() const {
            Matrix<T> transposed(this->cols, this->rows);
            for (size_t row = 0; row < this->rows; row++) {
                for (size_t col = 0; col < this->cols; col++) {
                    transposed(col, row) = (*this)(row, col);
                }
            }
            return transposed;
        }

        Matrix<T> dot(const Matrix<T>& rhs) const {
            if (cols != rhs.rows) {
                throw std::invalid_argument("Matrices are not aligned");
            }

            Matrix<T> product(this->rows, rhs.cols);
            for (size_t row = 0; row < product.rows; row++) {
                for (size_t col = 0; col < product.cols; col++) {
                    T sum = T{};
                    for (size_t k = 0; k < this->cols; k++) {
                        sum += (*this)(row, k) * rhs(k, col);
                    }
                    product(row, col) = sum;
                }
            }
            return product;
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
