# S220 — Relevés bruts

AMD Ryzen AI 7 350, 32 Go, Windows 11, rustc 1.97.0, `cargo run --offline --release -p water-core --example ordre_deux_s220 [fixture]`.
Chaque passage est un processus isolé, lancé après la suite de tests, sans compilation concurrente.
Le détail `max_leaf` est calculé **hors chronométrage**, après chaque partition.

## Base, passage 1 (15:43, sans détail de feuille)

```text
S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.332 image_time_admitted=true modes=4096
fixture=base grid_step=2 rectangles=3072 bound=0.115437612 first_order_bound=0.115437612 second_order_branch_max=0.209338635 gain_over_first=1.000000 gain_over_global=0.997691 bound_over_reference=1.641532 tighter_rectangles=1332 hessian_modes_mean=2168.0 hessian_modes_max=2168 grid_ms=2826.691
fixture=base grid_step=1 rectangles=12288 bound=0.109988004 first_order_bound=0.115437612 second_order_branch_max=0.109988004 gain_over_first=1.049547 gain_over_global=1.047124 bound_over_reference=1.564038 tighter_rectangles=12288 hessian_modes_mean=4032.0 hessian_modes_max=4032 grid_ms=11914.331
fixture=base grid_step=0.5 rectangles=49152 bound=0.079500645 first_order_bound=0.112293623 second_order_branch_max=0.079500645 gain_over_first=1.412487 gain_over_global=1.448681 bound_over_reference=1.130505 tighter_rectangles=49152 hessian_modes_mean=4096.0 hessian_modes_max=4096 grid_ms=47287.839
fixture=base order=First budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=1120.676 pool_bytes=655360
fixture=base order=First budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=4008.892 pool_bytes=655360
fixture=base order=First budget=32767 evaluations=32767 leaves=16384 bound=0.098479681 gain=1.169491 bound_over_reference=1.400389 stop=Evaluations partition_ms=17861.450 pool_bytes=655360
fixture=base order=First budget=65535 evaluations=65535 leaves=32768 bound=0.077374868 gain=1.488482 bound_over_reference=1.100277 stop=Evaluations partition_ms=37217.921 pool_bytes=655360
fixture=base order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=2037.301 pool_bytes=655360
fixture=base order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=7864.701 pool_bytes=655360
fixture=base order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.070740171 gain=1.628086 bound_over_reference=1.005931 stop=Evaluations partition_ms=33802.727 pool_bytes=655360
fixture=base order=Second budget=65535 evaluations=65535 leaves=32768 bound=0.070666455 gain=1.629784 bound_over_reference=1.004882 stop=Evaluations partition_ms=63486.732 pool_bytes=655360
```

## Lente (15:47–15:51)

