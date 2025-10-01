"""
This is an example program for ../matrices/multiply_matrices.py.

This script demonstrates how to use the multiply_matrices function to perform
matrix multiplication on two given matrices. It includes sample matrices and
shows the resulting product matrix.

Usage:
    Run this script to see an example of matrix multiplication.

Example:
    Given matrices:
    matrix1 = [[1, 2],
               [3, 4]]
    matrix2 = [[5, 6],
               [7, 8]]

    The resulting product matrix will be:
    result = [[19, 22],
              [43, 50]]
"""
from matrices.multiply_matrices import multiply_matrices
import numpy as np

matrix1 = np.array([[1, 2], [3, 4]])
matrix2 = np.array([[5, 6], [7, 8]])
result = multiply_matrices(matrix1, matrix2)
print(result)