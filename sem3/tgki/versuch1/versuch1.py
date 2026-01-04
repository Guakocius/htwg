import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def main():
    # Vorüberlegung: Bei 10cm starten, bis 70, Abstände ohne Fehlerrechnung (ohne Blatt)
    means = [1.426, 1.243, 1.107, 1.017, 0.9199, 0.8561, 0.8003, 0.7459, 0.7058, 0.6653, 0.6287,
             0.6107, 0.5698, 0.5531, 0.5268, 0.4945, 0.4732, 0.4744, 0.4567, 0.4353, 0.4189] # in V

    for i in range(10, 71, 3):
        df = pd.read_csv(f"./Versuch1-{i}.csv", sep=";", decimal=",", skiprows=1)
        mean = np.mean(df["(V)"])
        print(f"Mean for {i}cm: {mean}")
        std = np.std(df["(V)"], ddof=1)
        print(f"Std for {i}cm: {std}\n")
        #print(f"Data of {i}cm: {df}")

        #mean = np.mean(df["Kanal A"])

        #print(f"Mittelwert: {mean}")


    # Mittelwert und Standardabweichung


    # Vorüberlegung: DINA4-Blatt (mit Fehlerrechnung); Länge x Breite
    length_width = [[0.7627, 0.9578]]



if __name__ == "__main__":
    main()