import numpy as np
import matplotlib.pyplot as plt

# for i in range(1,17):
#     x, y = np.loadtxt(open("18/i0_"+str(i)), delimiter=',', usecols=(0, 1), unpack=True,skiprows=0)

#     plt.scatter(x,y, color="blue")

# plt.show()

ns = []
ks = []
errs = []


for n in range(9,99):
    try:
        n_tmp,k_tmp,err_tmp = np.loadtxt(open("landscape_from_classical_fixed_n0:" + str(n) + "_p:0.2_r:0.1"), delimiter=',', usecols=(0,1,4), unpack=True,skiprows=0)
    except:
        continue
    #print(cr_tmp)
    #print(err_tmp)
    if k_tmp.size > 1:
        for i in range(k_tmp.size):
            if k_tmp[i]/n_tmp[i] < 0.3:
                ns.append(n_tmp[i])
                ks.append(k_tmp[i])
                errs.append(err_tmp[i])
    else:
        if k_tmp / n_tmp < 0.3:
            ns.append(n_tmp)
            ks.append(k_tmp)
            errs.append(err_tmp)

plt.scatter(ns,np.log10(errs),c=np.divide(ks,ns),cmap="rainbow") 
plt.colorbar() 
plt.xlabel("n")   
plt.ylabel("Error rate (log10)")  

plt.show()

# k, err = np.loadtxt(open("n_21"), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)

# plt.scatter(k,np.log(err)) 

# plt.show()

# n = []
# k = []
# err = []

# for i in range(1):
#     n_tmp,k_tmp,err_tmp = np.loadtxt(open("fam5_n09_k02"), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
#     #print(cr_tmp)
#     #print(err_tmp)
#     if k_tmp.size > 1:
#         for i in range(k_tmp.size):
#             n.append(n_tmp[i])
#             k.append(k_tmp[i])
#             err.append(err_tmp[i])
#     else:
#         n.append(n_tmp)
#         k.append(k_tmp)
#         err.append(err_tmp)

# plt.scatter(n,np.log10(err),c=k,cmap="rainbow") 
# plt.colorbar() 
# plt.xlabel("n")   
# plt.ylabel("Error rate (log)")  
# plt.title("2/9 family")

# plt.show()