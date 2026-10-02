# La voie d'A289 : δ relatif à la dynamique de B — S369

2026-09-26. **A289** ([angle mort](../registres/ANGLES-MORTS.md), sévérité 3) bloque l'ordre E du lot 2 : sous une houle B
seule, sans perturbation, δ croît jusqu'à trois fois la houle ([MER-S319](MER-S319.md)), à un taux qui ne dépend ni de la
maille ni du pas (§8 de S319). L'utilisateur a délégué la voie, sous trois critères — *« le plus favorable au réalisme
ainsi que les performances, simple »* ([ADR-197](../adr/ADR-197-reponses-du-2026-09-26.md) D6). Machine de référence,
référence CPU de δ ; aucun téléchargement.

## Reproduire

- Commit de P4 de S369 ou plus récent. `cargo run -p water-core --release --offline --example transfert_oriente -- mer
  <maille> <houle>` (E1, `MER_DUREE`) ou `mer_paquet` (E2) ; options `MER_RELATIF=<masque>` (§1), `MER_GERME=<m>` (§3),
  `MER_TRACE`, `MER_PROFIL`, `MER_DT_US`. Bits d'essai du §3 : 8, 16, 32, 64, ajoutés au masque 7.
- Test `zero_delta_stays_zero_under_b_alone_when_relative_s369` (`tests_delta3d.rs`).
- Houle d'une composante, λ = 4 m (`k·h` = 3,1), vers `+x` ; tranche de deux rangées en `y`, 90 m, éponges de 18 m ;
  pas de 10 ms sauf mention. Taux : pente de `ln δ_max` par moindres carrés.

## En une phrase

La croissance de S319 est **forcée par ce que B linéaire laisse de ses propres équations** : retirés du pas couplé, ces
trois termes laissent δ nul au bit sous la houle, gratuitement ; mais derrière cette source se tenait une **instabilité
convective des perturbations** sous une houle raide, qui la remplace comme blocage de l'ordre E.

## 1. D'où vient la croissance : trois termes où δ ne figure pas

Le pas couplé (`delta3d_coupling.rs`, S297) résout la surface **totale** et donne à δ, comme sources, trois termes qui ne
dépendent que de B — le reste de B dans les équations complètes, que SPEC-004 §6.1 demandait de soustraire :

| bit | terme | où |
|---:|---|---|
| 1 | résidu de quantité de mouvement de B, `U_t + (U·∇)U + ∇p/ρ` | `extra3`, prédiction |
| 2 | transport de B entre le plan moyen et **sa propre** surface | la bande, `transport_coupled3` |
| 4 | erreur de pression de B à sa propre surface, `ρ·g·η_B − p_B(repos + η_B)` | les fantômes de surface |

`Volume3::set_relative_background(masque)` les retire : restent les termes **croisés** (B advecte δ, δ advecte B, la bande
et la pression de B entre sa surface et la surface totale) et ceux de δ seul. La surface « propre » de B est formée comme
la totale, pour qu'à δ nul les opérandes soient les mêmes au bit. Critère écrit avant : E1 à 40 s, δ nul au bit.

| E1, 25 cm, houle 5 cm, 40 s | aucun (S322) | 1 | 2 | 3 | 4 | 5 | 6 | **1+2+4** |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| δ maximal | 15,8 cm | 3,06 | 14,0 | 1,45 | 17,5 | 4,80 | 14,7 | **0, au bit** |
| taux, 10 → 30 s (s⁻¹) | 0,101 | 0,074 | 0,099 | 0,047 | 0,095 | 0,066 | 0,108 | — |

**Tenu.** Le résidu de quantité de mouvement porte l'essentiel ; aucun terme seul ne suffit — chacun nourrit la dérive
que les autres laissent. Sans eux, le calcul d'E1 prend 51 s au lieu de ≈ 170 : la pression n'a plus rien à résoudre.
Le test garde la propriété : 150 pas, hauteur, vitesses et pression nulles au bit ; le pas de S297, 3·10⁻³ m.

## 2. Pourquoi cette voie, et non les trois d'A289

