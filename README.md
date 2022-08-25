`csvalign`
===

A very simple command line app to take csv files and concatenate them, regardless of column order and consistency.

## Example
```
$ csvalign -v --at-least 5 -o out.csv survey_respondent_data_*
Found 1976 columns
365 columns left after filtering
Concatinating 2696 files
Done!
```

```
$ csvalign --help
csvalign
Align several csv by column name and concatinate them

USAGE:
    csvalign [OPTIONS] --out <OUT> [INPUT_FILES]...

ARGS:
    <INPUT_FILES>...

OPTIONS:
        --at-least <AT_LEAST>    Only use columns that are in at least this many files [default: 1]
    -h, --help                   Print help information
    -o, --out <OUT>
    -v, --verbose
```
