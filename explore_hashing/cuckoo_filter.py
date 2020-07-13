input = [3, 7, 14, 15, 19]

bucket = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 

def hash_1(x):
    return x  % 11 

def hash_2(x):
    return (x * 2) % 6 


def cuckoo_algo():

    while len(input) > 0:
        print(input)
        print(bucket)
        for element in input:
            if bucket[hash_1(element)] == 0:
                #place element in bucket
                bucket[hash_1(element)] = element
                input.remove(element)
            else:
                if bucket[hash_2(element)] == 0:
                    bucket[hash_2(element)] = element
                    input.remove(element)
                else:
                    y = bucket[hash_2(element)]
                    input.insert(0, y)
                    bucket[hash_2(element)] = element

cuckoo_algo()                
print(bucket)
