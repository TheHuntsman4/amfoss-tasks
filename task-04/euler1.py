n=int(input())
i=0
s=0
for i in range(n):
    if ((i%3==0)or(i%5==0)):
        s+=i
        i+=1
print(s)
