warning: unknown and unstable feature specified for `-Ctarget-feature`: `zmmul`
  |
  = note: it is still passed through to the codegen backend, but use of this feature might be unsound and the behavior of this feature can change in the future
  = help: consider filing a feature request

warning: 1 warning emitted

warning: unknown and unstable feature specified for `-Ctarget-feature`: `zmmul`
  |
  = note: it is still passed through to the codegen backend, but use of this feature might be unsound and the behavior of this feature can change in the future
  = help: consider filing a feature request

warning: 1 warning emitted


get_sign_rtic:	file format elf32-littleriscv

Disassembly of section .text:

00000000 <_start>:
       0: 97 11 00 50  	auipc	gp, 0x50001
       4: 93 81 01 80  	addi	gp, gp, -0x800

00000008 <.Lpcrel_hi1>:
       8: 17 13 00 50  	auipc	t1, 0x50001
       c: 13 03 83 ff  	addi	t1, t1, -0x8
      10: 13 71 03 ff  	andi	sp, t1, -0x10

00000014 <.Lpcrel_hi2>:
      14: 97 02 00 50  	auipc	t0, 0x50000
      18: 93 82 c2 fe  	addi	t0, t0, -0x14

0000001c <.Lpcrel_hi3>:
      1c: 97 03 00 50  	auipc	t2, 0x50000
      20: 93 83 43 fe  	addi	t2, t2, -0x1c

00000024 <.Lpcrel_hi4>:
      24: 17 03 00 50  	auipc	t1, 0x50000
      28: 13 03 c3 fd  	addi	t1, t1, -0x24
      2c: 63 fc 72 00  	bgeu	t0, t2, 0x44 <.Lline_table_start0+0x44>
      30: 03 2e 03 00  	lw	t3, 0x0(t1)
      34: 13 03 43 00  	addi	t1, t1, 0x4
      38: 23 a0 c2 01  	sw	t3, 0x0(t0)
      3c: 93 82 42 00  	addi	t0, t0, 0x4
      40: e3 e8 72 fe  	bltu	t0, t2, 0x30 <.Lline_table_start0+0x30>

00000044 <.Lpcrel_hi5>:
      44: 97 02 00 50  	auipc	t0, 0x50000
      48: 93 82 c2 fb  	addi	t0, t0, -0x44

0000004c <.Lpcrel_hi6>:
      4c: 97 03 00 50  	auipc	t2, 0x50000
      50: 93 83 43 fb  	addi	t2, t2, -0x4c
      54: 63 f8 72 00  	bgeu	t0, t2, 0x64 <.Lline_table_start0+0x64>
      58: 23 a0 02 00  	sw	zero, 0x0(t0)
      5c: 93 82 42 00  	addi	t0, t0, 0x4
      60: e3 ec 72 fe  	bltu	t0, t2, 0x58 <.Lline_table_start0+0x58>
      64: 97 00 00 00  	auipc	ra, 0x0
      68: e7 80 00 06  	jalr	0x60(ra) <.Lline_table_start0+0xc4>
      6c: 6f 00 80 02  	j	0x94 <.Lline_table_start0+0x94>

00000070 <DefaultHandler>:
      70: 6f 00 00 00  	j	0x70 <.Lline_table_start0+0x70>

00000074 <Interrupt0>:
      74: 13 01 01 ff  	addi	sp, sp, -0x10
      78: 13 05 40 1f  	li	a0, 0x1f4
      7c: 23 26 a1 00  	sw	a0, 0xc(sp)
      80: 03 20 c1 00  	lw	zero, 0xc(sp)
      84: 13 05 f5 ff  	addi	a0, a0, -0x1
      88: e3 1c 05 fe  	bnez	a0, 0x80 <.Lline_table_start0+0x80>
      8c: 13 01 01 01  	addi	sp, sp, 0x10
      90: 67 80 00 00  	ret

00000094 <main>:
      94: 13 01 01 ff  	addi	sp, sp, -0x10
      98: 23 26 11 00  	sw	ra, 0xc(sp)
      9c: 13 05 80 00  	li	a0, 0x8
      a0: 73 30 05 30  	csrc	mstatus, a0
      a4: 73 20 05 b2  	csrs	0xb20, a0
      a8: 73 60 01 b2  	csrsi	0xb20, 0x2
      ac: 97 00 00 00  	auipc	ra, 0x0
      b0: e7 80 c0 00  	jalr	0xc(ra) <.Lline_table_start0+0xb8>
      b4: 6f 00 00 00  	j	0xb4 <.Lline_table_start0+0xb4>

000000b8 <get_sign_rtic::app::main::__rtic_init_resources::h72150988e92d926f>:
      b8: 13 05 80 00  	li	a0, 0x8
      bc: 73 20 05 30  	csrs	mstatus, a0
      c0: 67 80 00 00  	ret

000000c4 <_setup_interrupts>:
      c4: 37 05 00 00  	lui	a0, 0x0
      c8: 13 05 45 07  	addi	a0, a0, 0x74
      cc: 13 55 25 00  	srli	a0, a0, 0x2
      d0: f3 15 05 b0  	csrrw	a1, mcycle, a0
      d4: 37 05 00 00  	lui	a0, 0x0
      d8: 13 05 85 0f  	addi	a0, a0, 0xf8
      dc: 13 55 25 00  	srli	a0, a0, 0x2
      e0: f3 15 15 b0  	csrrw	a1, 0xb01, a0
      e4: 37 05 00 00  	lui	a0, 0x0
      e8: 13 05 85 0f  	addi	a0, a0, 0xf8
      ec: 13 55 25 00  	srli	a0, a0, 0x2
      f0: f3 15 25 b0  	csrrw	a1, minstret, a0
      f4: 67 80 00 00  	ret

000000f8 <Interrupt2>:
      f8: 6f 00 00 00  	j	0xf8 <.Lline_table_start0+0xf8>
