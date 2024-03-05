# Rusty-Calculator
A terminal-based calculator in Rust

### Background
Rusty Calculator is still a work in progress. I hope to implement all of the functionality soon.

Whenever I need a calculator, I do not have one at my disposal, and the options on my phone are horrible. Sometimes I even despise using my handheld for basic math. I am usually sitting in front of my laptop when I need to do math. Yet, I dislike the calculator options available on my computer too. I need something fast. Something I can use comfortably without a mouse. Something I can customize a little. Rusty Calculator is my solution. 

* Is it impractical?
  	* Not in the slightest, I use it a lot already. 
* Is it too much?
	* Not for me. All of the functions and special characters are used incredibly often in my schoolwork.
* Why is it developed in Rust?
	* I liked the name Rusty Calculator.
	* I wanted to try programming in something I did not understand in the slightest.
* Why is it terminal-based? No GUI?
  	* The web-based calculators on Google are annoying to use because I have to click all over the place.
  	* So many buttons are needed. It is so much easier to type.   
### Commands
* /help: shows commands that eventually display all the info available here
* /reset: resets the calculator to default (all variable values, history, etc)
* /quit: closes the calculator application

### Numbers
* Numbers can be entered as you'd expect: 1, 100039.4589, etc.
* Capital 'E' must be used for scientific notation (e is reserved for Euler's number)

### Basic operators:
* Addition: '+'
	* This also works as the positive operator "4-+5 = -1"
* Subtraction: '-'
	* This also works as the negation operator "4+-5 = -1"
* Multiplication: 'x' or '*' 
	* Yes, they both work, because why not?
 	* Multiplication also works in the following scenarios:
  		* "(2)(4)," "4(2)," "(4)2," "2 4," "4l[10]," "r[5]4," "=4," etc.
		* If 4 is next to anything that is or results in its own distinct number, it is multiplied, because that's how I want it to work.
* Division: '/' or '\\' 
* Modulo: '#'
* Exponent: '^'
	* "5^2+1" = 26, "5^(2+1)" = 125, "-2^2" = -4
* Factorial: '!'
* Percent of: '%'	
	* "x%y" x percent of y
* Absolute Value: | |
	* For embedded absolute values, use parenthesis "|2-700 + (|40/-50|)|"
  
### Special Characters/Variables:
* Pi: 'p'
	* Default: 3.14159265359
* Euler's Number: 'e'
	* Default: 2.71828182845
* Ans: =
 	* Default: 1, unless a previous result exists
	* Works like Ans on a calculator
	* Cannot be redefined
* User Variables: i, j, k, l, m, n, o
	* Default: 1
* Redefine Operator: '_variable[]'
	* "_j[literally any valid equation]" will redefine the value of j
	* "_j[]" resets j to it's default
	* works with p, @, i, j, k, l, m, n, o 
 	* This is standalone:
		* "2+2 _j[4.5]" is invalid
		* "_j[4.5]2+2" will change the value of j and ignore 2+2.
		* "_j[4.5]; 2+2" works just fine

### Advanced Operators/Equations: 
* Start with a capital letter and end with ], containing fields of the form <>, {}, [], [x,y,z]
	* All of these fields support operations embedded within them: "L[R[L[7+2]]]" = whatever log(sqrt(log(9))) evaluates to		
* log: "L{}[]"
	* "L{2}[10]" log base 2 of 10
	* "L[10]" or "L{}[10]" log base 10 of 10
* ln: "N[]"
	* "N[5]" log base e of 5
	* "N{2}[5]" invalid
* root:	"R[]"
	* "R[4]" square root of 4
	* R{3}[4]		(cubed root of 4)
* Pythagorean Theorem: "H{4}[5]"
	* Calculates the hypotenuse length of a triangle with side lengths 4 and 5
 	* Always returns a positive number											
* Average: "A[1,2,3]"
* Standard Deviation: "O[1,2,3]"
* Summation: "S[]"
	* "S[1,2,3]" sum of listed numbers
	* "S&lt;startindex&gt;{upperlimit}[equation using x as variable]" love this one
* Product: "X[]"
	* "X[1,2,3]" product of listed numbers
	* "X&lt;startindex&gt;{upperlimit}[equation using x as variable]" love this one too
 
### Behavior:
* White space should not affect anything...ever! (other than the extremely specific whitespace multiplication instance "4 2." If whitespace was not included here, it would just be the number "42" instead of the product "8." 
* Separated tasks: ';'
	* Multiple equations can be separated by semicolons and computed individually, but displayed together
	* "2+4; =5; =+56;" (last semicolon is optional)
	* Results in: "6, 30, 86"
	* If '=' is used again in the next entry, it only refers to the final answer, which is 86 in this example
* Assumed Parenthesis:
	* Don't want to type unnecessary closing parenthesis? No worries!
	* (2 +5 evaluates to (2+5)
	* (2(5(4+3)-7(4; evaluates to (2(5(4+3)-7(4)));