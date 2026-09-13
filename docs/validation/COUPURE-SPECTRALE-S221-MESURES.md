# S221 — Relevés bruts

AMD Ryzen AI 7 350, 32 Go, Windows 11, rustc 1.97.0, `cargo run --offline --release -p water-core --example coupure_spectrale_s221 [fixture|micro]`.
Cinq processus isolés lancés à la suite (16:42:33–16:52:42) après la suite de tests, sans compilation concurrente ; détails de pire rectangle et de feuille maximale calculés hors chronométrage.

## Base, passage 1

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.680 image_time_admitted=true modes=4096
fixture=base grid=4x3 rectangles=1024 bound=0.115437612 second_order_max=0.115437612 cut2_max=0.169602275 cut1_max=0.117667072 cut_half_max=0.115685761 gain_over_second=1.000000 gain_over_global=0.997691 bound_over_reference=1.641532 wins_first=30 wins_second=0 wins_cut2=0 wins_cut1=994 wins_cut_half=0 mass_total=0.135052577 mass_fraction_lt_half=0.0062 mass_fraction_half_1=0.0556 mass_fraction_1_2=0.2917 mass_fraction_ge_2=0.6464 grid_ms=728.915
fixture=base grid=4x3 worst_cell=[16.000,-12.000] branch=ordre_un first=0.115437612 second=0.239123762 cut2: bound=0.143318594 corner=0.010104246 remainder=0.055232592 C_U=0.087302454 G_U=0.077167451 reserve=0.000814263 | cut1: bound=0.115584135 corner=0.002771028 remainder=0.002535920 C_U=0.126699701 G_U=0.109715439 reserve=0.000561729 | cut0.5: bound=0.115613692 corner=0.000343673 remainder=0.000063586 C_U=0.134211510 G_U=0.114668511 reserve=0.000537913
fixture=base grid=2x1.5 rectangles=4096 bound=0.114862204 second_order_max=0.115437612 cut2_max=0.188615158 cut1_max=0.116644293 cut_half_max=0.115503311 gain_over_second=1.005010 gain_over_global=1.002689 bound_over_reference=1.633350 wins_first=0 wins_second=0 wins_cut2=136 wins_cut1=3952 wins_cut_half=8 mass_total=0.135041401 mass_fraction_lt_half=0.0619 mass_fraction_half_1=0.2917 mass_fraction_1_2=0.6256 mass_fraction_ge_2=0.0208 grid_ms=3040.247
fixture=base grid=2x1.5 worst_cell=[16.000,-3.000] branch=coupure_demi first=0.115437612 second=0.131662637 cut2: bound=0.128114820 corner=0.032863937 remainder=0.091886677 C_U=0.002804678 G_U=0.002239268 reserve=0.001124902 | cut1: bound=0.116644293 corner=0.025000896 remainder=0.013808551 C_U=0.087291263 G_U=0.077159375 reserve=0.000675455 | cut0.5: bound=0.114862204 corner=0.003971571 remainder=0.000634032 C_U=0.126688510 G_U=0.109707393 reserve=0.000549187
fixture=base grid=1x0.75 rectangles=16384 bound=0.099504478 second_order_max=0.104472235 cut2_max=0.104469143 cut1_max=0.105573438 cut_half_max=0.104567230 gain_over_second=1.049925 gain_over_global=1.157446 bound_over_reference=1.414961 wins_first=0 wins_second=0 wins_cut2=16146 wins_cut1=228 wins_cut_half=10 mass_total=0.135041788 mass_fraction_lt_half=0.3536 mass_fraction_half_1=0.6256 mass_fraction_1_2=0.0208 mass_fraction_ge_2=0.0000 grid_ms=13948.899
fixture=base grid=1x0.75 worst_cell=[12.000,-0.750] branch=coupure_1 first=0.115437612 second=0.099923901 cut2: bound=0.099920802 corner=0.074440852 remainder=0.024637047 C_U=0.000000000 G_U=0.000000000 reserve=0.000842882 | cut1: bound=0.099504478 corner=0.073461659 remainder=0.022972753 C_U=0.002804678 G_U=0.002239268 reserve=0.000830778 | cut0.5: bound=0.100947633 corner=0.019729735 remainder=0.003452456 C_U=0.087291270 G_U=0.077159375 reserve=0.000606052
fixture=base order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=1967.999 pool_bytes=655360
fixture=base order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] cell_bound=0.115437612 spectral_branch=coupure_1 first=0.115437612 second=0.231279492 cut2=0.134771347 cut1=0.113935649 cut_half=0.115415588
fixture=base order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=7775.456 pool_bytes=655360
fixture=base order=Second budget=8191 max_leaf=[2.000000,4.000000]x[-6.000000,-4.500000] cell_bound=0.115437612 spectral_branch=coupure_1 first=0.115437612 second=0.133188710 cut2=0.129808888 cut1=0.107362226 cut_half=0.112963505
fixture=base order=Second budget=16383 evaluations=16383 leaves=8192 bound=0.098670244 gain=1.167232 bound_over_reference=1.403099 stop=Evaluations partition_ms=15661.393 pool_bytes=655360
fixture=base order=Second budget=16383 max_leaf=[38.000000,40.000000]x[-16.500000,-15.000000] cell_bound=0.098670244 spectral_branch=coupure_1 first=0.115437612 second=0.098670244 cut2=0.095315091 cut1=0.093314096 cut_half=0.111366391
fixture=base order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.070740171 gain=1.628086 bound_over_reference=1.005931 stop=Evaluations partition_ms=31978.616 pool_bytes=655360
fixture=base order=Second budget=32767 max_leaf=[9.201172,9.203125]x[-0.019043,-0.017578] cell_bound=0.070740171 spectral_branch=ordre_un first=0.070740171 second=0.070860974 cut2=0.070860982 cut1=0.070860982 cut_half=0.070860982
fixture=base order=Spectral budget=2047 evaluations=2047 leaves=1024 bound=0.114765890 gain=1.003531 bound_over_reference=1.631980 stop=Evaluations partition_ms=1458.622 pool_bytes=655360
fixture=base order=Spectral budget=2047 max_leaf=[-60.000000,-56.000000]x[24.000000,30.000000] cell_bound=0.114765890 spectral_branch=coupure_1 first=0.115437612 second=0.251751542 cut2=0.128132269 cut1=0.114765890 cut_half=0.115637429
fixture=base order=Spectral budget=8191 evaluations=8191 leaves=4096 bound=0.102004133 gain=1.129083 bound_over_reference=1.450507 stop=Evaluations partition_ms=6192.132 pool_bytes=655360
fixture=base order=Spectral budget=8191 max_leaf=[-50.000000,-48.000000]x[-48.000000,-45.000000] cell_bound=0.102004133 spectral_branch=coupure_1 first=0.115437612 second=0.146247000 cut2=0.120645143 cut1=0.102004133 cut_half=0.114159644
fixture=base order=Spectral budget=16383 evaluations=16383 leaves=8192 bound=0.091857083 gain=1.253807 bound_over_reference=1.306215 stop=Evaluations partition_ms=12662.888 pool_bytes=655360
fixture=base order=Spectral budget=16383 max_leaf=[24.000000,26.000000]x[-40.500000,-39.000000] cell_bound=0.091857083 spectral_branch=coupure_1 first=0.115437612 second=0.098686747 cut2=0.095347360 cut1=0.091857083 cut_half=0.111277096
fixture=base order=Spectral budget=32767 evaluations=32767 leaves=16384 bound=0.070740089 gain=1.628088 bound_over_reference=1.005930 stop=Evaluations partition_ms=26103.313 pool_bytes=655360
fixture=base order=Spectral budget=32767 max_leaf=[9.246094,9.250000]x[0.114258,0.117188] cell_bound=0.070740089 spectral_branch=ordre_deux first=0.070761286 second=0.070740089 cut2=0.070740096 cut1=0.070740096 cut_half=0.070740096
```

## Lente

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=lent age=9.806095 reference_s217=0.014352296 global=0.032634243 preparation_ms=6.908 image_time_admitted=true modes=4096
fixture=lent grid=4x3 rectangles=1024 bound=0.032658890 second_order_max=0.032723587 cut2_max=0.048346378 cut1_max=0.033644088 cut_half_max=0.032658890 gain_over_second=1.001981 gain_over_global=0.999245 bound_over_reference=2.275517 wins_first=0 wins_second=0 wins_cut2=0 wins_cut1=1002 wins_cut_half=22 mass_total=0.045270734 mass_fraction_lt_half=0.0221 mass_fraction_half_1=0.1578 mass_fraction_1_2=0.3739 mass_fraction_ge_2=0.4462 grid_ms=734.937
fixture=lent grid=4x3 worst_cell=[4.000,-9.000] branch=coupure_demi first=0.032723587 second=0.066653520 cut2: bound=0.041084517 corner=0.003702712 remainder=0.022093959 C_U=0.020197898 G_U=0.014983595 reserve=0.000304240 | cut1: bound=0.032769967 corner=0.003677017 remainder=0.002192277 C_U=0.037124511 G_U=0.026697766 reserve=0.000202898 | cut0.5: bound=0.032658890 corner=0.000519324 remainder=0.000077627 C_U=0.044270437 G_U=0.031880509 reserve=0.000181423
fixture=lent grid=2x1.5 rectangles=4096 bound=0.030595804 second_order_max=0.032723587 cut2_max=0.043895412 cut1_max=0.030595804 cut_half_max=0.031745184 gain_over_second=1.069545 gain_over_global=1.066625 bound_over_reference=2.131771 wins_first=0 wins_second=0 wins_cut2=0 wins_cut1=4096 wins_cut_half=0 mass_total=0.045267951 mass_fraction_lt_half=0.1800 mass_fraction_half_1=0.3739 mass_fraction_1_2=0.4355 mass_fraction_ge_2=0.0107 grid_ms=3044.811
fixture=lent grid=2x1.5 worst_cell=[0.000,-6.000] branch=coupure_1 first=0.032723587 second=0.039992806 cut2: bound=0.039344922 corner=0.015373049 remainder=0.023237435 C_U=0.000482830 G_U=0.000388966 reserve=0.000345461 | cut1: bound=0.030595804 corner=0.009848469 remainder=0.005523661 C_U=0.020195086 G_U=0.014981605 reserve=0.000242064 | cut0.5: bound=0.030961350 corner=0.003526058 remainder=0.000548117 C_U=0.037121702 G_U=0.026695779 reserve=0.000191393
fixture=lent grid=1x0.75 rectangles=16384 bound=0.022015430 second_order_max=0.022064881 cut2_max=0.022064051 cut1_max=0.022019655 cut_half_max=0.025638254 gain_over_second=1.002246 gain_over_global=1.482335 bound_over_reference=1.533931 wins_first=0 wins_second=0 wins_cut2=16258 wins_cut1=126 wins_cut_half=0 mass_total=0.045268383 mass_fraction_lt_half=0.5539 mass_fraction_half_1=0.4355 mass_fraction_1_2=0.0107 mass_fraction_ge_2=0.0000 grid_ms=13735.017
fixture=lent grid=1x0.75 worst_cell=[1.000,3.000] branch=coupure_1 first=0.032723587 second=0.022064881 cut2: bound=0.022064051 corner=0.015700966 remainder=0.006098312 C_U=0.000000000 G_U=0.000000000 reserve=0.000264767 | cut1: bound=0.022015430 corner=0.015554135 remainder=0.005809649 C_U=0.000482830 G_U=0.000388966 reserve=0.000262676 | cut0.5: bound=0.024592673 corner=0.008019018 remainder=0.001381067 C_U=0.020195087 G_U=0.014981607 reserve=0.000210977
fixture=lent order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=1858.114 pool_bytes=655360
fixture=lent order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] cell_bound=0.032723587 spectral_branch=coupure_1 first=0.032723587 second=0.063259780 cut2=0.037795454 cut1=0.029843468 cut_half=0.032361500
fixture=lent order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.032723587 gain=0.997270 bound_over_reference=2.280025 stop=Evaluations partition_ms=7572.545 pool_bytes=655360
fixture=lent order=Second budget=8191 max_leaf=[-6.000000,-4.000000]x[1.500000,3.000000] cell_bound=0.032723587 spectral_branch=coupure_1 first=0.032723587 second=0.032828312 cut2=0.032223590 cut1=0.026613427 cut_half=0.030975448
fixture=lent order=Second budget=16383 evaluations=16383 leaves=8192 bound=0.024563899 gain=1.328545 bound_over_reference=1.711496 stop=Evaluations partition_ms=15702.476 pool_bytes=655360
fixture=lent order=Second budget=16383 max_leaf=[18.000000,20.000000]x[46.500000,48.000000] cell_bound=0.024563899 spectral_branch=coupure_1 first=0.032723587 second=0.024563899 cut2=0.023989180 cut1=0.020816557 cut_half=0.027562719
fixture=lent order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.014519664 gain=2.247589 bound_over_reference=1.011662 stop=Evaluations partition_ms=31457.222 pool_bytes=655360
fixture=lent order=Second budget=32767 max_leaf=[1.648438,1.652344]x[4.277344,4.280273] cell_bound=0.014519664 spectral_branch=ordre_un first=0.014519664 second=0.014527678 cut2=0.014527678 cut1=0.014527678 cut_half=0.014527678
fixture=lent order=Spectral budget=2047 evaluations=2047 leaves=1024 bound=0.031855736 gain=1.024439 bound_over_reference=2.219557 stop=Evaluations partition_ms=1465.839 pool_bytes=655360
fixture=lent order=Spectral budget=2047 max_leaf=[48.000000,52.000000]x[-30.000000,-24.000000] cell_bound=0.031855736 spectral_branch=coupure_1 first=0.032723587 second=0.076562800 cut2=0.037092131 cut1=0.031855736 cut_half=0.032816391
fixture=lent order=Spectral budget=8191 evaluations=8191 leaves=4096 bound=0.026540974 gain=1.229580 bound_over_reference=1.849249 stop=Evaluations partition_ms=6385.036 pool_bytes=655360
fixture=lent order=Spectral budget=8191 max_leaf=[-62.000000,-60.000000]x[-18.000000,-15.000000] cell_bound=0.026540974 spectral_branch=coupure_1 first=0.032723587 second=0.045532893 cut2=0.037236378 cut1=0.026540974 cut_half=0.031004600
fixture=lent order=Spectral budget=16383 evaluations=16383 leaves=8192 bound=0.020825114 gain=1.567062 bound_over_reference=1.450995 stop=Evaluations partition_ms=12897.909 pool_bytes=655360
fixture=lent order=Spectral budget=16383 max_leaf=[38.000000,40.000000]x[-22.500000,-21.000000] cell_bound=0.020825114 spectral_branch=coupure_1 first=0.032723587 second=0.024576003 cut2=0.023998974 cut1=0.020825114 cut_half=0.027639346
fixture=lent order=Spectral budget=32767 evaluations=32767 leaves=16384 bound=0.014519630 gain=2.247595 bound_over_reference=1.011659 stop=Evaluations partition_ms=26110.608 pool_bytes=655360
fixture=lent order=Spectral budget=32767 max_leaf=[1.554688,1.558594]x[4.289062,4.291992] cell_bound=0.014519630 spectral_branch=ordre_un first=0.014519630 second=0.014527977 cut2=0.014527978 cut1=0.014527978 cut_half=0.014527978
```

