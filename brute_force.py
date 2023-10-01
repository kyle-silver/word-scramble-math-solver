import itertools
from typing import Optional


def word_sums(chars: dict[str, int], words: list[str]) -> int:
    return sum(
        sum(chars[char] * (10**index) for index, char in enumerate(reversed(word)))
        for word in words
    )


def check(permutation) -> Optional[dict[str, int]]:
    chars = {character: index for index, character in enumerate(permutation)}
    lhs = word_sums(chars, ["we", "want", "no", "new", "atomic"])
    rhs = word_sums(chars, ["weapon"])
    if lhs == rhs:
        return chars
    return None


letters = set("we want no new atomic weapon".replace(" ", ""))


for permutation in itertools.permutations(letters):
    if solution := check(permutation):
        print(f"Found! {solution}")
        break

# we   want   no   new   atomic   weapon
# 20 + 2149 + 48 + 402 + 198765 = 201384
