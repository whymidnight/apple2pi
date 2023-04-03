PTR	EQU	$06
PTRL	EQU	$06
PTRH	EQU	$07
SPINIDX	EQU	$07
BASEL	EQU	$28
CSWL	EQU	$36
CSWH	EQU	$37
KSWL	EQU	$38
KSWH	EQU	$39
RNDL	EQU	$4E
RNDH	EQU	$4F
CEVL	EQU	$3CE
CEVH	EQU	$3CF
OUTVECT0	EQU	$BE10
OUTVECT3	EQU	$BE16
INVECT0		EQU	$BE20
INVECT3	EQU	$BE26
VECTOUT	EQU	$BE30
VECTIN	EQU	$BE32
WARMDOS	EQU	$BE00
TEXTSCRN	EQU	$0400
WAIT	EQU	$FCA8
BELL1	EQU	$FBD9
COUT1	EQU	$FDF0
KEYBD	EQU	$C000
KBCLR	EQU	$C010
ANYKEY	EQU	$C010
	ORG	$9600
	CLD
	LDA	#$C1
	STA	PTRH
	LDA	#$00
	STA	PTRL
*
* SEARCH SLOTS FOR MOUSE CARDS
*
IDCARD	LDY	#$05
	LDA	(PTR),Y
	CMP	#$38	; PASCAL ID BYTE
	BNE	NEXTCARD
	LDY	#$07
	LDA	(PTR),Y
	CMP	#$18
	BNE	NEXTCARD	; PASCAL ID BYTE
ISMOU	LDY	#$0C
	LDA	(PTR),Y
	CMP	#$20
	BNE	NEXTCARD
	LDY	#$FB
	LDA	(PTR),Y
	CMP	#$D6
	BNE	NEXTCARD
	LDA	PTRH
	STA	MOUROM
	AND	#$07
	STA	MOUINDEX
	ASL
	ASL
	ASL
	ASL
	STA	MOUSLOT
NEXTCARD	INC	PTRH
	LDA	PTRH
	CMP	#$C8
	BNE	IDCARD
*
* GET SERIAL PORT SLOT AT $300 (768)
*
	LDA	$300
	STA	SSCSLOT
	BNE	ENTRY
	RTS		; NO A2PI SLOT, EXIT
*
* RESET ENTRYPOINT
*
ENTRY	CLI
*
* INIT MOUSE
*
INITMOU	LDX	$FBC0
	BNE	INITMOU1	; NOT //C
	JSR	$FB1E	; READ GAME CONTROLLER
	CPY	#$FF
	BEQ	INITMOU1	; NO JOYSTICK CHECK MOUSE
	STX	MOUINDEX
	STX	MOUROM
INITMOU1	LDY	MOUINDEX
	BEQ	INITSSC
	LDY	#$19	; INIT MOUSE
	JSR	CALLMOU
	LDY	#$12	; SET MOUSE
	LDA	#$01	; TRANSPARENT MODE
	JSR	CALLMOU
	LDA	#$00
	STA	$478
	STA	$4F8
	LDA	#$C0
	STA	$578
	LDA	#$3F
	STA	$5F8
	LDY	#$17	; CLAMP MOUSE
	LDA	#$00	; X AXIS
	JSR	CALLMOU
	LDY	#$17	; CLAMP MOUSE
	LDA	#$01	; Y AXIS
	JSR	CALLMOU
*
* INIT SSC CARD FOR 115K BAUD
*
INITSSC	LDY	SSCSLOT
	LDA	#$0B
	STA	$C089,Y	; RESET ACIA
	STA	$C08A,Y
	LDA	#$10
	STA	$C08B,Y	; 115K
*
* SYNCHRONIZE WITH PI
*
SYNC	LDA	#$80
	JSR	SENDACC
	INC	SPINIDX
	LDA	SPINIDX
	AND	#$07
	TAX
	LDA	SPINNER,X
	STA	TEXTSCRN
	LDA	#$FF
	JSR	WAIT
	LDA	KEYBD
	BPL	CHKACK
	JMP	EXIT
CHKACK	LDY	SSCSLOT
	LDA	$C089,Y
	AND	#$08
	BEQ	SYNC
	LDA	$C088,Y
	CMP	#$81
	BNE	SYNC
	STY	$301	; TELL CALLING PROGRAM SYNC OK
	LDA	#$A0
	STA	TEXTSCRN
	LDA	#$87
	JSR	BELL1
	LDA	ANYKEY
	STA	KEYSTAT
