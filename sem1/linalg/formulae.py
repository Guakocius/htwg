"""
This module provides functions and classes for performing linear algebra calculations.

Functions:
    add_matrices(matrix_a, matrix_b):
        Adds two matrices of the same dimensions.

    subtract_matrices(matrix_a, matrix_b):
        Subtracts matrix_b from matrix_a, both of the same dimensions.

    multiply_matrices(matrix_a, matrix_b):
        Multiplies two matrices if the number of columns in matrix_a is equal to the number of rows in matrix_b.

    transpose_matrix(matrix):
        Returns the transpose of the given matrix.

    determinant(matrix):
        Calculates the determinant of a square matrix.

    inverse_matrix(matrix):
        Calculates the inverse of a square matrix if it exists.

Classes:
    Vector:
        A class representing a mathematical vector with methods for vector operations such as addition, subtraction, dot product, and cross product.

    Matrix:
        A class representing a mathematical matrix with methods for matrix operations such as addition, subtraction, multiplication, and finding the determinant and inverse.
"""

import time
import numpy as np

class Vector:
    def add_vectors(self, vector_a, vector_b):
        """
        Add two vectors of the same dimensions.

        Args:
            vector_a (np.ndarray): The first vector.
            vector_b (np.ndarray): The second vector.

        Returns:
            np.ndarray: The sum of the two vectors.
        """
        return vector_a + vector_b

    def subtract_vectors(self, vector_a, vector_b):
        """
        Subtract vector_b from vector_a, both of the same dimensions.

        Args:
            vector_a (np.ndarray): The first vector.
            vector_b (np.ndarray): The second vector.

        Returns:
            np.ndarray: The result of subtracting vector_b from vector_a.
        """
        return vector_a - vector_b

    def dot_product(self, vector_a, vector_b):
        """
        Calculate the dot product of two vectors of the same dimensions.

        Args:
            vector_a (np.ndarray): The first vector.
            vector_b (np.ndarray): The second vector.

        Returns:
            float: The dot product of the two vectors.
        """
        return np.dot(vector_a, vector_b)

    def cross_product(self, vector_a, vector_b):
        """
        Calculate the cross product of two 3-dimensional vectors.

        Args:
            vector_a (np.ndarray): The first vector.
            vector_b (np.ndarray): The second vector.

        Returns:
            np.ndarray: The cross product of the two vectors.
        """
        return np.cross(vector_a, vector_b)

    def magnitude(self, vector):
        """
        Calculate the magnitude of a vector.

        Args:
            vector (np.ndarray): The vector.

        Returns:
            float: The magnitude of the vector.
        """
        return np.linalg.norm(vector)

    def normalize(self, vector):
        """
        Normalize a vector to have unit length.

        Args:
            vector (np.ndarray): The vector.

        Returns:
            np.ndarray: The normalized vector.
        """
        return vector / np.linalg.norm(vector)

    def angle_between_vectors(self, vector_a, vector_b):
        """
        Calculate the angle in radians between two vectors.

        Args:
            vector_a (np.ndarray): The first vector.
            vector_b (np.ndarray): The second vector.

        Returns:
            float: The angle in radians between the two vectors.
        """
        return np.arccos(np.dot(vector_a, vector_b) / (np.linalg.norm(vector_a) * np.linalg.norm(vector_b)))

    def project_vector(self, vector_to_project, projection_vector):
        """
        Project a vector onto another vector.

        Args:
            vector_to_project (np.ndarray): The vector to project.
            projection_vector (np.ndarray): The vector to project onto.

        Returns:
            np.ndarray: The projected vector.
        """
        return np.dot(vector_to_project, projection_vector) / np.dot(projection_vector, projection_vector) * projection_vector

    def scalar_projection(self, vector_to_project, projection_vector):
        """
        Calculate the scalar projection of a vector onto another vector.

        Args:
            vector_to_project (np.ndarray): The vector to project.
            projection_vector (np.ndarray): The vector to project onto.

        Returns:
            float: The scalar projection of the vector.
        """
        return np.dot(vector_to_project, projection_vector) / np.linalg.norm(projection_vector)

    def vector_projection(self, vector_to_project, projection_vector):
        """
        Calculate the vector projection of a vector onto another vector.

        Args:
            vector_to_project (np.ndarray): The vector to project.
            projection_vector (np.ndarray): The vector to project onto.

        Returns:
            np.ndarray: The vector projection of the vector.
        """
        return np.dot(vector_to_project, projection_vector) / np.dot(projection_vector, projection_vector) * projection_vector
    ## End of Vector class

class Matrix:

    def add_matrices(self, matrix_a, matrix_b):
        """
        Add two matrices of the same dimensions.

        Args:
            matrix_a (np.ndarray): The first matrix.
            matrix_b (np.ndarray): The second matrix.

        Returns:
            np.ndarray: The sum of the two matrices.
        """
        return matrix_a + matrix_b

    def subtract_matrices(self, matrix_a, matrix_b):
        """
        Subtract matrix_b from matrix_a, both of the same dimensions.

        Args:
            matrix_a (np.ndarray): The first matrix.
            matrix_b (np.ndarray): The second matrix.

        Returns:
            np.ndarray: The result of subtracting matrix_b from matrix_a.
        """
        return matrix_a - matrix_b

    def multiply_matrices(self, matrix_a, matrix_b):
        """
        Multiply two matrices if the number of columns in matrix_a is equal to the number of rows in matrix_b.

        Args:
            matrix_a (np.ndarray): The first matrix.
            matrix_b (np.ndarray): The second matrix.

        Returns:
            np.ndarray: The product of the two matrices.
        """
        return matrix_a @ matrix_b

    def transpose_matrix(self, matrix):
        """
        Return the transpose of the given matrix.

        Args:
            matrix (np.ndarray): The matrix to transpose.

        Returns:
            np.ndarray: The transpose of the matrix.
        """
        return matrix.T

    def determinant(self, matrix):
        """
        Calculate the determinant of a square matrix.

        Args:
            matrix (np.ndarray): The square matrix.

        Returns:
            float: The determinant of the matrix.
        """
        return np.linalg.det(matrix)

    def inverse_matrix(self, matrix):
        """
        Calculate the inverse of a square matrix if it exists.

        Args:
            matrix (np.ndarray): The square matrix.

        Returns:
            np.ndarray: The inverse of the matrix.
        """
        return np.linalg.inv(matrix)

    ## End of Matrix class

def main():
    print("This is the main function of the linear algebra module.")
    time.sleep(2)
    matrix_a = np.array([[1, 2], [3, 4]])
    matrix_b = np.array([[5, 6], [7, 8]])
    vector_a = np.array([1, 2, 3])
    vector_b = np.array([4, 5, 6])
    linear_algebra = Matrix()
    print("Matrix A:")
    print(matrix_a)
    print("Matrix B:")
    print(matrix_b)
    print("Adding Matrix A and Matrix B:")


if __name__ == "__main__":
    main()
