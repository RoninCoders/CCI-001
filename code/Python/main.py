print("Olá Mundo!")

def print_string_to_upper(string: str) -> None:
    if (type(string) != str):
        print("Must be a string, you provided ${}".format(type(string)))
        exit(1)
    print(string.upper())

def multiplier(x: int, y: int) -> int:
    if (type(x) and type(y) != int):
        print(f'Must be integers, you provided ${type(x)}, ${type(y)}')
        exit(1)
    result: int = (x * y)
    return result

print_string_to_upper(2)
print(multiplier(2, "3"))
