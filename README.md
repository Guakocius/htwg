# Linear Algebra in Python

Welcome to the Linear Algebra in Python project! This repository contains various Python scripts and modules to help you understand and perform linear algebra operations.

## Files and Modules

- **matrices/**: Contains functions and classes for matrix operations such as addition, multiplication, inversion, and more.
- **vectors/**: Provides functions and classes for vector operations including dot product, cross product, and normalization.
- **examples/**: This directory includes example scripts demonstrating how to use the functions and classes defined in `matrices.py` and `vectors.py`.

## Getting Started

To get started with this project, clone the repository and explore the example scripts to see how the linear algebra operations are implemented.

```bash
git clone https://github.com/yourusername/linalg.git
cd linalg
```

## Usage

You can import the modules in your own Python scripts to perform various linear algebra operations. For example:

```python
from matrices import Matrix
from vectors import Vector

# Create a matrix
A = Matrix([[1, 2], [3, 4]])

# Create a vector
v = Vector([1, 2])

# Perform operations
result = A.multiply(v)
print(result)
```

To use a function from another module for the `examples` directory, run the script using the following command from the project parent directory:

```bash
python3 -m examples.[filename].py
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

Happy coding!