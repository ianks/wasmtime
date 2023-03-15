window.SIDEBAR_ITEMS = {"enum":[["AluRmROpcode","ALU operations that don’t accept intermediates."],["AluRmiROpcode","Some basic ALU operations."],["Amode","Internal type Amode: defined at src/isa/x64/inst.isle line 976."],["Avx512Opcode",""],["AvxOpcode","Internal type AvxOpcode: defined at src/isa/x64/inst.isle line 1125."],["CC","These indicate condition code tests.  Not all are represented since not all are useful in compiler-generated code."],["CmpOpcode","Comparison operations."],["DivSignedness","Internal type DivSignedness: defined at src/isa/x64/inst.isle line 669."],["ExtKind","This defines the ways a value can be extended: either signed- or zero-extension, or none for types that are not extended. Contrast with [ExtMode], which defines the widths from and to which values can be extended."],["ExtMode","These indicate ways of extending (widening) a value, using the Intel naming: B(yte) = u8, W(ord) = u16, L(ong)word = u32, Q(uad)word = u64"],["FcmpImm","Encode the ways that floats can be compared. This is used in float comparisons such as `cmpps`, e.g.; it is distinguished from other float comparisons (e.g. `ucomiss`) in that those use EFLAGS whereas [FcmpImm] is used as an immediate."],["FenceKind","An x64 memory fence kind."],["Imm8Reg","An operand which is either an 8-bit integer immediate or a register."],["OperandSize","An operand’s size in bits."],["RegMem","An operand which is either an integer Register or a value in Memory.  This can denote an 8, 16, 32, 64, or 128 bit value."],["RegMemImm","An operand which is either an integer Register, a value in Memory or an Immediate.  This can denote an 8, 16, 32 or 64 bit value.  For the Immediate form, in the 8- and 16-bit case, only the lower 8 or 16 bits of `simm32` is relevant.  In the 64-bit case, the value denoted by `simm32` is its sign-extension out to 64 bits."],["RoundImm","Encode the rounding modes used as part of the Rounding Control field. Note, these rounding immediates only consider the rounding control field (i.e. the rounding mode) which only take up the first two bits when encoded. However the rounding immediate which this field helps make up, also includes bits 3 and 4 which define the rounding select and precision mask respectively. These two bits are not defined here and are implictly set to zero when encoded."],["ShiftKind","These indicate the form of a scalar shift/rotate: left, signed right, unsigned right."],["SseOpcode","Some SSE operations requiring 2 operands r/m and r."],["SyntheticAmode","A Memory Address. These denote a 64-bit value only. Used for usual addressing modes as well as addressing modes used during compilation, when the moving SP offset is not known."],["UnaryRmROpcode","Unary operations requiring register or memory and register operands."]],"struct":[["Gpr","A newtype wrapper around `Reg`."],["GprMem","A newtype wrapper around `RegMem` for general-purpose registers."],["GprMemImm","A newtype wrapper around `RegMemImm`."],["Imm8Gpr","A newtype wrapper around `Imm8Reg`."],["Imm8Xmm","A newtype wrapper around `Imm8Reg`."],["Xmm","A newtype wrapper around `Reg`."],["XmmMem","A newtype wrapper around `RegMem` for general-purpose registers."],["XmmMemAligned","A newtype wrapper around `RegMem` for general-purpose registers."],["XmmMemAlignedImm","A newtype wrapper around `RegMemImm`."],["XmmMemImm","A newtype wrapper around `RegMemImm`."]],"trait":[["FromWritableReg","An extension trait for converting `Writable<Reg>` to `Writable{Xmm,Gpr}`."],["ToWritableReg","An extenstion trait for converting `Writable{Xmm,Gpr}` to `Writable<Reg>`."]],"type":[["OptionWritableGpr","Optional writable Gpr."],["OptionWritableXmm","Optional writable Gpr."],["WritableGpr","Writable Gpr."],["WritableXmm","Writable Gpr."]]};