## Progress

 - [X] decoding instructions
 - [X] decoding instruction parameters
 - [ ] executing instructions
 - [ ] control flow instructions
 - [ ] stack instructions
 - [ ] integer arithmetic instructions
 - [ ] bitwise logic instructions
 - [ ] emulating I/O
    - [ ] I/O instructions
 - [ ] floating point instructions
 - [ ] deciding on an user interface
 - [ ] interrupts
 - [ ] timing 


| OPCODE | Instruction | Implemented | Tested |
|:-------|:------------|-------------|--------|
| 0x00   | ADD         | [ ]         | [ ]    |
| 0x01   | ADD         | [ ]         | [ ]    |
| 0x02   | ADD         | [ ]         | [ ]    |
| 0x03   | ADD         | [ ]         | [ ]    |
| 0x04   | ADD         | [ ]         | [ ]    |
| 0x05   | ADD         | [ ]         | [ ]    |
| 0x06   | PUSH        | [ ]         | [ ]    |
| 0x07   | POP         | [ ]         | [ ]    |
| 0x08   | OR          | [ ]         | [ ]    |
| 0x09   | OR          | [ ]         | [ ]    |
| 0x0A   | OR          | [ ]         | [ ]    |
| 0x0B   | OR          | [ ]         | [ ]    |
| 0x0C   | OR          | [ ]         | [ ]    |
| 0x0D   | OR          | [ ]         | [ ]    |
| 0x0E   | PUSH        | [ ]         | [ ]    |
| 0x0F   | --          | [ ]         | [ ]    |
| 0x10   | ADC         | [ ]         | [ ]    |
| 0x11   | ADC         | [ ]         | [ ]    |
| 0x12   | ADC         | [ ]         | [ ]    |
| 0x13   | ADC         | [ ]         | [ ]    |
| 0x14   | ADC         | [ ]         | [ ]    |
| 0x15   | ADC         | [ ]         | [ ]    |
| 0x16   | PUSH        | [ ]         | [ ]    |
| 0x17   | POP         | [ ]         | [ ]    |
| 0x18   | SBB         | [ ]         | [ ]    |
| 0x19   | SBB         | [ ]         | [ ]    |
| 0x1A   | SBB         | [ ]         | [ ]    |
| 0x1B   | SBB         | [ ]         | [ ]    |
| 0x1C   | SBB         | [ ]         | [ ]    |
| 0x1D   | SBB         | [ ]         | [ ]    |
| 0x1E   | PUSH        | [ ]         | [ ]    |
| 0x1F   | POP         | [ ]         | [ ]    |
| 0x20   | AND         | [ ]         | [ ]    |
| 0x21   | AND         | [ ]         | [ ]    |
| 0x22   | AND         | [ ]         | [ ]    |
| 0x23   | AND         | [ ]         | [ ]    |
| 0x24   | AND         | [ ]         | [ ]    |
| 0x25   | AND         | [ ]         | [ ]    |
| 0x26   | ES          | [ ]         | [ ]    |
| 0x27   | DAA         | [ ]         | [ ]    |
| 0x28   | SUB         | [ ]         | [ ]    |
| 0x29   | SUB         | [ ]         | [ ]    |
| 0x2A   | SUB         | [ ]         | [ ]    |
| 0x2B   | SUB         | [ ]         | [ ]    |
| 0x2C   | SUB         | [ ]         | [ ]    |
| 0x2D   | SUB         | [ ]         | [ ]    |
| 0x2E   | CS          | [ ]         | [ ]    |
| 0x2F   | DAS         | [ ]         | [ ]    |
| 0x30   | XOR         | [ ]         | [ ]    |
| 0x31   | XOR         | [ ]         | [ ]    |
| 0x32   | XOR         | [ ]         | [ ]    |
| 0x33   | XOR         | [ ]         | [ ]    |
| 0x34   | XOR         | [ ]         | [ ]    |
| 0x35   | XOR         | [ ]         | [ ]    |
| 0x36   | SS          | [ ]         | [ ]    |
| 0x37   | AAA         | [ ]         | [ ]    |
| 0x38   | CMP         | [ ]         | [ ]    |
| 0x39   | CMP         | [ ]         | [ ]    |
| 0x3A   | CMP         | [ ]         | [ ]    |
| 0x3B   | CMP         | [ ]         | [ ]    |
| 0x3C   | CMP         | [ ]         | [ ]    |
| 0x3D   | CMP         | [ ]         | [ ]    |
| 0x3E   | DS          | [ ]         | [ ]    |
| 0x3F   | AAS         | [ ]         | [ ]    |
| 0x40   | INC         | [ ]         | [ ]    |
| 0x41   | INC         | [ ]         | [ ]    |
| 0x42   | INC         | [ ]         | [ ]    |
| 0x43   | INC         | [ ]         | [ ]    |
| 0x44   | INC         | [ ]         | [ ]    |
| 0x45   | INC         | [ ]         | [ ]    |
| 0x46   | INC         | [ ]         | [ ]    |
| 0x47   | INC         | [ ]         | [ ]    |
| 0x48   | DEC         | [ ]         | [ ]    |
| 0x49   | DEC         | [ ]         | [ ]    |
| 0x4A   | DEC         | [ ]         | [ ]    |
| 0x4B   | DEC         | [ ]         | [ ]    |
| 0x4C   | DEC         | [ ]         | [ ]    |
| 0x4D   | DEC         | [ ]         | [ ]    |
| 0x4E   | DEC         | [ ]         | [ ]    |
| 0x4F   | DEC         | [ ]         | [ ]    |
| 0x50   | PUSH        | [ ]         | [ ]    |
| 0x51   | PUSH        | [ ]         | [ ]    |
| 0x52   | PUSH        | [ ]         | [ ]    |
| 0x53   | PUSH        | [ ]         | [ ]    |
| 0x54   | PUSH        | [ ]         | [ ]    |
| 0x55   | PUSH        | [ ]         | [ ]    |
| 0x56   | PUSH        | [ ]         | [ ]    |
| 0x57   | PUSH        | [ ]         | [ ]    |
| 0x58   | POP         | [ ]         | [ ]    |
| 0x59   | POP         | [ ]         | [ ]    |
| 0x5A   | POP         | [ ]         | [ ]    |
| 0x5B   | POP         | [ ]         | [ ]    |
| 0x5C   | POP         | [ ]         | [ ]    |
| 0x5D   | POP         | [ ]         | [ ]    |
| 0x5E   | POP         | [ ]         | [ ]    |
| 0x5F   | POP         | [ ]         | [ ]    |
| 0x60   | --          | [ ]         | [ ]    |
| 0x61   | --          | [ ]         | [ ]    |
| 0x62   | --          | [ ]         | [ ]    |
| 0x63   | --          | [ ]         | [ ]    |
| 0x64   | --          | [ ]         | [ ]    |
| 0x65   | --          | [ ]         | [ ]    |
| 0x66   | --          | [ ]         | [ ]    |
| 0x67   | --          | [ ]         | [ ]    |
| 0x68   | --          | [ ]         | [ ]    |
| 0x69   | --          | [ ]         | [ ]    |
| 0x6A   | --          | [ ]         | [ ]    |
| 0x6B   | --          | [ ]         | [ ]    |
| 0x6C   | --          | [ ]         | [ ]    |
| 0x6D   | --          | [ ]         | [ ]    |
| 0x6E   | --          | [ ]         | [ ]    |
| 0x6F   | --          | [ ]         | [ ]    |
| 0x70   | JO          | [ ]         | [ ]    |
| 0x71   | JNO         | [ ]         | [ ]    |
| 0x72   | JB          | [ ]         | [ ]    |
| 0x73   | JNB         | [ ]         | [ ]    |
| 0x74   | JZ          | [ ]         | [ ]    |
| 0x75   | JNZ         | [ ]         | [ ]    |
| 0x76   | JBE         | [ ]         | [ ]    |
| 0x77   | JA          | [ ]         | [ ]    |
| 0x78   | JS          | [ ]         | [ ]    |
| 0x79   | JNS         | [ ]         | [ ]    |
| 0x7A   | JPE         | [ ]         | [ ]    |
| 0x7B   | JPO         | [ ]         | [ ]    |
| 0x7C   | JL          | [ ]         | [ ]    |
| 0x7D   | JGE         | [ ]         | [ ]    |
| 0x7E   | JLE         | [ ]         | [ ]    |
| 0x7F   | JG          | [ ]         | [ ]    |
| 0x80   | GRP1        | [ ]         | [ ]    |
| 0x81   | GRP1        | [ ]         | [ ]    |
| 0x82   | GRP1        | [ ]         | [ ]    |
| 0x83   | GRP1        | [ ]         | [ ]    |
| 0x84   | TEST        | [ ]         | [ ]    |
| 0x85   | TEST        | [ ]         | [ ]    |
| 0x86   | XCHG        | [ ]         | [ ]    |
| 0x87   | XCHG        | [ ]         | [ ]    |
| 0x88   | MOV         | [ ]         | [ ]    |
| 0x89   | MOV         | [ ]         | [ ]    |
| 0x8A   | MOV         | [ ]         | [ ]    |
| 0x8B   | MOV         | [ ]         | [ ]    |
| 0x8C   | MOV         | [ ]         | [ ]    |
| 0x8D   | LEA         | [ ]         | [ ]    |
| 0x8E   | MOV         | [ ]         | [ ]    |
| 0x8F   | POP         | [ ]         | [ ]    |
| 0x90   | NOP         | [ ]         | [ ]    |
| 0x91   | XCHG        | [ ]         | [ ]    |
| 0x92   | XCHG        | [ ]         | [ ]    |
| 0x93   | XCHG        | [ ]         | [ ]    |
| 0x94   | XCHG        | [ ]         | [ ]    |
| 0x95   | XCHG        | [ ]         | [ ]    |
| 0x96   | XCHG        | [ ]         | [ ]    |
| 0x97   | XCHG        | [ ]         | [ ]    |
| 0x98   | CBW         | [ ]         | [ ]    |
| 0x99   | CWD         | [ ]         | [ ]    |
| 0x9A   | CALL        | [ ]         | [ ]    |
| 0x9B   | WAIT        | [ ]         | [ ]    |
| 0x9C   | PUSHF       | [ ]         | [ ]    |
| 0x9D   | POPF        | [ ]         | [ ]    |
| 0x9E   | SAHF        | [ ]         | [ ]    |
| 0x9F   | LAHF        | [ ]         | [ ]    |
| 0xA0   | MOV         | [ ]         | [ ]    |
| 0xA1   | MOV         | [ ]         | [ ]    |
| 0xA2   | MOV         | [ ]         | [ ]    |
| 0xA3   | MOV         | [ ]         | [ ]    |
| 0xA4   | MOVSB       | [ ]         | [ ]    |
| 0xA5   | MOVSW       | [ ]         | [ ]    |
| 0xA6   | CMPSB       | [ ]         | [ ]    |
| 0xA7   | CMPSW       | [ ]         | [ ]    |
| 0xA8   | TEST        | [ ]         | [ ]    |
| 0xA9   | TEST        | [ ]         | [ ]    |
| 0xAA   | STOSB       | [ ]         | [ ]    |
| 0xAB   | STOSW       | [ ]         | [ ]    |
| 0xAC   | LODSB       | [ ]         | [ ]    |
| 0xAD   | LODSW       | [ ]         | [ ]    |
| 0xAE   | SCASB       | [ ]         | [ ]    |
| 0xAF   | SCASW       | [ ]         | [ ]    |
| 0xB0   | MOV         | [X]         | [ ]    |
| 0xB1   | MOV         | [X]         | [ ]    |
| 0xB2   | MOV         | [X]         | [ ]    |
| 0xB3   | MOV         | [X]         | [ ]    |
| 0xB4   | MOV         | [X]         | [ ]    |
| 0xB5   | MOV         | [X]         | [ ]    |
| 0xB6   | MOV         | [X]         | [ ]    |
| 0xB7   | MOV         | [X]         | [ ]    |
| 0xB8   | MOV         | [X]         | [ ]    |
| 0xB9   | MOV         | [X]         | [ ]    |
| 0xBA   | MOV         | [ ]         | [ ]    |
| 0xBB   | MOV         | [ ]         | [ ]    |
| 0xBC   | MOV         | [ ]         | [ ]    |
| 0xBD   | MOV         | [ ]         | [ ]    |
| 0xBE   | MOV         | [ ]         | [ ]    |
| 0xBF   | MOV         | [ ]         | [ ]    |
| 0xC0   | --          | [ ]         | [ ]    |
| 0xC1   | --          | [ ]         | [ ]    |
| 0xC2   | RET         | [ ]         | [ ]    |
| 0xC3   | RET         | [ ]         | [ ]    |
| 0xC4   | LES         | [ ]         | [ ]    |
| 0xC5   | LDS         | [ ]         | [ ]    |
| 0xC6   | MOV         | [ ]         | [ ]    |
| 0xC7   | MOV         | [ ]         | [ ]    |
| 0xC8   | --          | [ ]         | [ ]    |
| 0xC9   | --          | [ ]         | [ ]    |
| 0xCA   | RETF        | [ ]         | [ ]    |
| 0xCB   | RETF        | [ ]         | [ ]    |
| 0xCC   | INT         | [ ]         | [ ]    |
| 0xCD   | INT         | [ ]         | [ ]    |
| 0xCE   | INTO        | [ ]         | [ ]    |
| 0xCF   | IRET        | [ ]         | [ ]    |
| 0xD0   | GRP2        | [ ]         | [ ]    |
| 0xD1   | GRP2        | [ ]         | [ ]    |
| 0xD2   | GRP2        | [ ]         | [ ]    |
| 0xD3   | GRP2        | [ ]         | [ ]    |
| 0xD4   | AAM         | [ ]         | [ ]    |
| 0xD5   | AAD         | [ ]         | [ ]    |
| 0xD6   | --          | [ ]         | [ ]    |
| 0xD7   | XLAT        | [ ]         | [ ]    |
| 0xD8   | --          | [ ]         | [ ]    |
| 0xD9   | --          | [ ]         | [ ]    |
| 0xDA   | --          | [ ]         | [ ]    |
| 0xDB   | --          | [ ]         | [ ]    |
| 0xDC   | --          | [ ]         | [ ]    |
| 0xDD   | --          | [ ]         | [ ]    |
| 0xDE   | --          | [ ]         | [ ]    |
| 0xDF   | --          | [ ]         | [ ]    |
| 0xE0   | LOOPNZ      | [ ]         | [ ]    |
| 0xE1   | LOOPZ       | [ ]         | [ ]    |
| 0xE2   | LOOP        | [ ]         | [ ]    |
| 0xE3   | JCXZ        | [ ]         | [ ]    |
| 0xE4   | IN          | [ ]         | [ ]    |
| 0xE5   | IN          | [ ]         | [ ]    |
| 0xE6   | OUT         | [ ]         | [ ]    |
| 0xE7   | OUT         | [ ]         | [ ]    |
| 0xE8   | CALL        | [ ]         | [ ]    |
| 0xE9   | JMP         | [ ]         | [ ]    |
| 0xEA   | JMP         | [ ]         | [ ]    |
| 0xEB   | JMP         | [ ]         | [ ]    |
| 0xEC   | IN          | [ ]         | [ ]    |
| 0xED   | IN          | [ ]         | [ ]    |
| 0xEE   | OUT         | [ ]         | [ ]    |
| 0xEF   | OUT         | [ ]         | [ ]    |
| 0xF0   | LOCK        | [ ]         | [ ]    |
| 0xF1   | --          | [ ]         | [ ]    |
| 0xF2   | REPNZ       | [ ]         | [ ]    |
| 0xF3   | REPZ        | [ ]         | [ ]    |
| 0xF4   | HLT         | [ ]         | [ ]    |
| 0xF5   | CMC         | [ ]         | [ ]    |
| 0xF6   | GRP3a       | [ ]         | [ ]    |
| 0xF7   | GRP3b       | [ ]         | [ ]    |
| 0xF8   | CLC         | [ ]         | [ ]    |
| 0xF9   | STC         | [ ]         | [ ]    |
| 0xFA   | CLI         | [ ]         | [ ]    |
| 0xFB   | STI         | [ ]         | [ ]    |
| 0xFC   | CLD         | [ ]         | [ ]    |
| 0xFD   | STD         | [ ]         | [ ]    |
| 0xFE   | GRP4        | [ ]         | [ ]    |
| 0xFF   | GRP5        | [ ]         | [ ]    |

![Opcode chart](https://pnx.tf/files/x86_opcode_structure_and_instruction_overview.png)
