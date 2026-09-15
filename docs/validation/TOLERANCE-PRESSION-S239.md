# La tolérance physique de la pression de δ — S239, 2026-09-15

Traite **A273**, ouverte par [PRESSION-PLANCHER-S238](PRESSION-PLANCHER-S238.md) §4.4 : des pas
**convergés** au critère premier rendent une divergence projetée de 1,02·10⁻⁵ (128×64, fond en
bosse de `delta_precision`) et 1,58·10⁻⁵ (256×128, famille S231) pour **10⁻⁵ déclaré avant
construction** par [CANDIDAT-DELTA-S199](CANDIDAT-DELTA-S199.md) §5 critère 4.

## 1. Protocole, écrit avant mesure et construction

### 1.1 Deux quantités, deux normes

- **Critère premier d'arrêt**, hérité de S199 et conservé par S231 : le vrai résidu recalculé
  vérifie `‖b − Ap‖₂ / ‖b‖₂ ≤ 10⁻⁶`. Norme **2**, relative au second membre. Aucune provenance
  physique (S238 §1.1).
- **Tolérance physique**, déclarée par S199 §5 critère 4 : `D = max|div u|·dx / max|u| ≤ 10⁻⁵`,
  sur le champ **corrigé**. Norme **maximale**, normalisée par la vitesse et la maille.
- **Identité discrète**, vérifiée par S238 P4 à 7,2·10⁻⁸ de son second membre (≤ 64 ε) :
  `div u = r/scale` dans chaque ligne du système, `scale = −ρ/dt`, parce que `scale·k1 = −1`.

### 1.2 Pourquoi le critère premier ne borne pas la tolérance — décomposition exacte

De l'identité, `D` se factorise **exactement** en trois nombres sans dimension :

```
D = ρ · θ · Λ        avec   ρ = ‖r‖₂/‖b‖₂        (ce que le critère premier borne, ≤ 10⁻⁶)
                            θ = max|r| / ‖r‖₂    (concentration du résidu, ∈ [N^(−1/2), 1])
                            Λ = ‖b‖₂·dx / (|scale|·max|u|)   (forme du second membre)
```

Le critère premier ne contrôle que `ρ`. `θ` et `Λ` dépendent de la taille de la grille et de la
scène, et **rien dans le chemin actuel ne les borne**. Un pas peut donc converger au critère
premier et manquer la tolérance : c'est A273, et ce n'est pas un défaut de convergence.

Deux comportements limites encadrent `θ` : un résidu étalé donne `θ ≈ N^(−1/2)`, un résidu
concentré sur quelques mailles donne `θ ≈ 1`. Pour un second membre lisse à l'échelle physique,
`Λ` est **indépendant de la résolution** (`‖b‖₂ ∝ √N`, `dx ∝ 1/n_x`). La croissance observée de
`D` avec la taille doit donc venir de `θ`, d'un second membre non lisse à l'échelle de la maille,
ou des deux. **Ce que le calcul ne tranche pas, la mesure le tranche** (§2).

### 1.3 Une question de définition, à mesurer et non à trancher d'autorité

`max|u|` est pris aujourd'hui sur **toutes** les faces, y compris celles des mailles d'air du
mode mobile, qui portent des vitesses extrapolées et non une contrainte, tandis que `max|div u|`
saute ces mailles. Une vitesse extrapolée grande **diminue** `D` sans qu'aucune eau soit mieux
projetée. L'écart entre `D` et sa variante restreinte aux faces mouillées est mesuré au §2. Ce
protocole ne change pas la définition de S199 ; il publie de combien elle dépend de ce choix.

### 1.4 Règle proposée

Une tolérance déclarée avant construction est une **condition d'acceptation**, pas un diagnostic
publié après coup.

