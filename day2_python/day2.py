# pip install pandas
import pandas as pd

# Function to check if a report is safe
def is_safe(report):
    levels = list(map(int, report.split()))
    increasing = all(1 <= levels[i+1] - levels[i] <= 3 for i in range(len(levels) - 1))
    decreasing = all(1 <= levels[i] - levels[i+1] <= 3 for i in range(len(levels) - 1))
    return increasing or decreasing


def is_safe_with_dampener(report):
    levels = list(map(int, report.split()))
    if is_safe(report):
        return True
    for i in range(len(levels)):
        modified_report = levels[:i] + levels[i+1:]
        if is_safe(" ".join(map(str, modified_report))):
            return True
    return False


#I prefer to dump the data given by AOC to a csv and read it in
file_path = "d2.csv"
data_from_csv = pd.read_csv(file_path, header=None)
safe_reports = sum(is_safe(report[0]) for report in data_from_csv.values)
print(f"Number of safe reports: {safe_reports}")

safe_reports_with_dampener = sum(is_safe_with_dampener(report[0]) for report in data_from_csv.values)
print(f"Number of safe reports with dampeners: {safe_reports_with_dampener}")
