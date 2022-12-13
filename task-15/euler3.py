#!/bin/python3
import sys
import math
def primefactor(n):
    prime=-1
    while n%2==0:
        prime=2
        n>>=1
    for i in range(3,int(math.sqrt(n))+1,2):
        while n%i==0:
            prime=i
            n//=i
    if n>2:
    	return n 
    else:
    	return prime
    
t = int(input().strip())
for _ in range(t):
    a = int(input().strip())
    print(primefactor(a))

