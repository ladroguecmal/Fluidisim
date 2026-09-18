# Bande du fond en quadrature linéaire — S273

Réception d'[ADR-166](../adr/ADR-166-quadrature-lineaire-de-la-bande.md) dans le transport réel
de `step_perturbation_mobile`, puis nouveau passage du banc temporel
[S272](RESIDU-TEMPOREL-S272.md). Critères écrits **avant** le code et la campagne.

## Critères de construction

1. **Quadrature** — fixture S272 (h = 1 m, k = π/2, phase 0,37, a = 0,01 m, repos aligné),
   surfaces du pas réel (demi-somme des colonnes à l'intérieur, colonne voisine plus fond au
   bord). Flux de bande de **toutes** les faces, calculés par le code produit, contre l'intégrale
   analytique `q·cosθ·(sinh k(h+ζ_f) − sinh kh)/(k cosh kh)`. Erreur L2 relative :
   ≤ 0,5 / 0,1 / 0,025 % à dx = 0,125 / 0,0625 / 0,03125, et rapport ≥ 3,5 à chaque
   raffinement (ordre deux ; diagnostic S272 : 0,419 / 0,089 / 0,016 %). Témoin rectangle
   recalculé dans le test : ≥ 1,5 % à la maille fine, pour prouver que le test distingue le défaut.
2. **Bandes signées et coupées** — fond affine `U = U0 + S·(z − repos)`, pour lequel la règle est
   exacte : surface sous et sur le repos, bande couvrant trois couches, surface sur une face de
   couche, fond solide au-dessus du repos à la face extérieure. Écart à l'intégrale exacte
   ≤ 2e-7 m²/s (arrondi f32 de S270).
3. **Identités** — fond nul S253 au bit, fond uniforme S270 ≤ 8 ε, témoin sans résidus,
   transaction et expiration : tests existants, inchangés.
4. **Suite complète** sans échec ; toute valeur imprimée qui bouge est expliquée par le fond
   non uniforme. Zéro allocation dans le pas (tests d'exécution existants).

## Critères de la campagne

Les cinq passages S272, mêmes commandes, traces `fluidisim-s273-*.log`. **Critères S272
inchangés** : erreur L2 de `η'` ≤ 2 % à la maille fine et décroissante ; demi-pas ≤ 0,5 % ;
champs divisés par a² ≤ 1 %. Aucun seuil relevé, aucune fenêtre raccourcie.

Qualification des deux biais, **diagnostic seulement**, déclarée ici avant mesure :

- **amplitude** : si `η' = a²η₂ + a³η₃ + …`, alors `N* = 2·N(a/2) − N(a)`, avec `N = η'/a²`,
  élimine le terme d'ordre trois. L'écart de `N*` à l'oracle `η₂` sépare l'erreur numérique
  (écart qui reste) de la troncature de l'oracle (écart qui disparaît) ;
- **temps** : la différence au demi-pas estime l'erreur temporelle (le double si ordre un).

Arrêt : verdict, ou diagnostic chiffré avec le prochain correctif concret. Pas de nouvelle
campagne de raffinement dans cette session.

## Réception de la construction

`band_layer` (`delta_coupling.rs`) porte la règle ; les faces intérieures l'appellent avec un
plancher nul et l'ouverture d'avant, les faces extérieures avec le fond de leur colonne.

| critère | résultat |
|---|---|
| 1. quadrature, flux produit de toutes les faces | **0,4190 / 0,0892 / 0,0162 %** (rapports 4,7 et 5,5), identiques au diagnostic S272 ; témoin rectangle 8,41 / 3,86 / 1,60 % |
| 2. fond affine, 9 bandes au bord × 3 couples (U0, S), 17 faces intérieures | écart maximal **3,9·10⁻⁸** m²/s |
| 3. identités | fond nul S253 au bit, fond uniforme S270, témoin sans résidus, refus et expirations : inchangés, verts |
| 4. suite complète | **480 réussis, 0 échec**, 21 ignorés (368 cœur, 17 intégrations, 95 harnais) ; release : S253 128 colonnes et harmonique reçus |

Valeurs déplacées, toutes par un fond non uniforme :

- S271, cinématique initiale : **3,048 / 1,237 / 0,585 % → 1,509 / 0,445 / 0,137 %** ; il reste
  la différence finie de la divergence, d'ordre deux.
- S253, harmonique `2k` couplée à 32 colonnes, une période : 2,17 → **2,14 %** (seuil 20 %) ;
  témoin sans résidus 99,56 % inchangé (bande éteinte).
- S254, intégration du B de production : 590 faces mouillées au-dessus du plan moyen, comme
  avant ; `u'` maximal 5,36·10⁻³ m/s aux murs.
- S253, campagne à 128 colonnes rejouée (`delta_mobile couple_cas couple`, une période) : profil
  couplé **0,162 → 0,148 %** à 5 cm et **0,213 → 0,178 %** à 10 cm ; `b₂` 0,34 → 0,35 % et
  0,53 → 0,53 % ; pas au plancher 116 → 119 et 27 → 23, tous reçus. Critères 3 et 4 (2 % et
  20 %) tenus. Durées non comparables : sept processus concurrents, ce n'est pas une mesure de
  coût.

## Campagne — refus maintenu aux critères S272, cause resserrée

Les sept passages terminent tous leurs pas, sans refus du solveur. Erreur L2 de `η'` seul, 40
instants de 0,05 à 2 s, toutes colonnes, contre l'oracle modal S272 inchangé (garde 128/256 modes
< 0,053 %).

