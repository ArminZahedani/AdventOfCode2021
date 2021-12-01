; As a bit of a challenge this year, i am implementing the task of day 1 in PP2 Assembly (A processor used in the past at TU/e). The silver star is in the other file, and the gold star is in this file. The final result is in R0.

; The main change to the silver file is that R1 starts at 0 and not at 1. The idea is basically that we do not have to sum the actual window, but can just check whether the new element that enters the window is larger than the element that leaves the window.

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
                LOAD R1 0                   ; loop counter

                LOAD R5 [GB + length]       ; Load the length
                SUB R5 3                    ; Subtract 3 since the sliding window has to end at the last element
                STOR R5 [GB + length]       ; Store it again in length
              
    main:       LOAD R5 R1
                ADD R5 data
                LOAD R2 [GB + R5]           ; Load into R2 the current value  at the index

                LOAD R4 [GB + length]       ; Load into R4 the loop condition and compare to R1 (the loop variant). If done, we jump to trap state
                CMP R1 R4
                BEQ trap

                LOAD R5 R1
                ADD R5 data
                ADD R5 3
                LOAD R3 [GB + R5]           ; Load the element at [GB + data + R1 (loop counter) + 3] into the R3. This is the element that comes in for R2.
                ADD R1 1                    ; Add 1 to the loop counter for next iteration

                CMP R2 R3                   ; Compare old and new value
                BGE main                    ; If the old value is greater than or equal to the new value, we do not add to the counter and jump directly back.

                ADD R0 1                    ; Add 1 to counter if R3 is bigger than R2.
                BRA main
@END