```text
S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee
fixture=lent age=9.806095 reference_s217=0.014352296 global=0.032634243 preparation_ms=6.507 image_time_admitted=true modes=4096
fixture=lent grid_step=2 rectangles=3072 bound=0.032723587 first_order_bound=0.032723587 second_order_branch_max=0.051593281 gain_over_first=1.000000 gain_over_global=0.997270 bound_over_reference=2.280025 tighter_rectangles=2508 hessian_modes_mean=2168.0 hessian_modes_max=2168 grid_ms=2821.073
fixture=lent grid_step=1 rectangles=12288 bound=0.024797585 first_order_bound=0.032723587 second_order_branch_max=0.024797585 gain_over_first=1.319628 gain_over_global=1.316025 bound_over_reference=1.727778 tighter_rectangles=12288 hessian_modes_mean=4032.0 hessian_modes_max=4032 grid_ms=11762.335
fixture=lent grid_step=0.5 rectangles=49152 bound=0.017090661 first_order_bound=0.026652928 second_order_branch_max=0.017090661 gain_over_first=1.559502 gain_over_global=1.909478 bound_over_reference=1.190796 tighter_rectangles=49152 hessian_modes_mean=4096.0 hessian_modes_max=4096 grid_ms=46001.019
fixture=lent order=First budget=2047 evaluations=2047 leaves=1024 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=1094.781 pool_bytes=655360
fixture=lent order=First budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.032723587 center_slope=0.000124064 first_branch=0.032723587 first_reserve=0.000089339 second_branch=0.063259780 second_reserve=0.000304241 corner_slope=0.000464482 second_remainder=0.062491037 hessian_modes=1246
fixture=lent order=First budget=8191 evaluations=8191 leaves=4096 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=4314.054 pool_bytes=655360
fixture=lent order=First budget=8191 max_leaf=[50.000000,52.000000]x[-15.000000,-13.500000] width=2.000000 height=1.500000 cell_bound=0.032723587 center_slope=0.000028376 first_branch=0.032723587 first_reserve=0.000089339 second_branch=0.024598183 second_reserve=0.000345471 corner_slope=0.000045899 second_remainder=0.024206810 hessian_modes=2488
fixture=lent order=First budget=32767 evaluations=32767 leaves=16384 bound=0.030864052 gain=1.057354 bound_over_reference=2.150461 stop=Evaluations partition_ms=17318.819 pool_bytes=655360
fixture=lent order=First budget=32767 max_leaf=[45.000000,46.000000]x[-1.500000,0.000000] width=1.000000 height=1.500000 cell_bound=0.030864052 center_slope=0.000001892 first_branch=0.030864052 first_reserve=0.000089339 second_branch=0.012861012 second_reserve=0.000302114 corner_slope=0.000005216 second_remainder=0.012553681 hessian_modes=3418
fixture=lent order=First budget=65535 evaluations=65535 leaves=32768 bound=0.021462433 gain=1.520529 bound_over_reference=1.495401 stop=Evaluations partition_ms=34910.028 pool_bytes=655360
fixture=lent order=First budget=65535 max_leaf=[32.000000,33.000000]x[-35.250000,-34.500000] width=1.000000 height=0.750000 cell_bound=0.021462433 center_slope=0.000005978 first_branch=0.021462433 first_reserve=0.000089339 second_branch=0.006374685 second_reserve=0.000264784 corner_slope=0.000010405 second_remainder=0.006099494 hessian_modes=4096
fixture=lent order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=1777.233 pool_bytes=655360
fixture=lent order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.032723587 center_slope=0.000124064 first_branch=0.032723587 first_reserve=0.000089339 second_branch=0.063259780 second_reserve=0.000304241 corner_slope=0.000464482 second_remainder=0.062491037 hessian_modes=1246
fixture=lent order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=7248.256 pool_bytes=655360
fixture=lent order=Second budget=8191 max_leaf=[-6.000000,-4.000000]x[1.500000,3.000000] width=2.000000 height=1.500000 cell_bound=0.032723587 center_slope=0.006274606 first_branch=0.032723587 first_reserve=0.000089339 second_branch=0.032828312 second_reserve=0.000345470 corner_slope=0.008276356 second_remainder=0.024206480 hessian_modes=2488
fixture=lent order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.014519664 gain=2.247589 bound_over_reference=1.011662 stop=Evaluations partition_ms=29723.404 pool_bytes=655360
fixture=lent order=Second budget=32767 max_leaf=[1.648438,1.652344]x[4.277344,4.280273] width=0.003906 height=0.002930 cell_bound=0.014519664 center_slope=0.014346586 first_branch=0.014519664 first_reserve=0.000089339 second_branch=0.014527678 second_reserve=0.000180222 corner_slope=0.014347054 second_remainder=0.000000400 hessian_modes=4096
fixture=lent order=Second budget=65535 evaluations=65535 leaves=32768 bound=0.014496987 gain=2.251105 bound_over_reference=1.010081 stop=Evaluations partition_ms=59512.542 pool_bytes=655360
fixture=lent order=Second budget=65535 max_leaf=[1.441406,1.445312]x[-4.335938,-4.330078] width=0.003906 height=0.005859 cell_bound=0.014496987 center_slope=0.014313648 first_branch=0.014523471 first_reserve=0.000089339 second_branch=0.014496987 second_reserve=0.000180368 corner_slope=0.014316119 second_remainder=0.000000498 hessian_modes=4096
```

## Longue (15:51–15:55)

