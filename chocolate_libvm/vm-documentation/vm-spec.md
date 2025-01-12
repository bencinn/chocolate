# Chocolate vm spec
(Almost) everything is currently subjected to change.

## Status
Status indicate the status of instruction after the last interrupt. If it is normal it means all instruction after the last interrupt are fine. If it is stuck at interrupt it means the last interrupt is invalid. If it is undefined it means the last instruction you ran is invalid.

## Register
- 16 8-bit register
- first two are reserved for program counter/stack pointer.

## Stack
- 128 in size
- have stack pointer (1-based)