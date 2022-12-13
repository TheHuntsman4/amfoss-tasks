
import sys


t = int(input().strip())
for a0 in range(t):
    n = int(input().strip())    
   
    n1,n2=1,2
    eve=[]
    nth=0
    while nth<n:
        nth = n1 + n2
        if(n2%2==0):
            eve.append(n2)
        n1 = n2
        n2 = nth
    s=0
    for z in range(len(eve)):
        s=s+(eve[z])
    print(s)

