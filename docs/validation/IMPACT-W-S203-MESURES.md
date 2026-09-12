# S203 — Relevés bruts : impact visible porté par W

2026-09-13. Sorties complètes de `render_impact` (release, AMD Ryzen AI 7 350, un fil).
Commandes depuis `code/` : `cargo run -p water-core --release --example render_impact <mode>`.
Analyse et limites : [IMPACT-W-S203](IMPACT-W-S203.md). Rien n'est interprété ici.

## scene

```text
# constats de construction S203
hs=0.25 recipe=0xf38d3e37a5d4d4d6 floor_L1=0.101365 directionnel=0.095543 pi/7=0.448799 marge=0.347434 hauteur_L1=0.3825m
hs=0.5 recipe=0xae7cc08b64111fda floor_L1=0.202731 directionnel=0.191085 pi/7=0.448799 marge=0.246068 hauteur_L1=0.7651m
hs=1 recipe=0x4f43024368a7aa17 floor_L1=0.405461 directionnel=0.382170 pi/7=0.448799 marge=0.043338 hauteur_L1=1.5302m
hs=1.5 recipe=0x7e5cc32275ccce4e floor_L1=0.608192 directionnel=0.573255 pi/7=0.448799 marge=-0.159393 hauteur_L1=2.2952m
scene hs=0.5 floor=0.202731 budget_impact=0.246068 lambda=3.3500m E=164.000J fraction=0.005 fraction_max=0.006698 E_ref=32800.0J
N=64 slope_max=0.212607 slope_L1=0.381646 floor+slope_max=0.415338 <= pi/7 true eta_centre=0.15482m
N=128 slope_max=0.212607 slope_L1=0.381645 floor+slope_max=0.415338 <= pi/7 true eta_centre=0.15482m
N=256 slope_max=0.212607 slope_L1=0.381646 floor+slope_max=0.415338 <= pi/7 true eta_centre=0.15482m
hs=0.5 pente_reelle_echantillonnee=0.140513 floor_L1=0.202731 rapport_L1/echantillon=1.4428 (512x512 m pas 1 m, t=0..120 s)
hs=1.5 scene refusee : plancher L1 de B 0.6081917 >= pi/7 0.44879895 : aucune composition possible
hs=1.5 pente_reelle_echantillonnee=0.421538 floor_L1=0.608192 rapport_L1/echantillon=1.4428 (512x512 m pas 1 m, t=0..120 s)
```

## seams

