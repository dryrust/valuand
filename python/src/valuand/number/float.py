# This is free and unencumbered software released into the public domain.

from dataclasses import dataclass

# See: https://docs.python.org/3/library/functions.html#float


class _Float:
    pass


@dataclass
class F32(_Float):
    value: float


@dataclass
class F64(_Float):
    value: float


Float = F32 | F64
