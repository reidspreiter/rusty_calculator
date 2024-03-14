# Rusty-Calculator
A terminal-based calculator in Rust

### Background
Whenever I need a calculator, I do not have one at my disposal, and the options on my phone are horrible. Even so, I still don't feel as efficient using a handheld when I have access to one. I am usually sitting in front of my laptop when I need to do math; yet, I dislike the calculator options available on my computer as well. I need something fast. Something I can use comfortably without a mouse. Something I can customize a little. Rusty Calculator is my solution. 

* Why is it developed in Rust?
	* I liked the name Rusty Calculator.
	* I have become quite comfortable with a set of languages and I wanted to try programming in something I did not understand in the slightest again.
* Why is it terminal-based? No GUI?
  	* The web-based calculators on Google are annoying to use because I have to click all over the place.
  	* So many buttons are needed to support all of the operators I require. It is so much easier to remember the small amount of rules and type.
 
The following information details the information required to use Rusty Calculator effectively. All of the information can be viewed while operating the calculator by exploring the `/help` menu as well. 

### General Information
* Numbers:
	* Numbers can be entered in integer and floating point format.
	* Use 'E' for scientific notation.
* Chained Operations:
	* Multiple operations can be queued at once with ';'
	* '4+4;2+2;4*5' will evaluate each operation separately.\n\
	* This also works with commands: 'R16;/help;5!'
* Whitespace:
	* Whitespace will not affect the results of any operation.
	* Commands do depend on whitespace.\n\
	* Mutating variables must not have whitespace between the underscore and the variable name (_p) but may have whitespace anywhere thereafter.
* Parenthesis Balancing:\n\
	* Operations with unbalanced parenthesis will become balanced.
	* '4(4(4(4' will be automatically interpreted as '4(4(4(4)))'
	* This also applies to values entered within complex operators.
	* 'S[1,4(4(4,5(5+6]' will be automatically interpreted as 'S[1,4(4(4)),5(5+6)]'

### Operators
* Addition: '+'
	* Add two numbers (x+y)
* Subtraction: '-'
	* Subtract two numbers (x-y)
	* Negation (-x*-y)
* Multiplication: '*' 
	* Multiply two numbers (x*y)
 	* (x y), (x)y, x(y), xy are all interpreted as (x*y)
* Division: '/'
	* Divide two numbers (x/y)
* Integer Division: '//' or '#'
	* Computes division result without remainder (x//y) (x#y)
* Modulo: '%'
	* Computes division remainder (x%y)
* Percent of: '%%' or '\\'
	* Computes x percent of y (x%%y) (x\\y)
* Exponent: '^'
	* Computes x to the power of y (x^y)
* Factorial: '!'
	* Computes factorial (x!)
	* Currently only works on positive integers
* Root: 'R'
	* Computes xth root of y (xRy)
	* Default root value is 2 (Rx = 2Rx)
* Logarithm: 'L'
	* Computes log base x of y (xLy)
	* Default base value is 10 (Lx = 10Lx)
* Natural Log: 'N'
	* Computes log base e of x (Nx)
* Pythagorean Theorem: 'L'
	* Computes hypotenuse length of right triangle with side lengths x and y (xHy)
	* Always returns a positive length value
* Absolute Value: 'A'
	* Computes absolute value of x (Ax)
  
### Variables
* Pi: 'p'
	* Interpreted as 3.14159265359
	* Cannot be mutated
* Euler's Number: 'e'
	* Interpreted as 2.71828182845
	* Cannot be mutated
* Answer: '='
 	* Interpreted as the most recent successful result value
	* Has a value of 0 if no previous answer exists or if /reset has been called
	* '=+1;=*5;=^2' will calculate a result of 25
	* Cannot be mutated
* User Variables: 'i', 'j', 'k', 'l', 'm', 'n', 'o'
	* These variables can be changed to any value specified by the user
	* Their default value is 1
	* To change values, use the following command: _variablename(any value or computable operation)
	* To change i to -41, enter: '_i(-41)'
	* To change n to whatever '5R41' evaluates to, enter: '_n(5R41)'

### Complex Operators
* General Info:
	* Complex operators are entered in the form: 'caital letter[comma separated values]'
	* Operations can be entered as values: 'P[5+6, R5, 6!]
	* Complex operators can be embedded within one another indefinitelly: 'P[1, P[1, 3, 4], 5]'
* Summation: 'S'
	* Computes summation from start value to upper limit of equation
	* 'S[start, upper limit, equation]'
	* Equation can include the variable 'x'
* Product: 'P'
	* Computes the product from start value to upper limit of equation
	* 'P[start, upper limit, equation]'
	* Equation can include the variable 'x'
* Mean: 'M'
	* Computes the mean of entered values
	* 'M[value, value, value, ...]'
* Standard Deviation: 'O'
	* Computes the standard deviation of entered values
	* 'O[value, value, value, ...]'
* Quadratic Formula: 'Q'
	* Computes the quadratic formula results of entered a, b, and c values
	* 'Q[a, b, c]'
	* If two real roots exist, both will be printed to the screen, but only the second root will be treated as the result.
											
### Order of Operations
Rusty Calculator's order of operations is detailed below. Operators in brackets have the same precedence and will be evaluated from left to right.

!, ^, ~ (negation), [R, L, N, H, A], [*, /, %, #, //], [%%, \\], [+, -]

Complex operators are evaluated to numerical values before order of operations are applied. They should be treated as numerical values.
