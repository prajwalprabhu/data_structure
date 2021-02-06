# data_structure
Program to Convert Infix expression into Postfix and then to slove it

Infix expression: If an operator is in between two operands it is called infix
				expression.
				
				Example: a + b,		 where a and b are the operands and + is an operator.
				
				
Postfix expression: 
				If an operator follows the two operands it is called post fix expression.
				
				Example: ab +
			
			
Algorithm for Infix to Postfix
1. Examine the next element in the input.
2. If it is operand, output it.
3. If it is opening parenthesis, push it on stack.
4. If it is an operator, then
   *If stack is empty, push operator on stack.
   *If the top of stack is opening parenthesis, push operator on stack
   *If it has higher priority than the top of stack, push operator on
	stack.
   *Else pop the operator from the stack and output it, repeat step 4.
5. If it is a closing parenthesis, pop operators from stack and output them
   until an opening parenthesis is encountered. Pop and discard the opening
   parenthesis.
6. If there is more input go to step 1.
7. If there is no more input, pop the remaining operators to output.




#Evaluating a postfix expression using stack

* The postfix expression to be evaluated is scanned from left to right.
* Each operator in a postfix string refers to the previous two operands
  in the string.
* Each time we read an operand we push it into a stack. When we
  reach an operator, its operands will then be top two elements on
  the stack.
* We can then pop these two elements, perform the indicated operation
  on them, and push the result on the stack.
* So that it will be available for use as an operand of the next operator.
* Initialize an empty stack.
* While character remain in the input stream
* Read next character.
* If character is an operand, push it into the stack.
* Else, if character is an operator, pop top two characters off the
  stack, apply the operator, and push the answer back into the
  stack.
* Pop the answer off the stack.
