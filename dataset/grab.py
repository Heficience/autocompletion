# grab http://www.lexique.org/databases/Lexique383/Lexique383.tsv and parse it
import os
import sys
import urllib.request
import urllib.parse
import urllib.error
import csv
import shutil

URL_TSV = "http://www.lexique.org/databases/Lexique383/Lexique383.tsv"

def download_tsv():
    """
    Download TSV file from URL_TSV
    """
    try:
        urllib.request.urlretrieve(URL_TSV, "Lexique383.tsv")
    except urllib.error.URLError as e:
        print("Error downloading TSV file:", e)
        sys.exit(1)


def parse_tsv_to_csv():
    """
    Parse TSV file to CSV file
    """
    with open("Lexique383.tsv", "r", encoding="utf-8") as tsv_file:
        tsv_reader = csv.reader(tsv_file, delimiter="\t")
        next(tsv_reader) # remove first line 
        with open("Lexique383.csv", "w", encoding="utf-8") as csv_file:
            csv_writer = csv.writer(csv_file, delimiter=";")
            for row in tsv_reader:
                csv_writer.writerow(row)

def parse_csv():
    """
    Keep only the first column of the CSV file
    """
    with open("Lexique383.csv", "r", encoding="utf-8") as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=";")
        with open("Lexique383_final.csv", "w", encoding="utf-8") as csv_file:
            csv_writer = csv.writer(csv_file, delimiter=";")
            for row in csv_reader:
                # calculate the moyenne of 2,3,4,5 columns
                moyenne = (float(row[6]) + float(row[7]) + float(row[8]) + float(row[9])) / 4
                csv_writer.writerow([row[0], moyenne])




def main():
    """
    Main function
    """
    print("Downloading TSV file...")
    download_tsv()
    print("Parsing TSV file...")
    parse_tsv_to_csv()
    print("Parsing CSV file...")
    parse_csv()
    print("Done.")

    # remove TSV file
    os.remove("Lexique383.tsv")
    # remove CSV file
    os.remove("Lexique383.csv")
    
    try:
        os.remove("../Lexique383.csv")
    except OSError:
        pass
    shutil.move("Lexique383_final.csv","./Lexique383.csv")



if __name__ == "__main__":
    main()
