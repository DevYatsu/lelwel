x = f"{{{'x':}>5}}"  # Edge case with nested format spec containing '>'
y = f"{'x':>5}"    # Normal case with alignment
z = f"{123:^10}"   # Another normal case with centering
w = f"{'x':.2f}"   # Normal case with precision
