// generated by edc2ports on 2023-06-17 13:44:48.238228+00:00

// Port definitions for pic32mx1xxfxxxb
//
// PORTA: ---------------------------dddaa
// PORTB: ----------------aaaaddddddddaaaa
//
#[cfg(feature = "pic32mx1xxfxxxb")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Analog>, true),
    RA1: (ra1, 1, Input<Analog>, true),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
]);

#[cfg(feature = "pic32mx1xxfxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB6: (rb6, 6, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB10: (rb10, 10, Input<Floating>),
    RB11: (rb11, 11, Input<Floating>),
    RB12: (rb12, 12, Input<Analog>, true),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);


// Port definitions for pic32mx2xxfxxxb
//
// PORTA: ---------------------------dddaa
// PORTB: ----------------aaa-ddddd-ddaaaa
//
#[cfg(feature = "pic32mx2xxfxxxb")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Analog>, true),
    RA1: (ra1, 1, Input<Analog>, true),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
]);

#[cfg(feature = "pic32mx2xxfxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB10: (rb10, 10, Input<Floating>),
    RB11: (rb11, 11, Input<Floating>),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);


// Port definitions for pic32mx2x4fxxxb
//
// PORTA: ---------------------------dddaa
// PORTB: ----------------aaa---ddd-ddaaaa
//
#[cfg(feature = "pic32mx2x4fxxxb")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Analog>, true),
    RA1: (ra1, 1, Input<Analog>, true),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
]);

#[cfg(feature = "pic32mx2x4fxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);


