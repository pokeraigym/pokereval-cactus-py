# pokerval-cactus
A python frontend to the Rust implementation of the python pokerhand evaluation library treys. This library basically implements the exact same API but, depending on the amounts of cards in the hand checked, up to 5 times faster than treys.

Comparision using treys own performance testing script:
```
7 card evaluation:
[*] Treys: Average time per evaluation: 0.000034
[*] Treys: Evaluations per second = 29755.067004
6 card evaluation:
[*] Treys: Average time per evaluation: 0.000011
[*] Treys: Evaluations per second = 95217.538371
5 card evaluation:
[*] Treys: Average time per evaluation: 0.000002
[*] Treys: Evaluations per second = 454642.458403
```
```
7 card evaluation:
[*] Cactus: Average time per evaluation: 0.000006
[*] Cactus: Evaluations per second = 156212.439479
6 card evaluation:
[*] Cactus: Average time per evaluation: 0.000004
[*] Cactus: Evaluations per second = 256347.345647
5 card evaluation:
[*] Cactus: Average time per evaluation: 0.000002
[*] Cactus: Evaluations per second = 476095.257554
```

