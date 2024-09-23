import numpy as np
import random

class Farmer:
    def __init__(self, name, let_skill, tom_skill):
        self.name = name
        self.let_skill = let_skill
        self.tom_skill = tom_skill
        self.let_value = 1 / let_skill
        self.tom_value = 1 / tom_skill
        self.let_inventory = 0
        self.tom_inventory = 0
        self.salads = 0
        self.trading = False
        self.credits = 100

    def grow_crops(self):
        adj = 0
        r = random.gauss(0, 1)
        if r > 1.0:
            adj = 1
        
        if random.random() < 0.5:
            self.let_inventory += self.let_skill + adj
        else:
            self.tom_inventory += self.tom_skill + adj

    def trade(self, other_farmer):
        if self.let_inventory > 0 and other_farmer.tom_inventory > 0:
            self.let_inventory -= 1
            other_farmer.tom_inventory -= 1
            self.tom_inventory += 1
            other_farmer.let_inventory += 1
            print(f"{self.name} and {other_farmer.name} traded crops.")

    def make_salad(self):
        if self.let_inventory > 0 and self.tom_inventory > 0:
            self.let_inventory -= 1
            self.tom_inventory -= 1
            self.salads += 1

def simulate_day(farmers):
    for farmer in farmers:
        farmer.grow_crops()

    for farmer in farmers:
        farmer.trading = True

    while any([f.trading for f in farmers]):
        for f in farmers:
            others = [o for o in farmers if o != f]
            made_trade = False
            for o in others:
                if o.let_inventory > 2 and f.let_value > o.let_value:
                    price = np.mean([f.let_value, o.let_value])
                    o.credits += price
                    o.let_inventory -= 1
                    f.credits -= price
                    f.let_inventory += 1
                    print(f'{f.name} bought 1 LET for {price} from {o.name}')
                    made_trade = True

                elif o.tom_inventory > 2 and f.tom_value > o.tom_value:
                    price = np.mean([f.tom_value, o.tom_value])
                    o.credits += price
                    o.tom_inventory -= 1
                    f.credits -= price
                    f.let_inventory += 1
                    print(f'{f.name} bought 1 TOM for {price} from {o.name}')
                    made_trade = True
                    
            if not made_trade:
                f.trading = False

    # for i in range(len(farmers)):
    #     for j in range(i + 1, len(farmers)):
    #         farmers[i].trade(farmers[j])

    for farmer in farmers:
        farmer.make_salad()

if __name__ == "__main__":

    farmers = [
        Farmer('Alice', 2, 1),
        Farmer('Bobby', 3, 3),
        # Farmer('Chuck', 1, 1),
        # Farmer('David', 1, 1),
    ]

    for day in range(50):
        print(f"\nDay {day + 1}:")
        simulate_day(farmers)

    print("\nFinal Inventories:")
    for farmer in farmers:
        print(f"{farmer.name}: let = {farmer.let_inventory}, tom = {farmer.tom_inventory}, salads: {farmer.salads}, credits {farmer.credits}")