```text
# coutures d'emprise S203, lambda=3.3500 E=164.0
N=64 A=2s R=32.0m bord_eta=0.001mm (t=1.88s) bord_pente=0.00001 horizon_eta=78.407mm (r=0.00m) horizon_pente=0.10586 seuil=3.000mm passe=false
N=128 A=2s R=68.0m bord_eta=0.000mm (t=1.63s) bord_pente=0.00000 horizon_eta=78.407mm (r=0.00m) horizon_pente=0.10586 seuil=3.000mm passe=false
N=256 A=2s R=139.5m bord_eta=0.000mm (t=2.00s) bord_pente=0.00000 horizon_eta=78.407mm (r=0.00m) horizon_pente=0.10586 seuil=3.000mm passe=false
N=512 A=2s R=282.5m bord_eta=0.000mm (t=0.00s) bord_pente=0.00000 horizon_eta=78.407mm (r=0.00m) horizon_pente=0.10586 seuil=3.000mm passe=false
N=64 A=4s R=29.0m bord_eta=0.003mm (t=3.09s) bord_pente=0.00000 horizon_eta=20.044mm (r=3.16m) horizon_pente=0.04948 seuil=3.000mm passe=false
N=128 A=4s R=64.5m bord_eta=0.000mm (t=2.61s) bord_pente=0.00000 horizon_eta=20.044mm (r=3.16m) horizon_pente=0.04948 seuil=3.000mm passe=false
N=256 A=4s R=136.0m bord_eta=0.000mm (t=3.16s) bord_pente=0.00000 horizon_eta=20.044mm (r=3.16m) horizon_pente=0.04948 seuil=3.000mm passe=false
N=512 A=4s R=279.0m bord_eta=0.000mm (t=3.56s) bord_pente=0.00000 horizon_eta=20.044mm (r=3.16m) horizon_pente=0.04948 seuil=3.000mm passe=false
N=64 A=6s R=26.0m bord_eta=0.004mm (t=4.22s) bord_pente=0.00001 horizon_eta=16.009mm (r=5.84m) horizon_pente=0.03817 seuil=3.000mm passe=false
N=128 A=6s R=61.5m bord_eta=0.000mm (t=4.09s) bord_pente=0.00000 horizon_eta=16.009mm (r=5.84m) horizon_pente=0.03817 seuil=3.000mm passe=false
N=256 A=6s R=133.0m bord_eta=0.000mm (t=2.05s) bord_pente=0.00000 horizon_eta=16.009mm (r=5.84m) horizon_pente=0.03817 seuil=3.000mm passe=false
N=512 A=6s R=276.0m bord_eta=0.000mm (t=5.62s) bord_pente=0.00000 horizon_eta=16.009mm (r=5.84m) horizon_pente=0.03817 seuil=3.000mm passe=false
N=64 A=8s R=22.5m bord_eta=0.018mm (t=7.94s) bord_pente=0.00004 horizon_eta=12.976mm (r=8.52m) horizon_pente=0.03305 seuil=3.000mm passe=false
N=128 A=8s R=58.5m bord_eta=0.000mm (t=7.24s) bord_pente=0.00000 horizon_eta=12.976mm (r=8.52m) horizon_pente=0.03305 seuil=3.000mm passe=false
N=256 A=8s R=129.5m bord_eta=0.000mm (t=1.02s) bord_pente=0.00000 horizon_eta=12.976mm (r=8.52m) horizon_pente=0.03305 seuil=3.000mm passe=false
N=512 A=8s R=272.5m bord_eta=0.000mm (t=2.62s) bord_pente=0.00000 horizon_eta=12.976mm (r=8.52m) horizon_pente=0.03305 seuil=3.000mm passe=false
N=64 A=12s R=16.0m bord_eta=1.983mm (t=12.00s) bord_pente=0.00315 horizon_eta=10.117mm (r=12.64m) horizon_pente=0.02578 seuil=3.000mm passe=false
N=128 A=12s R=52.0m bord_eta=0.000mm (t=12.00s) bord_pente=0.00000 horizon_eta=10.117mm (r=12.64m) horizon_pente=0.02578 seuil=3.000mm passe=false
N=256 A=12s R=123.5m bord_eta=0.000mm (t=6.22s) bord_pente=0.00000 horizon_eta=10.117mm (r=12.64m) horizon_pente=0.02578 seuil=3.000mm passe=false
N=512 A=12s R=266.0m bord_eta=0.000mm (t=11.90s) bord_pente=0.00000 horizon_eta=10.117mm (r=12.64m) horizon_pente=0.02578 seuil=3.000mm passe=false
N=64 A=16s R=9.5m bord_eta=12.285mm (t=9.12s) bord_pente=0.02980 horizon_eta=0.621mm (r=9.28m) horizon_pente=0.00176 seuil=3.000mm passe=false
N=128 A=16s R=45.5m bord_eta=0.001mm (t=14.31s) bord_pente=0.00000 horizon_eta=8.269mm (r=15.54m) horizon_pente=0.02114 seuil=3.000mm passe=false
N=256 A=16s R=117.0m bord_eta=0.000mm (t=4.15s) bord_pente=0.00000 horizon_eta=8.269mm (r=15.54m) horizon_pente=0.02114 seuil=3.000mm passe=false
N=512 A=16s R=259.5m bord_eta=0.000mm (t=7.55s) bord_pente=0.00000 horizon_eta=8.269mm (r=15.54m) horizon_pente=0.02114 seuil=3.000mm passe=false
N=64 A=24s aucun rayon admis
N=128 A=24s R=32.5m bord_eta=0.584mm (t=23.46s) bord_pente=0.00109 horizon_eta=6.164mm (r=23.78m) horizon_pente=0.01571 seuil=3.000mm passe=false
N=256 A=24s R=104.0m bord_eta=0.000mm (t=22.97s) bord_pente=0.00000 horizon_eta=6.164mm (r=23.78m) horizon_pente=0.01571 seuil=3.000mm passe=false
N=512 A=24s R=247.0m bord_eta=0.000mm (t=21.21s) bord_pente=0.00000 horizon_eta=6.164mm (r=23.78m) horizon_pente=0.01571 seuil=3.000mm passe=false
N=64 A=32s aucun rayon admis
N=128 A=32s R=19.5m bord_eta=7.382mm (t=19.29s) bord_pente=0.01839 horizon_eta=0.085mm (r=19.10m) horizon_pente=0.00035 seuil=3.000mm passe=false
N=256 A=32s R=91.0m bord_eta=0.000mm (t=31.53s) bord_pente=0.00000 horizon_eta=4.760mm (r=32.04m) horizon_pente=0.01243 seuil=3.000mm passe=false
N=512 A=32s R=234.0m bord_eta=0.000mm (t=8.82s) bord_pente=0.00000 horizon_eta=4.760mm (r=32.04m) horizon_pente=0.01243 seuil=3.000mm passe=false
# extension horizons longs
N=256 A=48s R=65.0m bord_eta=0.281mm (t=47.52s) bord_pente=0.00044 horizon_eta=3.246mm (r=46.18m) horizon_pente=0.00868 seuil=3.000mm passe=false
N=512 A=48s R=208.0m bord_eta=0.000mm (t=18.41s) bord_pente=0.00000 horizon_eta=3.246mm (r=46.18m) horizon_pente=0.00868 seuil=3.000mm passe=false
N=256 A=56s R=52.0m bord_eta=2.937mm (t=52.87s) bord_pente=0.00773 horizon_eta=2.255mm (r=51.08m) horizon_pente=0.00699 seuil=3.000mm passe=true
N=512 A=56s R=195.0m bord_eta=0.000mm (t=51.31s) bord_pente=0.00000 horizon_eta=2.796mm (r=54.44m) horizon_pente=0.00748 seuil=3.000mm passe=true
N=256 A=64s R=39.0m bord_eta=3.935mm (t=39.32s) bord_pente=0.01022 horizon_eta=0.010mm (r=38.94m) horizon_pente=0.00003 seuil=3.000mm passe=false
N=512 A=64s R=182.0m bord_eta=0.000mm (t=44.27s) bord_pente=0.00000 horizon_eta=2.440mm (r=61.54m) horizon_pente=0.00659 seuil=3.000mm passe=true
N=256 A=72s R=26.0m bord_eta=5.769mm (t=25.76s) bord_pente=0.01458 horizon_eta=0.001mm (r=25.82m) horizon_pente=0.00000 seuil=3.000mm passe=false
N=512 A=72s R=169.0m bord_eta=0.000mm (t=71.02s) bord_pente=0.00000 horizon_eta=2.159mm (r=69.80m) horizon_pente=0.00587 seuil=3.000mm passe=true
N=256 A=96s aucun rayon admis
N=512 A=96s R=130.5m bord_eta=0.145mm (t=95.93s) bord_pente=0.00018 horizon_eta=1.620mm (r=93.46m) horizon_pente=0.00436 seuil=3.000mm passe=true
# balayage du rayon, N512
N=512 A=56s R=30.0m bord_eta=5.047mm (t=29.69s) horizon_eta=0.006mm (r=29.30m) passe=false
N=512 A=56s R=35.0m bord_eta=4.372mm (t=35.39s) horizon_eta=0.017mm (r=34.40m) passe=false
N=512 A=56s R=40.0m bord_eta=3.838mm (t=40.46s) horizon_eta=0.072mm (r=39.58m) passe=false
N=512 A=56s R=45.0m bord_eta=3.407mm (t=45.53s) horizon_eta=0.463mm (r=44.98m) passe=false
N=512 A=56s R=50.0m bord_eta=3.057mm (t=51.22s) horizon_eta=1.877mm (r=49.98m) passe=false
N=512 A=56s R=55.0m bord_eta=2.764mm (t=55.66s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=60.0m bord_eta=2.038mm (t=55.44s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=65.0m bord_eta=1.227mm (t=55.51s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=70.0m bord_eta=0.676mm (t=56.00s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=75.0m bord_eta=0.280mm (t=55.52s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=80.0m bord_eta=0.112mm (t=55.61s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=85.0m bord_eta=0.041mm (t=55.55s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=90.0m bord_eta=0.014mm (t=55.36s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=95.0m bord_eta=0.007mm (t=56.00s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=100.0m bord_eta=0.003mm (t=55.69s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=105.0m bord_eta=0.001mm (t=55.28s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=110.0m bord_eta=0.001mm (t=55.83s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=115.0m bord_eta=0.000mm (t=55.45s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=120.0m bord_eta=0.000mm (t=56.00s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=125.0m bord_eta=0.000mm (t=55.32s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=130.0m bord_eta=0.000mm (t=55.81s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=135.0m bord_eta=0.000mm (t=55.52s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=140.0m bord_eta=0.000mm (t=56.00s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=145.0m bord_eta=0.000mm (t=54.50s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=150.0m bord_eta=0.000mm (t=55.00s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=155.0m bord_eta=0.000mm (t=53.40s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=160.0m bord_eta=0.000mm (t=47.69s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=165.0m bord_eta=0.000mm (t=54.44s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=170.0m bord_eta=0.000mm (t=52.81s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=175.0m bord_eta=0.000mm (t=55.47s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=180.0m bord_eta=0.000mm (t=47.73s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=185.0m bord_eta=0.000mm (t=25.48s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=190.0m bord_eta=0.000mm (t=36.35s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=56s R=195.0m bord_eta=0.000mm (t=51.31s) horizon_eta=2.796mm (r=54.44m) passe=true
N=512 A=64s R=30.0m bord_eta=5.047mm (t=29.69s) horizon_eta=0.005mm (r=0.00m) passe=false
N=512 A=64s R=35.0m bord_eta=4.372mm (t=35.39s) horizon_eta=0.005mm (r=34.70m) passe=false
N=512 A=64s R=40.0m bord_eta=3.838mm (t=40.46s) horizon_eta=0.012mm (r=39.78m) passe=false
N=512 A=64s R=45.0m bord_eta=3.407mm (t=45.53s) horizon_eta=0.045mm (r=44.94m) passe=false
N=512 A=64s R=50.0m bord_eta=3.057mm (t=51.22s) horizon_eta=0.188mm (r=49.40m) passe=false
N=512 A=64s R=55.0m bord_eta=2.771mm (t=56.28s) horizon_eta=0.981mm (r=54.98m) passe=true
N=512 A=64s R=60.0m bord_eta=2.533mm (t=61.35s) horizon_eta=2.210mm (r=59.28m) passe=true
N=512 A=64s R=65.0m bord_eta=2.268mm (t=63.86s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=70.0m bord_eta=1.615mm (t=63.52s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=75.0m bord_eta=1.002mm (t=63.53s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=80.0m bord_eta=0.591mm (t=64.00s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=85.0m bord_eta=0.277mm (t=63.53s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=90.0m bord_eta=0.124mm (t=63.64s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=95.0m bord_eta=0.050mm (t=63.61s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=100.0m bord_eta=0.019mm (t=63.46s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=105.0m bord_eta=0.008mm (t=64.00s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=110.0m bord_eta=0.004mm (t=63.87s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=115.0m bord_eta=0.002mm (t=63.48s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=120.0m bord_eta=0.001mm (t=64.00s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=125.0m bord_eta=0.000mm (t=63.58s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=130.0m bord_eta=0.000mm (t=64.00s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=135.0m bord_eta=0.000mm (t=63.73s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=140.0m bord_eta=0.000mm (t=63.07s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=145.0m bord_eta=0.000mm (t=63.62s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=150.0m bord_eta=0.000mm (t=63.26s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=155.0m bord_eta=0.000mm (t=61.70s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=160.0m bord_eta=0.000mm (t=60.14s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=165.0m bord_eta=0.000mm (t=62.56s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=170.0m bord_eta=0.000mm (t=59.08s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=175.0m bord_eta=0.000mm (t=59.64s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=64s R=180.0m bord_eta=0.000mm (t=60.04s) horizon_eta=2.440mm (r=61.54m) passe=true
N=512 A=72s R=30.0m bord_eta=5.047mm (t=29.69s) horizon_eta=0.001mm (r=29.98m) passe=false
N=512 A=72s R=35.0m bord_eta=4.372mm (t=35.39s) horizon_eta=0.002mm (r=34.98m) passe=false
N=512 A=72s R=40.0m bord_eta=3.838mm (t=40.46s) horizon_eta=0.003mm (r=39.98m) passe=false
N=512 A=72s R=45.0m bord_eta=3.407mm (t=45.53s) horizon_eta=0.007mm (r=44.32m) passe=false
N=512 A=72s R=50.0m bord_eta=3.057mm (t=51.22s) horizon_eta=0.023mm (r=49.44m) passe=false
N=512 A=72s R=55.0m bord_eta=2.771mm (t=56.28s) horizon_eta=0.109mm (r=54.72m) passe=true
N=512 A=72s R=60.0m bord_eta=2.533mm (t=61.35s) horizon_eta=0.464mm (r=59.34m) passe=true
N=512 A=72s R=65.0m bord_eta=2.331mm (t=66.41s) horizon_eta=1.491mm (r=64.28m) passe=true
N=512 A=72s R=70.0m bord_eta=2.161mm (t=71.47s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=75.0m bord_eta=1.857mm (t=72.00s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=80.0m bord_eta=1.315mm (t=71.58s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=85.0m bord_eta=0.845mm (t=71.55s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=90.0m bord_eta=0.523mm (t=72.00s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=95.0m bord_eta=0.270mm (t=71.54s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=100.0m bord_eta=0.134mm (t=71.67s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=105.0m bord_eta=0.059mm (t=71.67s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=110.0m bord_eta=0.025mm (t=71.54s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=115.0m bord_eta=0.010mm (t=71.33s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=120.0m bord_eta=0.005mm (t=72.00s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=125.0m bord_eta=0.002mm (t=71.66s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=130.0m bord_eta=0.001mm (t=71.27s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=135.0m bord_eta=0.001mm (t=71.85s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=140.0m bord_eta=0.000mm (t=71.34s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=145.0m bord_eta=0.000mm (t=71.86s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=150.0m bord_eta=0.000mm (t=71.47s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=155.0m bord_eta=0.000mm (t=69.96s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=160.0m bord_eta=0.000mm (t=70.47s) horizon_eta=2.159mm (r=69.80m) passe=true
N=512 A=72s R=165.0m bord_eta=0.000mm (t=70.99s) horizon_eta=2.159mm (r=69.80m) passe=true
```

