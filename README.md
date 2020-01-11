# bruteforcer - A cli program written in Rust to brute force hashes using libbruteforce

This cli uses [libbruteforce](https://github.com/phip1611/libbruteforce).

For help type `bruteforce -h`

```
Synopsis:
    $ bruteforcer <input_to_crack:string> <hashing_algo:string> <max_len:num> <min_len:num> [... flags]

hashing algorithm:
    - one of 'identity' (no hash), 'sha1', 'sha256', or 'md5'
    - The hashing algorithm will be applied to every possible value during bruteforce before
      it's compared to the target value

flags:
    -L: use capital letters (A-Z)
    -l: use lower case letters (a-z)
    -d: use digits (0-9)
    -U: use capital umlauts (Ä-Ü)
    -u: use lower case umlauts (ä-ü)
    -S: use all special chars on QWERTZ keyboards
    -s: use common special chars on QWERTZ keyboards
    -h: show help
    -A=<Characters>
        - if this is set the above flags will be ignored
        - there is no check if each character is only once in the string

    hint: flags can stand anywhere in the command
```