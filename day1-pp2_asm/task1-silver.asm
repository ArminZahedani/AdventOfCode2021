; As a bit of a challenge this year, i am implementing the task of day 1 in PP2 Assembly (A processor used in the past at TU/e). The silver star is in this file, the gold star in the other. The final result is in R0.

; Data is hardcoded in the data segment, and the length needs to be set by yourself. No idea how to do that in PP2 assembly :).
@DATA
    data      DW   199, 200, 208, 210, 200, 207, 240, 269, 260, 263;
    length    DW   10;

@CODE
    begin :     BRA  setup

; R0 final count
; R1 loop count
; R2 previous value
; R3 current value
; R4 loop condition
; R5 scratch


    trap:
                BRA trap                    ; some kind of trap state

    setup :     LOAD R0 0                   ; contain counter
                LOAD R1 1                   ; loop counter
              
    main:       LOAD R5 R1
                ADD R5 data
                SUB R5 1                    ; Deduct one from loop counter since we start at 1
                LOAD R2 [GB + R5]           ; Load [GB + data + R1 (loop counter)] into R2 to get the 'old' value

                LOAD R4 [GB + length]       ; Load the maximum length into R4 to compare
                CMP R1 R4
                BEQ trap
                
                LOAD R5 R1
                ADD R5 data
                LOAD R3 [GB + R5]           ; Load [GB + data + R1 (loop counter)] into R3 to get the current value
                ADD R1 1                    ; Add 1 to loop counter
                
                CMP R2 R3
                BGE main                    ; Compare R2 with R3. If R2 >= R3 jump straight back to main

                ADD R0 1                    ; Otherwise, add 1 to the counter and then jump back
                BRA main
@END