## controls

```text
# P3b-1 accord N256/N512, A56 R52
age=1s max|d_eta|=0.0000mm max|d_pente_x|=0.000000 accord=true
age=3s max|d_eta|=0.0000mm max|d_pente_x|=0.000000 accord=true
age=6s max|d_eta|=0.0000mm max|d_pente_x|=0.000000 accord=true
age=30s max|d_eta|=0.0000mm max|d_pente_x|=0.000000 accord=true
age=56s max|d_eta|=0.0001mm max|d_pente_x|=0.000000 accord=true
# P3b-2 homothetie lambda x2 (b=2 m), coutures relatives
lambda'=6.7000m E'=1312.0J (lambda=3.3500m E=164.0J)
N=256 R=52->104.00m A=56.000->79.196s bord 1.8971%->1.8971% horizon 1.4563%->1.4753% eta0 0.15482->0.21895m homothetie=true
N=512 R=55->110.00m A=64.000->90.510s bord 1.7900%->1.7904% horizon 0.6334%->0.6420% eta0 0.15482->0.21895m homothetie=true
```

## observer

```text
# observateur S203 : camera [0.0, -18.0, 7.0] -> [0.0, 35.0, 0.0], 640x360, champ vertical 50 deg
impact pixel=(320.0,224.1) distance=28.86m
lambda=3.3500m au point d'impact : le_long=11.05px en_travers=45.10px
lambda/2=1.6750m au point d'impact : le_long=5.51px en_travers=22.55px
lambda sous 2 px : le_long a partir de Some(67.0) m, en_travers a partir de None m (horizontal depuis la camera, marche limitee a 600 m)
lambda/2 sous 2 px : le_long a partir de Some(47.5) m, en_travers a partir de Some(325.5) m (horizontal depuis la camera, marche limitee a 600 m)
emprise R=52 m, observateur a 28.00 m du centre en plan : observateur_dans_emprise=true
# part non portee par W
Froude d'entree v/sqrt(g*2b)=1.806 (cavite franche au-dela de ~5, SPEC-002 §3)
energie hors ondes (1-f)*E_ref=32636J sur E_ref=32800J (f a calibrer B2)
majorant balistique d'une projection a v : hauteur v^2/2g=3.262m, duree 2v/g=1.631s
demi-largeur mouillee b=1m = 0,2985 lambda (impact_generator::ALPHA)
```