*
* GET COUT/CIN VECTORS
*
	JSR	$C300
	LDA	CSWL
	STA	CHAINOUT+1
	LDA	CSWH
	STA	CHAINOUT+2
	JSR	HOOK
*
* SET VECTOR FOR CHKEVENT
*
; LDX #<CHKEVENT
; STX CEVL
; LDX #>CHKEVENT
; STX CEVH
*
* SET UP RESET VECTOR
*
	LDA	RESET+1
	BNE	ISRESET
	LDA	$3F2
	STA	RESET
	LDA	$3F3
	STA	RESET+1
	LDA	#<ENTRY
	STA	$3F2
	LDA	#>ENTRY
	STA	$3F3
	EOR	#$A5
	STA	$3F4
	RTS
ISRESET	LDX	#$FF
	TXS
	JMP	WARMDOS
*
* CHAR OUT HOOK - CHECK ONCE FOR A2PI EVENTS
*
COUTPI	PHP
	CLI
	PHA
	STX	SAVEX
	STY	SAVEY
	LDA	#$98
	JSR	SENDACC
	PLA
	PHA
	JSR	SENDACC
	JSR	SENDACC
	LDA	CHRIN
	BNE	COUTNXT	; SKIP IF INPUT CHAR PENDING
	LDA	KEYBD
	CMP	#$83	; SKIP CTRL-C
	BEQ	COUTNXT
	CMP	#$93	; SKIP CTRL-S
	BEQ	COUTNXT
	LDA	#$01
	STA	WAITEV
	JSR	CHKEVENT	; CHECK ON EVENTS
	BCC	COUTNXT
	STA	CHRIN
COUTNXT	PLA
	LDY	SAVEY
	LDX	SAVEX
	PLP
	STA	SAVEY
CHAINOUT	JSR	COUT1
*
* SET INPUT/OUTPUT VECTORS
*
HOOK	STX	SAVEX
	LDX	#<COUTPI
	STX	CSWL
	LDX	#>COUTPI
	STX	CSWH
	LDX	#<CINPI
	STX	KSWL
	LDX	#>CINPI
	STX	KSWH
	LDX	SAVEX
	RTS
*
* CHAR IN HOOK - EVENT LOOP FOR A2PI DAEMON
*
CINPI	STA	(BASEL),Y	; RESTORE CURSOR
	LDA	CHRIN
	BNE	CINPIEX
	STX	SAVEX
	LDA	#$5F	; PRINT _
	JSR	CHAINOUT
	LDA	#$88	; BACKSAPCE
	JSR	CHAINOUT
	PHP
	CLI
EVENTLP	INC	RNDL	; UPDATE RND LIKE ANY PROPER RDKEY
	BNE	EVENTLP1
	INC	RNDH
EVENTLP1	JSR	CHKEVENT
	BCC	EVENTLP
	PLP	; HAVE CHAR FROM A2PI DAEMON
	LDX	SAVEX
CINPIEX	LDY	#$00
	STY	CHRIN
	RTS
*
* SEND MODIFIER (OPEN & CLOSED APPLE) KEYS/PADDLE BUTTONS
*
SENDMOD	LDA	$C062
	ASL
	LDA	$C061
	ROR
	AND	#$C0	; FALL THROUGH
*
* ACIA SEND BYTE IN ACCUMULATOR
*
SENDACC	;PHP
	;SEI		; Disable interrupts
	PHA
	LDA	SSCSLOT
	ORA	#$88+2
	TAY
	;LDA	#$07
	;STA	$C002-2,Y	; Enable transmit IRQ
	;LDA	$C001-2,Y	; Clear any oustanding interrupts
	PLA
	STA	$C000-2,Y	; AVOID PHANTOM READ FROM $C0XX
	PHA
	TXA
	PHA
	LDA	#$10
	JSR	WAIT
	PLA
	TAX
	;PHA
;SENDWT	LDA	$C001-2,Y
	;AND	#$80	; Check IRQ status
	;BPL	SENDWT
	;LDA	#$0B	; Disable transmit IRQ
	;STA	$C002-2,Y
