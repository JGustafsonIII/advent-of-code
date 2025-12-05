file = open("./test_data/day1_output.txt")
cmds = file.readlines()

start = 50
count = 0

for line in cmds:
    num = int(line[1:])

    if line[0] == "L":
        # Moving left (decreasing)
        new_pos = start - num

        # Count how many times we pass through 0
        # We pass 0 when going from positive to negative multiples of 100
        if new_pos < 0:
            count += (start - new_pos) // 100

        # Normalize position to 0-99 range
        start = new_pos % 100

    else:  # "R"
        # Moving right (increasing)
        new_pos = start + num

        # Count how many times we pass through 0
        # We pass 0 when crossing 100, 200, 300, etc.
        if new_pos >= 100:
            count += (new_pos - start) // 100

        # Normalize position to 0-99 range
        start = new_pos % 100

print(count)
file.close()
