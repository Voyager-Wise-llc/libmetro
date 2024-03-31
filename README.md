libmetro
========

This library can read and write Metrowerks Library files for  MacOS (System Software)
as of Code Warrior 6.

Most of the features documented in the CodeWarrior API manual will eventually be 
supported here.

Hopefully this will be used to translate assembled code in ELF format from more modern 
compilers into something CodeWarrior can work with natively, or converting into ELF to
aid dissasembly in a tool like Ghidra or plain-ol' binutils.

------

Initial research showed that for a pretty simple function, gcc is capable of producing
the same bytecode as CodeWarrior 6's C compiler. This is not that impressive, but a
start of a way to maybe bring newer compilers or alternative languages into the platform.

This would be useful in backporting built-in compiler features such as better 
memory-safety, mitigations like stack-canaries, RAII, and all the things gcc can do now.

A future might be a CW compiler plugin to utilize a 'sidecar' like a RaspPi to peform
compilation and translating ELF <-> MWOB. I think that'd be pretty neat.

```
int add(int a, int b) {
    return (a+b);
}
```

```
m68k-suse-linux-gcc -std=c99 -mc68000 \
-nostdlib -nodefaultlibs -nolibc \
-fomit-frame-pointer \
-fverbose-asm -Wa,-adhln=add.s \
-c add.c -o add.o
```

This produces a compiler object in ELF format which, of course, CodeWarrior can't read.