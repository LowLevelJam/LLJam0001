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
Loaded PAYLOAD at known position and got jumps working. Next step is to finish DynAsm symbol support.