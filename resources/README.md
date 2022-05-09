# New Player Resources

<!-- TOC -->

- [New Player Resources](#new-player-resources)
	- [What is a ...](#what-is-a-)
		- [virtual machine?](#virtual-machine)
		- [emulator?](#emulator)
	- [Tutorials](#tutorials)
		- [Chip Emulators](#chip-emulators)
		- [Application VMs](#application-vms)
		- [Miscellaneous](#miscellaneous)
		- [LLJS Tutorials](#lljs-tutorials)

<!-- /TOC -->

## What is a ... 
### virtual machine?
A virtual machine is an isolated analog of a computer. Virtual machines generally exist at the

- system level, where a physical machine is simulated. This can be done for density, isolation, testing and development or for software compatibility reasons.
- application level, where a process is run in a constrained runtime that provides specific capabilities or protections while hiding the details of an underlying machine. These often exist for security or portability reasons. An example being the JVM, wasm or even some regex runtimes.

### emulator?
Emulators are an extention of virtual machines allowing a user to recreate the properties of a machine, in software. Examples include, running software for a different architecture than a host machine, virtual consoles, device simulators and many more.


## Tutorials
### Chip Emulators
- [Emulator 101](http://www.emulator101.com/welcome.html) - Tutorials for full toolchains of many chipsets both real and virtual.
- [Easy 6502](https://skilldrick.github.io/easy6502/)
- [Writing a CHIP-8 Emulator](http://craigthomas.ca/tag/chip8.html) - A simple virtual console.
- [Writing your own VM](https://justinmeiners.github.io/lc3-vm/) - A much more in depth tutorial with many alternate languages implementations.
- [Design your own CPU Instruction Set Video Series](https://www.youtube.com/playlist?list=PLxLxbi4e2mYGvzNw2RzIsM_rxnNC8m2Kz) - A Gary Explains video series covering briefly how CPUs work, and walking through designing and emulating an instruction set. Note: Videos appear slightly out of order.
- [UXN](https://100r.co/site/uxn.html) - A wonderful virtual machine.
  - [Awesome UXN](https://github.com/hundredrabbits/awesome-uxn) - collection of software for UXN.
### Application VMs
- [LLJS 16-bit VM series](https://www.youtube.com/playlist?list=PLP29wDx6QmW5DdwpdwHCRJsEubS5NrQ9b) - about 6 hours of content
- [Writing a simple vm in less than 125 lines of C](https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c)
- [Crafting Interpreters](https://craftinginterpreters.com/a-virtual-machine.html) - A (optionally) free book culminating in a language with a runtime VM. This is a large amount of work but is both newbie friendly and gives a full-picture.
### Miscellaneous
- [Building a retro game for PICO-8](https://thenewstack.io/retro-game-pico-8-basics/)
- [Virtual6502](http://visual6502.org/JSSim/index.html) - full visual chip simulation of a MOS6502 chip.
- [Stack Computers: the new wave](https://users.ece.cmu.edu/~koopman/stack_computers/index.html) - A, slightly dated, e-book that describes stack machines with _FANTASTIC_ diagrams.

### LLJS Tutorials
- [The Bits and Bytes of Binary](https://www.youtube.com/playlist?list=PLP29wDx6QmW47oPsNBFNEi_SYTOLDJXqQ) - Entry-level friendly introductions to binary and common binary operations.
- [16-bit VM series](https://www.youtube.com/playlist?list=PLP29wDx6QmW5DdwpdwHCRJsEubS5NrQ9b) - about 6 hours of content
 