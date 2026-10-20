"""How the example greets someone."""


def greeting(name: str) -> str:
    """The greeting for `name`, or for a stranger when the name is blank."""
    name = name.strip()
    if not name:
        return "Hello, stranger!"
    return f"Hello, {name}!"