```text
S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee
fixture=long age=17.806095 reference_s217=0.066987507 global=0.134346545 preparation_ms=6.550 image_time_admitted=true modes=4096
fixture=long grid_step=2 rectangles=3072 bound=0.134652480 first_order_bound=0.134652480 second_order_branch_max=0.229649752 gain_over_first=1.000000 gain_over_global=0.997728 bound_over_reference=2.010113 tighter_rectangles=0 hessian_modes_mean=2168.0 hessian_modes_max=2168 grid_ms=2791.471
fixture=long grid_step=1 rectangles=12288 bound=0.110634618 first_order_bound=0.134652480 second_order_branch_max=0.110634618 gain_over_first=1.217092 gain_over_global=1.214327 bound_over_reference=1.651571 tighter_rectangles=12288 hessian_modes_mean=4032.0 hessian_modes_max=4032 grid_ms=11873.798
fixture=long grid_step=0.5 rectangles=49152 bound=0.077200100 first_order_bound=0.116187394 second_order_branch_max=0.077200100 gain_over_first=1.505016 gain_over_global=1.740238 bound_over_reference=1.152455 tighter_rectangles=49152 hessian_modes_mean=4096.0 hessian_modes_max=4096 grid_ms=46749.401
fixture=long order=First budget=2047 evaluations=2047 leaves=1024 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=1063.810 pool_bytes=655360
fixture=long order=First budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.134652480 center_slope=0.001164524 first_branch=0.134652480 first_reserve=0.000305917 second_branch=0.270729512 second_reserve=0.000883673 corner_slope=0.002720521 second_remainder=0.267125249 hessian_modes=1246
fixture=long order=First budget=8191 evaluations=8191 leaves=4096 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=4266.513 pool_bytes=655360
fixture=long order=First budget=8191 max_leaf=[50.000000,52.000000]x[-15.000000,-13.500000] width=2.000000 height=1.500000 cell_bound=0.134652480 center_slope=0.000230437 first_branch=0.134652480 first_reserve=0.000305917 second_branch=0.118536331 second_reserve=0.001304661 corner_slope=0.000829236 second_remainder=0.116402417 hessian_modes=2488
fixture=long order=First budget=32767 evaluations=32767 leaves=16384 bound=0.114180297 gain=1.176618 bound_over_reference=1.704501 stop=Evaluations partition_ms=17308.270 pool_bytes=655360
fixture=long order=First budget=32767 max_leaf=[-19.000000,-18.000000]x[-3.000000,-1.500000] width=1.000000 height=1.500000 cell_bound=0.114180297 center_slope=0.000012570 first_branch=0.114180297 first_reserve=0.000305917 second_branch=0.048579097 second_reserve=0.001068167 corner_slope=0.000013829 second_remainder=0.047497094 hessian_modes=3418
fixture=long order=First budget=65535 evaluations=65535 leaves=32768 bound=0.090740532 gain=1.480557 bound_over_reference=1.354589 stop=Evaluations partition_ms=34807.470 pool_bytes=655360
fixture=long order=First budget=65535 max_leaf=[-3.000000,-2.000000]x[-11.250000,-10.500000] width=1.000000 height=0.750000 cell_bound=0.090740532 center_slope=0.000010660 first_branch=0.090740532 first_reserve=0.000305917 second_branch=0.030417340 second_reserve=0.000975245 corner_slope=0.000022561 second_remainder=0.029419530 hessian_modes=4096
fixture=long order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=1735.837 pool_bytes=655360
fixture=long order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.134652480 center_slope=0.001164524 first_branch=0.134652480 first_reserve=0.000305917 second_branch=0.270729512 second_reserve=0.000883673 corner_slope=0.002720521 second_remainder=0.267125249 hessian_modes=1246
fixture=long order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=7263.475 pool_bytes=655360
fixture=long order=Second budget=8191 max_leaf=[32.000000,34.000000]x[0.000000,1.500000] width=2.000000 height=1.500000 cell_bound=0.134652480 center_slope=0.058639057 first_branch=0.134652480 first_reserve=0.000305917 second_branch=0.193849385 second_reserve=0.001304659 corner_slope=0.076143108 second_remainder=0.116401583 hessian_modes=2488
fixture=long order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.067499347 gain=1.990338 bound_over_reference=1.007641 stop=Evaluations partition_ms=30401.819 pool_bytes=655360
fixture=long order=Second budget=32767 max_leaf=[36.285156,36.289062]x[0.005859,0.008789] width=0.003906 height=0.002930 cell_bound=0.067499347 center_slope=0.066871777 first_branch=0.067531951 first_reserve=0.000305917 second_branch=0.067499347 second_reserve=0.000617395 corner_slope=0.066879399 second_remainder=0.000002539 hessian_modes=4096
fixture=long order=Second budget=65535 evaluations=65535 leaves=32768 bound=0.067410015 gain=1.992976 bound_over_reference=1.006307 stop=Evaluations partition_ms=61488.812 pool_bytes=655360
fixture=long order=Second budget=65535 max_leaf=[36.296875,36.304688]x[-0.117188,-0.111328] width=0.007812 height=0.005859 cell_bound=0.067410015 center_slope=0.066766120 first_branch=0.067779504 first_reserve=0.000305917 second_branch=0.067410015 second_reserve=0.000618799 corner_slope=0.066787310 second_remainder=0.000003891 hessian_modes=4096
```

