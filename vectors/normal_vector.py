import numpy as np

# (a) Normal vector n to the plane A: x2 - x3 = 0
def normal_vector():
    return np.array([0, 1, -1])

# (b) Check if the origin (0, 0, 0) is in the plane A
def is_origin_in_plane():
    x = np.array([0, 0, 0])
    return x[1] - x[2] == 0

# (c) Check if the point (1, 2, 3) is in the plane A
def is_point_in_plane(point):
    return point[1] - point[2] == 0

# (d) Parametric representation of the plane A
def parametric_representation():
    # x1 is free, let x1 = t
    # x2 = x3, let x2 = s and x3 = s
    t = np.linspace(-10, 10, 100)
    s = np.linspace(-10, 10, 100)
    x1, x2, x3 = np.meshgrid(t, s, s)
    return x1, x2, x3

# (e) Consideration about the coordinate representation of a line in IR2 or IR3
def line_representation():
    # In IR2, a line can be represented as ax + by = c
    # In IR3, a line can be represented parametrically as:
    # x = x0 + t * d1
    # y = y0 + t * d2
    # z = z0 + t * d3
    pass

if __name__ == "__main__":
    print("Normal vector to the plane A:", normal_vector())
    print("Is the origin in the plane A?", is_origin_in_plane())
    print("Is the point (1, 2, 3) in the plane A?", is_point_in_plane([1, 2, 3]))
    x1, x2, x3 = parametric_representation()
    print("Parametric representation of the plane A:")
    print("x1:", x1)
    print("x2:", x2)
    print("x3:", x3)