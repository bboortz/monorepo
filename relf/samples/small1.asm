; copied from https://stackoverflow.com/questions/53382589/smallest-executable-program-x86-64 
bits 64
global _start
_start:
   mov di,42        ; only the low byte of the exit code is kept,
                    ; so we can use di instead of the full edi/rdi
   xor eax,eax
   mov al,60        ; shorter than mov eax,60
   syscall          ; perform the syscall
