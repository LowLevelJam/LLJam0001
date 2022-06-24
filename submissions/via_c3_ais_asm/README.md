# VIA C3 Alternative Instruction Set Assembler

## Team
1. IsaacDynamo

## Introduction

In 2018 [xoreaxeaxeax](https://twitter.com/xoreaxeaxeax) gave the talk [GOD MODE UNLOCKED - Hardware Backdoors in x86 CPUs](https://www.youtube.com/watch?v=_eSAF_qT_FY). In the talk he explains how he found undocumented instructions in VIA C3 processors, that could be used in an exploit.

I was really impressed by the talk, and started to look through my old computer pile to see if I had a VIA C3 system. I did and over the years I messed around with it. Wrote an basic OS, with the idea to do some instruction fuzzing.
This was largely inspired by [orange_slice](https://github.com/gamozolabs/orange_slice), a single purpose Rust OS for fuzzing written by [Brandon Falk](https://twitter.com/gamozolabs).
I never finished the fuzzer, but I did execute some random instruction bits and could find some pattern in instruction bits and their side effects. At the time I, though it would be cool to reverse engineer the whole ISA and write an assembler for it. But I never got far with that project.

I was fascinated by the fact that VIA somewhere must have had documentation and an assembler, or may be even an compiler for this hidden instruction set. The secrecy around this lost obscure technology always kept me wondering what else there was to rediscover. A mystery waiting to be solved.

One month ago I had breakthrough in this mystery. I found some leaked confidential documents. The [VIA C3 Processor Alternative Instruction Set Application Note](http://www.bitsavers.org/components/viaTechnologies/C3-ais-appnote.pdf) and [VIA C3 Processor Alternative Instruction Set Programming Reference](http://www.bitsavers.org/components/viaTechnologies/C3-ais-reference.pdf). It was great to read through these docs, and confirm some of the observations that I had made during my previously reverse engineering attempts.

Armed with this new information an old desire resurfaced, writing an assembler for the VIA C3 Alternative Instruction Set. And get some code running on this alternative instruction set!

## Project

The project contains two Rust programs, `ais_asm` and `kernel`.

The `ais_asm` is the Alternative Instruction Set Assembler. It doesn't parse an input file, but it is dynamic assembler. A program is created with Rust code and calls into the assembler. Currently it will assemble and output the demo program.

The `kernel` is a mostly copied for an previous project of mine, and is changed to contain and start the assembled payload. It is minimal kernel that can be run on VIA C3 hardware. And has a multiboot2 header and can be loaded with GRUB onto a target system. When the kernel is loaded it will initialize as serial port for `println!()` messages. Then try to enable AIS, and panic if the target doesn't support AIS. The kernel image includes a copy of the assembled demo program, and it will run this payload. When the payload is done the result of register EAX is printed over serial.

## Demo

The demo program creates a hexadecimal number by combining multiple nibbles together.

The assembler current only supports jump and some arithmetic instructions, that made it a bit difficult to create a interesting demo. The pseudo code for the demo is something like this.

~~~
int result

int main() {
    result = 0
    push(0xB)
    push(0xA)
    push(0xD)
    push(0xC)
    push(0x0)
    push(0xD)
    push(0xE)
    return result
}

void push(int x) {
    result = result << 4 | x
}
~~~

The assembler can be run with `cd ais_asm; cargo run`

It will that list the generated instructions. Most instruction are ORI or ORIU, because these can be used together with the zero register to load any 32bit value into a register.

~~~
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(16)), rd: None, imm: Some(0), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(11), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(53), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(10), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(89), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(13), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(125), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(12), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(161), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(0), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(197), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(13), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(233), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(18)), rd: None, imm: Some(14), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(19)), rd: None, imm: Some(269), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(19)), rt: Some(Index(19)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(287), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(311), constant: None, offset: None, function: None }
Instruction { opcode: ORIU, rs: Some(Index(4)), rt: Some(Index(4)), rd: None, imm: Some(72), constant: None, offset: None, function: None }
Instruction { opcode: XJ, rs: None, rt: Some(Index(4)), rd: None, imm: None, constant: None, offset: None, function: None }
Instruction { opcode: ORI, rs: Some(Index(0)), rt: Some(Index(4)), rd: None, imm: Some(4), constant: None, offset: None, function: None }
Instruction { opcode: XALUR, rs: Some(Index(16)), rt: Some(Index(4)), rd: Some(Index(16)), imm: None, constant: None, offset: None, function: Some(Xalu(SHL, Word)) }
Instruction { opcode: XALUR, rs: Some(Index(16)), rt: Some(Index(18)), rd: Some(Index(16)), imm: None, constant: None, offset: None, function: Some(Xalu(OR, Word)) }
Instruction { opcode: XJ, rs: None, rt: Some(Index(19)), rd: None, imm: None, constant: None, offset: None, function: None }
~~~

