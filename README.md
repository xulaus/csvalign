`csvtools`
===

A set of very simple command line tools to do useful things with CSV files that other tools (eg `xsv`) don't do.

## `csv align`
Takes csv files and concatenate them, regardless of column order and consistency.

### Example
```
$ csv align -v --at-least 5 -o out.csv survey_respondent_data_*
Found 1976 columns
365 columns left after filtering
Concatinating 2696 files
Done!
```

### Help
```
$ csv align --help
Align several csv files by column name and concatenate them

Usage: csv align [OPTIONS] [INPUT_FILES]...

Arguments:
  [INPUT_FILES]...

Options:
      --at-least <AT_LEAST>  Only use columns that are in at least this many files [default: 1]
  -o, --out <OUT>            File to output to, if not specified standard out will be used
  -v, --verbose
  -h, --help                 Print help information
```

## `csv dedup`
Take a single csv file and make one of the columns unique.

### Help

```
$ csv dedup --help
Take a single csv file and make one of the columns unique

Usage: csv dedup [OPTIONS] <INPUT_FILE> <UNIQUE_COLUMN>

Arguments:
  <INPUT_FILE>
  <UNIQUE_COLUMN>  Column to ensure every entry is unique in

Options:
      --max-by <MAX_BY>  Choose which entry to keep by keeping the maximum value in this column
      --min-by <MIN_BY>  Choose which entry to keep by keeping the minimum value in this column
  -o, --out <OUT>        File to output to, if not specified standard out will be used
  -h, --help             Print help information
```

## `csv fix-width`
Pretty print the CSV, still in csv format

### Example
```
$ csv fix-width test.csv
1,  1.42, 1.45, 3.2,  3.4
2,  3.2,  3.3,  1.44, 1.45
3,  3,    3.3,  1.44, 1.5
4,  2.38, 2.48, 1.68, 1.71
5,      ,     ,     ,
6,  3.2,  3.3,  1.44, 1.45
7,  3.05, 3.2,  1.46, 1.48
8,  1.54, 1.69, 2.44, 2.86
9,  3,    6,    1.2,  1.49
10, 1.46, 2.34, 1.75, 3.9

```

### Help
```
$ csv fix-width --help
Pretty print the CSV, still in csv format

Usage: csv fix-width [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>

Options:
  -o, --out <OUT>  File to output to, if not specified standard out will be used
  -h, --help       Print help

```