- **Réalisme.** Dans le domaine, la mer de fond est B, exactement comme partout ailleurs : aucune couture au bord, la
  cohérence de phase de δ avec B est exacte par construction (4.21). Les perturbations gardent toute leur physique —
  entre elles et avec B. Un **rappel lent** aurait dû amortir toute perturbation plus vite que 0,1 s⁻¹ ; une **durée de
  vie bornée**, recréer les domaines avant dix secondes ; la **dispersion d'amplitude dans B** ne corrigeait que la
  vitesse de phase, pas les harmoniques liés, et le pas couplé ne porte que 0,85 fois celle de Stokes (S274).
- **Performances.** Gratuite : trois termes de moins, une pression qui converge sans itérer sous B seul.
- **Simplicité.** Trois retraits locaux, un drapeau ; B inchangé, donc la mer de tous inchangée.

Ce qu'elle abandonne : les corrections non linéaires de B à lui-même dans le domaine (crêtes de Stokes) — que B n'a nulle
part ailleurs. Rendre B plus réaliste reste possible, et resterait cohérent avec δ.

## 3. Ce qui se tenait derrière : une instabilité des perturbations

Critères écrits avant : un germe de 1 mm ne croît pas (taux < 0,01 s⁻¹ sur 40 s) ; E2, le paquet de l'ordre C (2 cm), reçu
à droite à 30 % du paquet seul (S317).

| masque 1+2+4, 25 cm sauf mention | houle | durée | δ max | croissance |
|---|---:|---:|---:|---|
| germe 1 mm | 5 cm | 40 s | 1,12 mm | aucune — **critère 2 tenu tel qu'écrit** |
| germe 1 mm, prolongé | 5 cm | 95 s | 12,7 mm | dès ≈ 50 s, **0,060 s⁻¹** |
| E2, paquet 2 cm | 5 cm | 95 s | 14,8 cm | dès ≈ 35 s, **0,052 s⁻¹** ; reçu 1 620 fois le paquet |
| E2, pas de 5 ms | 5 cm | 95 s | 14 cm | 0,051 s⁻¹ — le pas n'y est pour rien |
| E2, maille 12,5 cm | 5 cm | 86 s | 12,5 cm | 0,033 s⁻¹ (40 → 78 s), plus lente à maille fine ; le pas refuse à 86,6 s, une colonne hors du domaine |
| E2 | 2,5 cm | 95 s | 2,17 cm (départ) | **aucune** ; reçu 21 fois le paquet (§4) |

**Critère 3 manqué.** Le germe croît au même taux que le paquet : **une instabilité linéaire** de δ autour de B, que la
source de S319 masquait. Elle est **convective** : l'écart maximal croît le long du sens de la houle — ×e tous les ≈ 18 m
— jusqu'à l'éponge de sortie. Indépendante du pas de temps ; un peu plus lente à maille fine ; absente sous une houle
deux fois plus douce (`ak` = 0,039) sur 95 s. Son taux, à `ak` = 0,079, vaut quatre fois celui de Benjamin-Feir
(`ω(ak)²/2` = 0,012 s⁻¹) : pas la physique d'une vraie houle.

### Le terme qui la porte

Même bisection qu'au §1, sur les termes **croisés** (bits d'essai 8 à 64, `TRIAL_*` : chacun ampute le couplage, aucun
n'est une physique). Germe de 1 mm, houle de **7,5 cm** (`ak` = 0,118) pour qu'elle se montre avant 60 s ; taux de 35 à
59 s :

| retiré, en plus des trois termes propres | δ à 59 s | taux (s⁻¹) |
|---|---:|---:|
| rien (masque 7) | 32 mm | 0,115 |
| B advecte δ, `U·∇u'` (15) | 206 mm, dès 15 s | — (le couplage n'a plus de sens) |
| **δ advecte B, `u'·∇U` (23)** | **0,9 mm** | **−0,002 : plus d'instabilité** |
| la bande croisée (39) | 10 mm | 0,090 |
| la pression croisée (71) | 56 mm | 0,125 |

**Le terme de cisaillement `u'·∇U` porte l'instabilité** ; les autres la modulent. Son taux croît comme ≈ `a²` (0,115 à
`ak` = 0,118, 0,056 à 0,079, rien de visible à 0,039), vaut ≈ 4,5 fois Benjamin-Feir aux deux amplitudes, et **décroît
quand la maille s'affine** (0,052 → 0,033 de 25 à 12,5 cm) : la signature d'un défaut de discrétisation, pas d'une
physique. Hypothèse, non éprouvée : pour un fond et une perturbation irrotationnels, `U·∇u' + u'·∇U = ∇(U·u')` est un
gradient, que la projection absorbe et qui n'agit qu'à la surface ; discrétisés séparément, explicitement et centrés, les
deux termes ne forment plus un gradient discret et produisent de l'énergie. Remède à éprouver : les termes croisés sous
cette forme de Bernoulli — exacte, gratuite.

