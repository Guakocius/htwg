#include <msp430.h>
#include "TA1.h"

// Zeitspanne = 3.3 ms
// Timer-Clock = 613.75 kHz
// Teilungsfaktor = Timer-Clock * Zeitspanne = 2025.375
// 16 Bit-Timer: Max. Schritte 2^16
// Skalierungsfaktor = 2025.375 / 2^16 = 0.0309
//  => 1 = {/1} {/1}
// Einstellung CCR0 = 2025.375 / 1 = 2025

#define ACLKFRQ (613.75e3)
#define TICK(t) ((UInt)((ACLKFRQ * (t)) / 1.0) - 1)
#define INITVAL (TICK(3.3e-3))

#pragma FUNC_ALWAYS_INLINE(TA1_init)
GLOBAL inline Void TA1_init(Void) {
   TA1CTL   = 0; // stop mode, disable and clear flags
   TA1CCTL0 = 0; // no capture mode, compare mode
                 // clear and disable interrupt flag
   TA1CCR0  = INITVAL;     // set up Compare Register
   TA1EX0   = TAIDEX_0;    // set up expansion register
   TA1CTL   = TASSEL__ACLK // 613.75 kHz
            | MC__UP       // Up Mode
            | ID__1        // input divider
            | TACLR        // clear and start Timer
            | TAIE;        // enable interrupt
}

#pragma vector = TIMER1_A1_VECTOR
__interrupt Void TIMER1_A1_ISR(Void) {
   SETBIT(P3OUT, BIT4);
   CLRBIT(TA1CTL, TAIFG); // clear interrupt flag
   if (TSTBIT(~P1IN, BIT0 | BIT1)) {
      SETBIT(P1OUT, BIT2);
   } else {
      CLRBIT(P1OUT, BIT2);
   }
   CLRBIT(P3OUT, BIT4);
}
