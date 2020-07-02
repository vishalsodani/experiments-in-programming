I wondered why gcc first converts into assembly and then object file of machine code

https://cs.stackexchange.com/a/14752

1. The symbolic addresses used by assemblers instead of hard-coding machine addresses make code relocation much easier.

2. Linking code may involve safety checks such as type-checking, and that's easier to do with symbolic names.


STATIC LINKING
====================

all the required code is embedded in the binary a.out like printf function. I pushed to my server and ran a.out an dit works!

so, a.out is complete and self-contained


DYNAMIC LINKING
===================

1. a.out will have directive about so(shared libraries)
2. and loads at runtime