The assembler also outputs regular x86 disassembly of the payload. An this is where it gets interesting.
The first 3 instruction load EAX with the 0xB address, in a position independent manner.
Then at position 0x9 there is a bad instruction. But when AIS is enabled this is a `JMPAI EAX` instruction. It will jump to the EAX, which we loaded with 0xB, and it will start executing the alternative instruction.
All the `bound` instruction are not executed as x86 instruction but as alternative instruction, `AI`.
These `bound` instructions are a wrapper, and the last 32bit contain the actual `AI`.
Finally there is a `ret` instruction that will return from the payload back to the kernel.

~~~
out.bin:     file format binary

Disassembly of section .data:

00000000 <.data>:
   0:   e8 00 00 00 00          call   0x5
   5:   58                      pop    eax
   6:   83 c0 06                add    eax,0x6
   9:   0f 3f                   (bad)
   b:   62 80 00 00 10 34       bound  eax,QWORD PTR [eax+0x34100000]
  11:   62 80 0b 00 12 34       bound  eax,QWORD PTR [eax+0x3412000b]
  17:   62 80 35 00 13 34       bound  eax,QWORD PTR [eax+0x34130035]
  1d:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  23:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  29:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  2f:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  35:   62 80 0a 00 12 34       bound  eax,QWORD PTR [eax+0x3412000a]
  3b:   62 80 59 00 13 34       bound  eax,QWORD PTR [eax+0x34130059]
  41:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  47:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  4d:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  53:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  59:   62 80 0d 00 12 34       bound  eax,QWORD PTR [eax+0x3412000d]
  5f:   62 80 7d 00 13 34       bound  eax,QWORD PTR [eax+0x3413007d]
  65:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  6b:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  71:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  77:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  7d:   62 80 0c 00 12 34       bound  eax,QWORD PTR [eax+0x3412000c]
  83:   62 80 a1 00 13 34       bound  eax,QWORD PTR [eax+0x341300a1]
  89:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  8f:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  95:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  9b:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  a1:   62 80 00 00 12 34       bound  eax,QWORD PTR [eax+0x34120000]
  a7:   62 80 c5 00 13 34       bound  eax,QWORD PTR [eax+0x341300c5]
  ad:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  b3:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  b9:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  bf:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  c5:   62 80 0d 00 12 34       bound  eax,QWORD PTR [eax+0x3412000d]
  cb:   62 80 e9 00 13 34       bound  eax,QWORD PTR [eax+0x341300e9]
  d1:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  d7:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
  dd:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
  e3:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
  e9:   62 80 0e 00 12 34       bound  eax,QWORD PTR [eax+0x3412000e]
  ef:   62 80 0d 01 13 34       bound  eax,QWORD PTR [eax+0x3413010d]
  f5:   62 80 48 00 73 22       bound  eax,QWORD PTR [eax+0x22730048]
  fb:   62 80 1f 01 04 34       bound  eax,QWORD PTR [eax+0x3404011f]
 101:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
 107:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
 10d:   62 80 37 01 04 34       bound  eax,QWORD PTR [eax+0x34040137]
 113:   62 80 48 00 84 20       bound  eax,QWORD PTR [eax+0x20840048]
 119:   62 80 44 00 04 18       bound  eax,QWORD PTR [eax+0x18040044]
 11f:   62 80 04 00 04 34       bound  eax,QWORD PTR [eax+0x34040004]
 125:   62 80 00 80 04 8a       bound  eax,QWORD PTR [eax-0x75fb8000]
 12b:   62 80 15 80 12 8a       bound  eax,QWORD PTR [eax-0x75ed7feb]
 131:   62 80 44 00 13 18       bound  eax,QWORD PTR [eax+0x18130044]
 137:   c3                      ret
