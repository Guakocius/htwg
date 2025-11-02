import pandas as pd

def main():
    df = pd.read_csv("./computer_prices_all.csv")
    print(df)

    df.hist("device_type")


if __name__ == "__main__":
    main()
