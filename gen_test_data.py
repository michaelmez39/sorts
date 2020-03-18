# Makes these files 10000, 20000, 50000 and 100000
# In random, ascending, descending
import random
def make_random_range(elements):
    return [random.randint(0, (1 << 32) - 1) for i in range(0, int(elements))]

def make_ordered_range(elements):
    return [i for i in range(0, int(elements))]

def make_rev_range(elements):
    a = [i for i in range(0, int(elements))]
    list.reverse(a)
    return a

def list_to_file(name, arr):
    with open(name, 'w') as f:
        for item in arr:
            f.write("%s\n" % item)

for i in [10e3, 20e3, 50e3, 100e3]:
    name = "test_files/rand{}.txt".format(i)
    list_to_file(name, make_random_range(i))
    name = "test_files/ord{}.txt".format(i)
    list_to_file(name, make_ordered_range(i))
    name = "test_files/rev{}.txt".format(i)
    list_to_file(name, make_rev_range(i))
