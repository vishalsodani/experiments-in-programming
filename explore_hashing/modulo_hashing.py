# a hash table has buckets
# a hash function generates a value for an item and puts the item in a bucket
# we are creating 10 buckets and using modulo operator and assigning item base don remainder
# so 10 % 10 is 0 so belongs to bucket 0
# we have 10 buckets 0 1 2 3 4 5 6 7 8 9

def basic_hash_table(value_1, n_buckets):
    def hash_function(value_1, n_buckets):
        return int(value_1) % n_buckets

    hash_table = {i:[] for i in range(n_buckets)} # this is a bucket dictionary with each item being an empty list [] to which we will
    #add items

    print(hash_table)
    # we get {0: [], 1: [], 2: [], 3: [], 4: [], 5: [], 6: [], 7: [], 8: [], 9: []}

    for value in value_1:
        hash_value = hash_function(value, n_buckets)
        hash_table[hash_value].append(value)

    return hash_table


result = basic_hash_table([2, 10, 3,100,17],10)
#{0: [10, 100], 1: [], 2: [2], 3: [3], 4: [], 5: [], 6: [], 7: [17], 8: [], 9: []}
