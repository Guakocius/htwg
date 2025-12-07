import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

karten = pd.read_csv('./ninjagokarten.csv', sep=';')
colors = karten.drop("Name", axis=1)
cards = karten.drop('Name', axis=1).values.transpose()
print(cards)

print('100.2.2')

for i, j in zip(cards, colors):
    am = np.mean(i)
    stab = np.std(i)

    print(f'Mittelwert von {j}: {np.around(am, 3)}')
    print(f"Standardabweichung von {j}: {np.around(stab, 3)}")

print("_________")

print()
print('100.2.3')

colors = ['yellow', 'red', 'blue', 'limegreen']
xAx = ['speed', 'attack', 'power', 'defense']

fig, (ax0,ax1) = plt.subplots(1, 2)

ax0.boxplot(cards.transpose())
ax0.set_xticks(range(1,5),xAx)


print()
print('100.2.4')


for l in range(0,4):
    ax1.scatter(np.full((1,cards[l].size), l), cards[l], color=colors[l], label=colors[l])
ax1.set_xlabel('Eigenschaften')
ax1.set_ylabel('Werte')
ax1.set_xticks(range(0,4), xAx)
# plt.legend()

print()
print('100.2.5')
sns.pairplot(karten)

plt.show()

print()
print('100.2.6')
coef_cards = np.corrcoef(cards, rowvar=True)
print(f'Korrelationskoeffizient:\n{coef_cards}')

r = np.corrcoef(cards, rowvar=True)[0,1]

rsqrt=r**2
print(f'\nBestimmtheitsmaß: {rsqrt}')
