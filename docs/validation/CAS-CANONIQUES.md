# Cas canoniques de validation

- **Statut** : proposée
- **Session** : S03
- **Dépend de** : `SPEC-003`, `SPEC-001`, `SPEC-002`

Dix-huit montages, chacun avec une **référence indépendante du code testé**. Douze ont une
solution fermée : ce sont les seuls cas où l'on peut dire qu'un solveur est *faux* et non
simplement *différent*.

Chaque cas est un fichier de scénario (SPEC-003 §3). Ils constituent la batterie `physics`, et
les six premiers doivent passer avant qu'un solveur candidat soit admis en campagne B3.

---

## Tableau général

| # | Cas | Couche | Référence | Ce qu'il attrape |
|---|---|---|---|---|
| C01 | Repos hydrostatique sur pente | δ | **analytique exacte** | courants parasites |
| C02 | Dispersion monochromatique | δ, W | **analytique** | erreur de célérité → fixe `λ_cut` |
| C03 | Seiche en bassin clos | δ, W | **analytique** | dissipation numérique |
| C04 | Rupture de barrage (Ritter) | δ | **analytique** | fronts, mouillage/séchage |
| C05 | Absorption à la frontière | δ | cible < 1 % | réflexions de l'éponge |
| C06 | Invariance galiléenne | δ, ADR-002 | **auto-référencée** | biais d'advection, référentiels |
| C07 | Sillage profond et peu profond | W | **analytique** | angle de Kelvin, `Fr_h` |
| C08 | Convergence sous raffinement | δ | oracle / Richardson | solveur qui ne converge pas |
| C09 | Conservation masse et énergie | δ, V | **analytique** | fuites, instabilités |
| C10 | Cube flottant | flottabilité | **analytique** | tirant d'eau, période, masse ajoutée |
| C11 | Petit objet léger | flottabilité | **analytique** | divergence, bascule en mode contraint |
| C12 | Vidange d'un réservoir | V | **analytique** | intégration à charge variable |
| C13 | Remontée de bulle | ADR-014 | **analytique** | traînée, vitesse terminale |
| C14 | Couverture de moutons | ADR-014 | **statistique (Monahan)** | écume calée à l'œil |
| C15 | Croissance de glace | ADR-017 | **analytique (Stefan)** | modèle thermique |
| C16 | Ballottement en repère accéléré | ADR-002 | **analytique** | `g_eff` non injectée |
| C17 | Inondation limitée par l'air | ADR-015 | comparatif | arête d'évent absente |
| C18 | Invariants du système | tous | binaire | hash, allocations, budget |
| C19 | Aller-retour de persistance | B, W, V | **binaire** | sauvegarde, reconnexion, arrivée en cours de partie |
| C20 | Impact d'entrée dans l'eau | flottabilité | **analytique** | impulsion de slamming, durée d'impact |
| C21 | Masse d'un compartiment avec et sans δ | V, δ | **binaire** | propriété de la masse, forçage V→δ |

---

## C01 — Repos hydrostatique sur fond en pente

**Montage.** Bassin de 40 m, fond en pente 1:20, eau au repos, aucune perturbation, 60 s.
**Référence.** `u ≡ 0` exactement. La solution est triviale, ce qui est précisément l'intérêt.
**Assertion.** `max|u| < 1 mm/s` après 60 s ; `max|η − η₀| < 1 mm`.

Un très grand nombre de solveurs produisent des **courants parasites** sur un fond incliné, parce
que le gradient de pression et le terme de fond ne s'annulent pas exactement à la discrétisation.
Le symptôme en jeu est un lac qui frissonne sans raison et une eau qui « coule » lentement vers le
bas d'une plage.

C'est le test le moins spectaculaire, le plus rapide, et celui qui élimine le plus de candidats.
Il doit être le premier écrit.

## C02 — Dispersion d'une onde monochromatique

**Montage.** Canal périodique, onde de faible cambrure, λ balayé, `dx` fixé. Mesure de la célérité
sur 10 périodes.
**Référence.** `c = √(gλ/2π)` (SPEC-001 §1).
**Assertion.** erreur de célérité < 2 % pour λ ≥ N·dx, avec **N à déterminer — c'est le résultat
du test**.

