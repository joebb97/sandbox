#!/usr/bin/env python
import csv, sys
import datetime, time

def main():
    row = [datetime.datetime.utcnow(), "UGV1", 0]
    header = ["Time", "Vehicle Name", "Vehicle State"]
    with open(sys.argv[1], 'wb') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(header)
        writer.writerow(row)
        time.sleep(2)
        row = [datetime.datetime.utcnow(), "UGV1", 0]
        writer.writerow(row)


if __name__ == "__main__":
    main()
