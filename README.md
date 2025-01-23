# Chocolate
Chocolate is a basic virtual machine designed to be a target for compilation.

## Try it
You can do so at https://bencinn.github.io/chocolate/.
The vm documentation are in chocolate_libvm/vm-documentation, you might be interested in the instruction set.

## VM Executable
Coming Soon™️

## Build it
You can build chocolate_libvm with wasm-pack like this.
```
wasm-pack build --target web ./chocolate_libvm
```
Then open a web server/live preview for chocolate_libvm/index.html.

## Contributing
Please report bugs.
If you want more iset you can send a pull request for the specification or complain in the issues tracker.

## TBD (ordered from most to least important)
- [ ] CLI Executable
- [ ] Interrupts
- [ ] More isets
- [ ] More tests