;IRQWT	LDA	$C001-2,Y	; Wait for IRQ to clear
	;BMI	IRQWT
	PLA
	;PLP		; Restore interrupts
	RTS
*
* ACIA RECEIVE BYTE IN ACCUMULATOR
*
RECVACC	LDA	SSCSLOT
	ORA	#$88+2
	TAY
RECVWT	LDA	$C001-2,Y
	AND	#$08
	BEQ	RECVWT
	LDA	$C000-2,Y
	RTS
*
* CHECK INPUT STATE FOR CHANGE
*
CHKEVENT	LDA	ANYKEY	; CHECK FOR KEY PRESS/RELEASE
	CMP	KEYSTAT
	BNE	SENDKEY
	LDA	SSCSLOT	; CHECK FOR INPUT TO ACIA
	ORA	#$88+2
	TAY
	LDA	$C001-2,Y
	AND	#$08
	BEQ	CHKMOU
	LDA	$C000-2,Y
	JMP	HOSTREQ
CHKMOU	DEC	WAITEV	; CHECK FOR MOUSE UPDATE
	BEQ	UPDTMOU
EXEVENT	CLC
	RTS
*
* KEYBOARD EVENT
*
SENDKEY	TAX
	BPL	SENDKBEV
	LDA	#$80	; DEBOUNCE KEYPRESS
	JSR	WAIT
	LDX	ANYKEY
	CPX	#$FF
	BNE	SENDKBEV
	LDA	$C062
	BPL	SENDKBEV
	LDA	$C061
	BPL	SENDKBEV
UNHOOK	JSR	EXIT
	LDA	#$00
	STA	RESET
	STA	RESET+1
	LDA	#$0D
	SEC
	RTS
SENDKBEV	STX	KEYSTAT
	LDA	#$82	; KBEVENT CODE
	JSR	SENDACC
	JSR	SENDMOD
	TXA
	CLC
	JMP	SENDACC
*
* MOUSE EVENTS
*
UPDTMOU	LDY	#$14	; READ MOUSE
	LDA	#$80
	JSR	CALLMOU
	BCC	EXMOU
	LDY	MOUINDEX
	LDA	$478,Y
	STA	MOUXSTAT
	LDA	$4F8,Y
	STA	MOUYSTAT
	LDA	$778,Y
	PHA
	LDY	#$15	; CLEAR MOUSE
	LDA	#$80
	JSR	CALLMOU
	LDA	MOUXSTAT
	ORA	MOUYSTAT
	BEQ	CHKBTTN
	LDA	#$84	; SEND MOUSE POSITION
	JSR	SENDACC
	LDA	MOUXSTAT
	JSR	SENDACC
	LDA	MOUYSTAT
	JSR	SENDACC
CHKBTTN	LDY	MOUINDEX
	PLA
	STA	$778,Y	; GS CLEARS THIS IN CLEAR MOUSE CALL
	AND	#$80
	CMP	BTTNSTAT
	BEQ	EXMOU
SENDBTTN	STA	BTTNSTAT
	LDA	#$86
	JSR	SENDACC
	JSR	SENDMOD
	LDA	BTTNSTAT
	JSR	SENDACC
EXMOU	CLC
	RTS
*
* CALL MOUSE FIRMWARE
*
CALLMOU	LDX	MOUROM
	BEQ	EXMOU
	PHA
	LDA	PTRL	; SAVE ZERO PAGE LOCATIONS
	STA	SAVEZP
	LDA	PTRH
	STA	SAVEZP+1
	LDA	#$00
	STA	PTRL
	STX	PTRH
	LDA	(PTR),Y
	STA	PTRL
	PLA
	LDX	MOUROM
	LDY	MOUSLOT
	PHP
	SEI
	JSR	IJMP
	PLP
	LDA	SAVEZP	; RESTORE ZERO PAGE LOCATIONS
	STA	PTRL
	LDA	SAVEZP+1
	STA	PTRH
	SEC	; MOUSE PRESENT FLAG
	RTS
*
* HOST REQUEST/RESPONSE HANDLING
*
RECVADDR	JSR	SENDACC	; ECHO REQUEST
	JSR	SENDACC	; DUMMY VALUE
	JSR	SENDACC	; DUMMY VALUE
	JSR	RECVACC
	STA	PTRL
	TXA	; ACK
	JSR	SENDACC
	JSR	RECVACC
	STA	PTRH
	TXA		; ACK
	JMP	SENDACC
