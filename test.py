
test = [
  (0, 0, False),
  (1, 1, False),
  (0xffff, 0xffff, True),
  (0x8000, 0x8001, True),
  (0x8001, 0x8000, True),
  (0xffff, 1, True),
  (1, 0xffff, True),
  (1, 0xfffe, False),
]

for a,b,res in test:
  correct = a >= b
  print(correct, hex(a), hex(b), hex((a - b) & 0xffff), hex((b - a) & 0xffff))
  print()