"""
This module provides functionality for multiplying matrices.

    multiply_matrices: Multiplies two matrices and returns the result.

    Ensure that the number of columns in the first matrix is equal to the number of rows in the second matrix.

    Guakocius

    # 12/17/2024
"""
import numpy as np

def multiply_matrices(matrix1, matrix2):
    """
    Multiply two matrices if the number of columns in matrix1 is equal to the number of rows in matrix2.

    Args:
        matrix1 (np.ndarray): The first matrix.
        matrix2 (np.ndarray): The second matrix.

    Returns:
        np.ndarray: The product of the two matrices.
    """
    return matrix1 @ matrix2