// Port definitions for pic32mx37x
//
// PORTA: ----------------dd---aa-dddddddd
// PORTB: ----------------aaaaaaaaaaaaaaaa
// PORTC: ----------------dddd-------dddd-
// PORTD: ----------------ddddddddddddaaad
// PORTE: ----------------------ddaaaadadd
// PORTF: ------------------dd---ddddddddd
// PORTG: ----------------dddd--aaaa--dddd
//
#[cfg(feature = "pic32mx37x")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Floating>),
    RA1: (ra1, 1, Input<Floating>),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
    RA5: (ra5, 5, Input<Floating>),
    RA6: (ra6, 6, Input<Floating>),
    RA7: (ra7, 7, Input<Floating>),
    RA9: (ra9, 9, Input<Analog>, true),
    RA10: (ra10, 10, Input<Analog>, true),
    RA14: (ra14, 14, Input<Floating>),
    RA15: (ra15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Analog>, true),
    RB5: (rb5, 5, Input<Analog>, true),
    RB6: (rb6, 6, Input<Analog>, true),
    RB7: (rb7, 7, Input<Analog>, true),
    RB8: (rb8, 8, Input<Analog>, true),
    RB9: (rb9, 9, Input<Analog>, true),
    RB10: (rb10, 10, Input<Analog>, true),
    RB11: (rb11, 11, Input<Analog>, true),
    RB12: (rb12, 12, Input<Analog>, true),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTC, portc, [
    RC1: (rc1, 1, Input<Floating>),
    RC2: (rc2, 2, Input<Floating>),
    RC3: (rc3, 3, Input<Floating>),
    RC4: (rc4, 4, Input<Floating>),
    RC12: (rc12, 12, Input<Floating>),
    RC13: (rc13, 13, Input<Floating>),
    RC14: (rc14, 14, Input<Floating>),
    RC15: (rc15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTD, portd, [
    RD0: (rd0, 0, Input<Floating>),
    RD1: (rd1, 1, Input<Analog>, true),
    RD2: (rd2, 2, Input<Analog>, true),
    RD3: (rd3, 3, Input<Analog>, true),
    RD4: (rd4, 4, Input<Floating>),
    RD5: (rd5, 5, Input<Floating>),
    RD6: (rd6, 6, Input<Floating>),
    RD7: (rd7, 7, Input<Floating>),
    RD8: (rd8, 8, Input<Floating>),
    RD9: (rd9, 9, Input<Floating>),
    RD10: (rd10, 10, Input<Floating>),
    RD11: (rd11, 11, Input<Floating>),
    RD12: (rd12, 12, Input<Floating>),
    RD13: (rd13, 13, Input<Floating>),
    RD14: (rd14, 14, Input<Floating>),
    RD15: (rd15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTE, porte, [
    RE0: (re0, 0, Input<Floating>),
    RE1: (re1, 1, Input<Floating>),
    RE2: (re2, 2, Input<Analog>, true),
    RE3: (re3, 3, Input<Floating>),
    RE4: (re4, 4, Input<Analog>, true),
    RE5: (re5, 5, Input<Analog>, true),
    RE6: (re6, 6, Input<Analog>, true),
    RE7: (re7, 7, Input<Analog>, true),
    RE8: (re8, 8, Input<Floating>),
    RE9: (re9, 9, Input<Floating>),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTF, portf, [
    RF0: (rf0, 0, Input<Floating>),
    RF1: (rf1, 1, Input<Floating>),
    RF2: (rf2, 2, Input<Floating>),
    RF3: (rf3, 3, Input<Floating>),
    RF4: (rf4, 4, Input<Floating>),
    RF5: (rf5, 5, Input<Floating>),
    RF6: (rf6, 6, Input<Floating>),
    RF7: (rf7, 7, Input<Floating>),
    RF8: (rf8, 8, Input<Floating>),
    RF12: (rf12, 12, Input<Floating>),
    RF13: (rf13, 13, Input<Floating>),
]);

#[cfg(feature = "pic32mx37x")]
port!(PORTG, portg, [
    RG0: (rg0, 0, Input<Floating>),
    RG1: (rg1, 1, Input<Floating>),
    RG2: (rg2, 2, Input<Floating>),
    RG3: (rg3, 3, Input<Floating>),
    RG6: (rg6, 6, Input<Analog>, true),
    RG7: (rg7, 7, Input<Analog>, true),
    RG8: (rg8, 8, Input<Analog>, true),
    RG9: (rg9, 9, Input<Analog>, true),
    RG12: (rg12, 12, Input<Floating>),
    RG13: (rg13, 13, Input<Floating>),
    RG14: (rg14, 14, Input<Floating>),
    RG15: (rg15, 15, Input<Floating>),
]);


// Port definitions for pic32mx47x
//
// PORTA: ----------------dd---aa-dddddddd
// PORTB: ----------------aaaaaaaaaaaaaaaa
// PORTC: ----------------dddd-------dddd-
// PORTD: ----------------ddddddddddddaaad
// PORTE: ----------------------ddaaaadadd
// PORTF: ------------------dd---d--dddddd
// PORTG: ----------------dddd--aaaa----dd
//
#[cfg(feature = "pic32mx47x")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Floating>),
    RA1: (ra1, 1, Input<Floating>),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
    RA5: (ra5, 5, Input<Floating>),
    RA6: (ra6, 6, Input<Floating>),
    RA7: (ra7, 7, Input<Floating>),
    RA9: (ra9, 9, Input<Analog>, true),
    RA10: (ra10, 10, Input<Analog>, true),
    RA14: (ra14, 14, Input<Floating>),
    RA15: (ra15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Analog>, true),
    RB5: (rb5, 5, Input<Analog>, true),
    RB6: (rb6, 6, Input<Analog>, true),
    RB7: (rb7, 7, Input<Analog>, true),
    RB8: (rb8, 8, Input<Analog>, true),
    RB9: (rb9, 9, Input<Analog>, true),
    RB10: (rb10, 10, Input<Analog>, true),
    RB11: (rb11, 11, Input<Analog>, true),
    RB12: (rb12, 12, Input<Analog>, true),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTC, portc, [
    RC1: (rc1, 1, Input<Floating>),
    RC2: (rc2, 2, Input<Floating>),
    RC3: (rc3, 3, Input<Floating>),
    RC4: (rc4, 4, Input<Floating>),
    RC12: (rc12, 12, Input<Floating>),
    RC13: (rc13, 13, Input<Floating>),
    RC14: (rc14, 14, Input<Floating>),
    RC15: (rc15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTD, portd, [
    RD0: (rd0, 0, Input<Floating>),
    RD1: (rd1, 1, Input<Analog>, true),
    RD2: (rd2, 2, Input<Analog>, true),
    RD3: (rd3, 3, Input<Analog>, true),
    RD4: (rd4, 4, Input<Floating>),
    RD5: (rd5, 5, Input<Floating>),
    RD6: (rd6, 6, Input<Floating>),
    RD7: (rd7, 7, Input<Floating>),
    RD8: (rd8, 8, Input<Floating>),
    RD9: (rd9, 9, Input<Floating>),
    RD10: (rd10, 10, Input<Floating>),
    RD11: (rd11, 11, Input<Floating>),
    RD12: (rd12, 12, Input<Floating>),
    RD13: (rd13, 13, Input<Floating>),
    RD14: (rd14, 14, Input<Floating>),
    RD15: (rd15, 15, Input<Floating>),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTE, porte, [
    RE0: (re0, 0, Input<Floating>),
    RE1: (re1, 1, Input<Floating>),
    RE2: (re2, 2, Input<Analog>, true),
    RE3: (re3, 3, Input<Floating>),
    RE4: (re4, 4, Input<Analog>, true),
    RE5: (re5, 5, Input<Analog>, true),
    RE6: (re6, 6, Input<Analog>, true),
    RE7: (re7, 7, Input<Analog>, true),
    RE8: (re8, 8, Input<Floating>),
    RE9: (re9, 9, Input<Floating>),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTF, portf, [
    RF0: (rf0, 0, Input<Floating>),
    RF1: (rf1, 1, Input<Floating>),
    RF2: (rf2, 2, Input<Floating>),
    RF3: (rf3, 3, Input<Floating>),
    RF4: (rf4, 4, Input<Floating>),
    RF5: (rf5, 5, Input<Floating>),
    RF8: (rf8, 8, Input<Floating>),
    RF12: (rf12, 12, Input<Floating>),
    RF13: (rf13, 13, Input<Floating>),
]);

#[cfg(feature = "pic32mx47x")]
port!(PORTG, portg, [
    RG0: (rg0, 0, Input<Floating>),
    RG1: (rg1, 1, Input<Floating>),
    RG6: (rg6, 6, Input<Analog>, true),
    RG7: (rg7, 7, Input<Analog>, true),
    RG8: (rg8, 8, Input<Analog>, true),
    RG9: (rg9, 9, Input<Analog>, true),
    RG12: (rg12, 12, Input<Floating>),
    RG13: (rg13, 13, Input<Floating>),
    RG14: (rg14, 14, Input<Floating>),
    RG15: (rg15, 15, Input<Floating>),
]);


