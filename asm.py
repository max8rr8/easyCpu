#!/usr/bin/env python3

import struct
import sys


def parse_const(st):
  if len(st) >= 3 and st[0] == "'" and st[2] == "'":
    return ord(st[1])

  try:
    return int(st, 0)
  except ValueError:
    return None


print("Input file:", sys.argv[1])
asm = None
with open(sys.argv[1], 'r') as f:
  asm = f.read()

# print(asm)
lines = asm.split('\n')
lines = list(map(lambda x: x.split('#', 1)[0], lines))

registers = {
    'ZX': 0,
    'PC': 1,
    'R2': 2,
    'R3': 3,
    'R4': 4,
    'R5': 5,
    'R6': 6,
    'IA': 7
}


def reg_enc(reg, pos):
  rg = registers[reg]
  if pos == 1:
    return rg << 6
  elif pos == 2:
    return rg << 3
  else:
    return rg


def int_enc(val, si):
  k = int(val, 0)
  if k >= (1 << si):
    raise ValueError('Too high shift')
  res = abs(k) & ((1 << si) - 1)
  if k < 0:
    res = res | (1 << si)
  return res


def cmd_enc(cmd, flags, *a):
  res = cmd << 12
  res |= flags << 9
  for el in a:
    res |= el
  return res


# def cmd_flags()

commands = {
    # NOP
    'NOP':
    lambda flags: [0],
    # AND r0 r1 r2
    'AND':
    lambda flags, r0, r1, r2:
    [cmd_enc(0b0100, flags, reg_enc(r0, 1), reg_enc(r1, 2), reg_enc(r2, 3))],
    # ADD r0 r1 r2
    'ADD':
    lambda flags, r0, r1, r2:
    [cmd_enc(0b0101, flags, reg_enc(r0, 1), reg_enc(r1, 2), reg_enc(r2, 3))],
    # LOAD r0 r1 2
    'LOAD':
    lambda flags, r0, r1, sh:
    [cmd_enc(0b0010, flags ^ 0b110, reg_enc(r0, 1), reg_enc(r1, 2), int_enc(sh, 2))],
    # STORE r0 PC 0x2
    'STORE':
    lambda flags, r0, r1, sh:
    [cmd_enc(0b0011, flags ^ 0b110, reg_enc(r0, 1), reg_enc(r1, 2), int_enc(sh, 2))],
    # LOAD r0 r1 2
    'BRANCH':
    lambda flags, r0, sh:
    [cmd_enc(0b0001, flags, reg_enc(r0, 1), int_enc(sh, 5))],

    #HALT
    'HALT':
    lambda flags: [cmd_enc(0b1111, 0b111)],
}

commands.update({
    'SUB':
    lambda flags, *a: commands['ADD'](flags ^ 0b101, *a),
    'OR':
    lambda flags, *a: commands['AND'](flags ^ 0b111, *a),
    'NAND':
    lambda flags, *a: commands['AND'](flags ^ 0b001, *a),
    'NOR':
    lambda flags, *a: commands['AND'](flags ^ 0b111, *a),
    'NOT':
    lambda flags, r0, r1: commands['AND'](flags ^ 0b011, r0, r1, 'ZX'),
    'NEG':
    lambda flags, r0, r1: commands['ADD'](flags ^ 0b101, r0, 'ZX', r1),
})

const_hard_add = {0: 0, 1: 0b111, -1: 0b100, 2: 0b110}

labels = {}
compiled = []

for line in lines:
  if len(line) > 0 and line[-1] == ':':
    line = line[:-1]
    labels[line] = -1
    # print("LABEL:", line)


def load_const(flags, reg, const):
  n: int = parse_const(const)
  if n < 0:
    n = n & 0xffff

  if n > 0xffff:
    raise ValueError('Invalid const value')

  if n in const_hard_add:
    return commands['ADD'](const_hard_add[n], reg, 'ZX', 'ZX')

  if n < 2**12:
    return [n] + commands['LOAD'](0, reg, 'PC', "-0x1")

  if (~n) < 2**12:
    return [~n] + commands['LOAD'](0, reg, 'PC', "-0x1") + commands['NOT'](
        0, reg, reg)

  return ([n & 0xff] + commands['LOAD'](0b100, reg, 'PC', "-0x1") +  # Low
          [(n >> 8) & 0xff] + commands['LOAD'](0b011, reg, 'PC', "-0x1"))


commands['LC'] = load_const


def load_label(flags, reg, label, add=0):
  if labels[label] == -1:
    return [0x00, 0x00, 0x00]
  prev_shift = 0
  cur_shift = 1
  cur_set = []
  while cur_shift != prev_shift:
    prev_shift = cur_shift
    cur_shift = labels[label] - len(compiled) - add - len(cur_set) + 2
    cur_set = commands['LC'](0, "IA", str(cur_shift)) + commands['ADD'](
        0, reg, "IA", "PC")
    if (labels[label] - len(compiled) - add - len(cur_set) + 1) > cur_shift:
      cur_set += [0]

    print(cur_shift, len(cur_set), cur_set)
  return cur_set


commands['LL'] = load_label


def jump(flags, reg, label):
  if labels[label] == -1:
    return [0x00]
  # print(abs(labels[label] - len(compiled)))
  if abs(labels[label] - len(compiled)) < 32:
    return commands['BRANCH'](flags, reg, str(labels[label] - len(compiled)))
  else:
    cc = commands['LL'](0, "PC", label)
    return commands['BRANCH'](flags, (~reg) & 0b111, len(cc) + 1) + cc

  # cmd_enc(0b0100, flags, reg_enc(r0, 1), reg_enc(r1, 2), reg_enc(r2, 3))

commands['JMPC'] = jump

commands.update({
    'JMP':
    lambda flags, *a: commands['JMPC'](flags ^ 0b111, "ZX", *a),
    'JEQ':
    lambda flags, *a: commands['JMPC'](flags ^ 0b100, *a),
    'JNE':
    lambda flags, *a: commands['JMPC'](flags ^ 0b011, *a),
    'JGT':
    lambda flags, *a: commands['JMPC'](flags ^ 0b010, *a),
    'JLT':
    lambda flags, *a: commands['JMPC'](flags ^ 0b001, *a),
    'JGE':
    lambda flags, *a: commands['JMPC'](flags ^ 0b110, *a),
    'JLE':
    lambda flags, *a: commands['JMPC'](flags ^ 0b101, *a),
})

should_recompile = True
while should_recompile:
  print("Compiling")
  should_recompile = False
  compiled = []
  for line_number, line in enumerate(lines):

    line = line.strip().upper()
    if line == '':
      continue

    if line[-1] == ":":
      line = line[:-1]
      if labels[line] != len(compiled):
        labels[line] = len(compiled)
        should_recompile = True
        break
      else:
        continue

    if parse_const(line) is not None:
      compiled.append(parse_const(line))
    else:
      # print(line)

      line = line.split(' ')
      cmd = line[0].split('.')
      flags = 0
      if len(cmd) > 1:
        flags = int(cmd[1], 0)
        # print("Flags: " + flags)
      if cmd[0] in commands:
        compiled += commands[cmd[0]](flags, *line[1:])
      else:
        print("Unknown command", line_number + 1)

# print(list(lines))
print("Total length:", len(compiled))
print(', '.join([hex(a & 0xffff) for a in compiled]))

with open("./ram.bin", "wb") as f:
  to_write = [a & 0xffff for a in compiled]
  to_write = struct.pack("!" + "H" * len(to_write), *to_write)
  f.write(to_write)
# res = ""
# for c in compiled:
#   print(hex(c))