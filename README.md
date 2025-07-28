# MeLC: MeLc is a Little Converter #

## Purpose ##
MeLC is a CLI tool meant to convert numbers among
different usual bases: 

- Binary
- Octal
- Decimal
- Hexadecimal

##### Working with...
- Signed and unsigned integers in **Two's Complement**
- *QWORD*, *DWORD*, *WORD* and *BYTE* lengths.

## Options ##
You can input a number in any base, to do so
you must use the following options:

|   Option  |   Meaning    |
|:---------:|:------------:|
|    `-d`     |  decimal     |
|    `-x`     |  hexadecimal |
|    `-b`     |  binary      |
|    `-o`     |  octal       |

<br>

To set if the number is signed or not:
|   Option  |   Meaning    |
|:---------:|:------------:|
|     `-s`    |    signed    |
|     `-u`    |   unsigned   |

<br>

To set the number length:
|   Option  |   Meaning    |
|:---------:|:------------:|
|  `--byte` |     byte     |
|  `--word` |     word     |
|  `--dword` |     doubleword     |
|  `--qword` |     quadword     |


## Config file

It will contain the default definitions. This is so that you don't
need to call the tool with the all flags currently in use every time.

|   Key    |   Values   |  Default  |
|:--------:|:----------:|:---------:|
|  `signed`  |`true` / `false`|    `true`   |
|  `length`  | `8` / `16` / `32` / `64` |     `32`    |
|  `base`   |`"bin"` / `"oct"` / `"dec"` / `"hex"` |  `"hex"`  |

Using the `--set` flag within a call will adjust the config file
according to the current call flags.

<br>

So, a valid command would be:

    melc -ud --byte --set 171

which would be 171 as an unsigned decimal input that changes the config file.
