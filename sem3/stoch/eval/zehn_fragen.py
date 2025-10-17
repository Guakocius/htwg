import pandas as pd
import matplotlib.pyplot as plt


def main():

    frame = pd.read_csv("./dat/zehn_Fragen_ws25_ain_clean_2.csv")
    print("Frame:", frame)
    print("Info:", frame.info())

    frame["Q02_Schuhgröße"].plot(kind="hist", bins=10,      edgecolor="black", title="Verteilung der Schuhgrößen")
    plt.xlabel("Schuhgröße")
    plt.ylabel("Anzahl")
    plt.show()


if __name__ == "__main__":
    main()
