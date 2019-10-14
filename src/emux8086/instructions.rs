fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 00;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 01;
 // Args: mem16,reg16
    // Args: mem32,reg32
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 02;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 02;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 03;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 03;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 04;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 05;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 06;
 // Args: ES
 // Flags: none
 // Bytes: 1
computer }

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 07;
 // Args: ES
 // Flags: none
 // Bytes: 1
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 08;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 09;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 0A;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 0A;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 0B;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 0B;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="757"></a><a name="page470"></a>0C</p>

</td><td class="td" align="left">

<p class="table-para">or</p>

</td><td class="td" align="left">

<p class="table-para">AL,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 0D;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 0E;
 // Args: CS
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnae</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jna</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnbe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jpe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jpo</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnge</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="758"></a><a name="page471"></a>0F 8D</p>

</td><td class="td" align="left">

<p class="table-para">jge</p>

</td><td class="td" align="left">

<p class="table-para">rel32</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">7+,3</p>

</td><td class="td" align="left">

<p class="table-para">3,1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">6</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnl</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jng</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnle</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">OF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">9-38</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">12-41</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">10</p>

</td><td class="td" align="left">

<p class="table-para">adc</p>

</td><td class="td" align="left">

<p class="table-para">mem8,reg8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">7</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td><td class="td" align="left">

<p class="table-para">3<a name="759"></a><a name="page472"></a>

</p>

</td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 11;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 12;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 12;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 13;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 13;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 14;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 15;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 16;
 // Args: SS
 // Flags: none
 // Bytes: 1
computer }

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 17;
 // Args: SS
 // Flags: none
 // Bytes: 1
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 18;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 19;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1A;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1A;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1B;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1B;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1C;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 1D;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 1E;
 // Args: DS
 // Flags: none
 // Bytes: 1
computer }

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 1F;
 // Args: DS
 // Flags: none
 // Bytes: 1
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 20;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 21;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 22;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 22;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 23;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="760"></a><a name="page473"></a>23</p>

</td><td class="td" align="left">

<p class="table-para">and</p>

</td><td class="td" align="left">

