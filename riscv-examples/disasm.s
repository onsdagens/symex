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


get_sign:	file format elf32-littleriscv

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
      68: e7 80 80 0a  	jalr	0xa8(ra) <.Lline_table_start0+0x10c>
      6c: 6f 00 40 08  	j	0xf0 <.Lline_table_start0+0xf0>

00000070 <DefaultHandler>:
      70: 6f 00 00 00  	j	0x70 <.Lline_table_start0+0x70>

00000074 <symex_lib::symbolic_size::h9d0c41f93a31ee0e>:
      74: 13 01 01 ff  	addi	sp, sp, -0x10
      78: 93 05 40 00  	li	a1, 0x4
      7c: 23 26 b1 00  	sw	a1, 0xc(sp)
      80: 03 20 05 00  	lw	zero, 0x0(a0)
      84: 03 20 c1 00  	lw	zero, 0xc(sp)
      88: 13 01 01 01  	addi	sp, sp, 0x10
      8c: 67 80 00 00  	ret

00000090 <symex_lib::symbolic::h40a4f6ebc2c76203>:
      90: 13 01 01 ff  	addi	sp, sp, -0x10
      94: 93 05 40 00  	li	a1, 0x4
      98: 23 26 b1 00  	sw	a1, 0xc(sp)
      9c: 03 20 c1 00  	lw	zero, 0xc(sp)
      a0: 13 01 01 01  	addi	sp, sp, 0x10
      a4: 17 03 00 00  	auipc	t1, 0x0
      a8: 67 00 03 fd  	jr	-0x30(t1) <.Lline_table_start0+0x74>

000000ac <get_sign_inner>:
      ac: 63 56 a0 00  	blez	a0, 0xb8 <.Lline_table_start0+0xb8>
      b0: 13 05 10 00  	li	a0, 0x1
      b4: 67 80 00 00  	ret
      b8: 13 35 15 00  	seqz	a0, a0
      bc: 13 05 f5 ff  	addi	a0, a0, -0x1
      c0: 67 80 00 00  	ret

000000c4 <get_sign>:
      c4: 13 01 01 ff  	addi	sp, sp, -0x10
      c8: 23 26 11 00  	sw	ra, 0xc(sp)
      cc: 13 05 81 00  	addi	a0, sp, 0x8
      d0: 97 00 00 00  	auipc	ra, 0x0
      d4: e7 80 00 fc  	jalr	-0x40(ra) <.Lline_table_start0+0x90>
      d8: 03 25 81 00  	lw	a0, 0x8(sp)
      dc: 97 00 00 00  	auipc	ra, 0x0
      e0: e7 80 00 fd  	jalr	-0x30(ra) <.Lline_table_start0+0xac>
      e4: 83 20 c1 00  	lw	ra, 0xc(sp)
      e8: 13 01 01 01  	addi	sp, sp, 0x10
      ec: 67 80 00 00  	ret

000000f0 <main>:
      f0: 13 01 01 ff  	addi	sp, sp, -0x10
      f4: 23 26 11 00  	sw	ra, 0xc(sp)
      f8: 97 00 00 00  	auipc	ra, 0x0
      fc: e7 80 c0 fc  	jalr	-0x34(ra) <.Lline_table_start0+0xc4>
     100: 23 24 a1 00  	sw	a0, 0x8(sp)
     104: 03 20 81 00  	lw	zero, 0x8(sp)
     108: 6f 00 00 00  	j	0x108 <.Lline_table_start0+0x108>

0000010c <_setup_interrupts>:
     10c: 37 05 00 00  	lui	a0, 0x0
     110: 13 05 05 14  	addi	a0, a0, 0x140
     114: 13 55 25 00  	srli	a0, a0, 0x2
     118: f3 15 05 b0  	csrrw	a1, mcycle, a0
     11c: 37 05 00 00  	lui	a0, 0x0
     120: 13 05 05 14  	addi	a0, a0, 0x140
     124: 13 55 25 00  	srli	a0, a0, 0x2
     128: f3 15 15 b0  	csrrw	a1, 0xb01, a0
     12c: 37 05 00 00  	lui	a0, 0x0
     130: 13 05 05 14  	addi	a0, a0, 0x140
     134: 13 55 25 00  	srli	a0, a0, 0x2
     138: f3 15 25 b0  	csrrw	a1, minstret, a0
     13c: 67 80 00 00  	ret

00000140 <Interrupt2>:
     140: 6f 00 00 00  	j	0x140 <.Lline_table_start0+0x140>