## 4. Le critère de volume est mal posé dans une mer

Sous 2,5 cm de houle, sans croissance, la ligne reçoit **21 fois** le volume net du paquet, et la gauche en perd autant :
un transport à travers le domaine. Ordre de grandeur : le transport croisé `a_B·a_δ·ω/2` d'un paquet de 2 cm dans 2,5 cm
de houle, pendant les ≈ 15 s de son passage, vaut ≈ 7·10⁻³ m³ brut ; sur quelques battements (3,9 s), il ne s'annule qu'en
partie — 2,4·10⁻³ m³ restent. Le volume net d'un paquet presque de moyenne nulle (1,2·10⁻⁴ m³) est une quantité trop
petite pour se lire sous ce transport : le critère de S319 §3 doit être refondu (sur la durée, ou sur le transport propre
de δ), pas seulement tenu.

## 5. Limites

Une composante, une tranche de deux rangées, quatre amplitudes ; la croissance après 95 s n'est pas suivie ; la forme de
Bernoulli n'est pas essayée. Le fond plat
et une surface qui ne franchit pas de centre de maille rendent le point fixe exact ; ailleurs, l'erreur de surface est
interpolée entre deux colonnes (ordre `dx²`). La production GPU (`delta3d_step.wgsl`) garde le pas de S297.

## 6. S434 — la forme de Bernoulli éprouvée (C7d-3a) : elle ne suffit pas

2026-10-02, au poste, sans carte. **C7d-3a** de la campagne ([APIC-CARTE-S416](APIC-CARTE-S416.md) §22.3) : A320, le remède nommé par
ADR-198 D4.

