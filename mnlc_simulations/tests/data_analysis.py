import numpy as np
import matplotlib.pyplot as plt

x, y = np.loadtxt(open("hey.csv"), delimiter=',', usecols=(0, 2), unpack=True,skiprows=1)

plt.scatter(x,y)

k4 = []
k8 = []

for (i,k) in enumerate(x):
    if k == 4:
        k4.append(y[i])
    elif k == 8:
        k8.append(y[i])

print("k4:",np.mean(k4), np.std(k4), np.std(k4)/np.sqrt(len(k4)))
print("k8:",np.mean(k8), np.std(k8), np.std(k8)/np.sqrt(len(k8)))

plt.show()
