import subprocess
import time

subprocess.run(["cargo", "run", "asm", "asm/simp_calc_opt.s"])

def generate_test(op, inpa, inpb, res):
    return (
        op + "\n" + inpa + "\n" + inpb,
        f"""Hello, this is simple calculator!
Enter operation: Enter operand 1: 0xEnter operand 2: 0xResult:          0x{res}\nGoodbye!\n""",
    )


tests = [
  generate_test("+", "1234", "5678", "68ac"),
  generate_test("-", "aaff", "1346", "97b9"),
  generate_test("^", "b4f6", "c78d", "8484"),
]

for i, (inp, out) in enumerate(tests):
    res = subprocess.Popen(
        ["./obj_dir/Vour"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    for l in inp.split("\n"):
        time.sleep(0.01)
        res.stdin.write((l + "\n").encode("utf-8"))
        res.stdin.flush()

    res.wait()

    res = res.stdout.read().decode("utf-8")
    print(f"Test {i+1}:", "PASSED" if out == res else "FAILED")
    if out != res:
        print()
        print("Expected:", out)
        print("Got:", res)
        print()
