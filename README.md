# multidl

Download a file using multiple threads in parallel for faster download speeds. Uses 0 external dependencies.

## Usage

```
Usage: multidl [--help] ADDRESS_WITH_PORT PATH_TO_FILE
Download a file in multiple parts in parallel into data.bin.

--help - Show this help message, and exit with code 1.
ADDRESS_WITH_PORT - Can be IP address or a domain name. ( example 127.0.0.1:3000 or cdn.example.com:80 )
PATH_TO_FILE - Absoulte path to the file, including preceding slash. ( example / or /public/file.bin )
```

# Sample Files

A file `2-mb-file.txt` is in the repo for self hosting. Otherwise, checkout http://xcal1.vodafone.co.uk/. So,
a sample run command can be `cargo run 212.183.159.230:80 /10MB.zip`.

# License

This software is licensed under GNU General Public License v3.0 or later.
