# Résidu progressif temporel — S272

## Référence déclarée avant mesure

Prolonger S271 à une durée de 2 s. Domaine x∈[0,4], profondeur h=1 m,
k=π/2, phase 0,37, g=9,81, rho=1025 ; a=0,01 puis 0,005 m pour contrôler
la troncature d'ordre deux. dt=2 ms, dx=0,125 puis 0,0625 et 0,03125 si nécessaire.
Pas d'éponge : référence avec v normal nul aux bords, identique au contrat réel.
Il ne s'agit donc pas d'une réception de transparence avec éponge.

Écrire φ=φ1+φ2, η=η1+η2, où φ1=(ag/ω) C(z) sinθ, θ=kx−ωt+phase,
C=cosh(k(z+h))/cosh(kh), ω²=gk tanh(kh). Développement des conditions exactes
à z=η, en gardant l'ordre a² :

- η2_t = G φ2 + K ; K=−U1 η1_x + η1 W1_z = a q k sin(2θ), q=agk/ω.
- φ2_t = −g η2 + D ; D=−η1 φ1_tz −(U1²+W1²)/2 à z=0.
- Avec T=tanh(kh), D=D0+Dc cos(2θ), D0=a²gkT/2−q²(1+T²)/4,
  Dc=a²gkT/2−q²(1−T²)/4.

G est l'opérateur harmonique de profondeur finie à murs de Neumann. Dans la
base cos(nπx/L), λ_n=k_n tanh(k_n h). Chaque mode vérifie
η_n'=λ_n φ_n+K_n, φ_n'=−gη_n+D_n, η_n(0)=φ_n(0)=0.
Donc η_n''+gλ_nη_n=λ_nD_n+K_n', η_n'(0)=K_n(0).
Résoudre exactement cette ODE forcée sinusoïdale ; projections spatiales
analytiques, sans solveur MAC ni interpolation de ses résultats.

## Critères et arrêt

- Contrôler les projections par quadrature indépendante et l'ODE par intégration
  RK4 à pas divisé par deux ; erreur relative oracle <1e-6.
- Oracle tronqué à 128 puis 256 modes : écart L2 espace-temps <0,1 % sur les
  mêmes points de mesure (x centres, t de 0,05 à 2 s).
- Comparer **η' évolué**, pas B+η' dominé par le fond. Erreur L2 espace-temps
  <=2 % (seuil B4) et décroissante en raffinant dx ; diviser dt par deux doit
  changer la solution de <=0,5 % de la norme de l'oracle. Tous les pas reçus.
- Amplitude divisée par deux : comparer les champs normalisés par a², écart
  <=1 % à résolution reçue, sinon troncature non qualifiée.
- Si refus numérique ou seuil manqué : conserver la mesure, isoler une cause
  avant correction ; ne pas remplacer la métrique par l'erreur du champ total.
- Arrêt au reçu borné ou diagnostic chiffré avec prochain correctif concret.

La comparaison reçoit le résidu dans un domaine de perturbation fermé traversé
par un fond prescrit. Une houle incidente entièrement transparente, d'autres
spectres, la 3D, le budget et les forces restent hors de ce banc.


## Réception de l'instrument

`outils/residu_progressif.py` résout chaque mode en forme fermée. `test_residu_progressif.py`
contrôle les projections par quadrature (32 768 points), la solution par RK4 à
1 ms puis 0,5 ms, et les forçages K/D par développement des conditions exactes
à la surface. **3 tests réussis**. Pire écart RK4 testé : 3,37e-8 (mode 63),
réduit à 2,10e-9 au demi-pas ; mode 64 non forcé exactement nul. Un premier test
divisait par zéro pour ce mode nul : instrument corrigé avant verdict.

Différence 128/256 modes de l'oracle : 0,0295 %, 0,0369 %, 0,0528 % aux trois
grilles, sous le seuil de 0,1 %. Oracle reçu sur les points comparés ; aucune
supposition de frontière périodique. Les murs sont représentés par le potentiel
perturbatif de Neumann, tandis que le fond prescrit traverse le domaine.

## Campagne — verdict refusé

Les cinq passages terminent leurs 6 000 pas cumulés, sans refus du solveur.
L'erreur porte sur η' seul, sur les 40 instants de 0,05 à 2 s, toutes colonnes.

