from typing import Any

from ._dict_not_none import *


def dict_not_none(**kwargs: Any) -> Any:
    return {k: v for k, v in kwargs.items() if v is not None}
