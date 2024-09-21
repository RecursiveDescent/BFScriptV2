# A language that compiles to brainf*

Very incomplete and not much is implemented

Semicolons will break the code since the parser doesn't handle them yet.

Currently very limited

*Use `print` to output expressions and currently `read` is a constant which returns a single character from the input*

# Features

- [x] While loops
- [x] If statements
- [x] Variables
- [x] Input
- [x] Output
- [ ] Type system

# Supported operators

- [x] +
- [x] -
- [x] *
- [x] /
- [x] >
- [x] <
- [x] >=
- [x] <=
- [x] =
- [x] ==
- [x] !=

# Example

The following will output the alphabet up to the letter in the input.

(Will loop forever or until it wraps if you dont provide an input in bounds of the uppercase alphabet)

```
int i = 'A'

int stop = read

print i

while i != stop {
	i = i + 1
	
	print i
}
```