#include <msp430.h> /* --> msp430fr5729.h */
#include "..\base.h"

// die LED am Pin 7 vom Port 2 soll mit
// einer Frequenz von 1 Hz blinken
// => High dauert 0.5 Sek, Low dauert 0.5 Sek.

GLOBAL Int _system_pre_init(Void) {
   // stop watchdog timer
   WDTCTL = WDTPW + WDTHOLD;
   return 0;
}

//   DCO: 8.0 MHz
//  ACLK: DCO /  8 =   1.0 MHz
// SMCLK: DCO / 32 = 250.0 kHz
//  MCLK: DCO /  1 =   8.0 MHz
#pragma FUNC_ALWAYS_INLINE(CS_init)
LOCAL inline Void CS_init(Void) {
   CSCTL0_H = CSKEY_H;    // enable clock system
   CSCTL1 = DCOFSEL_3;    // DCO frequency = 8.0 MHz
                          // select clock sources
   CSCTL2 = SELA__DCOCLK  // ACLK  <- DCO
          | SELS__DCOCLK  // SMCLK <- DCO
          | SELM__DCOCLK; // MCLK  <- DCO
                          // set frequency divider
   CSCTL3 = DIVA__8       // ACLK  : /8
          | DIVS__32      // SMCLK : /32
          | DIVM__1;      // MCLK  : /1
   CSCTL4 = XT1OFF        // XT1 disabled
          | XT2OFF;       // XT2 disabled
   CSCTL5 = 0;            // disable and clear fault flags
   CSCTL6 = 0;            // disable conditional requests
   CSCTL0_H = 0;          // disable clock system
}

#define VAL_16BIT(arg1, arg2) ((unsigned)(((arg1) << 8) | (arg2)))

#pragma FUNC_ALWAYS_INLINE(GPIO_init)
LOCAL Void GPIO_init(Void) {
    // Port 2: Pin 7 => output, LED1
    // Port 1: Pin 2 => output, LED2

   //                   Port2       Port1
   //               Bit 76543210    76543210
   PAOUT  = VAL_16BIT(0b00000000, 0b00000000); // clear all outputs
   PADIR  = VAL_16BIT(0b10000000, 0b00000100); // direction, set outputs
   PAIFG  = VAL_16BIT(0b00000000, 0b00000000); // clear all interrupt flags
   PAIE   = VAL_16BIT(0b00000000, 0b00000000); // disable all GPIO interrupts
   PASEL0 = VAL_16BIT(0b00000000, 0b00000000);
   PASEL1 = VAL_16BIT(0b00000000, 0b00000000);
   PAREN  = VAL_16BIT(0b00000000, 0b00000000); // without pull up

   //                   Port4       Port3
   //               Bit 76543210    76543210
   PBOUT  = VAL_16BIT(0b00000000, 0b00000000); // clear all outputs
   PBDIR  = VAL_16BIT(0b00000000, 0b00000000); // direction, set outputs
   PBIFG  = VAL_16BIT(0b00000000, 0b00000000); // clear all interrupt flags
   PBIE   = VAL_16BIT(0b00000000, 0b00000000); // disable all GPIO interrupts
   PBSEL0 = VAL_16BIT(0b00000000, 0b00000000);
   PBSEL1 = VAL_16BIT(0b00000000, 0b00000000);
   PBREN  = VAL_16BIT(0b00000000, 0b00000000); // without pull up

}

// Zeitspanne = 500.0 ms
// Timer-Clock = 1.0 MHz
// Teilungsfaktor = Timer-Clock * Zeitspanne = 500000
// 16 Bit-Timer: Max. Schritte 2^16
// Skalierungsfaktor = 500000 / 2^16 = 7.6294 => 8 = {/8} {/1}
// Einstellung CCR0 = 500000 / 8 = 62500

#define ACLKFRQ (1.0e6)
#define DELAY   (500e-3)
#define TICKS   (DELAY * ACLKFRQ)
#define INITVAL (((UInt)(TICKS / 8)) - 1)

#pragma FUNC_ALWAYS_INLINE(TA0_init)
LOCAL Void TA0_init(Void) {
   TA0CTL   = 0; // stop mode, disable and clear flags
   TA0CCTL0 = 0; // no capture mode, compare mode
                 // clear and disable interrupt flag
   TA0CCR0  = INITVAL;     // set up Compare Register CCR0
   TA0EX0   = TAIDEX_0;    // set up expansion register /1
   TA0CTL   = TASSEL__ACLK // 1.0 MHz
            | MC__UP       // Up Mode
            | ID__8        // input divider /8
            | TACLR;       // clear and start Timer
}

/**
 * main.c
 */
GLOBAL Void main(Void) {

   CS_init();
   GPIO_init();
   TA0_init();
   SETBIT(P1OUT,  BIT2); // set led 2

   while(TRUE) {
      while (TSTBIT(TA0CTL, TAIFG) EQ 0) ;  // wait on interrupt
      TGLBIT(P2OUT,  BIT7);                 // toggle led 1
      CLRBIT(TA0CTL, TAIFG);                // clear interrupt flag
   }
}
