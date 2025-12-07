import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

def main():
    # Aufgabe 1.6.1
    max_speeds = [270, 370, 200, 300, 270, 300, 250, 330, 200, 400]
    # a)
    mean = np.mean(max_speeds)
    print("Arithmetisches Mittel:", mean)
    # b)
    median = np.median(max_speeds)
    print("Median:", median)
    # c)
    ninety_quantile = np.quantile(q=0.9, a=max_speeds)
    print("90% Quantil:", ninety_quantile)
    # d)
    std = np.std(max_speeds, ddof=1)
    print("Standardabweichung:", std)

    # Aufgabe 1.6.2



if __name__ == "__main__":
    main()
