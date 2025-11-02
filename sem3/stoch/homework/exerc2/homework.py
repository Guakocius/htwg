import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


def main():
    # Aufgabe 1.6
    # Teilaufgabe 1.6.1
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

    # Teilaufgabe 1.6.2
    # a) Siehe .tex file
    # b)

    # Aufgabe 1.7
    # Teilaufgabe 1.7.1
    X = np.arange(1, 7)
    Ay = np.array([-1, 0, 0, 0.5, 1, 1.5])
    By = np.array([0.5, 0, 0, -1, -1, -1.5])
    Cy = np.array([1, -0.5, -0.25, 0.35, -2.1, -1.6])

    Ac = np.corrcoef(X, Ay)
    Bc = np.corrcoef(X, By)
    Cc = np.corrcoef(X, Cy)
    print("Korrelationskoeffizient von:\nA: ", Ac, "\nB: ", Bc, "\nC: ", Cc)

    # Aufgabe 1.10
    # Teilaufgabe 1.10.2
    arr = np.array([4.2, 3.9, 4.3, 4.1, 4.1, 3.7, 4.3])
    span = arr.max() - arr.min()
    mean = np.mean(arr)
    quantile_1 = np.quantile(arr, 0.25)
    quantile_2 = np.quantile(arr, 0.5)
    quantile_3 = np.quantile(arr, 0.75)
    ten_quantile = np.quantile(arr, 0.1)
    std = np.std(arr, ddof=1)

    print(f"Arithmetisches Mittel: {mean}\n25%: {quantile_1}\t50%: {quantile_2}\t75%: {quantile_3}\n10%: {ten_quantile}\nSpannweite: {span}\nEmpirische Standardabweichung: {std}")

    # Aufgabe 1.12
    # a)
    duration_training = [9, 13, 15, 18, 20]
    salary = [18, 37, 61, 125, 59]
    coef = np.corrcoef(duration_training, salary)
    print(f"Empirischer Korrelationskoeffizient: {coef}")
    # b)
    plt.scatter(duration_training, salary)
    plt.title("Lineare Regression")
    plt.xlabel("Ausbildungsdauer")
    plt.ylabel("Jahresgehalt")
    plt.show()



if __name__ == "__main__":
    main()
