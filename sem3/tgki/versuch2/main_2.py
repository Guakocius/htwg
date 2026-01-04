# -*- coding: utf-8 -*-

import numpy as np
import cv2
import matplotlib.pyplot as plt
from skimage.color import rgb2gray


cap = cv2.VideoCapture(0)

def main():
    for i in range(1, 11):
        
        ret, frame = cap.read()
        grayscale = rgb2gray(frame)
        fig, ax = plt.subplots(figsize=(8, 4))
        ax.imshow(grayscale, cmap=plt.cm.gray)
        # cv2.imshow(grayscale, cmap=plt.cm.gray)
        ax.set_title("Grayscale")
        plt.show()
        cv2.imwrite(f"./versuch2-2_{i}.png", frame)
        
    print("done")


if __name__ == "__main__":
    main()