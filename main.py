from typing import NewType

import rftypes as rft

uint8: rft.UInt8 = rft.UInt8
x: uint8 = uint8(244)
y = x
x = y - uint8(20)
print(x)