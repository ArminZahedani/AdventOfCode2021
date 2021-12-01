#Small python helper script to transform the input file into something that can easily be read by PP2.
with open("input.txt", "r") as file:
    a = file.readlines()
    a = [int(line.replace("\n", "")) for line in a]

with open("aaa.txt", "w") as file:
    file.write(str(a))