<p class="table-para">reg16,mem16</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 24;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 25;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_daa (&mut Computer) -> &mut Computer {
let opcode = 27;
 // Args: none
 // Flags: SF,ZF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">OF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 28;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 29;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2A;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2A;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2B;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2B;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2C;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 2D;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_das (&mut Computer) -> &mut Computer {
let opcode = 2F;
 // Args: none
 // Flags: SF,ZF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">OF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 30;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 31;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 32;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 32;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 33;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 33;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 34;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 35;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_aaa (&mut Computer) -> &mut Computer {
let opcode = 37;
 // Args: none
 // Flags: AF,CF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,PF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="761"></a><a name="page474"></a>38</p>

</td><td class="td" align="left">

<p class="table-para">cmp</p>

</td><td class="td" align="left">

<p class="table-para">reg8,reg8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 38;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 39;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 3A;
 // Args: reg8,mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 3B;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 3B;
 // Args: reg16,mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 3C;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 3D;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_aas (&mut Computer) -> &mut Computer {
let opcode = 3F;
 // Args: none
 // Flags: AF,CF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,PF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 40;
 // Args: AX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 41;
 // Args: CX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ECX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 42;
 // Args: DX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 43;
 // Args: BX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 44;
 // Args: SP
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 45;
 // Args: BP
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 47;
 // Args: SI
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 48;
 // Args: AX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = 48;
 // Args: DI
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 49;
 // Args: CX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ECX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 4A;
 // Args: DX
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="762"></a><a name="page475"></a>4B</p>

</td><td class="td" align="left">

<p class="table-para">dec</p>

</td><td class="td" align="left">

<p class="table-para">BX</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 4C;
 // Args: SP
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 4D;
 // Args: BP
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 4E;
 // Args: SI
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = 4F;
 // Args: DI
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 50;
 // Args: AX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 51;
 // Args: CX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ECX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 52;
 // Args: DX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 53;
 // Args: BX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 54;
 // Args: SP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 55;
 // Args: BP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 56;
 // Args: SI
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 57;
 // Args: DI
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 58;
 // Args: AX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 59;
 // Args: CX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ECX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 5A;
 // Args: DX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 5B;
 // Args: BX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 5C;
 // Args: SP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 5D;
 // Args: BP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="763"></a><a name="page476"></a>5E</p>

</td><td class="td" align="left">

<p class="table-para">pop</p>

</td><td class="td" align="left">

<p class="table-para">SI</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 5F;
 // Args: DI
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_pusha (&mut Computer) -> &mut Computer {
let opcode = 60;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">pushad</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_popa (&mut Computer) -> &mut Computer {
let opcode = 61;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">popad</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 68;
 // Args: imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = 69;
 // Args: reg16,reg16,imm16
 // Flags: OF,CF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,imm32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left">

<p class="table-para">9-38</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = 69;
 // Args: reg16,mem16,imm16
 // Flags: OF,CF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32,imm32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left">

<p class="table-para">12-41</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = 6A;
 // Args: imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = 6B;
 // Args: reg16,imm8
 // Flags: OF,CF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = 6B;
 // Args: reg16,reg16,imm8
 // Flags: OF,CF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = 6B;
 // Args: reg16,mem16,imm8
 // Flags: OF,CF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jo (&mut Computer) -> &mut Computer {
let opcode = 70;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

fn instruction_jno (&mut Computer) -> &mut Computer {
let opcode = 71;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

fn instruction_jb (&mut Computer) -> &mut Computer {
let opcode = 72;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnae</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jc (&mut Computer) -> &mut Computer {
let opcode = 72;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

fn instruction_jae (&mut Computer) -> &mut Computer {
let opcode = 73;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jnc (&mut Computer) -> &mut Computer {
let opcode = 73;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

fn instruction_je (&mut Computer) -> &mut Computer {
let opcode = 74;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jne (&mut Computer) -> &mut Computer {
let opcode = 75;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jbe (&mut Computer) -> &mut Computer {
let opcode = 76;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jna</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_ja (&mut Computer) -> &mut Computer {
let opcode = 77;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnbe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="764"></a><a name="page477"></a>78</p>

</td><td class="td" align="left">

<p class="table-para">js</p>

</td><td class="td" align="left">

<p class="table-para">rel8</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">7+,3</p>

</td><td class="td" align="left">

<p class="table-para">3,1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td>

</tr>

fn instruction_jns (&mut Computer) -> &mut Computer {
let opcode = 79;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

fn instruction_jp (&mut Computer) -> &mut Computer {
let opcode = 7A;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jpe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jnp (&mut Computer) -> &mut Computer {
let opcode = 7B;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jpo</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jl (&mut Computer) -> &mut Computer {
let opcode = 7C;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnge</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jge (&mut Computer) -> &mut Computer {
let opcode = 7D;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnl</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jle (&mut Computer) -> &mut Computer {
let opcode = 7E;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jng</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jg (&mut Computer) -> &mut Computer {
let opcode = 7F;
 // Args: rel8
 // Flags: none
 // Bytes: 7+,3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">jnle</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 80;
 // Args: mem8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="765"></a><a name="page478"></a>81</p>

</td><td class="td" align="left">

<p class="table-para">add</p>

</td><td class="td" align="left">

<p class="table-para">reg16,imm16</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 81;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_adc (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_add (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_and (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="766"></a><a name="page479"></a>83</p>

</td><td class="td" align="left">

<p class="table-para">and</p>

</td><td class="td" align="left">

<p class="table-para">mem16,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">3+</p>

</td><td class="td" align="left">

<p class="table-para">7</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmp (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_or (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sbb (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_sub (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xor (&mut Computer) -> &mut Computer {
let opcode = 83;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = 84;
 // Args: reg8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = 84;
 // Args: mem8,reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = 85;
 // Args: reg16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = 85;
 // Args: mem16,reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 86;
 // Args: reg8,reg8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 86;
 // Args: reg8,mem8
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 87;
 // Args: reg16,reg16
 // Flags: none
 // Bytes: 2
computer }

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 87;
 // Args: reg16,mem16
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 88;
 // Args: mem8,reg8
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 89;
 // Args: mem16,reg16
 // Flags: none
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8A;
 // Args: reg8,reg8
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">8A</p>

</td><td class="td" align="left">

<p class="table-para">mov</p>

</td><td class="td" align="left">

<p class="table-para">reg8,mem8</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1<a name="767"></a><a name="page480"></a>

</p>

</td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8B;
 // Args: reg16,reg16
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8B;
 // Args: reg16,mem16
 // Flags: none
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8C;
 // Args: reg16, sreg
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8C;
 // Args: mem16,sreg
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_lea (&mut Computer) -> &mut Computer {
let opcode = 8D;
 // Args: reg32,mem32
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = 8E;
 // Args: sreg, reg16
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">8E</p>

</td><td class="td" align="left">

<p class="table-para">mov</p>

</td><td class="td" align="left">

<p class="table-para">sreg,mem16</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">3<sup>[<a name="ap-etablefnt01" href="#ftn.ap-etablefnt01">*</a>]</sup>

</p>

</td><td class="td" align="left">

<p class="table-para">2<sup>[<a href="#ftn.ap-etablefnt01">*</a>]</sup>

</p>

</td>

</tr>

fn instruction_pop (&mut Computer) -> &mut Computer {
let opcode = 8F;
 // Args: mem16
 // Flags: none
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 91;
 // Args: AX, CX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, ECX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 92;
 // Args: AX, DX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, EDX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 93;
 // Args: AX, BX
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, EBX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 94;
 // Args: AX, SP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, ESP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 95;
 // Args: AX, BP
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, EBP</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 96;
 // Args: AX, SI
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, ESI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xchg (&mut Computer) -> &mut Computer {
let opcode = 97;
 // Args: AX, DI
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, EDI</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cbw (&mut Computer) -> &mut Computer {
let opcode = 98;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_cwde (&mut Computer) -> &mut Computer {
let opcode = 98;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_cdq (&mut Computer) -> &mut Computer {
let opcode = 99;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_cwd (&mut Computer) -> &mut Computer {
let opcode = 99;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_call (&mut Computer) -> &mut Computer {
let opcode = 9A;
 // Args: far direct
 // Flags: none
 // Bytes: 7
computer }

fn instruction_pushf (&mut Computer) -> &mut Computer {
let opcode = 9C;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">pushfd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_popf (&mut Computer) -> &mut Computer {
let opcode = 9D;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">popfd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">A0</p>

</td><td class="td" align="left">

<p class="table-para">mov</p>

</td><td class="td" align="left">

<p class="table-para">AL, direct</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1<a name="769"></a><a name="page481"></a>

</p>

</td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = A1;
 // Args: AX, direct
 // Flags: none
 // Bytes: 5
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, direct</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = A2;
 // Args: direct ,AL
 // Flags: none
 // Bytes: 5
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = A3;
 // Args: direct, AX
 // Flags: none
 // Bytes: 5
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">direct, EAX</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_movsb (&mut Computer) -> &mut Computer {
let opcode = A4;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_movsw (&mut Computer) -> &mut Computer {
let opcode = A5;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">movsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmpsb (&mut Computer) -> &mut Computer {
let opcode = A6;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_cmpsw (&mut Computer) -> &mut Computer {
let opcode = A7;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = A8;
 // Args: AL,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = A9;
 // Args: AX,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_stosb (&mut Computer) -> &mut Computer {
let opcode = AA;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_stosw (&mut Computer) -> &mut Computer {
let opcode = AB;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">stosd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_lodsb (&mut Computer) -> &mut Computer {
let opcode = AC;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_lodsw (&mut Computer) -> &mut Computer {
let opcode = AD;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">lodsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_scasb (&mut Computer) -> &mut Computer {
let opcode = AE;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_scasw (&mut Computer) -> &mut Computer {
let opcode = AE;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B0;
 // Args: AL, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B1;
 // Args: CL, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B2;
 // Args: DL, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B3;
 // Args: BL, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B4;
 // Args: AH, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B5;
 // Args: CH, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B6;
 // Args: DH, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B7;
 // Args: BH, imm8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B8;
 // Args: AX, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EAX, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = B9;
 // Args: CX, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ECX, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="770"></a><a name="page482"></a>BA</p>

</td><td class="td" align="left">

<p class="table-para">mov</p>

</td><td class="td" align="left">

<p class="table-para">DX, imm16</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDX, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = BB;
 // Args: BX, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EBX, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = BC;
 // Args: SP, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESP, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = BD;
 // Args: BP, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EPB, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = BE;
 // Args: SI, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ESI, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = BF;
 // Args: DI, imm16
 // Flags: none
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">EDI, imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = C0;
 // Args: reg8, imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = C0;
 // Args: mem8, imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = C0;
 // Args: reg8, imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = C0;
 // Args: mem8, imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = C1;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = C1;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = C1;
 // Args: reg16,imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">reg32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = C1;
 // Args: mem16,imm8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 3+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">mem32,imm8</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_ret (near) (&mut Computer) -> &mut Computer {
let opcode = C2;
 // Args: imm16
 // Flags: none
 // Bytes: 3
computer }

fn instruction_ret (near) (&mut Computer) -> &mut Computer {
let opcode = C3;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = C6;
 // Args: mem8, imm8
 // Flags: none
 // Bytes: 3+
computer }

fn instruction_mov (&mut Computer) -> &mut Computer {
let opcode = C7;
 // Args: mem16,imm16
 // Flags: none
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="771"></a><a name="page483"></a>CA</p>

</td><td class="td" align="left">

<p class="table-para">ret (far)</p>

</td><td class="td" align="left">

<p class="table-para">imm16</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td><td class="td" align="left">

<p class="table-para">18+</p>

</td><td class="td" align="left">

<p class="table-para">14</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td>

</tr>

fn instruction_ret (far) (&mut Computer) -> &mut Computer {
let opcode = CB;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D0;
 // Args: reg8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D0;
 // Args: mem8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D0;
 // Args: reg8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D0;
 // Args: mem8
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D1;
 // Args: reg16
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D1;
 // Args: reg16
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D1;
 // Args: reg16
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D1;
 // Args: reg16
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D2;
 // Args: reg8, CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D2;
 // Args: mem8, CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D2;
 // Args: reg8, CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D2;
 // Args: mem8, CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D3;
 // Args: reg16,CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rol (&mut Computer) -> &mut Computer {
let opcode = D3;
 // Args: mem16,CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">ror</p>

</td><td class="td" align="left">

<p class="table-para">mem32,CL</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_shl/sal (&mut Computer) -> &mut Computer {
let opcode = D3;
 // Args: reg16,CL
 // Flags: SF,ZF,OF,CF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">reg32,CL</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="772"></a><a name="page484"></a>D3</p>

</td><td class="td" align="left">

<p class="table-para">shl/sal</p>

</td><td class="td" align="left">

<p class="table-para">mem16,CL</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">7</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td><td class="td" align="left">

<p class="table-para">4</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">shr</p>

</td><td class="td" align="left">

<p class="table-para">mem32,CL</p>

</td><td class="td" align="left">

<p class="table-para">AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">sar</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_aam (&mut Computer) -> &mut Computer {
let opcode = D4 0A;
 // Args: none
 // Flags: SF,ZF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">OF,AF,CF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_aad (&mut Computer) -> &mut Computer {
let opcode = D5 0A;
 // Args: none
 // Flags: SF,ZF,PF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">OF,AF,CF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_xlat (&mut Computer) -> &mut Computer {
let opcode = D7;
 // Args: none
 // Flags: none
 // Bytes: 1
computer }

fn instruction_loopne (&mut Computer) -> &mut Computer {
let opcode = E0;
 // Args: none
 // Flags: none
 // Bytes: 11+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">loopnz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_loope (&mut Computer) -> &mut Computer {
let opcode = E1;
 // Args: none
 // Flags: none
 // Bytes: 11+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">loopz</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_loop (&mut Computer) -> &mut Computer {
let opcode = E2;
 // Args: none
 // Flags: none
 // Bytes: 11+
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">E3</p>

</td><td class="td" align="left">

<p class="table-para">jecxz</p>

</td><td class="td" align="left">

<p class="table-para">rel8</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6,5</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td>

</tr>

fn instruction_call (&mut Computer) -> &mut Computer {
let opcode = E8;
 // Args: rel32
 // Flags: none
 // Bytes: 5
computer }

fn instruction_jmp (&mut Computer) -> &mut Computer {
let opcode = E9;
 // Args: rel32
 // Flags: none
 // Bytes: 5
computer }

fn instruction_jmp (&mut Computer) -> &mut Computer {
let opcode = EB;
 // Args: rel8
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">F2</p>

</td><td class="td" align="left">

<p class="table-para">repnz</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repne</p>

</td><td class="td" align="left">

<p class="table-para">(string instruction prefix)</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repne (&mut Computer) -> &mut Computer {
let opcode = F2 A6;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repne (&mut Computer) -> &mut Computer {
let opcode = F2 A7;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsw</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repne</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repne (&mut Computer) -> &mut Computer {
let opcode = F2 AE;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repne (&mut Computer) -> &mut Computer {
let opcode = F2 AF;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasw</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repne</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">F3</p>

</td><td class="td" align="left">

<p class="table-para">rep</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repz repe</p>

</td><td class="td" align="left">

<p class="table-para">(string instruction prefix)</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rep (&mut Computer) -> &mut Computer {
let opcode = F3 A4;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">movsb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">

<a name="773"></a><a name="page485"></a>F3 A5</p>

</td><td class="td" align="left">

<p class="table-para">rep</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">none</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">7+4n</p>

</td><td class="td" align="left">

<p class="table-para">12+3n</p>

</td><td class="td" align="left">

<p class="table-para">13+4n</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">movsw</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">rep</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">movsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rep stosb (&mut Computer) -> &mut Computer {
let opcode = F3 A6;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

fn instruction_repe (&mut Computer) -> &mut Computer {
let opcode = F3 A6;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_rep stosw (&mut Computer) -> &mut Computer {
let opcode = F3 A7;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">rep</p>

</td><td class="td" align="left">

<p class="table-para">stosd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repe (&mut Computer) -> &mut Computer {
let opcode = F3 A7;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsw</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">cmpsd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repe (&mut Computer) -> &mut Computer {
let opcode = F3 AE;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasb</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_repe (&mut Computer) -> &mut Computer {
let opcode = F3 AF;
 // Args: none
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasw</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">repe</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">scasd</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_cmc (&mut Computer) -> &mut Computer {
let opcode = F5;
 // Args: none
 // Flags: CF
 // Bytes: 1
computer }

fn instruction_div (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2
computer }

fn instruction_div (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2+
computer }

fn instruction_idiv (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2
computer }

fn instruction_idiv (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2+
computer }

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: OF,CF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: OF,CF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mul (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: OF,CF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_mul (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: OF,CF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_neg (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

fn instruction_neg (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

fn instruction_not (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8
 // Flags: none
 // Bytes: 2
computer }

fn instruction_not (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: mem8
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = F6;
 // Args: reg8,imm8
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 3
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">F6</p>

</td><td class="td" align="left">

<p class="table-para">test</p>

</td><td class="td" align="left">

<p class="table-para">mem8,imm8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,CF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">3+</p>

</td><td class="td" align="left">

<p class="table-para">5</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">2<a name="774"></a><a name="page486"></a>

</p>

</td>

</tr>

fn instruction_div (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">38</p>

</td><td class="td" align="left">

<p class="table-para">40</p>

</td><td class="td" align="left">

<p class="table-para">41</p>

</td>

</tr>

fn instruction_div (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">41</p>

</td><td class="td" align="left">

<p class="table-para">40</p>

</td><td class="td" align="left">

<p class="table-para">41</p>

</td>

</tr>

fn instruction_idiv (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">43</p>

</td><td class="td" align="left">

<p class="table-para">43</p>

</td><td class="td" align="left">

<p class="table-para">48</p>

</td>

</tr>

fn instruction_idiv (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: SF,ZF,OF,PF,AF ?
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">46</p>

</td><td class="td" align="left">

<p class="table-para">44</p>

</td><td class="td" align="left">

<p class="table-para">48</p>

</td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: OF,CF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">9-38</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: OF,CF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">12-41</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_imul (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: OF,CF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left">

<p class="table-para">9-38</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_mul (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: OF,CF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">9-38</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_mul (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: OF,CF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF, PF,AF ?</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">12-41</p>

</td><td class="td" align="left">

<p class="table-para">13-42</p>

</td><td class="td" align="left">

<p class="table-para">10</p>

</td>

</tr>

fn instruction_neg (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_neg (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_not (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16
 // Flags: none
 // Bytes: 2
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_not (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16
 // Flags: none
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: reg16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">reg32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_test (&mut Computer) -> &mut Computer {
let opcode = F7;
 // Args: mem16,imm16
 // Flags: SF,ZF,OF,CF,PF,AF
 // Bytes: 4+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32,imm32</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">6+</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_clc (&mut Computer) -> &mut Computer {
let opcode = F8;
 // Args: none
 // Flags: CF
 // Bytes: 1
computer }

fn instruction_stc (&mut Computer) -> &mut Computer {
let opcode = F9;
 // Args: none
 // Flags: CF
 // Bytes: 1
computer }

fn instruction_cld (&mut Computer) -> &mut Computer {
let opcode = FC;
 // Args: none
 // Flags: DF
 // Bytes: 1
computer }

fn instruction_std (&mut Computer) -> &mut Computer {
let opcode = FD;
 // Args: none
 // Flags: DF
 // Bytes: 1
computer }

<tr valign="top">

<td class="td" align="left">

<p class="table-para">FE</p>

</td><td class="td" align="left">

<p class="table-para">dec</p>

</td><td class="td" align="left">

<p class="table-para">reg8</p>

</td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">2</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td><td class="td" align="left">

<p class="table-para">1</p>

</td>

</tr>

<tr valign="top">

<td class="td" align="left">

<p class="table-para">FE</p>

</td><td class="td" align="left">

<p class="table-para">dec</p>

</td><td class="td" align="left">

<p class="table-para">mem8</p>

</td><td class="td" align="left">

<p class="table-para">SF,ZF,OF,PF,AF</p>

</td><td class="td" align="left">

<p class="table-para">2+</p>

</td><td class="td" align="left">

<p class="table-para">6</p>

</td><td class="td" align="left">

<p class="table-para">3</p>

</td><td class="td" align="left">

<p class="table-para">3<a name="775"></a><a name="page487"></a>

</p>

</td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = FE;
 // Args: reg8
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 2
computer }

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = FE;
 // Args: mem8
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 2+
computer }

fn instruction_call (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: reg32 (near indirect)
 // Flags: none
 // Bytes: 2
computer }

fn instruction_call (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: mem32 (near indirect)
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_call (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: far indirect
 // Flags: none
 // Bytes: 6
computer }

fn instruction_dec (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: mem16
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_inc (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: mem16
 // Flags: SF,ZF,OF,PF,AF
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

fn instruction_jmp (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: reg32
 // Flags: none
 // Bytes: 2
computer }

fn instruction_jmp (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: mem32
 // Flags: none
 // Bytes: 2+
computer }

fn instruction_push (&mut Computer) -> &mut Computer {
let opcode = FF;
 // Args: mem16
 // Flags: none
 // Bytes: 2+
computer }

<tr valign="top">

<td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left">

<p class="table-para">mem32</p>

</td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td><td class="td" align="left"> </td>

</tr>

</tbody>

<tr>

<td class="td" colspan="8">

<div class="footnote">

<p>

<a name="768"></a><sup>[<a name="ftn.ap-etablefnt01" href="#ap-etablefnt01">*</a>]</sup>timing varies</p>

</div>

</td>

</tr>

</table>

</div>

</div>
<br><hr size="1">
</body>
</html>
