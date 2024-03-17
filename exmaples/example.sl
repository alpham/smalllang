a = 2
b = 3
c = a * 2 + b - 7

pring(c)


expression: assignment
assignment: (variable "=")? expression | term
term: factor ( ("+" | "-") factor )*
factor: primary ( ("*" | "/") primary )*
primary: variable | number | "(" expression ")"
funcall: variable "(" expression ")"

