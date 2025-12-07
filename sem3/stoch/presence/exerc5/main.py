import numpy as np
from numpy.typing import NDArray
import matplotlib.pyplot as plt
def E(x: NDArray, P_x: NDArray):
    X = 0
    for i in range(len(x)):
        X += (x[i] * P_x[i])
    return X

def main():
    # a)
    # M ~ Bin(10, 0.45)
    # W ~ geom(0.45)
    M_X =  np.array(list(range(1, 11)))
    #M_P = np.array(10)
    #for i in range(10):
    #    M_P[i] = 0.45

    #M_E = E(M_X, M_P)

    #print("M_E: ",M_E)

    q_3 = 0

    # b)

    # 5.2
    B_x = np.array(list(range(1,6)))
    P_B_x = np.array([0.1,0.1,0.2,0.2,0.4])
    cdf = np.cumsum(P_B_x)
    plt.step(B_x, cdf)
    plt.show()


    E_B = E(B_x, P_B_x)
    print(E_B)





if __name__ == "__main__":
    main()
