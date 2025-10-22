#include <msp430.h>
#include "TA0.h"
#include "event.h"

// Zeitspanne = 500 ms
// Timer-Clock = 1.0 MHz
// Teilungsfaktor = Timer-Clock * Zeitspanne = 500000
// 16 Bit-Timer: Max. Schritte 2^16
// Skalierungsfaktor = 500000 / 2^16 = 7.6294 => 8 = {/8} {/1}
// Einstellung CCR0 = 500000 / 8 = 62500

#define ACLKFRQ  (1.0e6)
#define TICK(t)  ((UInt)((ACLKFRQ * (t)) / 8.0) - 1)
#define INITVAL  (TICK(500.0e-3))

#pragma FUNC_ALWAYS_INLINE(TA0_init)
GLOBAL inline Void TA0_init(Void) {
   TA0CTL   = 0; // stop mode, disable and clear flags
   TA0CCTL0 = 0; // no capture mode, compare mode
                 // clear and disable interrupt flag
   TA0CCR0  = INITVAL;      // set up Compare Register CCR0
   TA0EX0   = TAIDEX_0;     // set up expansion register /1
   TA0CTL   = TASSEL__ACLK  // 1.0 MHz
            | MC__UP        // Up Mode
            | ID__8         // input divider /8
            | TACLR         // clear and start Timer
            | TAIE;         // enable interrupt
}

#pragma vector = TIMER0_A1_VECTOR
__interrupt Void TIMER0_A1_ISR(Void) {
   Event_set(EVENT_IMA);            // set up event
   CLRBIT(TA0CTL, TAIFG);           // clear interrupt flag
   __low_power_mode_off_on_exit();  // restore Active Mode on return
}
