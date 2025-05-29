num = 1234.56789
int_num = 42
text = "centered"
val = "café"
f"{val!s}"  # 'café'
f"{val!r}"  # "'café'"
f"{val!a}"  # "'caf\\xe9'"

f"{val:}"

f"{text:<20}"     # 'centered           ' (left align)
f"{text:>20}"     # '           centered' (right align)
f"{text:^20}"     # '     centered      ' (centered)

f"{text:*^20}"    # '*****centered*****'
f"{text:.^20}"    # '.....centered.....'

f"{text:<20}"     # 'centered           ' (left align)
f"{text:>20}"     # '           centered' (right align)
f"{text:^20}"     # '     centered      ' (centered)

f"{text:*^20}"    # '*****centered*****'
f"{text:.^20}"    # '.....centered.....'

f"{int_num:d}"      # '42' decimal
f"{int_num:b}"      # '101010' binary
f"{int_num:o}"      # '52' octal
f"{int_num:x}"      # '2a' hex lowercase
f"{int_num:X}"      # '2A' hex uppercase
f"{int_num:#x}"     # '0x2a' with prefix
f"{int_num:#o}"     # '0o52'
f"{int_num:#b}"     # '0b101010'

f"{int_num:05d}"      # '00042' pad with 0s
f"{-int_num:=5}"      # '-  42' sign before padding
f"{int_num:+5}"       # '  +42' force sign
f"{int_num: 5}"       # '   42' with space for positive

f"{num:2}"         # '1234.57' fixed 2 decimals
f"{num:.0f}"         # '1235' rounded
f"{num:10.2f}"       # '   1234.57' width 10, 2 decimals
f"{num:e}"           # '1.234568e+03' scientific
f"{num:.2e}"         # '1.23e+03'
f"{num:%}"           # '123456.789000%' (value * 100 + %)

f"{1000000:,}"       # '1,000,000'
f"{1000000:_}"       # '1_000_000'

f"{text:.5}"         # 'cente' — max 5 chars
f"{text:>10.5}"      # '     cente'

name = "Lelwel"
amount = 1234567.89123
print(f"{name:^10s} | {amount:>15,.2f}")  # '   Lelwel  |     1,234,567.89'

width = 10
f"{'test':{width}}"  # 'test     '

f"{'x':!>5}"   # '!!!!x'
f"{'x':}>5}"   # should produce a SyntaxError
f"{'x':}>5"  
f"{'x':>5}"  

f"{1234:>10}"  # uses default str() formatting

f"{float('inf'):.2f}"   # 'inf'
f"{float('nan'):.2f}"   # 'nan'

f"{'汉字':^10}"   # May not align visually depending on terminal

# f"{1234:Q}"      # ValueError
# f"{1234:.}"      # ValueError

f"{{ {1234} }}"  # '{ 1234 }'
