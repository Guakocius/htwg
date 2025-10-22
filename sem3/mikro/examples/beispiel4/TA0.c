#include <msp430.h>
#include "TA0.h"

// Zeitspanne = 500 ms
// Timer-Clock = 613.75 kHz
// Teilungsfaktor = Timer-Clock * Zeitspanne = 306875
// 16 Bit-Timer: Max. Schritte 2^16
// Skalierungsfaktor = 306875 / 2^16 = 4.6825
//  => 5 = {/1} {/5}
// Einstellung CCR0 = 306875 / 5 = 61375

#define ACLKFRQ (613.75e3)
#define TICK(t) ((UInt)((ACLKFRQ * (t)) / 5.0) - 1)
#define INITVAL (TICK(500e-3))

#pragma FUNC_ALWAYS_INLINE(TA0_init)
GLOBAL inline Void TA0_init(Void) {
   TA0CTL   = 0; // stop mode, disable and clear flags
   TA0CCTL0 = 0; // no capture mode, compare mode
                 // clear and disable interrupt flag
   TA0CCR0  = INITVAL;     // set up Compare Register
   TA0EX0   = TAIDEX_4;    // set up expansion register /5
   TA0CTL   = TASSEL__ACLK // 613.75 kHz
            | MC__UP       // Up Mode
            | ID__1        // input divider /1
            | TACLR        // clear and start Timer
            | TAIE;        // enable interrupt
}

#pragma vector = TIMER0_A1_VECTOR
__interrupt Void TIMER0_A1_ISR(Void) {
   CLRBIT(TA0CTL, TAIFG);    // clear interrupt flag
   TGLBIT(P2OUT, BIT7);      // toggle led
}

