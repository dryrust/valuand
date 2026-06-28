# This is free and unencumbered software released into the public domain.

from dataclasses import dataclass

# See: https://docs.python.org/3/library/functions.html#int


class _Natural:
    pass


@dataclass
class U8(_Natural):
    value: int


@dataclass
class U16(_Natural):
    value: int


@dataclass
class U32(_Natural):
    value: int


@dataclass
class U64(_Natural):
    value: int


Natural = U8 | U16 | U32 | U64
