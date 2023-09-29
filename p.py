from ctypes import CDLL

a = 1
b = 2
print("===========================================================")
print(f"I'm about to call some Rust function using {a} and {b} parameters.")

result = CDLL("./target/debug/libsome_lib.dylib").add(a, b)

print(f"Back to python interpreter. The sum of {a} and {b} is {result}")
