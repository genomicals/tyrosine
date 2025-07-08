
def prob(size, ratio, i):
    if i < 0 or i > size - 1:
        return -1.0
    if i != 0:
        i += 1
    if i > size * ratio:
        return 1 - ((size - i) / (size * (1 - ratio) * 2))
    else:
        return i / (size * ratio * 2)

print(f"0.0     {prob(1000, 0.5, 0)}")
print(f"0.25    {prob(1000, 0.5, 249)}")
print(f"0.5     {prob(1000, 0.5, 499)}")
print(f"0.75    {prob(1000, 0.5, 749)}")
print(f"1.0     {prob(1000, 0.5, 999)}")

print()

print(f"0.0     {prob(500, 0.5, 0)}")
print(f"0.25    {prob(500, 0.5, 124)}")
print(f"0.5     {prob(500, 0.5, 249)}")
print(f"0.75    {prob(500, 0.5, 374)}")
print(f"1.0     {prob(500, 0.5, 499)}")

print()

print(f"0.0     {prob(1000, 0.1, 0)}")
print(f"0.25    {prob(1000, 0.1, 49)}")
print(f"0.5     {prob(1000, 0.1, 99)}")
print(f"0.75    {prob(1000, 0.1, 549)}")
print(f"1.0     {prob(1000, 0.1, 999)}")


