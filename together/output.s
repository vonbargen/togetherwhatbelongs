	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 16, 0
	.globl	_oberon_Add                     ; -- Begin function oberon_Add
	.p2align	2
_oberon_Add:                            ; @oberon_Add
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	mov	x8, x0
	add	x0, x0, x1
	stp	x1, x8, [sp], #16
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_oberon_Init                    ; -- Begin function oberon_Init
	.p2align	2
_oberon_Init:                           ; @oberon_Init
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
Lloh0:
	adrp	x8, _oberon_count@PAGE
	str	xzr, [sp, #8]
	str	xzr, [x8, _oberon_count@PAGEOFF]
Lloh1:
	adrp	x8, _oberon_points@PAGE
Lloh2:
	add	x8, x8, _oberon_points@PAGEOFF
LBB1_1:                                 ; %forcond
                                        ; =>This Inner Loop Header: Depth=1
	ldr	x9, [sp, #8]
	cmp	x9, #9
	b.gt	LBB1_3
; %bb.2:                                ; %forbody
                                        ;   in Loop: Header=BB1_1 Depth=1
	ldr	x9, [sp, #8]
	lsl	x10, x9, #4
	add	x9, x9, #1
	str	xzr, [x8, x10]
	str	x9, [sp, #8]
	b	LBB1_1
LBB1_3:                                 ; %forcont
	add	sp, sp, #16
	ret
	.loh AdrpAdd	Lloh1, Lloh2
	.loh AdrpAdrp	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	_oberon_Init
	mov	w0, #5
	mov	w1, #10
	bl	_oberon_Add
	mov	x8, x0
	adrp	x9, _oberon_count@PAGE
	mov	w0, wzr
	str	x8, [x9, _oberon_count@PAGEOFF]
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_oberon_count                   ; @oberon_count
.zerofill __DATA,__common,_oberon_count,8,3
	.globl	_oberon_points                  ; @oberon_points
.zerofill __DATA,__common,_oberon_points,160,4
.subsections_via_symbols
