# This is free and unencumbered software released into the public domain.

from dataclasses import dataclass

# See: https://docs.python.org/3/library/functions.html#int


class _Integer:
    pass


@dataclass
class I8(_Integer):
    value: int


@dataclass
class I16(_Integer):
    value: int


@dataclass
class I32(_Integer):
    value: int


@dataclass
class I64(_Integer):
    value: int


Integer = I8 | I16 | I32 | I64