## cost

```text
# cout par point S203 : 4096 points dans R, 3 chauffes / 11 mesures, release, un fil
age_s B_med_us B_max_us W_med_us W_max_us BW_med_us BW_max_us BW_us_par_point points_BW_dans_2ms table_build_med_us table_M table_eval_med_us table_us_par_point max_err_table_mm
1 6551.5 6794.1 49974.1 57528.0 57137.6 58042.0 13.950 143 2748.4 249 62.7 0.0153 0.0062
3 6701.5 7346.9 51811.0 59049.6 59027.2 60676.6 14.411 139 2655.9 249 62.4 0.0152 0.0061
6 6625.9 6996.7 51813.7 54386.5 59343.7 63929.9 14.488 138 2759.7 249 83.3 0.0203 0.0035
30 6495.0 6765.9 56225.1 58927.2 58562.7 60411.6 14.298 140 2757.4 249 62.3 0.0152 0.0014
```

## render ../captures 1 · 3 · 6

```text
age=1s t=13.000s hs=0.5 N=256 R=52m A=56s lambda=3.350m E=164.0J
impact water=289269 unresolved=0 refused=0 evals=9499461 w_evals=5075829 touched_px=125979 residual_max=0.003000m render_ms=55190.725 rgb_fnv=0xae92143729673447
temoin water=289269 unresolved=0 refused=0 evals=9499414 w_evals=0 touched_px=125979 residual_max=0.003000m render_ms=10507.541 rgb_fnv=0xdb298dee39248f0d
pixels_differents=6780 dont_hors_emprise=0
age=3s t=15.000s hs=0.5 N=256 R=52m A=56s lambda=3.350m E=164.0J
impact water=289260 unresolved=0 refused=0 evals=9495898 w_evals=5062984 touched_px=125979 residual_max=0.003000m render_ms=56739.189 rgb_fnv=0x3dba0d3acf15447a
temoin water=289260 unresolved=0 refused=0 evals=9495848 w_evals=0 touched_px=125979 residual_max=0.003000m render_ms=10582.851 rgb_fnv=0x14271a7145740ba1
pixels_differents=8640 dont_hors_emprise=0
age=6s t=18.000s hs=0.5 N=256 R=52m A=56s lambda=3.350m E=164.0J
impact water=289245 unresolved=0 refused=0 evals=9490157 w_evals=5064027 touched_px=125979 residual_max=0.003000m render_ms=58171.796 rgb_fnv=0x7e057b5f0ccac53e
temoin water=289245 unresolved=0 refused=0 evals=9490843 w_evals=0 touched_px=125979 residual_max=0.003000m render_ms=10620.828 rgb_fnv=0x590a5964b0a93cf0
pixels_differents=15737 dont_hors_emprise=0
```

## Contrôle d'extraction S201 (support ray_view)

```text
image=../captures/b-s201.ppm 640x360 t=12s Hs=1.5m Tp=6s N=32 seed=201 recipe=0x7e5cc32275ccce4e
water=289491 unresolved=0 evals=10021895 residual_max=0.003000m render_ms=11787.267 rgb_fnv=0xa52ff81902b150c3
```
