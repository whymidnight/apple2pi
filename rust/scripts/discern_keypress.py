from sys import argv

def get_keycode(char_code):
    key_down = char_code << 2
    key_up = char_code

    return hex(key_down), hex(key_up)


if __name__ == "__main__":
    key_down, key_up = get_keycode(ord(argv[1]))

    print(f"""
    key down: {key_down}
    key up: {key_up}
    """)

