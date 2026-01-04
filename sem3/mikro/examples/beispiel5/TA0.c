#include <msp430.h>
#include "TA0.h"

// Zeitspanne = MAX (740 ms, 260 ms, 330 ms, 1450 ms) = 1450 ms
// Timer-Clock = 613.75 kHz
// Teilungsfaktor = Timer-Clock * Zeitspanne = 889937.5
// 16 Bit-Timer: max. Schritte 2^15
// Skalierungsfaktor = 889937.5 / 2^15 = 27.1587 => 28 = {/4} {/7}

#define HIGH     (0x8000)
#define LOW      (0x0000)

#define ACLKFRQ  (613.75e3)
#define TICK(t)  ((UInt)((ACLKFRQ * (t)) / 28.0) - 1)
#define TABSIZE  (5)

LOCAL const Int muster[TABSIZE] = {
   HIGH | TICK( 740e-3), // High
   LOW  | TICK( 260e-3), // Low
   HIGH | TICK( 330e-3), // High
   LOW  | TICK(1450e-3), // Low
   0};

LOCAL const Int *ptr;

#pragma FUNC_ALWAYS_INLINE(TA0_init)
GLOBAL inline Void TA0_init(Void) {
   ptr  = &muster[0];

   TA0CTL   = 0; // stop mode, disable and clear flags
   TA0CCTL0 = 0; // no capture mode, compare mode
                 // clear and disable interrupt flag
   TA0CCR0  = 0;           // set up Compare Register
   TA0EX0   = TAIDEX_6;    // set up expansion register /7
   TA0CTL   = TASSEL__ACLK // 613.75 kHz
            | MC__UP       // Up Mode
            | ID__4        // input divider /4
            | TACLR        // clear and start Timer
            | TAIE         // enable interrupt
            | TAIFG;       // set interrupt flag
}

#pragma vector = TIMER0_A1_VECTOR
__interrupt Void TIMER0_A1(Void) {

   if (*ptr EQ 0) {
      ptr = &muster[0];
   }

   UInt cnt = *ptr++;

   if (TSTBIT(cnt, HIGH)) {
      SETBIT(P2OUT, BIT7);
      CLRBIT(cnt, HIGH);
   } else {
      CLRBIT(P2OUT, BIT7); 
   }

   CLRBIT(TA0CTL, TAIFG);
   TA0CCR0 = cnt;
}
