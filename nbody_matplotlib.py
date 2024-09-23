import numpy as np
import matplotlib.pyplot as plt

G = 1
MIN_R = 1

class Body:
    def __init__(self, m, p, v, a=np.array([0., 0.])):
        self.m = m
        self.p = p
        self.v = v
        self.a = a

    def accelerate(self, bodies):
        bodies = [b for b in bodies if b != self]
        acc = []
        for body in bodies:
            r2 = (body.p[0] - self.p[0])**2 + (body.p[1] - self.p[1])**2
            if r2 > MIN_R:
                F = G * ((body.m * self.m) / r2)
                f = ((body.p - self.p) / np.sqrt(r2)) * F
                acc.append(f / self.m)
            else:
                acc.append(np.array([0., 0.]))

        self.a = sum(acc)
        self.v += self.a

    def debug(self):
        return f'Mass: {self.m}, Pos: {self.p}, Vel: {self.v}'

bodies = [
    Body(20, np.array([0.0, 0.0]), np.array([0.0, 0.0])),
    Body(1, np.array([0.0, 10.0]), np.array([-1.0, -1.0])),
    Body(1, np.array([0.0, -10.0]), np.array([1.0, 1.0])),
    Body(1, np.array([10.0, 0.0]), np.array([-1.0, 1.0])),
    Body(1, np.array([-10.0, 0.0]), np.array([1.0, -1.0])),
]

pos = {id(b):np.array(b.p) for b in bodies}

for t in range(40):
    # print(f'T={t}')
    for body in bodies:
        body.accelerate(bodies)
        # print(f' {body.debug()}')
        pos[id(body)] = np.vstack((pos[id(body)], body.p))

    for body in bodies:
        body.p += body.v

colors = ['purple', 'blue', 'red', 'green', 'darkorange']

for v in pos.values():
    c = colors.pop()
    # plt.scatter(v[:, 0], v[:, 1], marker='o', color=c)
    plt.scatter(v[-1, 0], v[-1, 1], marker='*', s=[200], color=c)
    plt.plot(v[:, 0], v[:, 1], color=c)

plt.show()
