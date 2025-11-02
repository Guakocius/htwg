import pandas as pd
import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt


def main():

    x = np.linspace(0, 140)

    # Aufgabe 100.2
    # Teilaufgaben 100.2.1 und 100.2.2
    N = pd.read_csv("./ninjagokarten.csv", delimiter=";")
    print(N)
    colors = ["gelb", "rot", "blau", "gruen"]
    c = ["#FFFF00", "#FF0000", "#0000FF", "#008000"]

    for i in colors:
        mean = np.mean(N[i])
        std = np.std(N[i], ddof=1)
        print(f"Mittelwert von {i}: {round(mean, 3)}")
        print(f"Standardabweichung von {i}: {round(std, 3)}")

    fig, ax = plt.subplots()
    # TODO: Strip/Scatter Plot von Aufg. 100.2.3
    #ax.plot(, x, c=c[0])

    # Teilaufgabe 100.2.4

    plt.show()


if __name__ == "__main__":
    main()