Ce cas ne valide pas seulement le solveur : il **produit `λ_cut`**. La plus petite longueur d'onde
que δ transporte correctement, rapportée à `dx`, fixe la frontière W/δ, donc la largeur d'éponge et
le coût minimal d'un domaine (ADR-005 §2.1). À exécuter avant toute autre décision de
dimensionnement.

## C03 — Seiche en bassin clos

**Montage.** Bassin rectangulaire fermé, L = 20 m, h = 2 m, surface initiale inclinée, 20 périodes.
**Référence.** `T = 2L/√(gh) = 9,03 s`.
**Assertions.** erreur de période < 1 % ; **demi-vie d'amplitude** > 15 périodes.

La demi-vie d'amplitude est la mesure directe de la dissipation numérique. C'est elle qui décide si
une houle traverse un domaine ou s'y éteint — et c'est un chiffre qu'on ne pense presque jamais à
mesurer, alors qu'il explique la majorité des « l'eau est molle ».

## C04 — Rupture de barrage (solution de Ritter)

**Montage.** Canal plat sans frottement, hauteur `h₀ = 1 m` à gauche, lit sec à droite, lâcher à
`t = 0`.
**Référence analytique**, pour `−√(gh₀) ≤ x/t ≤ 2√(gh₀)` :

```
h(x,t) = (1/9g)·(2√(g h₀) − x/t)²        u(x,t) = (2/3)·(x/t + √(g h₀))
front aval : x = 2√(g h₀)·t = 6,26 m/s
en x = 0   : h = 4h₀/9 = 0,444 m ,  u = (2/3)√(g h₀) = 2,09 m/s
```

**Assertions.** position du front à ±3 % à t = 2 s ; `h(0)` à ±3 %.

Teste le front de mouillage sur lit sec, cas où beaucoup de solveurs produisent une hauteur
négative ou un front trop lent.

## C05 — Absorption à la frontière

**Montage.** Onde entrante d'amplitude connue, éponge de largeur `L_s = λ/2`, mesure de l'amplitude
réfléchie par séparation des trains montant et descendant.
**Assertion.** `R < 1 %` pour λ ∈ [λ_cut, 4·λ_cut].

Valide directement ADR-005 §2 et le réglage `σ_max ≈ 4c/L_s`.

## C06 — Invariance galiléenne

**Montage.** Le même impact, exécuté une fois dans un repère au repos, une fois dans un repère en
translation uniforme à 10 m/s, une fois dans un repère en rotation.
**Référence.** Les trois résultats doivent coïncider après changement de repère.
**Assertion.** écart RMS de hauteur < 2 % ; forces intégrées sur le solide < 2 %.

Attrape les biais directionnels de l'advection **et** valide qu'`g_eff` et le référentiel sont bien
injectés partout (ADR-002, I-07). Un solveur qui échoue ici produira une eau qui « traîne » derrière
un bateau rapide.

## C07 — Sillage en eau profonde et en eau peu profonde

**Montage.** Coque de 12 m à 5, 8, 10 et 15 m/s, par 200 m puis par 5 m de fond.
**Référence.** demi-angle 19,47° en profond ; `arcsin(1/Fr_h)` au-delà de `Fr_h = 1`
(SPEC-001 §5).
**Assertion.** angle mesuré sur le champ d'écume à ±2° ; en `Fr_h ≈ 1`, amplitude nettement
supérieure au cas profond.

Attrape l'angle de Kelvin codé en dur (angle mort A05).

## C08 — Convergence sous raffinement

**Montage.** Un cas de C02, C04 ou C09, exécuté à `dx`, `dx/2`, `dx/4`.
**Référence.** solution analytique si elle existe, sinon l'oracle lent (SPEC-003 §5.1).
**Mesure.** ordre observé par extrapolation de Richardson :
`p = log₂( |e_h − e_{h/2}| / |e_{h/2} − e_{h/4}| )`.
**Assertion.** `p > 0,8`.

Un solveur qui ne converge pas ne résout pas l'équation qu'on croit : il est **faux**, pas
imprécis. Aucun raffinement ne le sauvera, et aucune campagne de performance n'a de sens tant que
ce test ne passe pas.

