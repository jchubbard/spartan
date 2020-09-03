# Project Spartan Guidelines

## Declaration

```spartan
// implicit typing
x = 5;

// explicit typing
x (int) = 5;
```

## Types

### Basic

```spartan
x (int);
x (float);
x (str);
x (bool);
```

### Complex
```spartan
x (vector<T>);
X (enum);
X (struct);
```

## Operations

### Basic
```spartan
x = 5 + 5;
x = 5 - 5;
x = 5 * 5;
x = 5 / 5;

x = 5 % 5;
x = 5**5;
```

### Complex
```spartan
print(x);

ceil(x);

floor(x);
```

## Functions

```spartan
// no return type
foo(int x) {

}

// return type
string bar(int x) {

}
```

## Logic

### Loops
```spartan
// normal while loop
while (condition) {

}

// loops 10 times, i counts from 0 to 9
for (i, 10) {

}

// i is pre-declared
for (i != 0; i--) {

}

// i is not pre-declared
for (int i = 9; i != 0; i--) {

}
```

### If Logic
```spartan
if (condition) {

}
```