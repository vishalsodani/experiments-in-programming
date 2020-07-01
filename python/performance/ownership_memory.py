#from https://pythonspeed.com/articles/function-calls-prevent-garbage-collection/

import numpy as np

class Owner(object):
    def __init__(self, data):
        self.data = data

def load_1GB_of_data():
    return np.ones((2 ** 30), dtype=np.uint8)

def process_data():
    data = Owner(load_1GB_of_data())
    return modify2(modify1(data))

def modify1(owned_data):
    data = owned_data.data
    owned_data.data = None
    return data * 2

def modify2(data):
    return data + 10

process_data()