## C09 — Conservation de la masse et de l'énergie

**Montage.** Domaine clos, sans frottement, perturbation initiale, 120 s.
**Assertions.** `|dm/dt| < 10⁻³ s⁻¹` ; `dE/dt ≤ 0` en tout temps.

Une énergie qui croît est une instabilité, quel que soit l'aspect visuel à l'instant t. Une masse
qui dérive de 3 % par seconde rend le régime substitutif inutilisable (ADR-010 §6) et le défaut
resterait invisible pendant des mois.

## C10 — Cube flottant

**Montage.** Cube de 0,5 m, `ρ = 500 kg/m³`, lâché à la surface d'une eau calme.
**Référence.**

```
tirant d'eau      d = (ρ_corps/ρ_eau)·H = 0,25 m
période de pilonnement, sans masse ajoutée :
T = 2π·√(ρ_corps·H / (ρ_eau·g)) = 1,00 s
```

**Assertions.** tirant à ±1 % ; période à ±5 % dans la variante sans masse ajoutée ; **période
sensiblement plus longue** dans la variante avec masse ajoutée.

L'écart entre les deux variantes *est* le test de la masse ajoutée (angle mort A26). S'il est nul,
elle n'est pas implémentée.

## C11 — Petit objet léger

**Montage.** Sphère de 4 cm, 2,7 g ; caisse de 1 m², 50 kg ; débris.
**Référence.** `ω = √(ρgA/(m+m_a))` — SPEC-001, ADR-008 §3.
**Assertions.** aucune divergence sur 120 s ; bascule en mode contraint effective dès
`ω·dt > 1` ; aucun tremblement visible en mode contraint.

Le cas le plus petit est le plus dur (leçon L07). Il doit être dans la batterie de base, pas dans
les cas exotiques.

## C12 — Vidange d'un réservoir

**Montage.** Réservoir de 1 m² de section, hauteur d'eau 1 m, orifice de 10 cm² à arête vive au
fond.
**Référence — à charge variable :**

```
t_vidange = (A_réservoir / (C_d · a_orifice)) · √(2·h₀/g)
          = (1 / (0,62·10⁻³)) · √(2/9,81) = 728 s ≈ 12 min
```

**Assertion.** temps de vidange à ±3 % ; masse conservée à la milli-fraction près (arithmétique
entière, ADR-010 §4).

> **Correction apportée par ce cas.** ADR-010 §3 donnait « ≈6 min », valeur obtenue en supposant la
> charge constante. La charge décroît, le débit avec elle, et la vidange réelle dure **deux fois
> plus longtemps**. L'erreur d'un facteur 2 est systématique et se propage à tout l'équilibrage
> d'un gameplay d'avarie. Corrigé dans ADR-010.

## C13 — Remontée d'une bulle

**Montage.** Bulles de 0,1, 1 et 5 mm lâchées à 3 m de profondeur, eau au repos.
**Référence.** Stokes pour 0,1 mm : `v = ρgd²/(18μ) = 5,45 mm/s`. Régime intermédiaire tabulé
(SPEC-002 §2) pour les autres.
**Assertion.** vitesse terminale à ±15 % ; trajectoire verticale selon `−g_eff`, pas selon `+Z`.

## C14 — Couverture de moutons

**Montage.** Haute mer, `U10` de 5 à 25 m/s, mesure de la fraction de surface où le canal actif du
champ `F` dépasse un seuil, moyennée sur 120 s.
**Référence.** `W ≈ 3,84·10⁻⁶·U10^3,41` (SPEC-002 §1).
**Assertion.** dans un facteur 1,5 de la valeur prédite sur toute la plage.

Empêche la dérive vers le trop-blanc, qui est le sens dans lequel un réglage à l'œil part toujours
(leçon L14).

## C15 — Croissance de la glace

**Montage.** Lac abrité, `FDD` imposé, 30 jours simulés en temps accéléré.
**Référence.** `h ≈ 0,035·√FDD` (SPEC-002 §4).
**Assertions.** épaisseur à ±10 % ; aucune plaque ne se forme tant que `Hs > 0,15 m` ; masse
conservée sur un cycle gel/dégel complet.

