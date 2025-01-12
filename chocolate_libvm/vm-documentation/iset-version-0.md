# Chocolate vm instruction set ver 0

## Flags
- 0 flag: equal
- 1 flag: less than
- 2 flag: more than
- 3 flag: overflow

## Basic
- (0) Halt

## Registers
8-bit registers are counted from 0 (in the iset we refer to i8-0 as the 0th 8-bit register)
Note: If you try to access register which does not exist, the vm will perform a remainder operation and use that register instead.
- (1) Mov p1 p2
Copy value from p2 to p1
- (2) AddR p1 p2
Increment p1 register by register p2
- (3) SubR p1 p2
Subtract p1 register by register p2
- (4) Add p1 p2
Increment p1 register by p2
- (5) Sub p1 p2
Subtract p1 register by p2

## Stack
- (6) Read p1
Read and copy the value to i8-$p1
- (7) Push p1
Push the value and increment the stack pointer. If the stack is empty this is undefined
- (8) Pop p1
Copy the stack head value to i8-$p1. If the stack is empty this is undefined

## Interrupt
Interrupt here works a bit different to others. The vm itself handle the interrupt. There are only one command: (9) Int p1, which will generate an interrupt. Interrupt can't modify the vm's data. If the interrupt doesn't exist, nothing is done. Here are all the possible interrupt.
- 0: stop the machine

## Control Flows
Jump in this cases means setting the next instruction to be the value
- (10) Cmp p1 p2
Compare value in register p1 and register p2 and set vm flags according to the result
- (11) Jmp p1
Jump to address in register p1 (specifically, set the next inst pointer to p1)
- (12) Je p1
Jump to address in register p1 if equal
