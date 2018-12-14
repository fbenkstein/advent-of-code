from utils.decorators import time_it

with open('input') as f:
    puzzle_input = f.read().strip()


@time_it
def part_one(n):
    n = int(n)
    recipes = [3, 7]
    elf_one = 0
    elf_two = 1
    made = 4
    while made < (n + 10):
        made += 1
        new_recipe = recipes[elf_one] + recipes[elf_two]
        if new_recipe > 9:
            a, b = [c for c in str(new_recipe)]
            recipes.append(int(a))
            recipes.append(int(b))
        else:
            recipes.append(new_recipe)
        elf_one += 1 + recipes[elf_one]
        elf_one %= len(recipes)

        elf_two += 1 + recipes[elf_two]
        elf_two %= len(recipes)

    return ''.join(map(str, recipes[n:n+10]))


@time_it
def part_two(n):
    n = [int(c) for c in n]
    recipes = [3, 7]
    elf_one = 0
    elf_two = 1
    while True:
        new_recipe = recipes[elf_one] + recipes[elf_two]

        recipes.extend([int(c) for c in str(new_recipe)])
        elf_one += 1 + recipes[elf_one]
        elf_one %= len(recipes)

        elf_two += 1 + recipes[elf_two]
        elf_two %= len(recipes)

        if recipes[-len(n):] == n or recipes[-len(n)-1:-1] == n:
            break
    return ''.join(map(str, recipes)).index(''.join(map(str, n)))


test_one = {
    '9': '5158916779',
    '5': '0124515891',
    '18': '9251071085',
    '2018': '5941429882'
}

test_two = {
    '51589': '9',
    '01245': '5',
    '92510': '18',
    '59414': '2018'
}

for test, ans in test_one.items():
    p1 = part_one(test)
    print(f'Test one: {p1}')
    assert p1 == ans

for test, ans in test_two.items():
    p2 = part_two(test)
    print(f'Test two: {p2}')
    assert p2 == int(ans)

print(f'Part 1: {part_one(puzzle_input)}')
print(f'Part 2: {part_two(puzzle_input)}')
