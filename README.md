Radix Conversion Tool
=====================

This simple, command-line tool displays numbers in decimal,
hexadecimal, and binary representations. Here's an example
output:

```
$ conv 1 -1 -127 -129 0xbeef 0b10100101
           1:      1    0x01           0b00000001
          -1:     -1    0xFF           0b11111111
        -127:   -127    0x81           0b10000001
        -129:   -129  0xFF7F  0b11111111_01111111
      0xbeef:  48879  0xBEEF  0b10111110_11101111
  0b10100101:    165    0xA5           0b10100101
```

The tool uses big integer representation internally so the
precision is arbitrary:

```
$ conv -123456789012345678901234567890
  -123456789012345678901234567890:  -123456789012345678901234567890  0xFE7116F0093C8C1F11B1C0F52E  0b11111110_01110001_00010110_11110000_00001001_00111100_10001100_00011111_00010001_10110001_11000000_11110101_00101110
```

License
=======

Copyright (c) 2015 Sergey "SnakE" Gromov

See the file license.txt for copying permission.

