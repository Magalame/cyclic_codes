import numpy as np
import matplotlib.pyplot as plt
from random import uniform

# for i in range(1,17):
#     x, y = np.loadtxt(open("18/i0_"+str(i)), delimiter=',', usecols=(0, 1), unpack=True,skiprows=0)

#     plt.scatter(x,y, color="blue")

# plt.show()

# cr = []
# k = []
# err = []

# n = 31

# for i in range(1,n):
#     try:
#         cr_tmp,k_tmp,err_tmp = np.loadtxt(open(str(n)+"/i0_"+str(i)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
#     except:
#         continue
#     #print(cr_tmp)
#     #print(err_tmp)
#     if k_tmp.size > 1:
#         for i in range(k_tmp.size):
#             cr.append(cr_tmp[i])
#             k.append(k_tmp[i])
#             err.append(err_tmp[i])
#     else:
#         cr.append(cr_tmp)
#         k.append(k_tmp)
#         err.append(err_tmp)

# plt.scatter(cr,np.log10(err),c=k,cmap="rainbow") 
# plt.colorbar() 
# plt.xlabel("Crossing number")   
# plt.ylabel("Error rate (log)")  
# plt.title("n = "+str(n))

# plt.show()

# k, err = np.loadtxt(open("n_21"), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)

# plt.scatter(k,np.log(err)) 

# plt.show()



# n0 = 9
# k0 = 2

# n = []
# k = []
# err = []

# for i in range(1):
#     n_tmp,k_tmp,err_tmp = np.loadtxt(open("fam5_range_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
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

# plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow") 
# plt.colorbar() 
# plt.xlabel("n")   
# plt.ylabel("Error rate (log)")  
# plt.title(" approximate 2/9 family")

# # plt.title("~"+str(k0)+"/"+str(n0)+ " family, weight 3")

# plt.show()

n0 = 12
k0 = 2