## Longue

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=long age=17.806095 reference_s217=0.066987507 global=0.134346545 preparation_ms=6.305 image_time_admitted=true modes=4096
fixture=long grid=4x3 rectangles=1024 bound=0.134652480 second_order_max=0.134652480 cut2_max=0.185398623 cut1_max=0.136620700 cut_half_max=0.134903938 gain_over_second=1.000000 gain_over_global=0.997728 bound_over_reference=2.010113 wins_first=26 wins_second=0 wins_cut2=0 wins_cut1=998 wins_cut_half=0 mass_total=0.155018970 mass_fraction_lt_half=0.0067 mass_fraction_half_1=0.0516 mass_fraction_1_2=0.2481 mass_fraction_ge_2=0.6937 grid_ms=737.127
fixture=long grid=4x3 worst_cell=[32.000,-9.000] branch=ordre_un first=0.134652480 second=0.275658697 cut2: bound=0.157959506 corner=0.008386806 remainder=0.052033897 C_U=0.107529350 G_U=0.096655101 reserve=0.000883670 | cut1: bound=0.135597244 corner=0.004091548 remainder=0.002639307 C_U=0.145989150 G_U=0.128223851 reserve=0.000642514 | cut0.5: bound=0.134902477 corner=0.000473051 remainder=0.000074266 C_U=0.153983966 G_U=0.133737639 reserve=0.000617486
fixture=long grid=2x1.5 rectangles=4096 bound=0.133739024 second_order_max=0.134652480 cut2_max=0.206178546 cut1_max=0.135038793 cut_half_max=0.134140119 gain_over_second=1.006830 gain_over_global=1.004543 bound_over_reference=1.996477 wins_first=0 wins_second=0 wins_cut2=132 wins_cut1=3960 wins_cut_half=4 mass_total=0.155007988 mass_fraction_lt_half=0.0583 mass_fraction_half_1=0.2481 mass_fraction_1_2=0.6714 mass_fraction_ge_2=0.0223 grid_ms=3089.146
fixture=long grid=2x1.5 worst_cell=[40.000,-3.000] branch=coupure_demi first=0.134652480 second=0.152251706 cut2: bound=0.147910073 corner=0.034360126 remainder=0.109488025 C_U=0.003451079 G_U=0.002757231 reserve=0.001304647 | cut1: bound=0.134940639 corner=0.024534723 remainder=0.013008975 C_U=0.107518330 G_U=0.096647084 reserve=0.000749827 | cut0.5: bound=0.133739024 corner=0.004233970 remainder=0.000659892 C_U=0.145978138 G_U=0.128215879 reserve=0.000629247
fixture=long grid=1x0.75 rectangles=16384 bound=0.104148582 second_order_max=0.104476646 cut2_max=0.104473941 cut1_max=0.104148582 cut_half_max=0.123930335 gain_over_second=1.003150 gain_over_global=1.289951 bound_over_reference=1.554746 wins_first=0 wins_second=0 wins_cut2=15992 wins_cut1=392 wins_cut_half=0 mass_total=0.155008435 mass_fraction_lt_half=0.3064 mass_fraction_half_1=0.6714 mass_fraction_1_2=0.0223 mass_fraction_ge_2=0.0000 grid_ms=13882.419
fixture=long grid=1x0.75 worst_cell=[36.000,-0.750] branch=coupure_1 first=0.134652480 second=0.104476646 cut2: bound=0.104473941 corner=0.074081041 remainder=0.029417681 C_U=0.000000000 G_U=0.000000000 reserve=0.000975199 | cut1: bound=0.104148582 corner=0.073057212 remainder=0.027373798 C_U=0.003451079 G_U=0.002757231 reserve=0.000960319 | cut0.5: bound=0.118523456 corner=0.017940780 remainder=0.003252660 C_U=0.107518338 G_U=0.096647091 reserve=0.000682905
fixture=long order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=1798.622 pool_bytes=655360
fixture=long order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] cell_bound=0.134652480 spectral_branch=coupure_1 first=0.134652480 second=0.270729512 cut2=0.152663663 cut1=0.132092446 cut_half=0.134559885
fixture=long order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.134652480 gain=0.997728 bound_over_reference=2.010113 stop=Evaluations partition_ms=7764.617 pool_bytes=655360
fixture=long order=Second budget=8191 max_leaf=[32.000000,34.000000]x[0.000000,1.500000] cell_bound=0.134652480 spectral_branch=coupure_1 first=0.134652480 second=0.193849385 cut2=0.190041766 cut1=0.118536726 cut_half=0.132870004
fixture=long order=Second budget=16383 evaluations=16383 leaves=8192 bound=0.117760770 gain=1.140843 bound_over_reference=1.757951 stop=Evaluations partition_ms=15767.705 pool_bytes=655360
fixture=long order=Second budget=16383 max_leaf=[-22.000000,-20.000000]x[1.500000,3.000000] cell_bound=0.117760770 spectral_branch=coupure_1 first=0.134652480 second=0.117760770 cut2=0.113661885 cut1=0.110820480 cut_half=0.129913822
fixture=long order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.067499347 gain=1.990338 bound_over_reference=1.007641 stop=Evaluations partition_ms=32057.881 pool_bytes=655360
fixture=long order=Second budget=32767 max_leaf=[36.285156,36.289062]x[0.005859,0.008789] cell_bound=0.067499347 spectral_branch=ordre_deux first=0.067531951 second=0.067499347 cut2=0.067499354 cut1=0.067499354 cut_half=0.067499354
fixture=long order=Spectral budget=2047 evaluations=2047 leaves=1024 bound=0.133651733 gain=1.005199 bound_over_reference=1.995174 stop=Evaluations partition_ms=1518.271 pool_bytes=655360
fixture=long order=Spectral budget=2047 max_leaf=[-40.000000,-36.000000]x[24.000000,30.000000] cell_bound=0.133651733 spectral_branch=coupure_1 first=0.134652480 second=0.290617317 cut2=0.147071421 cut1=0.133651733 cut_half=0.134808570
fixture=long order=Spectral budget=8191 evaluations=8191 leaves=4096 bound=0.120512746 gain=1.114791 bound_over_reference=1.799033 stop=Evaluations partition_ms=6281.252 pool_bytes=655360
fixture=long order=Spectral budget=8191 max_leaf=[56.000000,58.000000]x[12.000000,15.000000] cell_bound=0.120512746 spectral_branch=coupure_1 first=0.134652480 second=0.170581013 cut2=0.140590951 cut1=0.120512746 cut_half=0.133265033
fixture=long order=Spectral budget=16383 evaluations=16383 leaves=8192 bound=0.110683419 gain=1.213791 bound_over_reference=1.652299 stop=Evaluations partition_ms=12875.303 pool_bytes=655360
fixture=long order=Spectral budget=16383 max_leaf=[-30.000000,-28.000000]x[-7.500000,-6.000000] cell_bound=0.110683419 spectral_branch=coupure_1 first=0.134652480 second=0.117811814 cut2=0.113688253 cut1=0.110683419 cut_half=0.129822806
fixture=long order=Spectral budget=32767 evaluations=32767 leaves=16384 bound=0.067499273 gain=1.990341 bound_over_reference=1.007640 stop=Evaluations partition_ms=26275.122 pool_bytes=655360
fixture=long order=Spectral budget=32767 max_leaf=[36.246094,36.248047]x[-0.041016,-0.038086] cell_bound=0.067499273 spectral_branch=ordre_un first=0.067499273 second=0.067590736 cut2=0.067590743 cut1=0.067590743 cut_half=0.067590743
```

## Base tardive, hors durée d'image

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=base_tard age=18.836567 reference_s217=0.044552851 global=0.116219424 preparation_ms=6.310 image_time_admitted=false modes=4096
fixture=base_tard grid=4x3 rectangles=1024 bound=0.116488002 second_order_max=0.116488002 cut2_max=0.172222018 cut1_max=0.117059350 cut_half_max=0.116609052 gain_over_second=1.000000 gain_over_global=0.997694 bound_over_reference=2.614603 wins_first=10 wins_second=0 wins_cut2=0 wins_cut1=1006 wins_cut_half=8 mass_total=0.136093915 mass_fraction_lt_half=0.0058 mass_fraction_half_1=0.0481 mass_fraction_1_2=0.3107 mass_fraction_ge_2=0.6354 grid_ms=698.550
fixture=base_tard grid=4x3 worst_cell=[28.000,-36.000] branch=ordre_un first=0.116488002 second=0.232926503 cut2: bound=0.137179255 corner=0.001346053 remainder=0.058292124 C_U=0.086472966 G_U=0.076708406 reserve=0.000832648 | cut1: bound=0.116651677 corner=0.002424139 remainder=0.002261443 C_U=0.128763750 G_U=0.111403137 reserve=0.000562945 | cut0.5: bound=0.116515808 corner=0.000225947 remainder=0.000058784 C_U=0.135305494 G_U=0.115689114 reserve=0.000541949
fixture=base_tard grid=2x1.5 rectangles=4096 bound=0.112866536 second_order_max=0.116488002 cut2_max=0.155904487 cut1_max=0.114336982 cut_half_max=0.114879958 gain_over_second=1.032086 gain_over_global=1.029707 bound_over_reference=2.533318 wins_first=0 wins_second=0 wins_cut2=106 wins_cut1=3984 wins_cut_half=6 mass_total=0.136082649 mass_fraction_lt_half=0.0539 mass_fraction_half_1=0.3108 mass_fraction_1_2=0.6148 mass_fraction_ge_2=0.0205 grid_ms=3111.016
fixture=base_tard grid=2x1.5 worst_cell=[28.000,-3.000] branch=coupure_1 first=0.116488002 second=0.139755905 cut2: bound=0.136401132 corner=0.041235588 remainder=0.091807730 C_U=0.002796028 G_U=0.002226253 reserve=0.001131516 | cut1: bound=0.112866536 corner=0.020906109 remainder=0.014573478 C_U=0.086461678 G_U=0.076700211 reserve=0.000686717 | cut0.5: bound=0.112939000 corner=0.000426745 remainder=0.000565406 C_U=0.128752470 G_U=0.111394972 reserve=0.000551864
fixture=base_tard grid=1x0.75 rectangles=16384 bound=0.075287186 second_order_max=0.075290248 cut2_max=0.075287186 cut1_max=0.075802058 cut_half_max=0.101680055 gain_over_second=1.000041 gain_over_global=1.543681 bound_over_reference=1.689840 wins_first=0 wins_second=0 wins_cut2=16008 wins_cut1=376 wins_cut_half=0 mass_total=0.136083066 mass_fraction_lt_half=0.3646 mass_fraction_half_1=0.6148 mass_fraction_1_2=0.0205 mass_fraction_ge_2=0.0000 grid_ms=13904.665
fixture=base_tard grid=1x0.75 worst_cell=[24.000,-0.750] branch=coupure_2 first=0.116488002 second=0.075290248 cut2: bound=0.075287186 corner=0.049826588 remainder=0.024612356 C_U=0.000000000 G_U=0.000000000 reserve=0.000848221 | cut1: bound=0.075802058 corner=0.049786389 remainder=0.022953233 C_U=0.002796028 G_U=0.002226253 reserve=0.000836154 | cut0.5: bound=0.101680055 corner=0.020722324 remainder=0.003643757 C_U=0.086461686 G_U=0.076700211 reserve=0.000613752
fixture=base_tard order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=1807.387 pool_bytes=655360
fixture=base_tard order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] cell_bound=0.116488002 spectral_branch=coupure_1 first=0.116488002 second=0.235041410 cut2=0.138887018 cut1=0.114837110 cut_half=0.116376519
fixture=base_tard order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.116488002 gain=0.997694 bound_over_reference=2.614603 stop=Evaluations partition_ms=7675.652 pool_bytes=655360
fixture=base_tard order=Second budget=8191 max_leaf=[18.000000,20.000000]x[12.000000,13.500000] cell_bound=0.116488002 spectral_branch=coupure_1 first=0.116488002 second=0.122551501 cut2=0.120075554 cut1=0.099232875 cut_half=0.113325395
fixture=base_tard order=Second budget=16383 evaluations=16383 leaves=8192 bound=0.091421701 gain=1.271245 bound_over_reference=2.051983 stop=Evaluations partition_ms=15609.983 pool_bytes=655360
fixture=base_tard order=Second budget=16383 max_leaf=[24.000000,25.000000]x[0.000000,1.500000] cell_bound=0.091421701 spectral_branch=coupure_2 first=0.116488002 second=0.091421701 cut2=0.091391511 cut1=0.092409313 cut_half=0.108879186
fixture=base_tard order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.045091338 gain=2.577422 bound_over_reference=1.012087 stop=Evaluations partition_ms=31973.292 pool_bytes=655360
fixture=base_tard order=Second budget=32767 max_leaf=[24.898438,24.902344]x[-0.281250,-0.275391] cell_bound=0.045091338 spectral_branch=ordre_deux first=0.045199446 second=0.045091338 cut2=0.045091342 cut1=0.045091342 cut_half=0.045091342
fixture=base_tard order=Spectral budget=2047 evaluations=2047 leaves=1024 bound=0.115908585 gain=1.002682 bound_over_reference=2.601598 stop=Evaluations partition_ms=1456.378 pool_bytes=655360
fixture=base_tard order=Spectral budget=2047 max_leaf=[56.000000,60.000000]x[6.000000,12.000000] cell_bound=0.115908585 spectral_branch=coupure_1 first=0.116488002 second=0.254347324 cut2=0.129638180 cut1=0.115908585 cut_half=0.116585441
fixture=base_tard order=Spectral budget=8191 evaluations=8191 leaves=4096 bound=0.103082366 gain=1.127442 bound_over_reference=2.313710 stop=Evaluations partition_ms=6319.330 pool_bytes=655360
fixture=base_tard order=Spectral budget=8191 max_leaf=[58.000000,60.000000]x[-6.000000,-3.000000] cell_bound=0.103082366 spectral_branch=coupure_1 first=0.116488002 second=0.145646125 cut2=0.120903365 cut1=0.103082366 cut_half=0.115466930
fixture=base_tard order=Spectral budget=16383 evaluations=16383 leaves=8192 bound=0.091391511 gain=1.271665 bound_over_reference=2.051306 stop=Evaluations partition_ms=12912.191 pool_bytes=655360
fixture=base_tard order=Spectral budget=16383 max_leaf=[24.000000,25.000000]x[0.000000,1.500000] cell_bound=0.091391511 spectral_branch=coupure_2 first=0.116488002 second=0.091421701 cut2=0.091391511 cut1=0.092409313 cut_half=0.108879186
fixture=base_tard order=Spectral budget=32767 evaluations=32767 leaves=16384 bound=0.045091256 gain=2.577427 bound_over_reference=1.012085 stop=Evaluations partition_ms=26246.299 pool_bytes=655360
fixture=base_tard order=Spectral budget=32767 max_leaf=[24.925781,24.929688]x[0.298828,0.301758] cell_bound=0.045091256 spectral_branch=ordre_deux first=0.045118038 second=0.045091256 cut2=0.045091260 cut1=0.045091260 cut_half=0.045091260
```

