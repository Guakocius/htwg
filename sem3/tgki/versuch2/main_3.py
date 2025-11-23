# -*- coding: utf-8 -*-

import numpy as np
import cv2
import matplotlib.pyplot as plt
from skimage.color import rgb2gray

cap = cv2.VideoCapture(0)

def main():
    for i in range(1, 11):
        
        ret, frame = cap.read()
        hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
        h, s, v = cv2.split(hsv)
        lim = 255 - 40
        v[v > lim] = 255
        v[v <= lim] += 40
        final_hsv = cv2.merge((h, s, v))
        img = cv2.cvtColor(final_hsv, cv2.COLOR_HSV2BGR)
        grayscale = rgb2gray(img)
        fig, ax = plt.subplots(figsize=(8, 4))
        #grayscale = increase_brightness(grayscale, 40)
        ax.imshow(grayscale, cmap=plt.cm.gray)
        # cv2.imshow(grayscale, cmap=plt.cm.gray)
        ax.set_title("Grayscale")
        plt.show()
        cv2.imwrite(f"./versuch2-3_{i}.png", frame)
        
    print("done")


def increase_brightness(img, value=30):
    hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
    h, s, v = cv2.split(hsv)

    lim = 255 - value
    v[v > lim] = 255
    v[v <= lim] += value

    final_hsv = cv2.merge((h, s, v))
    img = cv2.cvtColor(final_hsv, cv2.COLOR_HSV2BGR)
    return img

if __name__ == "__main__":
    main()