n,k,err = np.loadtxt(open("fam5_fcwpf_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow", vmin=0, vmax=.8) 
plt.show()

# n0 = 9
# k0 = 2

# n,k,err = np.loadtxt(open("fam5_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow", vmin=0, vmax=.8) 

# n0 = 17
# k0 = 8

# n,k,err = np.loadtxt(open("fam5_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow", vmin=0, vmax=.8) 


# n0 = 5
# k0 = 4

# n,k,err = np.loadtxt(open("fam5_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow", vmin=0, vmax=.8) 

# n0 = 24
# k0 = 2

# n,k,err = np.loadtxt(open("fam5_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(n,np.log10(err),c=np.divide(k,n),cmap="rainbow", vmin=0, vmax=.8) 


# plt.colorbar() 
# plt.xlabel("n")   
# plt.ylabel("Error rate")  
# plt.title("2/15 family")

# # plt.title("~"+str(k0)+"/"+str(n0)+ " family, weight 3")

# plt.show()

# n0 = 15
# k0 = 2

# n = []
# k = []
# err = []

# for i in range(1):
#     n_tmp,k_tmp,err_tmp, mul_tmp = np.loadtxt(open("fam3_range_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
#     #print(cr_tmp)
#     #print(err_tmp)
#     if k_tmp.size > 1:
#         n = n_tmp
#         k = k_tmp
#         err = err_tmp
#         mul = mul_tmp
#     else:
#         n.append(n_tmp)
#         k.append(k_tmp)
#         err.append(err_tmp)
#         mul.append(mul_tmp)

# plt.scatter(n,np.log10(err),c=mul,cmap="rainbow") 
# plt.colorbar() 
# plt.xlabel("n")   
# plt.ylabel("Error rate (log)")  
# plt.title("~"+str(k0)+"/"+str(n0)+ " family, weight 3")

# plt.show()

# n = 30
# k = 16

# #cr,err = np.loadtxt(open("err_and_cr_n:"+str(n)+"_k:"+str(k)), delimiter=',', usecols=(0,2), unpack=True,skiprows=0)
# cr,ks,err = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(cr,np.log10(err),c=ks,cmap="rainbow") n = 30
# k = 16

# #cr,err = np.loadtxt(open("err_and_cr_n:"+str(n)+"_k:"+str(k)), delimiter=',', usecols=(0,2), unpack=True,skiprows=0)
# cr,ks,err = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.scatter(cr,np.log10(err),c=ks,cmap="rainbow") 
# plt.xlabel("Crossing number")   
# plt.ylabel("Error rate (log)")  
# plt.title("n = "+str(n))

# plt.show()


# n = 30
# k = 16

# #cr,err = np.loadtxt(open("err_and_cr_n:"+str(n)+"_k:"+str(k)), delimiter=',', usecols=(0,2), unpack=True,skiprows=0)
# ks,err = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)
# cr = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(2), unpack=True,skiprows=0,dtype=str)
# plots = []

# for i in range(len(ks)):
#    # print(cr[i])
#     crnb = list(map(float, cr[i][1:-1].split("/")))
#     #print(crnb)

#     plots.append(plt.figure(i))

#     plt.hist(crnb)
#     plt.xlabel("Crossing number")   
#     plt.ylabel("Count")  
#     plt.title("n = "+str(n) + " k =" + str(ks[i]) + " err=" + str(err[i]))

#     plots[i].show()

# input()

# n = 29

# #cr,err = np.loadtxt(open("err_and_cr_n:"+str(n)+"_k:"+str(k)), delimiter=',', usecols=(0,2), unpack=True,skiprows=0)
# ks,err = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)
# cr = np.loadtxt(open("err_and_cr_n:"+str(n)), delimiter=',', usecols=(2), unpack=True,skiprows=0,dtype=str)
# plots = []

# crlow = [] # low error
# errlow = []
# siglow= []
# crhigh = [] # high error
# errhigh = []
# sighigh = []

# for i in range(len(ks)):
#     crnb = list(map(float, cr[i][1:-1].split("/")))

#     if ks[i] == 8:
#         if err[i] < 0.17:
#             crlow = crlow + crnb
#             errlow.append(err[i])
#             siglow.append(np.std(crnb))
#         else:
#             crhigh = crhigh + crnb
#             errhigh.append(err[i])
#             sighigh.append(np.std(crnb))


# i = 0

# plots.append(plt.figure(i))

# plt.hist(crlow, bins = 30, alpha = .5, density = True)
# plt.hist(crhigh, bins = 30, alpha = .5, density = True)
# plt.xlabel("Crossing number")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]) + ", err=" + str(np.mean(errlow)) + "+/-" + str(np.std(errlow)/np.sqrt(len(errlow))))

# plots[i].show()

# i = 1

# plots.append(plt.figure(i))

# plt.hist(errlow, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)
# plt.hist(errhigh, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)

# plt.gca().set_xscale("log")
# plt.xlabel("error")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]))

# plots[i].show()

# i = 2

# plots.append(plt.figure(i))

# plt.hist(siglow, bins = 30, alpha = .5, density = True)
# plt.hist(sighigh, bins = 30, alpha = .5, density = True)
# plt.xlabel("sigmas")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]))

# plots[i].show()


# input()




# n,k = np.loadtxt(open("all_codes_3"), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)


# plt.scatter(n,k,c=np.divide(k,n),cmap="rainbow") 
# plt.colorbar() 
# plt.xlabel("n")   
# plt.ylabel("k")  
# plt.title("All codes of weight 5")

# plt.show()

# target_k = 4
# n = 15

# ks,err,cr = np.loadtxt(open("err_and_cr_n_bicy:"+str(n)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

# plt.hist(err,bins=np.logspace(np.log10(0.001),np.log10(1.0), 50))
# plt.gca().set_xscale("log")

# plt.show()

# input()


# plots = []

# crlow = [] # low error
# errlow = []
# crmid = [] # low error
# errmid = []
# crhigh = [] # high error
# errhigh = []



# for i in range(len(ks)):

#     if ks[i] == target_k:
#         if cr[i] < 50:
#             crlow.append(cr[i])
#             errlow.append(err[i])
#         elif cr[i] > 83:
#             crhigh.append(cr[i])
#             errhigh.append(err[i])
#         else:
#             crmid.append(cr[i])
#             errmid.append(err[i])
            


# i = 0

# plots.append(plt.figure(i))

# plt.hist(crlow, bins = 30, alpha = .5, density = True)
# plt.hist(crmid, bins = 30, alpha = .5, density = True)
# plt.hist(crhigh, bins = 30, alpha = .5, density = True)
# plt.xlabel("Crossing number")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]) + ", err=" + str(np.mean(errlow)) + "+/-" + str(np.std(errlow)/np.sqrt(len(errlow))))

# plots[i].show()

# i = 1

# plots.append(plt.figure(i))

# plt.hist(errlow, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)
# plt.hist(errmid, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)
# plt.hist(errhigh, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)

# plt.gca().set_xscale("log")
# plt.xlabel("error")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]))

# plots[i].show()

# i = 2

# plots.append(plt.figure(i))
# plt.scatter(errlow , crlow )
# plt.scatter(errmid,crmid)
# plt.scatter(errhigh, crhigh)

# plt.xscale("log")

# plots[i].show()

# input()


# target_k = 4

# n = 15

# ks,err,cr = np.loadtxt(open("err_and_cr_n_bicy:"+str(n)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)



# plt.hist(err,bins=np.logspace(np.log10(0.0001),np.log10(1.0), 100))
# plt.gca().set_xscale("log")
# plt.show()
# input()

# s_err = []
# for i in range(len(ks)):

#     if ks[i] == target_k:
#         s_err.append(err[i])

# plt.hist(s_err,bins=np.logspace(np.log10(0.0001),np.log10(1.0), 50))
# plt.gca().set_xscale("log")
# plt.show()
# input()


# plots = []

# crlow = [] # low error
# errlow = []
# crmid = [] # low error
# errmid = []
# crhigh = [] # high error
# errhigh = []



# for i in range(len(ks)):

#     if ks[i] == target_k:
#         if err[i] < 6.8e-2:
#             crlow.append(cr[i])
#             errlow.append(err[i])
#         else:
#             crhigh.append(cr[i])
#             errhigh.append(err[i])


# i = 0

# plots.append(plt.figure(i))

# plt.hist(crlow, bins = 30, alpha = .5, density = True)
# plt.hist(crhigh, bins = 30, alpha = .5, density = True)
# plt.xlabel("Crossing number")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]) + ", err=" + str(np.mean(errlow)) + "+/-" + str(np.std(errlow)/np.sqrt(len(errlow))))

# plots[i].show()

# i = 1

# plots.append(plt.figure(i))

# plt.hist(errlow, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)
# plt.hist(errhigh, bins=np.logspace(np.log10(0.001),np.log10(1.0), 50), alpha = .5)

# plt.gca().set_xscale("log")
# plt.xlabel("error")   
# plt.ylabel("Count")  
# plt.title("n = "+str(n) + ", k =" + str(ks[i]))

# plots[i].show()

# i = 2

# plots.append(plt.figure(i))
# plt.scatter(errlow , crlow )
# plt.scatter(errhigh, crhigh)

# plt.xscale("log")

# plots[i].show()

# input()

# ratios = [
#     (9,2),
#     (10,4),
#     (12,2),
#     (14,3),
#     (15,2),
#     (15,4),
#     (15,6),
#     (18,2),
#     (20,4),
#     (21,5)
# ]

# n_all = []
# k_all = []
# err_all = []
# cr_all = []

# for (n0,k0) in ratios:
#     n,k,err,cr = np.loadtxt(open("fam5_cr_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
#     n_all = np.concatenate((n_all,n))
#     k_all = np.concatenate((k_all,k))
#     err_all = np.concatenate((err_all,err))
#     cr_all = np.concatenate((cr_all,cr))

# n = n_all
# k = k_all
# err = err_all
# cr = cr_all
# plt1 = plt.figure(1)
# #n2,k2,err2,cr2 = np.loadtxt(open("fam5_range_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
# plt.scatter(n,np.log10(err),c=cr,marker="s",cmap="rainbow", alpha = .5) 
# #plt.scatter(n2,np.log10(err2),c=cr2,cmap="rainbow", alpha = .5) 
# plt.title(str(k0) +"/" + str(n0)+" family")
# plt.xlabel("n")   
# plt.ylabel("Error (log10)") 
# plt.colorbar() 
# plt1.show()
# input()




# n0 = 12
# k0 = 2

# n,k,err,cr = np.loadtxt(open("fam5_cr_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)

# plt1 = plt.figure(1)
# #n2,k2,err2,cr2 = np.loadtxt(open("fam5_range_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
# plt.scatter(n,np.log10(err),c=cr,marker="s",cmap="rainbow", alpha = .5) 
# #plt.scatter(n2,np.log10(err2),c=cr2,cmap="rainbow", alpha = .5) 
# plt.title(str(k0) +"/" + str(n0)+" family")
# plt.xlabel("n")   
# plt.ylabel("Error (log10)") 
# plt.colorbar() 
# plt1.show()
# input()


# n0 = 21
# k0 = 3

# # tn = 63

# def cr_vs_err(n0,k0,tn):

#     n_tmp,k_tmp,err_tmp,cr_tmp = np.loadtxt(open("fam5_cr_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)

#     n= []
#     k = []
#     err= []
#     cr = []

#     for i in range(len(n_tmp)):
#       if n_tmp[i] == tn:
#           n.append(n_tmp[i])
#           k.append(k_tmp[i])
#           err.append(err_tmp[i])
#           cr.append(cr_tmp[i]) 

#     plt.scatter(cr,np.log10(err))
    

# cr_vs_err(n0,k0,42)
# cr_vs_err(n0,k0,63)
# cr_vs_err(n0,k0,84)
# plt.title(str(k0) +"/" + str(n0)+" family")
# plt.xlabel("cr number")   
# plt.ylabel("Error (log10)") 
# plt.show()

# n0 = 35
# r0 = 27
# w0 = 5
# # tn = 63

# def cr_vs_err(n0,r0,w0,tk):

#     k_tmp,err_tmp,cr_tmp = np.loadtxt(open("err_and_cccr_n:"+str(n0)+"_r:"+str(r0)+"_w:"+str(w0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)

#     n= []
#     k = []
#     err= []
#     cr = []

#     for i in range(len(k_tmp)):
#       if k_tmp[i] == tk:
#           k.append(k_tmp[i])
#           err.append(err_tmp[i])
#           cr.append(cr_tmp[i]) 

#     plt.scatter(cr,np.log10(err))
    

# cr_vs_err(n0,r0,w0,8)
# plt.title("n:" + str(n0) +", r:" + str(r0)+", w:"+str(w0)+" family")
# plt.xlabel("cr number")   
# plt.ylabel("Error (log10)") 
# plt.show()

# n0 = 9
# k0 = 2

# n,k,err,mul = np.loadtxt(open("fam5_mul_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
# plt.scatter(n,np.log10(err),c=np.log(mul),cmap="rainbow") 
# plt.show()



# n0 = 9
# k0 = 2

# n,k,err,cr = np.loadtxt(open("fam5_cr_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)
# n2,k2,err2,cr2 = np.loadtxt(open("fam5_range_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2,3), unpack=True,skiprows=0)

# plt1 = plt.figure(1)
# plt.scatter(n,np.log10(err),c=cr,marker="s",cmap="rainbow", alpha = .5) 
# plt.scatter(n2,np.log10(err2),c=cr2,cmap="rainbow", alpha = .5) 
# plt.title(str(k0) +"/" + str(n0)+" family")
# plt.xlabel("n")   
# plt.ylabel("Error (log10)") 
# plt.colorbar() 
# plt1.show()
# input()


# n,k= np.loadtxt(open("all_first_codes_5"), delimiter=',', usecols=(0,1), unpack=True,skiprows=0)
# x = np.linspace(5,np.max(n),100)
# y= x/2
# plt.scatter(n,k,c=np.divide(k,n),cmap="rainbow")
# plt.show()

#char vs err
# n0 = 12
# k0 = 2
# hashes = {}
# colors = []

# n,k,err = np.loadtxt(open("fam5_fc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
# chars = np.loadtxt(open("fam5_fc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(3), unpack=True,skiprows=0,dtype=str)

# for char in chars:
#     if char not in hashes:
#         hashes[char] = uniform(0,10)

#     colors.append(hashes[char])

# plt.scatter(n,np.log10(err),c=colors,cmap="rainbow") 
# plt.show()

#char vs err, each n has different hashes
# n0 = 12
# k0 = 2
# hashes = {}
# counts = {}
# colors = []

# n,k,err = np.loadtxt(open("fam5_fc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
# chars = np.loadtxt(open("fam5_fc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(3), unpack=True,skiprows=0,dtype=str)

# for i in range(len(chars)):
#     if n[i] not in hashes:
#         hashes[n[i]] = {}
#         counts[n[i]] = 1.

#     if chars[i] not in hashes[n[i]]:
#         hashes[n[i]][chars[i]] = counts[n[i]]
#         counts[n[i]] += 1.

# for n_ in hashes.keys():
#     m = max(hashes[n_].values())
#     for char in hashes[n_].keys():
#         hashes[n_][char] = uniform(0,m)

# for i in range(len(chars)):
#     #print(n[i])
#     colors.append(hashes[n[i]][chars[i]])

# plt.scatter(n,np.log10(err),c=colors,cmap="rainbow") 
# plt.show()


#maps char vs cr
# n0 = 12
# k0 = 2
# hashes = {}
# colors = []

# n,k,cr = np.loadtxt(open("fam5_cvc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(0,1,2), unpack=True,skiprows=0)
# chars = np.loadtxt(open("fam5_cvc_n0"+str(n0)+"_k0"+str(k0)), delimiter=',', usecols=(3), unpack=True,skiprows=0,dtype=str)

# for char in chars:
#     if char not in hashes:
#         hashes[char] = uniform(0,10)

#     colors.append(hashes[char])

# rand_col = {}
# new_colors = []

# for color in hashes.values():
#     rand_col[color] = uniform(0,len(hashes.values()))

# for color in colors:
#     new_colors.append(rand_col[color])

# plt.scatter(colors, np.log(cr), c=new_colors,cmap="rainbow") 
# plt.show()

