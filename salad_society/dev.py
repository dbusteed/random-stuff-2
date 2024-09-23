import re

SALADS = {
    'CLST': '.*C+.*L+.*S+.*T+.*',
    'LST': '.*L+.*S+.*T+.*',
    'LT': '.*L+.*T+.*',
    'L': '.*L+.*',
}

class Farmer:
    def __init__(self):
        self.costs = {
            'C': 4,
            'L': 1,
            'S': 2,
            'T': 2,
        }
        self.bag = {
            'C': 0,
            'L': 0,
            'S': 0,
            'T': 0,
        }
        self.values = {
            'C': 0,
            'L': 0,
            'S': 0,
            'T': 0,
        }
        self.happiness = 0
        self.credits = 100

    def eat_salad(self):
        bag = ''.join([k*self.bag[k] for k in sorted(list(self.bag.keys()))])
        if re.match(SALADS['CLST'], bag):
            self.bag['C'] -= 1
            self.bag['L'] -= 1
            self.bag['S'] -= 1
            self.bag['T'] -= 1
            self.happiness += 15
        elif re.match(SALADS['LST'], bag):
            self.bag['L'] -= 1
            self.bag['S'] -= 1
            self.bag['T'] -= 1
            self.happiness += 10
        elif re.match(SALADS['LT'], bag):
            self.bag['L'] -= 1
            self.bag['T'] -= 1
            self.happiness += 5
        elif re.match(SALADS['L'], bag):
            self.bag['L'] -= 1
            self.happiness += 1
        else:
            self.happiness -= 2

    def evaluate(self, tmp_bag):
        # print(tmp_bag)
        bag = ''.join([k*tmp_bag[k] for k in sorted(list(tmp_bag.keys()))])
        if re.match(SALADS['CLST'], bag):
            # print(15)
            return 15
        elif re.match(SALADS['LST'], bag):
            # print(10)
            return 10
        elif re.match(SALADS['LT'], bag):
            # print(5)
            return 5
        elif re.match(SALADS['L'], bag):
            # print(1)
            return 1
        else:
            # print(0)
            return 0

    def calc_value(self):
        for ing in self.values.keys():
            tmp_bag = self.bag.copy()
            tmp_bag[ing] += 1
            self.values[ing] = self.evaluate(tmp_bag)

    def grow(self, crop):
        self.bag[crop] += 4


f = Farmer()

for _ in range(2):
    f.eat_salad()
    f.calc_value()
    c = max(f.values, key=f.values.get)
    f.grow(c)