## Base tardive, hors durée d'image (15:55–15:58)

```text
S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee
fixture=base_tard age=18.836567 reference_s217=0.044552851 global=0.116219424 preparation_ms=8.994 image_time_admitted=false modes=4096
fixture=base_tard grid_step=2 rectangles=3072 bound=0.116488002 first_order_bound=0.116488002 second_order_branch_max=0.176941097 gain_over_first=1.000000 gain_over_global=0.997694 bound_over_reference=2.614603 tighter_rectangles=1946 hessian_modes_mean=2168.0 hessian_modes_max=2168 grid_ms=2771.295
fixture=base_tard grid_step=1 rectangles=12288 bound=0.080093130 first_order_bound=0.116488002 second_order_branch_max=0.080093130 gain_over_first=1.454407 gain_over_global=1.451054 bound_over_reference=1.797711 tighter_rectangles=12288 hessian_modes_mean=4032.0 hessian_modes_max=4032 grid_ms=11628.571
fixture=base_tard grid_step=0.5 rectangles=49152 bound=0.053817019 first_order_bound=0.086482756 second_order_branch_max=0.053817019 gain_over_first=1.606978 gain_over_global=2.159529 bound_over_reference=1.207937 tighter_rectangles=49152 hessian_modes_mean=4096.0 hessian_modes_max=4096 grid_ms=46185.008
fixture=base_tard order=First budget=2047 evaluations=2047 leaves=1024 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=1110.540 pool_bytes=655360
fixture=base_tard order=First budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.116488002 center_slope=0.000565926 first_branch=0.116488002 first_reserve=0.000268573 second_branch=0.235041410 second_reserve=0.000832650 corner_slope=0.002967516 second_remainder=0.231241211 hessian_modes=1246
fixture=base_tard order=First budget=8191 evaluations=8191 leaves=4096 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=4283.575 pool_bytes=655360
fixture=base_tard order=First budget=8191 max_leaf=[50.000000,52.000000]x[-15.000000,-13.500000] width=2.000000 height=1.500000 cell_bound=0.116488002 center_slope=0.000082859 first_branch=0.116488002 first_reserve=0.000268573 second_branch=0.098700985 second_reserve=0.001131545 corner_slope=0.000156443 second_remainder=0.097412989 hessian_modes=2488
fixture=base_tard order=First budget=32767 evaluations=32767 leaves=16384 bound=0.098750442 gain=1.176900 bound_over_reference=2.216479 stop=Evaluations partition_ms=17351.266 pool_bytes=655360
fixture=base_tard order=First budget=32767 max_leaf=[52.000000,53.000000]x[21.000000,22.500000] width=1.000000 height=1.500000 cell_bound=0.098750442 center_slope=0.000026556 first_branch=0.098750442 first_reserve=0.000268573 second_branch=0.041472111 second_reserve=0.000931796 corner_slope=0.000061832 second_remainder=0.040478475 hessian_modes=3418
fixture=base_tard order=First budget=65535 evaluations=65535 leaves=32768 bound=0.077684335 gain=1.496047 bound_over_reference=1.743644 stop=Evaluations partition_ms=34828.680 pool_bytes=655360
fixture=base_tard order=First budget=65535 max_leaf=[61.000000,62.000000]x[-7.500000,-6.750000] width=1.000000 height=0.750000 cell_bound=0.077684335 center_slope=0.000022476 first_branch=0.077684335 first_reserve=0.000268573 second_branch=0.025535021 second_reserve=0.000848283 corner_slope=0.000070188 second_remainder=0.024616545 hessian_modes=4096
fixture=base_tard order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=1752.734 pool_bytes=655360
fixture=base_tard order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.116488002 center_slope=0.000565926 first_branch=0.116488002 first_reserve=0.000268573 second_branch=0.235041410 second_reserve=0.000832650 corner_slope=0.002967516 second_remainder=0.231241211 hessian_modes=1246
fixture=base_tard order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=7197.013 pool_bytes=655360
fixture=base_tard order=Second budget=8191 max_leaf=[18.000000,20.000000]x[12.000000,13.500000] width=2.000000 height=1.500000 cell_bound=0.116488002 center_slope=0.010477178 first_branch=0.116488002 first_reserve=0.000268573 second_branch=0.122551501 second_reserve=0.001131542 corner_slope=0.024007885 second_remainder=0.097412057 hessian_modes=2488
fixture=base_tard order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.045091338 gain=2.577422 bound_over_reference=1.012087 stop=Evaluations partition_ms=30490.342 pool_bytes=655360
fixture=base_tard order=Second budget=32767 max_leaf=[24.898438,24.902344]x[-0.281250,-0.275391] width=0.003906 height=0.005859 cell_bound=0.045091338 center_slope=0.044545401 first_branch=0.045199446 first_reserve=0.000268573 second_branch=0.045091338 second_reserve=0.000542321 corner_slope=0.044546902 second_remainder=0.000002108 hessian_modes=4096
fixture=base_tard order=Second budget=65535 evaluations=65535 leaves=32768 bound=0.045046199 gain=2.580005 bound_over_reference=1.011073 stop=Evaluations partition_ms=60917.480 pool_bytes=655360
fixture=base_tard order=Second budget=65535 max_leaf=[24.867188,24.871094]x[0.386719,0.392578] width=0.003906 height=0.005859 cell_bound=0.045046199 center_slope=0.044497766 first_branch=0.045151819 first_reserve=0.000268573 second_branch=0.045046199 second_reserve=0.000542321 corner_slope=0.044501763 second_remainder=0.000002109 hessian_modes=4096
```

