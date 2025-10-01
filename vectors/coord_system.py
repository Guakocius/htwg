"""
This module provides functionality to display vectors in a
coordinate system using matplotlib and numpy.

Functions:
    plot_vector(vector, origin=[0, 0], **kwargs): Plots a single vector in a 2D coordinate system.
    plot_vectors(vectors, origins=None, **kwargs): Plots multiple vectors in a 2D coordinate system.
    set_axes_equal(ax): Sets the aspect ratio of the plot to be equal.

Usage example:
    import matplotlib.pyplot as plt

    vectors = np.array([[2, 3], [3, -2], [-1, 4]])
    origins = np.array([[0, 0], [1, 1], [2, 2]])

    fig, ax = plt.subplots()
    plot_vectors(vectors, origins=origins, ax=ax)
    set_axes_equal(ax)
    plt.show()
"""
import numpy as np
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

vectors = {
    'a': np.array([2, 3]),
    'b': np.array([3, -2]),
    'c': np.array([-1, 4])
}

TITLESTRING = 'Plotting Vectors in a 2D Coordinate System'

for name, vector in vectors.items():
    ax.quiver(0, 0, vector[0], vector[1], angles='xy',
              scale_units='xy', scale=1, color='blue', label=name)
    plt.text(vector[0], vector[1], f'{name}: ({vector[0]}, {vector[1]})', fontsize=12, color='red')

    ax.set_xlim(-5, 5)
    ax.set_ylim(-5, 5)
    ax.axhline(0, color='black',linewidth=0.5)
    ax.axvline(0, color='black',linewidth=0.5)
    ax.grid(True, which='both')

    plt.title(TITLESTRING)
    plt.xlabel('x')
    plt.ylabel('y')


plt.show() # Output: A plot of the vectors in a 2D coordinate system.