**Reproduire** : `MER_RELATIF=7 MER_GERME=0.001 MER_DUREE=95 MER_TRACE=1 … transfert_oriente -- mer 0.25 <houle>`, plus
`MER_BERNOULLI=1` (la forme), `MER_LW=1` (le terme de second ordre d'ADR-209, actif en production), `MER_BERNOULLI_ESSAI=1|2` (G seule,
R seule — des amputations d'essai) ; `mer_paquet` pour le paquet. Essai `zero_delta_stays_zero_with_the_bernoulli_cross_terms_s434`.

**La forme** (`Volume3::set_cross_bernoulli`, éteinte par défaut) : B étant irrotationnel, les termes croisés de la face d'axe `a`,
`U·∇u′_a + u′·∇U_a`, s'écrivent `∂_a(U·u′) + Σ_b U_b (∂_b u′_a − ∂_a u′_b)` ; **G**, la différence de `φ = U·u′` entre les deux mailles de
la face (un gradient discret exact) ; **R**, la partie rotationnelle. Aux faces du sommet et dans un ensemble épars, l'ancienne forme.
δ nul reste nul au bit sous B seul avec elle.

| 25 cm, masque 7, germe de 1 mm (sauf mention) | taux 35–59 s (s⁻¹) | δ en fin |
|---|---:|---:|
| houle 7,5 cm — témoin (S433, la forme de S369) | 0,115 | 32 mm à 59 s |
| — **Bernoulli** | **0,097** (60–95 s : 0,066) | 17,6 cm à 94 s |
| — le terme d'ADR-209 seul | 0,114 | 20 cm à 94 s |
| — Bernoulli et ADR-209 | 0,0955 | 17,7 cm à 94 s |
| — **G seule** · **R seule** (amputations) | **−0,005 · −0,005** | 0,8 · 0,9 mm à 59 s |
| houle 5 cm — Bernoulli | 0,009 ; 60–95 s : **0,050** | 6,4 mm (S369 sans : 12,7) |
| paquet de l'ordre C, houle 5 cm — Bernoulli | — | 12,8 cm : **6,4 fois** le paquet (S369 : 7,4) |
| 12,5 cm, houle 7,5 cm — Bernoulli | arrêté à 35 s (≈ 40 min) | 16 mm, un saut à 11 mm dès 5 s, sans témoin à cette maille |

**Les critères de C7d-3a** (germe : taux < 0,01 s⁻¹ sur 95 s ; paquet : au plus 1,5 fois) : **manqués**. La forme freine A320 de 15 à
20 %, pas davantage. **Ce que cela dit.** (1) Le terme de second ordre d'ADR-209 n'y fait rien : A320 **n'est pas** l'instabilité FTCS
d'A321. (2) **La bisection réfute l'hypothèse de S369** : ni la partie gradient seule ni la partie rotationnelle seule ne croissent ;
seule leur somme — le terme croisé entier — porte la croissance ; ce n'est pas un défaut de forme d'un des deux termes. **Restent à
éprouver** : la forme antisymétrique (conservative de l'énergie) de l'advection de `u′` par `U` ; la condition dynamique de surface (le
gradient `∂(U·u′)` n'est absorbé par la projection qu'à l'intérieur) ; l'intégration en temps du couplage — le taux ne dépend pas du pas
(S369) ; et la question de fond : une interaction physique d'ondes courtes portées par une houle, que le mode relatif linéaire laisse sans
borne. Un témoin à 12,5 cm (≈ 2 h de calcul) avant toute conclusion sur la maille.

## 7. S435 — la question physique : où croît δ, et vers quoi quand la maille s'affine

2026-10-02, au poste, sans carte. C7d-3a, suite. Critères écrits avant (`notes/EN-COURS.md`, S435 P1) : **Benjamin-Feir** si (a) à
25 cm, plus de la moitié de l'énergie de δ en fin de calcul tombe dans la bande instable `|k − K| ≤ 2√2·ak·K`, et (b) le taux
extrapolé à maille nulle est entre la moitié et le double de `ω(ak)²/2` ; **numérique** si l'énergie est d'abord sous `4·dx`, ou si la
limite est sous 0,01 s⁻¹ ; autrement, indécis.

**Reproduire** : `MER_RELATIF=7 MER_GERME=0.001 MER_DUREE=95 MER_TRACE=1 MER_SPECTRE=1 … transfert_oriente -- mer <dx> <houle>` ;
`MER_SPECTRE` donne le spectre de δ hors des éponges en fin de calcul et, dans la trace, l'amplitude de δ dans la bande et sous `4·dx`
chaque seconde ; `MER_PROLONGEMENT=<m>` allonge le domaine (10 m par défaut, au bit). Le taux est celui de l'amplitude de la bande,
ajusté de 35 à 59 s.

| houle 7,5 cm (`ak` = 0,118, `ω(ak)²/2` = 0,0273 s⁻¹) | taux de la bande (s⁻¹) | part de la bande en fin | sous `4·dx` | pic `k/K` |
|---|---:|---:|---:|---:|
| 50 cm | 0,093 | 0,90 | 0,003 | 1,18 |
| 31,25 cm | 0,112 | 0,99 | 0,003 | 0,97 |
| 25 cm | 0,106 (δ max : 0,1151, le témoin d'A320 au chiffre près) | **0,996** | 0,001 | 0,96 |
| 15,625 cm | 0,084 | 0,99 | 0,000 | 0,97 |
| 25 cm, domaine allongé de 40 m | δ max : 0,116 | — | — | — |

37,5 cm ne se mesure pas : la surface au repos n'y tombe pas sur une face, le point fixe se perd dès 2 s. À 12,5 cm, un autre défaut
(ci-dessous) contamine la mesure. Amplitude, à 25 cm (taux maximal de δ sur 20 s, δ < 5 cm) : 2,5 cm, rien en 95 s ; 5 cm, 0,074 ;
7,5 cm, 0,128 ; 10 cm, 0,162 (le domaine refuse à 61 s).

**Verdict, tel qu'écrit.** (a) **tenu** : δ croît **à la longueur d'onde de la houle** — 99 % de son énergie dans la bande, rien à
l'échelle de la maille. (b) : l'ajustement en `dx` sur les trois mailles les plus fines donne **0,056 s⁻¹ à maille nulle, 2,05 fois**
Benjamin-Feir, juste hors de la fourchette : **indécis**. Et **pas numérique** : rien sous `4·dx`, une limite bien au-dessus de 0,01.
Le domaine allongé ne change rien : pas une boucle par les bords.

**Ce que cela dit.** A320 n'est pas une instabilité de grille : l'advection antisymétrique, qui borne l'énergie des modes de maille,
**n'est plus la piste**. C'est une instabilité de modulation de la houle portée par δ, d'ordre `ω(ak)²`, dont le taux baisse d'un
cinquième de 25 à 15,6 cm. Une raison de fond la rend attendue : δ est linéarisé autour d'**Airy**, qui n'est solution qu'au premier
ordre ; l'opérateur de δ autour de B est donc faux en `(ak)²` — l'ordre même de Benjamin-Feir. Le taux d'une modulation de δ autour
d'Airy n'a de sens que par son ordre de grandeur. **Le critère de C7d-3a était mal posé** : sous 7,5 cm, une vraie houle module à
0,027 s⁻¹ ; « < 0,01 » y est hors d'atteinte de toute physique. À réécrire avant la prochaine mesure, rapporté à Benjamin-Feir. En eau
profonde, la houle de Stokes ne diffère d'Airy, jusqu'au troisième ordre, que par sa surface (harmoniques 2 et 3) et sa fréquence
`Ω(1 + (ak)²/2)` — son potentiel reste celui d'Airy : un essai de B cohérent au second ordre, pour une composante, est à portée.

### Un second défaut : la surface de B franchit un centre de maille (A324)

À 12,5 cm sous 7,5 cm de houle, δ saute de 1 à 10 mm **en une seconde**, en pics isolés, loin du germe, à l'échelle de la maille
(54 % de l'énergie sous `4·dx`). Le témoin, sans germe, reste nul : le défaut multiplie δ. **Le seuil est net** :

| 12,5 cm, germe 1 mm, à 1 s | δ max | sous `4·dx` | pic `k/K` |
|---|---:|---:|---:|
| houle 6 cm, masque 7 | 1,1 mm | 0,000 | 2,0 (le germe) |
| houle 6,5 cm, masque 7 | **8,6 mm** | **0,73** | **10** |
| houle 6 cm · 6,5 cm, masque 0 | 6,6 · 8,1 mm | 0,04 · 0,05 | 2,0 · 2,0 |

6,25 cm est **la demi-maille** : au-delà, la surface de B franchit les centres de mailles de δ. Le mode relatif seul y est sensible.
*Correction du 2026-10-02, S436* (§8) : « le témoin, sans germe, reste nul » était faux — en mode germe, le banc `mer` ne fait pas
avancer son témoin ; un banc pas à pas le montre non nul : A324 est une rupture du point fixe.
Toute vraie mer à 25 cm a des houles de plus de 12,5 cm : **le mode relatif est inutilisable en l'état hors du banc** — cela précède
C7d-3b (le mode relatif sur la carte). Non attribué ; l'échantillonnage de B aux faces que sa surface traverse est le premier suspect.

## 8. S436 — A324 : le fantôme latéral du mode relatif (corrigée)

2026-10-02, au poste, sans carte. Critères écrits avant (`notes/EN-COURS.md`, S436 P1) : à 12,5 cm sous 6,5 et 7,5 cm, germe de 1 mm,
δ max ≤ 1,5 mm à 1 s et part sous `4·dx` ≤ 1 % à 2 s ; le témoin nul au bit ; à 25 cm sous 7,5 cm (aucun franchissement), le pas au
bit et le taux d'A320 inchangé ; la suite ; un essai du cœur.

**Reproduire** : `cargo run -p water-core --release --offline --example a324_franchissement -- <houle_m>` (20 m × 2 rangées à
12,5 cm, pas à pas, le témoin à côté ; `A324_ANCIEN=1`, le fantôme d'avant) ; `transfert_oriente -- mer 0.125 <houle>` avec
`MER_RELATIF=7 MER_GERME=0.001 MER_SPECTRE=1` (`MER_A324=0`, l'ancien). Essai `zero_delta_stays_zero_when_b_crosses_a_cell_centre_s436`.

**Ce qui se passait.** Au §7, A324 était décrite comme une amplification de δ, le témoin restant nul. C'était faux : en mode germe,
le banc `mer` ne fait pas avancer son témoin. Pas à pas, **le témoin ne reste pas nul** sous 6,5 cm : 1 mm en 50 ms, 1 cm en 0,3 s,
des jets de 0,2 m/s — sous 6 cm, nul au bit. **Le point fixe du mode relatif se rompt.** Les bits d'essai qui amputent l'advection
(8, 16) et la bande croisée (32) n'y changent rien. **La cause** : entre une colonne mouillée et une sèche au même étage — une face qui
n'existe que si la surface franchit un centre de maille entre elles —, le fantôme de pression latéral retranchait l'erreur de B
**interpolée entre les deux colonnes**, alors que la valeur qu'il corrige est la pression de B **au point de surface** sur la face :
sous B seul, un reste d'ordre `dx²`, divisé par `θ` dans l'opérateur. S369 le savait exact « seulement là où la mouillure de B seul est
déjà celle-ci » ; sous une houle d'au plus une demi-maille, elle l'est partout.

**Le remède, au quatrième essai.** (1) L'erreur prise au point de B, sans le terme `ρg(z − repos)` que `ghost_side3` ajoute : pire.
(2) Avec lui : le témoin nul au bit, mais le germe à 4,6 mm — une bascule entre deux formules quand δ déplace le franchissement. (3) Le
point de B prolongé, continu : 6,9 mm — ce point se divise par la pente de B, nulle aux crêtes, et c'est aux crêtes que 6,5 cm franchit.
(4) **Retenu** (`Volume3::set_lateral_own_ghost`, **le défaut depuis S436**) : en mode relatif, le fantôme latéral **interpole entre
les deux colonnes ce que porte leur fantôme vertical** — `ρgη′` et le reste de B, nul au bit à δ nul — et ne lit plus la pression de
B au point latéral.

| 12,5 cm, germe 1 mm | δ max à 1 s | sous `4·dx` à 2 s |
|---|---:|---:|
| houle 6 cm (aucun franchissement) | 1,1 mm | 0 |
| houle 6,5 cm — avant · **après** | 8,6 mm · **1,1 mm** | 0,73 · **0** |
| houle 7,5 cm — avant · **après** | 10 mm · **1,1 mm** | 0,54 · **0** |
| banc `a324`, témoin sous 6,5 cm, 100 pas — avant · **après** | 8 mm · **0 au bit** | `u′` 0,2 m/s · **5 mm/s** (germe) |

À 25 cm sous 7,5 cm, sur 95 s : les traces sont **identiques au bit jusqu'à 70 s** — δ y atteint 14,5 cm et fait alors franchir des
centres à lui seul — ; le taux d'A320, **0,1151**, inchangé. **Les cinq critères sont tenus : A324 est corrigée.** Suite 758, zéro
avertissement. **A320 à 12,5 cm, enfin mesurable** (germe de 1 mm, houle de 7,5 cm, taux de la bande de 35 à 59 s) : **0,027 s⁻¹, 1,0 fois Benjamin-Feir** (0,0273) — contre 0,106 à 25 cm ; sous 6,5 cm, 0,028 (1,4 fois). Pour séparer la maille du franchissement : sous **6 cm** (aucun franchissement, le remède sans objet), 0,077 · 0,062 · 0,054 s⁻¹ à 25 · 15,6 · 12,5 cm, soit ≈ 0,031 extrapolé à maille nulle — **1,8 fois** Benjamin-Feir (0,0175), dans la fourchette de S435. La maille fine ralentit A320 vers l'ordre de Benjamin-Feir ; le franchissement (la mouillure de δ qui suit la houle) la ralentit davantage — reste à le séparer du remède. Ces chiffres sont les données d'entrée de C7d-3a.

**Limites.** Le fond plat, une composante, deux rangées ; les faces coupées (`cut`) ne sont pas éprouvées sous le remède. La
production GPU ne porte pas encore le mode relatif : le fantôme latéral est à porter avec lui (C7d-3b).

## 9. S437 — A320 tient à la place de la surface dans la maille, et à une houle mal résolue

2026-10-02, au poste, sans carte. **Le critère de C7d-3a, réécrit avant toute mesure** (`notes/EN-COURS.md`, S437 P1) — l'ancien,
« < 0,01 s⁻¹ », était hors d'atteinte de toute physique (§7) : reçu si, **à 25 cm**, le taux de la bande d'un germe de 1 mm sous 6 et
7,5 cm de houle est **au plus 1,5 fois `ω(ak)²/2`** (0,026 et 0,041 s⁻¹), **quelle que soit la place du repos dans la maille** ; δ nul
reste nul au bit ; le paquet de l'ordre C sous 5 cm, au plus 1,5 fois son amplitude.

**Reproduire** : `transfert_oriente -- mer 0.25 <houle>` avec `MER_RELATIF=7 MER_GERME=0.001 MER_DUREE=95 MER_TRACE=1 MER_SPECTRE=1`,
plus `MER_DECALAGE=<f>` (le repos `f·dx` au-dessus d'une face), `MER_TEMOIN=1` (le témoin avance ; la trace dit s'il est nul au bit),
`MER_AIR=<n>`, et, pour la houle longue, `MER_TP=4.527 MER_PROFONDEUR=4 MER_GERME_LAMBDA=8`. Essai de localisation :
`MER_SURFACE_ESSAI=1|2` (`Volume3::set_cross_surface_trial`, des amputations).

| 25 cm, houle de 4 m, taux de la bande 35–59 s (s⁻¹) | repos sur une face | ¼ | au centre | ¾ |
|---|---:|---:|---:|---:|
| 6 cm (Benjamin-Feir 0,0175 ; critère 0,026) | 0,077 | 0,066 | **0,018** | 0,080 (refus à 64 s) |
| 7,5 cm (Benjamin-Feir 0,0273 ; critère 0,041) | 0,106 | 0,068 | **0,036** | décroît |

**Le témoin est nul au bit à chaque seconde, à toutes les places** : A324 corrigée tient hors d'une face. **Le taux dépend de la place
du repos dans la maille** — une physique ne le ferait pas : l'excès d'A320 est un défaut de discrétisation au voisinage de la surface.
Ni les termes croisés aux faces de surface (amputés : 0,095 sur une face, 0,026 au centre), ni la bande (`band3` prend déjà la pente
verticale de B) ne le portent seuls.

**Une vraie mer.** Le régime « repos près d'une face, sans franchissement » n'existe que sous une houle de moins d'une demi-maille.
Sous une **houle de 8 m** (32 mailles par longueur d'onde à 25 cm ; 4 m d'eau ; germe de 8 m, qui sort du domaine en ≈ 30 s), la bande
ne croît ensuite qu'à ≈ 0,007 (sur une face) et ≈ 0,004 s⁻¹ (au centre) sous 15 cm — Benjamin-Feir 0,019, la surface de B franchit des
centres dans les deux cas —, et à ≈ 0,009 et ≈ 0 sous 10 cm (Benjamin-Feir 0,0086 ; sur une face, sans franchissement) : **au plus
Benjamin-Feir, aux deux places**.

**Verdict.** Le critère, tel qu'écrit pour la houle de 4 m du banc, est **manqué** sur une face : **C7d-3a n'est pas reçu**, et le
critère n'est pas déplacé après coup. Mais A320 change de nature : son excès est **un défaut de discrétisation près de la surface, qui
ne se montre qu'à 16 mailles par longueur d'onde de la houle** (4 m à 25 cm) ; à 32 il disparaît. **Limites** : une composante, deux
rangées ; sous 8 m, les taux se lisent sur un plateau après la sortie du germe — un signe de croissance absente, non une mesure fine.

## 10. S438 — l'excès d'A320, une affaire de mailles par longueur d'onde ? Indécis

2026-10-02, au poste, sans carte. **Hypothèse** : le taux de Benjamin-Feir passe par la réponse liée du second ordre, l'onde `2K` collée
à la surface (`e^{2Kz}` : 32 cm sous la houle de 4 m, à peine plus d'une maille de 25 cm) ; mal résolue, elle fausserait le coefficient
cubique, selon la place de la surface dans la maille — l'excès serait alors une fonction de `λ_B/dx` seul. (Une autre hypothèse fut
écartée avant tout code : une quasi-résonance de triades ouverte par la dispersion discrète de δ — en eau profonde, les triades sont
loin de résonner, d'un écart d'ordre `Ω/2`.) **Critères, écrits avant** : l'échelle est confirmée si, à deux nouvelles paires de
16 mailles par longueur d'onde, le taux sur une face est au moins 2,5 fois Benjamin-Feir et au moins deux fois celui du centre ;
réfutée si une paire reste sous 1,5 fois aux deux places.

**Reproduire** : `transfert_oriente -- mer <dx> <houle>` avec `MER_RELATIF=7 MER_GERME=0.001 MER_DUREE=95 MER_TRACE=1 MER_SPECTRE=1
MER_AIR=1`, `MER_DECALAGE=0|0.5`, `MER_TP`, `MER_PROFONDEUR`, `MER_GERME_LAMBDA` (le germe à `2K`).

| `ak` = 0,118, taux de la bande (s⁻¹) | `λ_B/dx` | sur une face | au centre | Benjamin-Feir |
|---|---:|---:|---:|---:|
| 4 m à 25 cm, 7,5 cm (S437) | 16 | 0,106 (3,9 fois) | 0,036 (1,3) | 0,0273 |
| **8 m à 50 cm**, 15 cm | 16 | **0,042** (2,2 fois ; 0,059 en fin) | 0,010 (0,5) | 0,0193 |
| **2 m à 12,5 cm**, 3,75 cm | 16 | fenêtre 35–59 s saturée (refus à 55 s) ; phase linéaire **0,207** (5,4 fois) | **0,074** (1,9) | 0,0386 |
| 8 m à 25 cm, 15 cm, germe à `2K` | 32 | 0,0085 | 0,010 | 0,0193 |
| 4 m à 12,5 cm, 6 cm (S436) | 32 | 0,054 (3,1 fois) | — | 0,0175 |

**Verdict, tel qu'écrit : indécis.** Le motif « sur une face ≫ au centre » tient aux trois paires de 16 mailles (rapport 2,8 à 4,2),
mais 8 m à 50 cm reste à 2,2 fois sur la fenêtre écrite, et la fenêtre de 2 m à 12,5 cm est saturée. Surtout, **l'échelle en `λ_B/dx`
n'est pas propre** : à 32 mailles, 8 m à 25 cm reste sous Benjamin-Feir, mais 4 m à 12,5 cm montait à 3,1 fois. **La cause n'est pas
trouvée.** Ce qui est établi, sur trois sessions : A320 est **une instabilité de modulation à la longueur d'onde de la houle**, d'un
ordre `ω(ak)²` ; son excès sur Benjamin-Feir **dépend de la place de la surface dans la maille** (un défaut de discrétisation près de la
surface) ; ni la forme des termes croisés (S434), ni les termes croisés aux faces de surface, ni la bande (S437) ne le portent seuls.
**Enveloppe mesurée à 25 cm** (la maille de la production) : sous une houle de 8 m, au plus Benjamin-Feir ; sous une houle de 4 m et
6 à 7,5 cm, 0,04 à 0,11 s⁻¹ selon la place du repos.

## 11. S442 — A320 sous la bande Lax-Wendroff : inchangée

2026-10-02. La bande relative est sous Lax-Wendroff par défaut depuis S442 (A322, [MULTIGRILLE-3D-S385](MULTIGRILLE-3D-S385.md) §8).
Même banc qu'au §9 (`MER_BANDE_LW=0` rend la bande centrée) : sous 7,5 cm, **0,1057** sur une face, **0,0676** à ¼, **0,0345** au
centre ; sous 6 cm, **0,0768** et **0,0197** — au chiffre près ceux de S437. Le témoin nul au bit partout. À 25 cm, `C = U·dt/dx`
≈ 0,01 : le FTCS de la bande n'y pèse rien, et **n'est pas la cause d'A320**. C7d-3a n'est pas reçu.

