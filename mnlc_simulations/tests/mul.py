def mul(f,g):
    for i in f:
        row = []
        for j in g:
            row.append(i+j)
        print(row)

mul([0,5,6,7,9,10,11],[0,1,3,5,6])