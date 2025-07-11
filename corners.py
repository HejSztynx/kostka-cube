import numpy as np
import matplotlib.pyplot as plt

half = 1.0 / 2

def gen_corners(x, y, z, rotation) :
    y_p = y + half
    # y_pp = y - half

    R = half * np.sqrt(2)
    sin_r = np.sin(rotation) * R
    cos_r = np.cos(rotation) * R

    change = [(sin_r, cos_r), (cos_r, -sin_r)]
    results = []

    for ch in change :
        for d in [1, -1] :
            results.append((
                x + d * ch[0],
                y_p,
                z + d * ch[1]
            ))

    return results

def plot_corners(rotation) :
    x, y, z = 2, 2, 2
    corners = gen_corners(x, y, z, rotation)

    x_vals = [corner[0] for corner in corners]
    z_vals = [corner[2] for corner in corners]

    plt.figure(figsize=(6, 6))
    plt.scatter(x_vals, z_vals, color='blue', label='Corners')
    plt.title(f"{rotation / np.pi * 180} degrees")
    plt.xlabel('X')
    plt.ylabel('Z')
    plt.grid(True)
    plt.axis('equal')
    plt.legend()
    plt.show()

for rotation in np.linspace(0, np.pi, 10) :
    plot_corners(rotation)
