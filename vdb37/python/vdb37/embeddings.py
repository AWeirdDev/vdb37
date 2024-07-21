from typing import List, Tuple, Union, overload

try:
    import httpx
except (ImportError, ModuleNotFoundError):
    raise ImportError(
        "httpx is not installed. Please install it with `pip install httpx`."
    )


from .vdb37 import Vector


@overload
def embed(inputs: List[str]) -> List[Vector]:
    """Embed with a pre-existing API.

    ```python
    embed(["My text to be embedded", "My other text to be embedded"])
    ```

    Args:
        inputs (List[str]): The inputs to embed.

    Returns:
    """


@overload
def embed(inputs: List[Tuple[str, str]]) -> List[Vector]:
    """Embed with a pre-existing API.

    Use ``(metadata, text)``.

    ```python
    embed([
        ("some metadata", "My text to be embedded"),
        ("metadata2", "My other text to be embedded"),
    ])
    ```

    Args:
        inputs (list[tuple[str, str]]): The inputs to embed. (Mapped with ``(metadata, text)``)

    Returns:
        List[Vector]: The embedded vectors.
    """


def embed(inputs: Union[List[str], List[Tuple[str, str]]]) -> List[Vector]:
    has_metadata = isinstance(inputs[0], tuple)

    with httpx.Client() as client:
        res = client.post(
            "https://aweirddev-embeddings.hf.space/embeddings",
            json={
                "input": inputs if not has_metadata else [item[1] for item in inputs]
            },
        )
        res.raise_for_status()

    outputs = res.json()

    vectors = []
    for i, out in enumerate(outputs):
        vectors.append(Vector(out, meta=inputs[i][0] if has_metadata else None))

    return vectors


@overload
async def aembed(inputs: List[str]) -> List[Vector]:
    """Async embed with a pre-existing API.

    ```python
    await aembed(["My text to be embedded", "My other text to be embedded"])
    ```

    Args:
        inputs (List[str]): The inputs to embed.

    Returns:
        List[Vector]: The embedded vectors.
    """


@overload
async def aembed(inputs: List[Tuple[str, str]]) -> List[Vector]:
    """Async embed with a pre-existing API.

    Use ``(metadata, text)``.

    ```python
    await aembed([
        ("some metadata", "My text to be embedded"),
        ("metadata2", "My other text to be embedded"),
    ])
    ```

    Args:
        inputs (list[tuple[str, str]]): The inputs to embed. (Mapped with ``(metadata, text)``)

    Returns:
        List[Vector]: The embedded vectors.
    """


async def aembed(inputs: Union[List[str], List[Tuple[str, str]]]) -> List[Vector]:
    has_metadata = isinstance(inputs[0], tuple)

    async with httpx.AsyncClient() as client:
        res = await client.post(
            "https://aweirddev-embeddings.hf.space/embeddings",
            json={
                "input": inputs if not has_metadata else [item[1] for item in inputs]
            },
        )
        res.raise_for_status()

    outputs = res.json()

    vectors = []
    for i, out in enumerate(outputs):
        vectors.append(Vector(out, meta=inputs[i][0] if has_metadata else None))

    return vectors
