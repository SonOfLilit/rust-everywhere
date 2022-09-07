from rust_everywhere_python import reverse

def symmetric(s):
    if not s:
        return s
    r = reverse(s)
    return s + r[1:]
