#
#   programming challenge question. function
#   takes a string of some math/programming 
#   expression and indicates whether it's
#   a valid use of paranthesis
#

def paran_check(s):
    count = 0
    for char in s:
        if char == '(':
            count += 1
        elif char == ')':
            count -= 1

        if count < 0:
            return False

    if count == 0:
        return True
    else:
        return False
        
s = input('enter a string to check: ')
print(paran_check(s))