## Base, passage 2 (15:58–16:02)

```text
S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.290 image_time_admitted=true modes=4096
fixture=base grid_step=2 rectangles=3072 bound=0.115437612 first_order_bound=0.115437612 second_order_branch_max=0.209338635 gain_over_first=1.000000 gain_over_global=0.997691 bound_over_reference=1.641532 tighter_rectangles=1332 hessian_modes_mean=2168.0 hessian_modes_max=2168 grid_ms=2726.829
fixture=base grid_step=1 rectangles=12288 bound=0.109988004 first_order_bound=0.115437612 second_order_branch_max=0.109988004 gain_over_first=1.049547 gain_over_global=1.047124 bound_over_reference=1.564038 tighter_rectangles=12288 hessian_modes_mean=4032.0 hessian_modes_max=4032 grid_ms=11537.791
fixture=base grid_step=0.5 rectangles=49152 bound=0.079500645 first_order_bound=0.112293623 second_order_branch_max=0.079500645 gain_over_first=1.412487 gain_over_global=1.448681 bound_over_reference=1.130505 tighter_rectangles=49152 hessian_modes_mean=4096.0 hessian_modes_max=4096 grid_ms=46355.770
fixture=base order=First budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=1083.406 pool_bytes=655360
fixture=base order=First budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.115437612 center_slope=0.000125690 first_branch=0.115437612 first_reserve=0.000266517 second_branch=0.231279492 second_reserve=0.000814265 corner_slope=0.000624217 second_remainder=0.229840994 hessian_modes=1246
fixture=base order=First budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=4417.465 pool_bytes=655360
fixture=base order=First budget=8191 max_leaf=[50.000000,52.000000]x[-15.000000,-13.500000] width=2.000000 height=1.500000 cell_bound=0.115437612 center_slope=0.000036233 first_branch=0.115437612 first_reserve=0.000266517 second_branch=0.098733753 second_reserve=0.001124932 corner_slope=0.000099489 second_remainder=0.097509317 hessian_modes=2488
fixture=base order=First budget=32767 evaluations=32767 leaves=16384 bound=0.098479681 gain=1.169491 bound_over_reference=1.400389 stop=Evaluations partition_ms=17340.011 pool_bytes=655360
fixture=base order=First budget=32767 max_leaf=[41.000000,42.000000]x[19.500000,21.000000] width=1.000000 height=1.500000 cell_bound=0.098479681 center_slope=0.000010398 first_branch=0.098479681 first_reserve=0.000266517 second_branch=0.041596606 second_reserve=0.000926652 corner_slope=0.000015735 second_remainder=0.040654212 hessian_modes=3418
fixture=base order=First budget=65535 evaluations=65535 leaves=32768 bound=0.077374868 gain=1.488482 bound_over_reference=1.100277 stop=Evaluations partition_ms=34718.036 pool_bytes=655360
fixture=base order=First budget=65535 max_leaf=[36.000000,37.000000]x[24.000000,24.750000] width=1.000000 height=0.750000 cell_bound=0.077374868 center_slope=0.000016941 first_branch=0.077374868 first_reserve=0.000266517 second_branch=0.025506319 second_reserve=0.000842943 corner_slope=0.000022265 second_remainder=0.024641108 hessian_modes=4096
fixture=base order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=1744.937 pool_bytes=655360
fixture=base order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] width=4.000000 height=3.000000 cell_bound=0.115437612 center_slope=0.000125690 first_branch=0.115437612 first_reserve=0.000266517 second_branch=0.231279492 second_reserve=0.000814265 corner_slope=0.000624217 second_remainder=0.229840994 hessian_modes=1246
fixture=base order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=7295.044 pool_bytes=655360
fixture=base order=Second budget=8191 max_leaf=[2.000000,4.000000]x[-6.000000,-4.500000] width=2.000000 height=1.500000 cell_bound=0.115437612 center_slope=0.004098490 first_branch=0.115437612 first_reserve=0.000266517 second_branch=0.133188710 second_reserve=0.001124926 corner_slope=0.034556031 second_remainder=0.097507708 hessian_modes=2488
fixture=base order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.070740171 gain=1.628086 bound_over_reference=1.005931 stop=Evaluations partition_ms=30773.158 pool_bytes=655360
fixture=base order=Second budget=32767 max_leaf=[9.201172,9.203125]x[-0.019043,-0.017578] width=0.001953 height=0.001465 cell_bound=0.070740171 center_slope=0.070322238 first_branch=0.070740171 first_reserve=0.000266517 second_branch=0.070860974 second_reserve=0.000537251 corner_slope=0.070322558 second_remainder=0.000001150 hessian_modes=4096
fixture=base order=Second budget=65535 evaluations=65535 leaves=32768 bound=0.070666455 gain=1.629784 bound_over_reference=1.004882 stop=Evaluations partition_ms=59810.007 pool_bytes=655360
fixture=base order=Second budget=65535 max_leaf=[9.246094,9.248047]x[0.042480,0.043945] width=0.001953 height=0.001465 cell_bound=0.070666455 center_slope=0.070248529 first_branch=0.070666455 first_reserve=0.000266517 second_branch=0.070790343 second_reserve=0.000537251 corner_slope=0.070251919 second_remainder=0.000001152 hessian_modes=4096
```

## Passage interrompu

Un premier passage de la fixture lente a été arrêté volontairement après la partition d'ordre un à 32767 évaluations, pour ajouter le détail de feuille. Ses valeurs imprimées sont identiques à celles du passage publié ci-dessus ; il n'est pas reproduit.
