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

tf = [False, True]


def neg(a):
  return ''.join(['0' if x == '1' else '1' for x in a])


def xor(a, b):
  return ''.join(
      ['0' if (x == '1' and y == '1') else '1' for x, y in zip(a, b)])


def do_op(nx, ny, no, X, Y):

  a = X
  b = Y
  if nx: a = ~a
  if ny: b = ~b
  res = a & b
  # dres = b + a
  if no: res = ~res
  return res


for nx in tf:
  for ny in tf:
    for no in tf:
      res1 = do_op(nx, ny, no, 10, 6)
      res2 = do_op(nx, ny, no, 6, 10)
      res3 = do_op(nx, ny, no, 7, 3)
      print(nx, ny, no, res1, res2, res3)