RECVCNT	JSR	RECVACC
	STA	CNTL
	TXA		; ACK
	JSR	SENDACC
	JSR	RECVACC
	STA	CNTH	; INC FOR LOOP CONTROL
	INC	CNTH
	TXA		; ACK
	JMP	SENDACC
HOSTREQ	LDX	PTRL	; SAVE ZERO PAGE LOCATIONS
	STX	SAVEZP
	LDX	PTRH
	STX	SAVEZP+1
READREQ	CMP	#$90	; READ BYTES
	BNE	WRITEREQ
	LDX	#$91
	JSR	RECVADDR
	JSR	RECVCNT
	LDX	#$00
READLP	LDA	(PTR,X)
	JSR	SENDACC
	INC	PTRL
	BNE	READDEC
	INC	PTRH
READDEC	DEC	CNTL
	BNE	READLP
	DEC	CNTH
	BNE	READLP
	BEQ	OKREQ
WRITEREQ	CMP	#$92	; WRITE BYTES
	BNE	CALLREQ
	SEI		; DISABLE INTERRUPTS
	LDX	#$93
	JSR	RECVADDR
	JSR	RECVCNT
	LDX	#$00
WRITELP	JSR	RECVACC
	STA	(PTR,X)
	INC	PTRL
	BNE	WRITEDEC
	INC	PTRH
WRITEDEC	DEC	CNTL
	BNE	WRITELP
	DEC	CNTH
	BNE	WRITELP
	BEQ	OKREQ
CALLREQ	CMP	#$94
	BEQ	CALLREQ1
	CMP	#$9A
	BNE	CINREQ
CALLREQ1	TAX
	INX
	JSR	RECVADDR
	JSR	IJMP
OKREQ	LDX	#$9E	; REQ COMPLETE OK
SENDCOMP	CLI		; ENABLE INTERRUPTS
	PHP
	PHA
	TXA
	JSR	SENDACC
	PLA
	JSR	SENDACC	; ACCUM VALUE
	PLA
	JSR	SENDACC	; FLAGS VALUE
	LDA	SAVEZP	; RESTORE ZERO PAGE LOCATIONS
	STA	PTRL
	LDA	SAVEZP+1
	STA	PTRH
EXREQ	CLC
	RTS
CINREQ	CMP	#$96
	BNE	RESYNC
	LDX	#$97
	JSR	RECVADDR
	LDA	#$9E	; REQ COMPLETE OK
	JSR	SENDACC
	LDA	PTRL
	LDX	SAVEZP	; RESTORE ZERO PAGE LOCATIONS
	STX	PTRL
	LDX	SAVEZP+1
	STX	PTRH
	JSR	SENDACC	; ECHO VALUE
	JSR	SENDACC	; ECHO VALUE
	SEC		; RETURN CHAR TO RDKEY
	RTS
RESYNC	CMP	#$80	; RESYNC WITH HOST
	BNE	BADREQ
	JSR	SENDACC	; ECHO
	JSR	RECVACC
	CMP	#$81
	BEQ	EXREQ
* JMP UNHOOK
	BNE	RESYNC
BADREQ	LDX	#$9F	; REQ BAD
	BNE	SENDCOMP
*
* CLEANUP
*
EXIT	LDA	KBCLR
	JSR	$C300
	LDA	RESET+1
	BEQ	EX1
	STA	$3F3
	EOR	#$A5
	STA	$3F4
	LDA	RESET
	STA	$3F2
EX1	LDY	#$12	; SET MOUSE
	LDA	#$00	; DISABLE MOUSE
	STA	RESET
	STA	RESET+1
	JMP	CALLMOU
IJMP	JMP	(PTR)
MOUROM	DB	0
MOUSLOT	DB	0
SSCSLOT	DB	0
MOUINDEX	DB	0
MOUXSTAT	DB	0
MOUYSTAT	DB	0
BTTNSTAT	DB	0
KEYSTAT	DB	0
WAITEV	DB	0
CNTL	DB	0
CNTH	DB	0
RESET	DW	0
SAVEZP	DW	0
SAVEX	DB	0
SAVEY	DB	0
CHRIN	DB	0
SPINNER	ASC	"|\-/|\-/"