Le dernier point est le plus important : un lac qui gèle puis dégèle ne doit ni gagner ni perdre
d'eau. Le défaut n'apparaît qu'après des dizaines d'heures de jeu.

## C16 — Ballottement en référentiel accéléré

**Montage.** Cuve de 8 m, remplie à 1,5 m, embarquée dans un référentiel soumis à 0,3 g latéral
puis à une rotation de 0,313 rad/s.
**Référence.** `T = 2π/√( g·(π/L)·tanh(π·h/L) ) ≈ 3,5 s` ; surface au repos perpendiculaire à
`g_eff` ; en rotation, surface d'équilibre cylindrique de flèche 50 cm pour L = 20 m et r = 100 m
(ADR-002 §2.2).
**Assertion.** période à ±10 % ; inclinaison de la surface au repos à ±1° de la normale à `g_eff`.

Échoue immédiatement si un `−9,81·Z` traîne quelque part (I-07).

## C17 — Inondation limitée par l'air

**Montage.** Compartiment étanche de 10 m³, brèche de 1 dm² à 2 m sous la flottaison ; deux
variantes, avec et sans arête d'évent.
**Référence.** sans évent, le débit doit tendre vers zéro à mesure que l'air se comprime ; avec
évent, le débit suit Torricelli.
**Assertion.** rapport des temps de remplissage supérieur à 5 entre les deux variantes.

Valide l'angle mort A29. Si les deux variantes donnent le même temps, l'air n'est pas modélisé et
tous les temps d'avarie du jeu sont trop courts.

## C18 — Invariants du système

Batterie binaire, mode `check`, sans GPU, à chaque commit :

| Assertion | Invariant |
|---|---|
| hash de B identique sur toutes les plateformes cibles | I-03 |
| zéro allocation après initialisation, sur 10 000 ticks | I-06 |
| aucun domaine ne dépasse son budget individuel | I-05 |
| l'hôte serveur compile et tourne sans δ ni rendu | I-04, ADR-020 §3 |
| aucune force gameplay ne varie quand δ est désactivé | I-04 |
| le nombre de paquets `W_rep` au-dessus du seuil est identique sur deux profils de qualité différents *(S05)* | I-15, ADR-021 §4 |
| aucune capacité dérivée n'est lue depuis un profil de qualité *(S05)* | I-16 |

La dernière ligne est la vérification mécanique de l'invariant central : **le même scénario, joué
avec et sans solveur volumétrique, doit produire exactement les mêmes trajectoires d'objets.**
Toute différence est une violation d'I-04, détectée automatiquement plutôt qu'en revue de code.


---

## C19 — Aller-retour de persistance

*(Ajouté en S10, ADR-022 §6.1.)*

**Montage.** Simuler `N` secondes. Écrire un `WaterPersistentState` (ADR-022 §4.2). Repartir d'un
système neuf restauré depuis cet objet, et simuler `M` secondes de plus.

**Assertion.** Le hash de B et de W répliqué à `t = N + M` est **identique** à celui d'une
simulation continue de `N + M` secondes. Les volumes de tous les nœuds V le sont aussi, à
l'entier près — ils sont entiers.

**Ce que cela attrape.** Un descripteur de région oublié dans le format, un événement dont le `ttl`
est mal recalculé au rechargement, un nœud V dont l'écart à la valeur d'auteur est mal appliqué, et
surtout tout état qui aurait été omis du format sans qu'on s'en aperçoive.

**Pourquoi le cas est binaire, et pourquoi c'est remarquable.** Le test est en régime **D1**
(SPEC-003 §2), donc exécutable en mode `check` — moins de 60 s, à chaque commit. Il ne le serait
pas si un état de δ figurait dans la sauvegarde : δ n'est jamais D1, et l'assertion aurait dû être
statistique, c'est-à-dire faible. C'est un bénéfice secondaire direct de l'invariant **I-17**.

