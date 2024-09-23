from string import ascii_letters, digits
from random import randint
from sys import argv

if len(argv) > 1:
    length = int(argv[1])
else:
    length = 20

chars = ascii_letters #+ digits + "@#$%&"

out = ''
for _ in range(length):
    out += chars[randint(0, len(chars)-1)]

print(out)