| dx (m) | amplitude (m) | dt (ms) | erreur L2 du résidu | itérations max |
|---|---:|---:|---:|---:|
| 0,125 | 0,01 | 2 | 22,5206 % | 64 |
| 0,0625 | 0,01 | 2 | 13,0423 % | 127 |
| 0,03125 | 0,01 | 2 | 8,6778 % | 243 |
| 0,03125 | 0,005 | 2 | 6,5779 % | 243 |
| 0,03125 | 0,01 | 1 | 8,6181 % | 243 |

**Seuil 2 % manqué.** La variation au demi-pas vaut 0,6693 % de la norme oracle,
au-dessus du contrôle 0,5 %. La variation des champs divisés par a² entre les deux
amplitudes vaut 2,3922 %, au-dessus de 1 % : la troncature d'ordre deux n'est pas
qualifiée pour une réception à ce seuil. Ne pas attribuer tout l'écart au solveur.
Le temps n'explique cependant pas à lui seul l'écart : le demi-pas reste à 8,62 %.
Erreurs sur horizons de 0,25 / 0,5 / 1 / 2 s : 3,54 / 5,65 / 6,09 / 8,68 %,
diagnostic seulement ; aucune fenêtre raccourcie choisie pour obtenir un reçu.

## Diagnostic indépendant et prochain correctif

`diagnostic_bande_s272.py` isole la quadrature verticale de la bande, avec exactement
les mêmes surfaces aux faces, contre l'intégrale analytique de U. La méthode
actuelle multiplie la longueur de bande par U au centre de couche ; pour une
bande partielle, ce point n'est pas le centre de la bande. Une reconstruction
linéaire utilisant la dérivée verticale déjà fournie donne :

| dx | a | erreur flux actuel | erreur reconstruction linéaire |
|---|---:|---:|---:|
| 0,125 | 0,01 | 8,4069 % | 0,4189 % |
| 0,0625 | 0,01 | 3,8570 % | 0,0892 % |
| 0,03125 | 0,01 | 1,6015 % | 0,0162 % |
| 0,03125 | 0,005 | 1,9261 % | 0,0223 % |

C'est une **erreur de quadrature démontrée**, pas l'attribution de tout l'écart
temporel. La reconstruction utilise U(zc) et ∂zU(zc), sans nouvel échantillon.
Elle n'est pas intégrée au produit en S272. Lot S273 : l'intégrer dans le transport
réel, aux faces internes et externes, préserver le fond nul, tester les bandes
signées/coupées et la transaction ; rejouer le banc et qualifier les deux biais
restants avant verdict. Cette construction prime un troisième lot d'instrument
ou de simples raffinements. A276 reste prioritaire avant toute étude de coût ;
V, B2 et bathymétrie ne sont pas retirés de la file.

## Reproduction

```powershell
cargo build --release --offline --locked --manifest-path code/Cargo.toml -p water-core --example delta_progressive
& code/target/release/examples/delta_progressive.exe 0.125 0.01 2000 > "$env:TEMP/fluidisim-s272-coarse.log"
& code/target/release/examples/delta_progressive.exe 0.0625 0.01 2000 > "$env:TEMP/fluidisim-s272-medium.log"
& code/target/release/examples/delta_progressive.exe 0.03125 0.01 2000 > "$env:TEMP/fluidisim-s272-fine.log"
& code/target/release/examples/delta_progressive.exe 0.03125 0.005 2000 > "$env:TEMP/fluidisim-s272-halfamplitude.log"
& code/target/release/examples/delta_progressive.exe 0.03125 0.01 1000 > "$env:TEMP/fluidisim-s272-halfdt.log"
python -B -X utf8 outils/test_residu_progressif.py
python -B -X utf8 outils/residu_progressif.py "$env:TEMP/fluidisim-s272-fine.log"
python -B -X utf8 outils/residu_progressif.py --sensibilite "$env:TEMP/fluidisim-s272-fine.log" "$env:TEMP/fluidisim-s272-halfdt.log"
python -B -X utf8 outils/residu_progressif.py --sensibilite "$env:TEMP/fluidisim-s272-fine.log" "$env:TEMP/fluidisim-s272-halfamplitude.log"
python -B -X utf8 outils/diagnostic_bande_s272.py
```

L'analyse retourne code 1 quand le seuil 2 % est manqué : résultat attendu ici,
à conserver. Les sensibilités impriment une mesure, sans constituer à elles seules
un verdict automatique. Aucun coût temps réel mesuré, aucune bibliothèque produit
changée, suite S270 non rejouée. Le démarrage S271 reste reçu sur son périmètre.
