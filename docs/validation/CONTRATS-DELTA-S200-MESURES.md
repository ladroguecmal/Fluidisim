# S200 — Rejeu du filtre S199

2026-09-13, release ; empreinte attendue et obtenue0x0ad3f695685ca27a.
Le texte initial du programme conserve son rappel de tests S199 ; les limites
exactes sont dans CONTRATS-DELTA-S200 et CANDIDAT-DELTA-S199 §7.

```text
S199 — filtres d'entree du premier candidat delta (ADR-038 §4)
domaine physique 8 x 4 m, rho=1025, g=9.81, dt=0.002, eta = z0 + 0.01·sin(2πx/L)

-- filtre 1 : equilibrage
  lac au repos sur fond coupe, 1000 pas : vitesse exactement nulle **en bits**
  idem a g = 1,62 / 9,81 / 24,79 : exact dans les trois cas
  (recu par les tests unitaires du module ; voir tests_volume.rs)

-- filtre 2 : ordre en espace, un pas depuis le repos
fond        | cellules | debit median (m2/s) | residu | iterations
plat        |       32 | +2.288879177e-4 | 6.36e-7 |         57
plat        |       64 | +2.295123618e-4 | 9.58e-7 |        111
plat        |      128 | +2.296743546e-4 | 9.05e-7 |        217
  -> ordre = 1.947 | residu de Richardson a dx/2 = 0.025 %
lisse       |       32 | +2.317928411e-4 | 8.85e-7 |         94
lisse       |       64 | +2.282843388e-4 | 8.87e-7 |        178
lisse       |      128 | +2.264018967e-4 | 9.38e-7 |        345
  -> ordre = 0.898 | residu de Richardson a dx/2 = 0.963 %
avec marche |       32 | +2.314901603e-4 | 8.38e-7 |         92
avec marche |       64 | +2.280175254e-4 | 8.89e-7 |        173
avec marche |      128 | +2.261498116e-4 | 9.21e-7 |        324
  -> ordre = 0.895 | residu de Richardson a dx/2 = 0.961 %

-- verdict ADR-038 §4
  fond plat        : PASSE, ordre 1.95
  fond lisse       : ELIMINE, ordre 0.90 — C04 hors d'atteinte
  fond avec marche : ELIMINE, ordre 0.89 — C04 hors d'atteinte

-- determinisme (I-03)
  deux executions du meme montage : identiques (bits 0x3f2de2ffa1000000 contre 0x3f2de2ffa1000000)

empreinte (deux executions doivent la reproduire) : 0x0ad3f695685ca27a
```

Workspace debug :
```text
test result: ok. 246 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 53.44s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 93 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 50.15s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
