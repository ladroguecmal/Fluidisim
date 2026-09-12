# S202 — Coûts bruts et vérification

2026-09-13, AMD Ryzen AI7 350, Windows x86_64, Rust1.97.0.
Commande depuis code : cargo run -p water-core --release --example block_cost.
Coûts uniques par série de onze mesures, aucune promesse de borne.

```text
S202 release CPU sequentiel ; profil60Hz eau2ms ; dt=1/60s ; domaine8x4m fond plat0.4m eta=z0+0.02sin(2pi*x/8)
3 chauffes +11 mesures, vitesses reinitialisees avant chaque pas ; aucune I/O/configuration incluse ; un domaine=un bloc
nx nz dx limite iterations degrade residu median_ms max_ms max_sur_2ms cout_seul_compatible
16 8 0.500 1 1 true 3.25122e-1 0.011200 0.011500 0.006 true
16 8 0.500 64 30 false 8.10605e-7 0.079300 0.093600 0.047 true
16 8 0.500 512 30 false 8.10605e-7 0.079400 0.079500 0.040 true
32 16 0.250 1 1 true 3.30167e-1 0.031600 0.031800 0.016 true
32 16 0.250 64 60 false 7.21670e-7 0.589600 0.903800 0.452 true
32 16 0.250 512 60 false 7.21670e-7 0.586500 0.656700 0.328 true
64 32 0.125 1 1 true 3.32400e-1 0.113800 0.114000 0.057 true
64 32 0.125 64 64 true 1.82025e-3 2.405100 2.702800 1.351 false
64 32 0.125 512 113 false 9.72040e-7 4.788600 5.320100 2.660 false
Compatible en cout seul != admissible physiquement : filtre fond coupe, surface mobile et precision non recus ; cout B/W/rendu absent du tableau.
```

Workspace debug :
```text
test result: ok. 246 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 52.72s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 93 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 52.93s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Quatre tests intégration delta_runtime passent aussi en release.
