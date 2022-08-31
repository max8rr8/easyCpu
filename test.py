# tf = [True, False]

# def neg(a):
#   return ''.join(['0' if x=='1' else '1' for x in a])


# def xor(a, b):
#   return ''.join(['0' if (x=='1' and y =='1') else '1' for x,y in zip(a, b)])

# for nx in tf:
#   for ny in tf:
#     for no in tf:
#       a = '1010'
#       b = '1100'
#       if nx: a = neg(a)
#       if ny: b = neg(b)
#       res = xor(a, b)
#       if no: res = neg(res)
#       print(nx, ny, no, res)

tf = [True, False]

def neg(a):
  return ''.join(['0' if x=='1' else '1' for x in a])
  

def xor(a, b):
  return ''.join(['0' if (x=='1' and y =='1') else '1' for x,y in zip(a, b)])

for nx in tf:
  for ny in tf:
    for no in tf:
      a = 3
      b = 7
      if nx: a = ~a
      if ny: b = ~b
      res = a + b
      # dres = b + a
      if no: res = ~res
      print(nx, ny, no, res)
print()
for nx in tf:
  for ny in tf:
    for no in tf:
      a = 7
      b = 3
      if nx: a = ~a
      if ny: b = ~b
      res = a + b
      # dres = b + a
      if no: res = ~res
      print(nx, ny, no, res)