**Portée non évidente.** Le fichier de sauvegarde et la charge utile d'une arrivée en cours de
partie sont le **même objet** (ADR-022 §4.1). Ce cas unique couvre donc quatre situations :
sauvegarde/rechargement, arrivée en cours de partie, reconnexion et redémarrage de serveur.


---

## C20 — Impact d'entrée dans l'eau

*(Ajouté en S12, ADR-023 §2.6.)*

**Montage.** Un dièdre de relèvement `β` connu, de demi-largeur `b`, entrant verticalement dans une
eau au repos à vitesse `v` imposée. Trois valeurs de `β` — 10°, 30°, 45° — et deux vitesses.

**Deux références indépendantes, toutes deux fermées.**

1. **Conservation de la quantité de mouvement.** L'impulsion verticale reçue par le solide vaut
   `J = Δ(½·π·ρ·c²)·v_rel` par mètre de longueur mouillée, `c` étant la demi-largeur mouillée
   finale. Assertion : écart < 5 % sur `J`, et **bilan eau + solide conservé à la précision de
   l'intégrateur**. C'est l'assertion principale : elle ne dépend d'aucune théorie de la pression.
2. **Décroissance de la durée d'impact.** `t_impact = 2·b·tan β/(π·v)` : la durée doit varier
   **linéairement en `tan β`** et **en `1/v`**. Assertion sur la pente, pas sur la valeur absolue —
   une pente juste avec un décalage constant révèle un défaut de détection de contact, une pente
   fausse révèle un défaut de modèle.

**Ce que le cas attrape.** Un terme d'impact échantillonné au tick au lieu d'être intégré
analytiquement — il donnerait une impulsion qui dépend de la phase du tick, donc une dispersion
sur des entrées identiques. C'est le défaut que §2.1 d'ADR-023 décrit comme intermittent et
introuvable en jeu.

**Ce que le cas ne cherche pas.** La pression de pic. Elle diverge quand `β → 0` (Wagner), le
coussin d'air et la compressibilité l'écrêtent, et aucune référence fermée n'existe. Un banc qui
l'assertait mesurerait sa propre incertitude — c'est pourquoi ADR-023 §2.3 publie l'impulsion.

**Rattachement** : banc **B6** (flottabilité), batterie `physics`.

---

## C21 — Masse d'un compartiment avec et sans domaine δ

*(Ajouté en S15. Action annoncée par [ADR-025](../adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md)
§4 en S14 et retrouvée non exécutée par l'audit des registres.)*

**Montage.** Un compartiment s'inonde par un orifice, en référentiel fixe puis accéléré. Le scénario
est joué **deux fois** : une fois sans domaine δ, une fois avec un domaine substitutif actif au-dessus
du nœud pendant toute la durée de l'inondation.

**Assertion.** `volume_ml` du nœud est **identique à l'entier près, à tout instant**, dans les deux
exécutions.

**Ce que le cas attrape.** Toute réapparition du transfert de propriété de masse qu'ADR-025 a retiré.
Si un solveur δ rendait sa masse au nœud, les deux exécutions divergeraient de sa dérive — et la
divergence serait d'autant plus grande que le solveur fuit. C'est la vérification mécanique de
l'invariant **I-04** appliqué à la couche V : *aucune force ni aucune quantité capable de changer une
issue de jeu ne provient de δ*.

**Pourquoi le cas est binaire.** V est en arithmétique **entière** et en régime **D1** (I-03, amendé
en S10) : l'égalité est exacte, pas approchée. Le cas tourne donc en mode `check`, à chaque commit,
sans GPU — ce qui n'aurait pas été possible si la masse transitait par δ, jamais D1.

**Contrôle complémentaire, non binaire.** Le forçage d'ADR-025 §3.2 ramène la masse du domaine δ vers
celle du nœud avec une relaxation `τ ≈ 1 s`. L'écart de niveau résiduel — `dérive_par_s · τ · h` —
est mesuré et comparé au seuil d'admission en régime substitutif (ADR-025 §3.3, 1 %/s à calibrer).
Cette partie relève de la batterie `physics`, pas du mode `check`.

**Rattachement** : banc **B3** (solveur δ, scénario 4 — compartiment inondé en référentiel accéléré),
batterie `check` pour l'assertion principale.