~~~

Running `ais_asm` created `out.bin`. This payload can now be combined with the kernel.

Run `cd ../kernel; cargo build` to build the kernel. The kernel just `core::include_bytes!()` the `out.bin`,  so the kernel should be rebuild when `out.bin` has changed.

When building is done there will be a multiboot elf file in `target/viac3-unknown-none/debug/kernel`.

This elf file can be run with QEMU, but it will panic, because it will be missing AIS support. Run `make` to start QEMU with the kernel. Control-a x to exit QEMU. You will need `qemu-system-i386` and `grub-rescue`.
~~~
Kernel started
panicked at 'AIS is not supported or not enabled', src/main.rs:33:5
~~~

To really run the payload you will need real hardware that support the AIS. To make things easy I will just show the serial output of the kernel after it ran the payload.
~~~
Kernel started
AIS is supported and has been enabled
Run payload at 0x00480000
Result EAX = 0x0BADC0DE
Done
~~~

The result of the demo is the value `0x0BADC0DE` that was created by the sequence of pushes.

## Improvements
Currently the assembler only support for small number of instruction, but a lot more are documented. So these can be added.
I did add support for XIOR and XIOW, with the idea to use them to write to the serial port but I never got them working on real hardware, this need some debugging.
The AIS doens't have documented support for conditional jumps but I expect that these still implemented, so maybe they can be found by fuzzing the hardware.

## Extra info
Xoreaxeaxeax notes on AIS can by found in the [rosenbridge](https://github.com/xoreaxeaxeax/rosenbridge) repo.

The dynamic assembler design is from the youtube series [Bitwise](https://www.youtube.com/user/pervognsen), where one of the projects is a [RISCV assembler](https://github.com/pervognsen/bitwise/tree/master/ion/riscv).
Unfortunately the series ended abruptly, but still there is a lot of good content.

## Log
### Day 1: 0.5h
Watched kickoff video and read theme, and browsed the discord. Try the figure out if my initial pre-kickoff idea fitted the theme.

### Day 2: 14h
Decided that I would work on my original idea, because it was something that was on my project todo list for quite some time, and this Jam was good motivation to get started.
First coding session. Setup ASM encoder and decoder, and a very basic assembler. Had a lot of fun, and time flew by. Continued way to long into the night.

### Day 3: 2h
Kind of burned out by yesterdays long session, and tired by short night. Also has some other IRL stuff, so no second long session.
Cleaned up yesterdays code and started with log.

### Day 4: 4h
Added the intro.
Copied pieces of an old kernel, this will only be used as loader for the AIS payload.
Tested kernel with QEMU but AIS will not run under QEMU, so next step is to move to real hardware.

### Day 5: 4h
Dusted of test hardware and worked on getting code to run on it. After fixing some issues, I could run my addition example on VIA C3 with alternative instructions.
Next step will be to add more support to the assembler and write a nice demo.

### Day 6: 4h
Worked on DynAsm symbol support, but could not really find a good use for it yet. AIS doesn't have conditional jumps, at least not according the leaked documents. And the unconditional jump is absolute, but I want/need my payload codegen to be position independent. And getting instruction pointer with AIS is done via XPUSHIP so I will need to look into that. There also no native call instruction, I think you need to perform a XPUSHIP and jump (XJ).

### Day 7: 5h
Tried to get IO port writes to work so it is possible to send bytes from within AIS code. Couldn't get that to work. Still some time left tomorrow, but will need to finish the assembler, write some demo with stuff that does work. And looking into how submit the project.

### Day 8: TBD, but < 10.5h until deadline
Payload is now loaded at known position and got jumps working. Next step is to finish DynAsm symbol support.
DynAsm symbol support work. Only jump and some arithmetic instruction are supported by the assembler for now, how do I make an interesting demo with that?
Made a somewhat interesting demo that implements call and ret via a return register.
Add stuff to readme.