1. **Critère premier inchangé** : `ρ ≤ 10⁻⁶` reste nécessaire.
2. **La tolérance physique devient nécessaire dans tout chemin d'acceptation** :
   `accepté ⟺ b = 0`, ou `D ≤ 10⁻⁵` **et** (`ρ ≤ 10⁻⁶` ou plancher certifié d'ADR-143).
   Le chemin du plancher l'exigeait déjà (ADR-143) ; le chemin premier ne l'exigeait pas.
3. **La boucle poursuit** tant que `ρ ≤ 10⁻⁶` est atteint sans que `D ≤ 10⁻⁵` le soit. La cible
   resserrée est `‖r‖₂ ≤ 10⁻⁵·|scale|·max|u|/dx`, qui suffit car `max|r| ≤ ‖r‖₂` — une
   majoration, pas un facteur choisi.
4. **Un pas qui ne peut pas la tenir est dégradé** : plafond d'itérations (ADR-007 §2) ou
   plancher d'arrondi (ADR-143). Il le déclare au lieu de l'annoncer reçu.
5. `D` est calculée dans la boucle **sans tampon nouveau ni allocation** : `max|div u|` par
   l'identité `max|r|/|scale|` sur les lignes du système, `max|u|` par une réduction qui applique
   la correction face par face sans la matérialiser. Le contrat « un `Err` numérique conserve
   u/w/p » interdit d'écrire dans `u`/`w` pendant la boucle, et il est respecté.

Aucun nombre nouveau : `10⁻⁶` est celui de S231, `10⁻⁵` celui de S199.

### 1.5 Critères de réception, déclarés avant construction

1. **Loi mesurée avant la règle** : `ρ`, `θ`, `Λ` et `D` contre la taille (16×8 à 256×128, second
   membre S231 et fond en bosse), plus le nombre d'itérations supplémentaires pour atteindre
   `D ≤ 10⁻⁵` et la valeur de `D` atteignable au plancher f32.
2. **Au bit** : tout cas aujourd'hui accepté dont `D ≤ 10⁻⁵` garde ses bits — empreinte du filtre
   S232 `0xc5ab1eadb094d058`, tests S231/S233/S237, plancher S238.
3. **Les deux cas d'A273** tiennent `10⁻⁵` ou sont **dégradés** ; aucun pas accepté au-dessus.
4. **Réception S237 conservée** : 5 cm / 128 colonnes, profil ≤ 2 %, harmonique `2k` ≤ 20 %,
   tolérances de S237 inchangées ; plancher d'ADR-143 conservé.
5. **Système sans solution** (Neumann pur à second membre de moyenne non nulle) : dégradé.
6. **Identité** `max|div u| = max|r|/|scale|` vérifiée sur les lignes, modes fixe et mobile.
7. **Coût** (ADR-131) : itérations et millisecondes par pas avant/après, sur le même banc ;
   techniques présentes, absentes et domaine mesuré.

### 1.6 Arrêt

Règle construite, ADR, cas reçus. **Ou** constat chiffré que f32 ne peut pas tenir `10⁻⁵` à ces
tailles — et alors une requalification datée de la tolérance, à provenance physique, jamais un
seuil relevé après coup pour obtenir du vert.

## 2. La loi, mesurée avant la règle

Test ignoré `tolerance_law_against_size_s239`. Un pas depuis le repos, `dt = 2 ms`, domaine 8 × 4 m,
surface `4 + 0,01·sin`, deux fonds : plat (`0,5`) et en bosse (`0,4 + 0,6·e^{−d²}`, celui de
`delta_precision`). Plafond 20 000 itérations, celui du banc. `ρ`, `θ` et `Λ` sont relevés à la
relance qui arrête le solveur ; `D` calculée par le même code que le rapport, et identique à lui.

**Au critère premier actuel (`ρ ≤ 10⁻⁶`)** :

| grille | mailles | fond | itérations | `ρ` | `θ` | `Λ` | `D` |
|---|---:|---|---:|---:|---:|---:|---:|
| 16×8 | 128 | plat | 28 | 6,84·10⁻⁷ | 0,314 | 17,8 | 3,82·10⁻⁶ |
| 16×8 | 128 | bosse | 52 | 5,33·10⁻⁷ | 0,299 | 17,3 | 2,76·10⁻⁶ |
| 32×16 | 512 | plat | 58 | 4,96·10⁻⁷ | 0,143 | 33,8 | 2,40·10⁻⁶ |
| 32×16 | 512 | bosse | 95 | 6,35·10⁻⁷ | 0,179 | 33,3 | 3,79·10⁻⁶ |
| 64×32 | 2 048 | plat | 112 | 7,88·10⁻⁷ | 0,0936 | 69,3 | 5,12·10⁻⁶ |
| 64×32 | 2 048 | bosse | 179 | 8,70·10⁻⁷ | 0,1388 | 68,8 | 8,30·10⁻⁶ |
| 128×64 | 8 192 | plat | 219 | 7,16·10⁻⁷ | 0,0728 | 154,7 | 8,07·10⁻⁶ |
| 128×64 | 8 192 | bosse | 347 | 8,01·10⁻⁷ | 0,0829 | 153,8 | **1,021·10⁻⁵** |
| 256×128 | 32 768 | plat | 417 | 9,97·10⁻⁷ | 0,0437 | 362,0 | **1,578·10⁻⁵** |
| 256×128 | 32 768 | bosse | 671 | 9,50·10⁻⁷ | 0,0494 | 360,2 | **1,691·10⁻⁵** |

**Ce que la décomposition rend lisible.**

1. **`Λ` double à chaque raffinement** — 17,8 / 33,8 / 69,3 / 154,7 / 362,0, soit `Λ ∝ n_x`. Le §1.2
   annonçait `Λ` constant pour un second membre **lisse à l'échelle physique** : il ne l'est pas. Un
   `Λ ∝ n_x` signifie `‖div u*‖₂/√N ∝ max|u|/dx` — la divergence prédite vit **à l'échelle de la
   maille**, pas à celle de l'écoulement. C'est attendu d'un champ obtenu en un pas depuis le repos,
   dont la divergence est portée par la couche de surface et le fond coupé, et non réparti.
2. **`θ` décroît plus lentement que `N^(−1/2)`** — 0,314 / 0,143 / 0,0936 / 0,0728 / 0,0437, soit des
   rapports 0,46 / 0,65 / 0,78 / 0,60 par raffinement, quand un résidu étalé donnerait 0,50 et un
   résidu concentré 1,00. Le résidu du gradient conjugué **n'est ni étalé ni ponctuel**.
3. **Leur produit croît** : `θ·Λ` = 5,6 / 4,8 / 6,5 / 11,3 / 15,8 (plat). À `ρ` fixé, `D` croît donc
   d'environ 30 % par raffinement. **La taille seule fait franchir la tolérance**, et aucune valeur du
   critère premier ne l'en empêche : c'est A273, mesurée et non plus constatée.
4. Le fond en bosse aggrave `θ` sans toucher `Λ` : il concentre le résidu, il ne change pas la forme
   du second membre.

**En resserrant le critère premier** (remplacement de seuil réservé aux tests) :

| grille | fond | cible | itérations | `ρ` | `D` | issue |
|---|---|---:|---:|---:|---:|---|
| 128×64 | bosse | 10⁻⁶ | 347 | 8,01·10⁻⁷ | 1,021·10⁻⁵ | au-dessus de la tolérance |
| 128×64 | bosse | 3·10⁻⁷ | **370** | 2,75·10⁻⁷ | **2,99·10⁻⁶** | tenue, **+6,6 % d'itérations** |
| 256×128 | plat | 10⁻⁶ | 417 | 9,97·10⁻⁷ | 1,578·10⁻⁵ | au-dessus de la tolérance |
| 256×128 | plat | 3·10⁻⁷ | 448 | 7,74·10⁻⁷ | **1,093·10⁻⁵** | **plancher d'ADR-143**, dégradé |

**Le résultat qui tranche.** À 8 192 mailles, la tolérance de S199 est **tenue pour 7 % d'itérations
en plus**. À 32 768 mailles, elle ne l'est **pas** : le certificat d'arrondi d'ADR-143 arrête le
solveur à `ρ = 7,7·10⁻⁷`, `D = 1,09·10⁻⁵`, et aucune itération supplémentaire ne peut abaisser
`max|r|`, qui est alors indiscernable de l'arrondi de son propre calcul. **f32 manque la tolérance
de 9 % à 32 768 mailles** avec cet opérateur et ce second membre. 32 768 mailles n'a jamais été reçu
(S231 : 8 192 ; S238 : 16 384 en mode mobile, où `D` valait 1,45·10⁻⁷).

**Impasse de l'instrument, à ne pas reproduire.** Abaisser le seuil *dès le départ* place le solveur
dans un régime où le résidu **récurrent** ne réclame jamais la convergence : la boucle interne ne
ressort pas, aucune relance n'a lieu, et **le certificat d'arrondi — qui ne vit qu'au point de
relance — n'est jamais consulté**. Le cas 256×128 en bosse a ainsi consommé le plafond de 20 000
itérations sans verdict, et la mesure a été abandonnée. La règle du §1.4 n'a pas ce défaut : elle
atteint d'abord le critère premier, et c'est **là**, à la relance, que le certificat est consulté.

## 3. La règle, après mesure — et pourquoi elle ne porte pas sur toutes les lignes

Le §1.4 proposait la tolérance comme condition d'acceptation sur **toutes** les mailles mouillées.
Construite ainsi, elle a fait refuser au **premier pas** le cas de topologie mobile de S237
(`mobile_step_keeps_rest_conserves_volume_and_changes_topology_s237`, 16×20, onde de 10 cm). Le
diagnostic est écrit ici parce qu'il décide la règle.

**Ce que la mesure montre** (test ignoré `a273_mobile_first_steps_s239`, pas 0) :

| relance | `ρ` | `θ` | `D` toutes lignes | `D` lignes franches |
|---:|---:|---:|---:|---:|
| 61 | 8,12·10⁻⁷ | 0,192 | 7,50·10⁻⁵ | 7,50·10⁻⁵ |
| 69 | 1,25·10⁻⁷ | 0,649 | 6,00·10⁻⁵ | **7,09·10⁻⁶** |
| 76 | 2,12·10⁻⁸ | 0,239 | 1,53·10⁻⁵ | 2,16·10⁻⁶ |
| 81 | 1,12·10⁻⁸ | 0,450 | 1,54·10⁻⁵ | 7,44·10⁻⁷ |
| 83 | 8,51·10⁻⁹ | 0,595 | 1,51·10⁻⁵ | **4,46·10⁻⁷** |

`ρ` descend de deux ordres de grandeur ; `D` **plafonne à 1,5·10⁻⁵** et n'y bouge plus. Les cinq
pires lignes, au refus :

| ligne | `(i,k)` | `|r|` | `θ_surface` | voisin du haut mouillé | préconditionneur |
|---|---|---:|---:|---|---:|
| 258 | (2,16) | **3,1250·10⁻²** = 2⁻⁵ | 0,206 | non | 1,99·10⁻³ |
| 256 | (0,16) | **1,5625·10⁻²** = 2⁻⁶ | 0,296 | non | 2,91·10⁻³ |
| 257 | (1,16) | **1,5625·10⁻²** = 2⁻⁶ | 0,266 | non | 2,31·10⁻³ |
| 237 | (13,14) | 7,8125·10⁻³ = 2⁻⁷ | 0,794 | non | 3,67·10⁻³ |
| 249 | (9,15) | 7,8125·10⁻³ = 2⁻⁷ | 0,268 | non | 2,32·10⁻³ |

Ce sont **exactement les mailles de surface** — la face haute donne sur un fantôme —, leur diagonale
vaut environ 500 (`1/θ_surface`), et leurs résidus sont des **puissances de deux exactes** : un ulp
d'une ligne de magnitude ≈ 2¹⁸. Le résidu y est au plancher de f32, comme ADR-143 le décrit ; aucune
itération ne l'abaisse. Les lignes **franches** du même pas, elles, descendent de 7,5·10⁻⁵ à
4,5·10⁻⁷ : la conservation, là où elle est énoncée, converge.

**Deux corroborations écrites avant S239, jamais rapprochées.** S199 a inscrit dans son propre tableau
de réception, en face du `< 10⁻⁵`, « configuration unitaire testée ; diagnostic pondéré par
ouvertures, **pas borne universelle** ». Et S237 a desserré à 10⁻⁴ l'assertion de divergence de son
essai de projection mobile (`mobile_operator_is_symmetric_with_side_and_top_ghosts_s237`) sans dire
pourquoi. A273 partait donc d'une prémisse à moitié vraie : le nombre **est** franchi, mais il n'avait
jamais été une borne universelle.

### Règle finale ([ADR-144](../adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md))

1. `accepté ⟺ b = 0`, ou `(ρ ≤ 10⁻⁶ ou plancher certifié)` **et** `D_franches ≤ 10⁻⁵`.
2. **Ligne franche** : maille mouillée dont aucune face ouverte ne donne sur un fantôme de surface.
   Sans fantôme — couvercle fixe —, `D_franches = D`, et la règle est celle du §1.4 sans réserve.
3. Critère premier atteint, tolérance non tenue → la boucle **poursuit**, cible
   `‖r‖₂² ← ‖r‖₂²·(10⁻⁵/D_franches)²`. Aucun facteur choisi : `D` est proportionnelle à `max|r|` par
   l'identité, et l'acceptation reste la valeur **exacte** à la relance suivante.
4. Plafond ou plancher atteint sans la tolérance → **dégradé**, déclaré.
5. `Report.divergence` garde exactement son sens ; `Report.divergence_plain` porte la décision.

## 4. Réception — 2026-09-15, une machine sur secteur

### 4.1 Les deux cas d'A273

| cas | avant S239 | après S239 |
|---|---|---|
| 128×64, fond en bosse, 8 192 mailles | 347 itérations, `D` = **1,021·10⁻⁵**, annoncé **reçu** | **348** itérations, `D` = **5,84·10⁻⁶**, reçu |
| 256×128, fond plat, 32 768 mailles | 417 itérations, `D` = **1,578·10⁻⁵**, annoncé **reçu** | **420** itérations, plancher d'ADR-143, `D` = 1,335·10⁻⁵, **dégradé** |

Test `physical_tolerance_is_required_for_acceptance_s239` (le premier cas, exécuté par la suite) et
réception ignorée `a273_at_32768_cells_is_declared_degraded_s239` (le second). Aucun pas n'est plus
annoncé reçu au-dessus de la tolérance. **Le coût de la correction est d'une itération ; le coût du
refus, de trois.**

### 4.2 Au bit — ce qui bouge et ce qui ne bouge pas

`delta_filters`, campagne S232, comparée à [FLUX-COUPES-S232](FLUX-COUPES-S232.md) :

| fond | `Q₁₂₈` S232 | `Q₁₂₈` S239 | itérations | ordre |
|---|---|---|---|---|
| plat | 2,296743171·10⁻⁴ | **2,296743171·10⁻⁴** | 219 → **219** | 1,947 → 1,947 |
| lisse | 2,260028664·10⁻⁴ | 2,260028660·10⁻⁴ | 347 → **348** | 1,957 → 1,957 |
| transition tanh | 2,257532700·10⁻⁴ | 2,257539558·10⁻⁴ | 330 → **348** | 1,966 → 1,959 |

Le fond plat — dont `D` valait 8,07·10⁻⁶, sous la tolérance — est **identique jusqu'au dernier
chiffre publié**, itérations comprises. Les deux fonds coupés, dont `D` la dépassait, poursuivent et
changent au neuvième chiffre. Verdict d'ADR-038 §4 conservé : ordres 1,95 / 1,96 / 1,96 ≥ 1,8,
résidus de Richardson 0,025 / 0,024 / 0,024 %, rejeu local identique en bits.
**Empreinte : `0xc5ab1eadb094d058` → `0xfb12b2092df4ee6d`**, et ces deux cas en sont la cause entière.

`delta_precision` : dix cas, aucun dégradé ; les huit dont `D` était sous la tolérance gardent leurs
itérations (28, 52, 58, 69, 95, 95, 112, 179, 219) ; le dixième — 128 sur fond en bosse — passe de
347 à 348 et sa divergence de 1,021·10⁻⁵ à 5,842·10⁻⁶.

### 4.3 La réception de S237 et S238 est conservée

`delta_mobile plancher` : 5 cm, une période, pas de 1 ms, plafond 4 000.

| colonnes | profil `/a` | harmonique `2k` | pas au plancher | divergence max au plancher | itérations max | ms médian |
|---:|---:|---:|---:|---:|---:|---:|
| 32 | **0,850 %** | **2,44 %** | 13 | 1,454·10⁻⁷ | 142 | 4,67 |
| 64 | **0,552 %** | **1,41 %** | 33 | 1,502·10⁻⁷ | 297 | 34,7 |
| 128 | **0,252 %** | **0,71 %** | 34 | 1,498·10⁻⁷ | 592 | **258,4** |

Sous 2 % et 20 %, décroissants — les tolérances de S237 sont inchangées, et S238 publiait
0,850 / 0,550 / 0,252 % et 2,44 / 1,41 / 0,71 %. Dérive de volume 7,45·10⁻⁹ / 1,86·10⁻⁹ /
2,79·10⁻⁹ m. **Le mode mobile n'est pas dégradé par la règle** : ses lignes franches tiennent la
tolérance, et ce sont elles qui décident.

### 4.4 Coût (ADR-131)

- **Techniques présentes** : préconditionnement diagonal, f32, certificat d'arrondi et détection de
  cycle d'ADR-143, une correction et une divergence supplémentaires par relance qui atteint le
  critère premier, cible resserrée déduite de la mesure.
- **Techniques absentes** : tout préconditionneur plus fort (multigrille, factorisation incomplète),
  tout résidu recalculé en précision double, toute exploitation du GPU.
- **Domaine mesuré** : une machine sur secteur, 2D, 128 à 32 768 mailles, une période, un `dt`.
- **Chiffres** : 258,4 ms par pas médian à 128 colonnes contre **256 ms** en S238, soit **+0,9 %** ;
  pire pas 592 itérations contre 526, soit **+12,5 %** ; +1 itération sur 348 pour le cas d'A273
  reçu ; +3 sur 420 pour le cas refusé. Aucun budget n'est confronté ici.

### 4.5 Ce qui n'est pas reçu

- **Les lignes à fantôme de surface.** Leur divergence plafonne — 1,5·10⁻⁵ mesuré sur un pas, jusqu'à
  7,5·10⁻⁵ avant la règle — et rien ici ne la borne. Le plancher mesuré vaut pour **une** géométrie ;
  sa loi en fonction de `θ_surface` et de la taille reste à établir.
- **Au-delà de 32 768 mailles en 2D**, et **la 3D** : l'opérateur à six faces demande `γ₁₀`, et le mur
  mesuré ici y sera rencontré d'emblée. Le remède candidat — résidu recalculé en précision double, ou
  préconditionneur plus fort — n'est pas éprouvé.
- Un seul `dt`, une seule famille de second membre au-delà de 16 384 mailles, une seule machine.