## Base, passage 2

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.842 image_time_admitted=true modes=4096
fixture=base grid=4x3 rectangles=1024 bound=0.115437612 second_order_max=0.115437612 cut2_max=0.169602275 cut1_max=0.117667072 cut_half_max=0.115685761 gain_over_second=1.000000 gain_over_global=0.997691 bound_over_reference=1.641532 wins_first=30 wins_second=0 wins_cut2=0 wins_cut1=994 wins_cut_half=0 mass_total=0.135052577 mass_fraction_lt_half=0.0062 mass_fraction_half_1=0.0556 mass_fraction_1_2=0.2917 mass_fraction_ge_2=0.6464 grid_ms=702.133
fixture=base grid=4x3 worst_cell=[16.000,-12.000] branch=ordre_un first=0.115437612 second=0.239123762 cut2: bound=0.143318594 corner=0.010104246 remainder=0.055232592 C_U=0.087302454 G_U=0.077167451 reserve=0.000814263 | cut1: bound=0.115584135 corner=0.002771028 remainder=0.002535920 C_U=0.126699701 G_U=0.109715439 reserve=0.000561729 | cut0.5: bound=0.115613692 corner=0.000343673 remainder=0.000063586 C_U=0.134211510 G_U=0.114668511 reserve=0.000537913
fixture=base grid=2x1.5 rectangles=4096 bound=0.114862204 second_order_max=0.115437612 cut2_max=0.188615158 cut1_max=0.116644293 cut_half_max=0.115503311 gain_over_second=1.005010 gain_over_global=1.002689 bound_over_reference=1.633350 wins_first=0 wins_second=0 wins_cut2=136 wins_cut1=3952 wins_cut_half=8 mass_total=0.135041401 mass_fraction_lt_half=0.0619 mass_fraction_half_1=0.2917 mass_fraction_1_2=0.6256 mass_fraction_ge_2=0.0208 grid_ms=2992.373
fixture=base grid=2x1.5 worst_cell=[16.000,-3.000] branch=coupure_demi first=0.115437612 second=0.131662637 cut2: bound=0.128114820 corner=0.032863937 remainder=0.091886677 C_U=0.002804678 G_U=0.002239268 reserve=0.001124902 | cut1: bound=0.116644293 corner=0.025000896 remainder=0.013808551 C_U=0.087291263 G_U=0.077159375 reserve=0.000675455 | cut0.5: bound=0.114862204 corner=0.003971571 remainder=0.000634032 C_U=0.126688510 G_U=0.109707393 reserve=0.000549187
fixture=base grid=1x0.75 rectangles=16384 bound=0.099504478 second_order_max=0.104472235 cut2_max=0.104469143 cut1_max=0.105573438 cut_half_max=0.104567230 gain_over_second=1.049925 gain_over_global=1.157446 bound_over_reference=1.414961 wins_first=0 wins_second=0 wins_cut2=16146 wins_cut1=228 wins_cut_half=10 mass_total=0.135041788 mass_fraction_lt_half=0.3536 mass_fraction_half_1=0.6256 mass_fraction_1_2=0.0208 mass_fraction_ge_2=0.0000 grid_ms=13977.380
fixture=base grid=1x0.75 worst_cell=[12.000,-0.750] branch=coupure_1 first=0.115437612 second=0.099923901 cut2: bound=0.099920802 corner=0.074440852 remainder=0.024637047 C_U=0.000000000 G_U=0.000000000 reserve=0.000842882 | cut1: bound=0.099504478 corner=0.073461659 remainder=0.022972753 C_U=0.002804678 G_U=0.002239268 reserve=0.000830778 | cut0.5: bound=0.100947633 corner=0.019729735 remainder=0.003452456 C_U=0.087291270 G_U=0.077159375 reserve=0.000606052
fixture=base order=Second budget=2047 evaluations=2047 leaves=1024 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=1931.813 pool_bytes=655360
fixture=base order=Second budget=2047 max_leaf=[-28.000000,-24.000000]x[30.000000,33.000000] cell_bound=0.115437612 spectral_branch=coupure_1 first=0.115437612 second=0.231279492 cut2=0.134771347 cut1=0.113935649 cut_half=0.115415588
fixture=base order=Second budget=8191 evaluations=8191 leaves=4096 bound=0.115437612 gain=0.997691 bound_over_reference=1.641532 stop=Evaluations partition_ms=7800.926 pool_bytes=655360
fixture=base order=Second budget=8191 max_leaf=[2.000000,4.000000]x[-6.000000,-4.500000] cell_bound=0.115437612 spectral_branch=coupure_1 first=0.115437612 second=0.133188710 cut2=0.129808888 cut1=0.107362226 cut_half=0.112963505
fixture=base order=Second budget=16383 evaluations=16383 leaves=8192 bound=0.098670244 gain=1.167232 bound_over_reference=1.403099 stop=Evaluations partition_ms=15844.798 pool_bytes=655360
fixture=base order=Second budget=16383 max_leaf=[38.000000,40.000000]x[-16.500000,-15.000000] cell_bound=0.098670244 spectral_branch=coupure_1 first=0.115437612 second=0.098670244 cut2=0.095315091 cut1=0.093314096 cut_half=0.111366391
fixture=base order=Second budget=32767 evaluations=32767 leaves=16384 bound=0.070740171 gain=1.628086 bound_over_reference=1.005931 stop=Evaluations partition_ms=31626.523 pool_bytes=655360
fixture=base order=Second budget=32767 max_leaf=[9.201172,9.203125]x[-0.019043,-0.017578] cell_bound=0.070740171 spectral_branch=ordre_un first=0.070740171 second=0.070860974 cut2=0.070860982 cut1=0.070860982 cut_half=0.070860982
fixture=base order=Spectral budget=2047 evaluations=2047 leaves=1024 bound=0.114765890 gain=1.003531 bound_over_reference=1.631980 stop=Evaluations partition_ms=1459.411 pool_bytes=655360
fixture=base order=Spectral budget=2047 max_leaf=[-60.000000,-56.000000]x[24.000000,30.000000] cell_bound=0.114765890 spectral_branch=coupure_1 first=0.115437612 second=0.251751542 cut2=0.128132269 cut1=0.114765890 cut_half=0.115637429
fixture=base order=Spectral budget=8191 evaluations=8191 leaves=4096 bound=0.102004133 gain=1.129083 bound_over_reference=1.450507 stop=Evaluations partition_ms=6205.925 pool_bytes=655360
fixture=base order=Spectral budget=8191 max_leaf=[-50.000000,-48.000000]x[-48.000000,-45.000000] cell_bound=0.102004133 spectral_branch=coupure_1 first=0.115437612 second=0.146247000 cut2=0.120645143 cut1=0.102004133 cut_half=0.114159644
fixture=base order=Spectral budget=16383 evaluations=16383 leaves=8192 bound=0.091857083 gain=1.253807 bound_over_reference=1.306215 stop=Evaluations partition_ms=12887.259 pool_bytes=655360
fixture=base order=Spectral budget=16383 max_leaf=[24.000000,26.000000]x[-40.500000,-39.000000] cell_bound=0.091857083 spectral_branch=coupure_1 first=0.115437612 second=0.098686747 cut2=0.095347360 cut1=0.091857083 cut_half=0.111277096
fixture=base order=Spectral budget=32767 evaluations=32767 leaves=16384 bound=0.070740089 gain=1.628088 bound_over_reference=1.005930 stop=Evaluations partition_ms=26154.954 pool_bytes=655360
fixture=base order=Spectral budget=32767 max_leaf=[9.246094,9.250000]x[0.114258,0.117188] cell_bound=0.070740089 spectral_branch=ordre_deux first=0.070761286 second=0.070740089 cut2=0.070740096 cut1=0.070740096 cut_half=0.070740096
```

## Micro-mesure alternée, code publié (ordre deux par la passe `<false>`)

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.136 image_time_admitted=true modes=4096
micro round=0 which=ordre_un us_per_call=490.8
micro round=0 which=ordre_deux us_per_call=844.9
micro round=0 which=spectrale us_per_call=752.0
micro round=1 which=spectrale us_per_call=708.9
micro round=1 which=ordre_deux us_per_call=911.4
micro round=1 which=ordre_un us_per_call=506.1
micro round=2 which=ordre_un us_per_call=527.4
micro round=2 which=ordre_deux us_per_call=902.1
micro round=2 which=spectrale us_per_call=755.1
micro round=3 which=spectrale us_per_call=719.6
micro round=3 which=ordre_deux us_per_call=881.1
micro round=3 which=ordre_un us_per_call=502.2
micro round=4 which=ordre_un us_per_call=528.2
micro round=4 which=ordre_deux us_per_call=900.5
micro round=4 which=spectrale us_per_call=758.2
micro round=5 which=spectrale us_per_call=728.4
micro round=5 which=ordre_deux us_per_call=921.6
micro round=5 which=ordre_un us_per_call=499.6
micro best_us ordre_un=490.8 ordre_deux=844.9 spectrale=708.9
```

## Expérience non publiée : ordre deux passé par la passe `<true>`, source rétabli ensuite

```text
S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.178 image_time_admitted=true modes=4096
micro round=0 which=ordre_un us_per_call=484.9
micro round=0 which=ordre_deux us_per_call=715.8
micro round=0 which=spectrale us_per_call=693.6
micro round=1 which=spectrale us_per_call=709.6
micro round=1 which=ordre_deux us_per_call=710.5
micro round=1 which=ordre_un us_per_call=522.8
micro round=2 which=ordre_un us_per_call=525.9
micro round=2 which=ordre_deux us_per_call=715.2
micro round=2 which=spectrale us_per_call=719.8
micro round=3 which=spectrale us_per_call=717.3
micro round=3 which=ordre_deux us_per_call=710.7
micro round=3 which=ordre_un us_per_call=500.1
micro round=4 which=ordre_un us_per_call=497.0
micro round=4 which=ordre_deux us_per_call=757.3
micro round=4 which=spectrale us_per_call=715.0
micro round=5 which=spectrale us_per_call=720.2
micro round=5 which=ordre_deux us_per_call=717.5
micro round=5 which=ordre_un us_per_call=503.5
micro best_us ordre_un=484.9 ordre_deux=710.5 spectrale=693.6
```
