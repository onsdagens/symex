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
       4: 93 81 41 80  	addi	gp, gp, -0x7fc

00000008 <.Lpcrel_hi1>:
       8: 17 13 00 50  	auipc	t1, 0x50001
       c: 13 03 83 ff  	addi	t1, t1, -0x8
      10: 13 71 03 ff  	andi	sp, t1, -0x10

00000014 <.Lpcrel_hi2>:
      14: 97 02 00 50  	auipc	t0, 0x50000
      18: 93 82 02 ff  	addi	t0, t0, -0x10

0000001c <.Lpcrel_hi3>:
      1c: 97 03 00 50  	auipc	t2, 0x50000
      20: 93 83 83 fe  	addi	t2, t2, -0x18

00000024 <.Lpcrel_hi4>:
      24: 17 03 00 50  	auipc	t1, 0x50000
      28: 13 03 03 fe  	addi	t1, t1, -0x20
      2c: 63 fc 72 00  	bgeu	t0, t2, 0x44 <.Lline_table_start0+0x44>
      30: 03 2e 03 00  	lw	t3, 0x0(t1)
      34: 13 03 43 00  	addi	t1, t1, 0x4
      38: 23 a0 c2 01  	sw	t3, 0x0(t0)
      3c: 93 82 42 00  	addi	t0, t0, 0x4
      40: e3 e8 72 fe  	bltu	t0, t2, 0x30 <.Lline_table_start0+0x30>

00000044 <.Lpcrel_hi5>:
      44: 97 02 00 50  	auipc	t0, 0x50000
      48: 93 82 02 fc  	addi	t0, t0, -0x40

0000004c <.Lpcrel_hi6>:
      4c: 97 03 00 50  	auipc	t2, 0x50000
      50: 93 83 83 fb  	addi	t2, t2, -0x48
      54: 63 f8 72 00  	bgeu	t0, t2, 0x64 <.Lline_table_start0+0x64>
      58: 23 a0 02 00  	sw	zero, 0x0(t0)
      5c: 93 82 42 00  	addi	t0, t0, 0x4
      60: e3 ec 72 fe  	bltu	t0, t2, 0x58 <.Lline_table_start0+0x58>
      64: 97 00 00 00  	auipc	ra, 0x0
      68: e7 80 00 0c  	jalr	0xc0(ra) <.Lline_table_start0+0x124>
      6c: 6f 00 40 07  	j	0xe0 <.Lline_table_start0+0xe0>

00000070 <DefaultHandler>:
      70: 6f 00 00 00  	j	0x70 <.Lline_table_start0+0x70>

00000074 <Interrupt1>:
      74: 37 05 00 50  	lui	a0, 0x50000
      78: 03 45 05 00  	lbu	a0, 0x0(a0)
      7c: 63 0a 05 04  	beqz	a0, 0xd0 <.Lline_table_start0+0xd0>
      80: 13 00 00 00  	nop
      84: 13 00 00 00  	nop
      88: 13 00 00 00  	nop
      8c: 13 00 00 00  	nop
      90: 13 00 00 00  	nop
      94: 13 00 00 00  	nop
      98: 13 00 00 00  	nop
      9c: 13 00 00 00  	nop
      a0: 13 00 00 00  	nop
      a4: 13 00 00 00  	nop
      a8: 13 00 00 00  	nop
      ac: 13 00 00 00  	nop
      b0: 13 00 00 00  	nop
      b4: 13 00 00 00  	nop
      b8: 13 00 00 00  	nop
      bc: 13 00 00 00  	nop
      c0: 13 00 00 00  	nop
      c4: 13 00 00 00  	nop
      c8: 13 00 00 00  	nop
      cc: 13 00 00 00  	nop
      d0: 67 80 00 00  	ret

000000d4 <Interrupt0>:
      d4: 37 05 00 50  	lui	a0, 0x50000
      d8: 23 00 05 00  	sb	zero, 0x0(a0)
      dc: 67 80 00 00  	ret

000000e0 <main>:
      e0: 13 01 01 ff  	addi	sp, sp, -0x10
      e4: 23 26 11 00  	sw	ra, 0xc(sp)
      e8: 13 05 80 00  	li	a0, 0x8
      ec: 73 30 05 30  	csrc	mstatus, a0
      f0: 73 20 15 b2  	csrs	0xb21, a0
      f4: 73 60 11 b2  	csrsi	0xb21, 0x2
      f8: 73 20 05 b2  	csrs	0xb20, a0
      fc: 73 60 01 b2  	csrsi	0xb20, 0x2
     100: 97 00 00 00  	auipc	ra, 0x0
     104: e7 80 c0 00  	jalr	0xc(ra) <.Lline_table_start0+0x10c>
     108: 6f 00 00 00  	j	0x108 <.Lline_table_start0+0x108>

0000010c <get_sign_rtic::app::main::__rtic_init_resources::h72150988e92d926f>:
     10c: 37 05 00 50  	lui	a0, 0x50000
     110: 93 05 10 00  	li	a1, 0x1
     114: 23 00 b5 00  	sb	a1, 0x0(a0)
     118: 13 05 80 00  	li	a0, 0x8
     11c: 73 20 05 30  	csrs	mstatus, a0
     120: 67 80 00 00  	ret

00000124 <_setup_interrupts>:
     124: 37 05 00 00  	lui	a0, 0x0
     128: 13 05 45 0d  	addi	a0, a0, 0xd4
     12c: 13 55 25 00  	srli	a0, a0, 0x2
     130: f3 15 05 b0  	csrrw	a1, mcycle, a0
     134: 37 05 00 00  	lui	a0, 0x0
     138: 13 05 45 07  	addi	a0, a0, 0x74
     13c: 13 55 25 00  	srli	a0, a0, 0x2
     140: f3 15 15 b0  	csrrw	a1, 0xb01, a0
     144: 37 05 00 00  	lui	a0, 0x0
     148: 13 05 85 15  	addi	a0, a0, 0x158
     14c: 13 55 25 00  	srli	a0, a0, 0x2
     150: f3 15 25 b0  	csrrw	a1, minstret, a0
     154: 67 80 00 00  	ret

00000158 <Interrupt2>:
     158: 6f 00 00 00  	j	0x158 <.Lline_table_start0+0x158>
