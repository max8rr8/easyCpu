import csv
import io

cols = []
test_order = []
tests_stat = {}


with open("perf.csv") as f:
    reader = csv.reader(f)
    cols = next(reader)[1:]

    for row in reader:
        test_name = row[0]
        tests_stat[test_name] = row[1:]
        if "$STACKOPT" not in test_name:
            test_order.append(test_name)


def calc_diff(opt, nonopt):
    if nonopt == 0 and opt == 0:
        return 0
    elif opt == 0:
        return -100
    else:
        return (float(opt) - float(nonopt)) / float(nonopt) * 100


def write_test_table(f: io.TextIOWrapper, nonopt: list[int], opt: list[int]):
    f.write(
        f"<table><thead><tr><td>Metric</td><td>Nonopt</td><td>Opt</td><td>Diff</td></tr></thead><tbody>\n"
    )

    for metr, nonopt, opt in zip(cols, nonopt, opt):
        diff = calc_diff(int(opt), int(nonopt))
        f.write(
            f"<tr><td>{metr}</td><td>{nonopt}</td><td>{opt}</td><td>{diff:3.2f}%</td></tr>\n"
        )

    f.write("</tbody></table>\n")


with open("report.html", "w") as f:
    f.write(
        """
<!DOCTYPE html>
<html>
<head>
  <title>Performance report</title>
  <style>
    * {
      font-family: monospace;
    }
          
    table, th, td {
      border: 1px solid black;
      border-collapse: collapse;
      font-size: 1.2em;
      padding: 4px;
    }
  </style>
</head>
<body>\n"""
    )

    total_counts = ([0] * len(cols), [0] * len(cols))

    for tename in test_order:
        nonopt = tests_stat[tename]
        opt = None
        if (tename + "$STACKOPT") in tests_stat:
            opt = tests_stat[tename + "$STACKOPT"]
        else:
            continue

        for i in range(len(cols)):
            nonopt[i] = int(nonopt[i])
            total_counts[0][i] += int(nonopt[i])
            opt[i] = int(opt[i])
            total_counts[1][i] += int(opt[i])
        f.write(f"<h1>{tename}</h1>\n")
        write_test_table(f, nonopt, opt)
    
    f.write(f"<h1>Total</h1>\n")
    write_test_table(f, total_counts[0], total_counts[1])

    f.write("</body></html")

# print(test_order)
