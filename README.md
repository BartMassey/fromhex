# fromhex: hex command-line argument to stdout byte string
Bart Massey 2022

This program accepts a string of hex digits as its
command-line argument and writes the corresponding bytes to
`stdout`. If the number of hex digits is odd, the argument
is zero-padded on the left.

For example:

    fromhex e0

will write the single byte `0xe0` to `stdout`. Note that no
newline padding is done.


This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
