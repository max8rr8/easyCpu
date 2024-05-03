import csv

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
        return ((float(opt) - float(nonopt)) / float(nonopt) * 100)


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

    for tename in test_order:
        nonopt = tests_stat[tename]
        opt = None
        if (tename + "$STACKOPT") in tests_stat:
            opt = tests_stat[tename + "$STACKOPT"]
        else:
            continue

        f.write(f"<h1>{tename}</h1>\n")
        f.write(
            f"<table><thead><tr><td>Metric</td><td>Nonopt</td><td>Opt</td><td>Diff</td></tr></thead><tbody>\n"
        )

        for metr, nonopt, opt in zip(cols, nonopt, opt):
            diff = calc_diff(int(opt), int(nonopt))
            f.write(
                f"<tr><td>{metr}</td><td>{nonopt}</td><td>{opt}</td><td>{diff:3.2f}%</td></tr>\n"
            )

        f.write("</tbody></table>\n")

    f.write("</body></html")

# print(test_order)
