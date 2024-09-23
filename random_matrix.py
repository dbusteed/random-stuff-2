#
#   generates a random covariance matrix
#   given the dimensions
#

from random import random, choice

def random_cov_matrix(n):

    mat = []
    for _ in range(n):
        mat.append( [1 for _ in range(n)] )

    vals = [round(random() * choice((-1, 1)), 2) for _ in range(n)]

    for i in range(n):
        for j in range(n):
            if i != j:
                mat[i][j] = vals[(i+j-1)]

    return mat


print(random_cov_matrix(3))