| dx (m) | a (m) | dt (ms) | erreur S272 | **erreur S273** | itérations max |
|---|---:|---:|---:|---:|---:|
| 0,125 | 0,01 | 2 | 22,52 % | **9,11 %** | 64 |
| 0,0625 | 0,01 | 2 | 13,04 % | **6,18 %** | 127 |
| 0,03125 | 0,01 | 2 | 8,68 % | **5,88 %** | 243 |
| 0,03125 | 0,005 | 2 | 6,58 % | **3,23 %** | 242 |
| 0,03125 | 0,01 | 1 | 8,62 % | **5,78 %** | 243 |

**Critères S272 manqués** : 5,88 % > 2 % ; demi-pas 0,668 % > 0,5 % ; champs divisés par a²
2,89 % > 1 %. Aucun seuil relevé, aucune fenêtre raccourcie.

**Qualification déclarée en P2.** Les passages à a/2 aux mailles grossière et moyenne ont été
ajoutés **après** la mesure fine, pour voir la convergence du diagnostic ; deux passages de plus,
même fixture, pas de nouvelle maille.

| dx | brute, a = 1 cm | brute, a = 5 mm | écart des champs / a² | **N\* extrapolé** |
|---|---:|---:|---:|---:|
| 0,125 | 9,11 % | 7,99 % | 2,74 % | **7,74 %** |
| 0,0625 | 6,18 % | 3,90 % | 2,85 % | **2,92 %** |
| 0,03125 | 5,88 % | 3,23 % | 2,89 % | **1,77 %** |

Lecture :

- L'écart des champs normalisés par a² **ne dépend pas de la maille** (2,74 → 2,89 %) : ce
  n'est pas une erreur de discrétisation. C'est le contenu d'ordre trois de la solution, absent
  de l'oracle d'ordre deux — environ 5,8 % de `η'` à 1 cm (le double de l'écart normalisé).
- La part d'ordre deux extrapolée **converge** : 7,74 → 2,92 → 1,77 %. À la maille fine, elle
  contient encore l'erreur temporelle (demi-pas : 0,668 %, soit ≈ 1,3 % si l'ordre est un).
- Les nombres se recoupent : avec 1,77 % numérique, la troncature vaudrait √(5,88² − 1,77²) =
  5,61 % à 1 cm et √(3,23² − 1,77²) = 2,70 % à 5 mm, soit proportionnelle à a. Recoupement
  d'ordre de grandeur, pas une décomposition démontrée.

**Conséquence.** Le seuil S272 compare le solveur à une référence qui s'écarte elle-même
d'environ 5,6 % de la solution à cette amplitude : il ne peut pas recevoir le solveur à 2 %,
quel que soit le raffinement. La correction de quadrature était nécessaire (elle divise
l'erreur brute par 2,5 à la maille grossière) mais le reste n'est plus, pour l'essentiel, un
défaut du pas.

**Prochain correctif concret** : un protocole de réception sur `N*`, écrit avant mesure et
justifié par l'indépendance à la maille mesurée ici — `N*` ≤ 2 % et décroissant aux trois
mailles, demi-pas sur `N*` ≤ 0,5 %, à dt = 1 ms avec contrôle 0,5 ms. Sinon, un oracle d'ordre
trois. Hors de ce banc : houle incidente transparente, autres spectres, 3D, coût.

## Reproduction

```powershell
cargo test --offline --locked --manifest-path code/Cargo.toml -p water-core --lib s273 -- --nocapture
cargo build --release --offline --locked --manifest-path code/Cargo.toml -p water-core --example delta_progressive
& code/target/release/examples/delta_progressive.exe 0.03125 0.01 2000 > "$env:TEMP/fluidisim-s273-fine.log"
& code/target/release/examples/delta_progressive.exe 0.03125 0.005 2000 > "$env:TEMP/fluidisim-s273-halfamplitude.log"
python -B -X utf8 outils/residu_progressif.py "$env:TEMP/fluidisim-s273-fine.log"
python -B -X utf8 outils/residu_progressif.py --richardson "$env:TEMP/fluidisim-s273-fine.log" "$env:TEMP/fluidisim-s273-halfamplitude.log"
```

Les autres passages suivent RESIDU-TEMPOREL-S272 (mêmes arguments, préfixe `s273`) ; mailles
grossière et moyenne à a/2 : `0.125 0.005 2000`, `0.0625 0.005 2000`. `test_residu_progressif.py`
porte désormais quatre tests, dont l'extrapolation exacte d'un champ `a²f + a³g`.
