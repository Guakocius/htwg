| regulärer Ausdruck a | zugehörige Sprache L(a) |

---|---
| a | b | {a, b}
| (a&#124;b) (a&#124;b) | {{a,a},{a,b},{b,b}}
| a(a&#124;b) &#124; b(a|b) | {{a,a,b,a}, {a,b,b,a},...}
| a\* | {{},a}
| {}\* | {ε}
| ε\* | {ε}
