# La porte D sur la référence CPU — S333

2026-09-24. **Porte D**, chemin de la v1 ([ADR-189](../adr/ADR-189-la-v1-d-abord.md)) : une coque de jeu sur une
houle de B, qui flotte sous l'autorité de B, et δ qui porte **la perturbation qu'elle ajoute** — sans aucune
autorité sur le jeu (I-04, [ADR-008](../adr/ADR-008-flottabilite-et-autorite.md) §1). Suite de
[CORPS-RIGIDE-S331](CORPS-RIGIDE-S331.md) §4. **Partie numérique** : la porte attend le verdict visuel de
l'utilisateur (ADR-178 D3, ADR-189 D3).

## Reproduire

- Commit `3f03f02c` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s333 -- --nocapture` — trois essais, 3 s ; lignes `S333`.
- `cargo run -p water-core --release --offline --example porte_d -- --images viewer/captures/s333` — lignes
  `PORTE_D`, ≈ 3 min 30 ; quatre scènes et quatre cartes en PPM. `--decalage-y 0.055` : le placement de
  §4 ; `--temoin` : le plancher de volume sans coque ; `--pas N` : une scène plus courte.
- `python outils/apercu_ppm.py viewer/captures/s333/<image>.ppm` — l'aperçu PNG, à côté.
- Valeurs attendues : critère 1, 1,04889·a ; 1 bis, 0,99059·ξ ; 4, 3,816958749069954·10⁻⁹ m³ ; 5, 0,09385 m ;
  empreintes des images au §3.
- Cœur : 494 réussis, 14 ignorés ; intégration : 23.

## En une phrase

La coque du jeu flotte sur la houle de B — elle pilonne au régime forcé prédit à 3·10⁻⁵ près et cavale avec
l'eau à 0,1 % —, δ la reçoit **relative à l'eau qui la porte** et rayonne son mouvement relatif en anneaux,
jusqu'à 9,4 cm pour un lâcher de 10 cm, et la trajectoire de jeu reste **identique au bit** avec ou sans δ.

## 1. Ce qui change

- **B derrière la requête du corps** (`BackgroundWater`) : élévation, pente et vitesse orbitale de surface de
  la houle analytique, dans le repère local de son ancre.
- **La poussée suit le gradient de la pression que le proxy suppose.** Le proxy intégrait `p = ρg(η(x) − z)`
  mais n'en gardait que la part verticale : sur une houle, le corps montait et descendait sans être entraîné —
  le défaut qu'ADR-008 §2 nomme « immédiatement perceptible ». La force par volume plongé est `ρg(−∇η, 1)`.
  En eau calme, rien ne change au bit : les valeurs de S331–S332 sont inchangées.
- **La coque dans δ, relative à l'eau qui la porte** (`HullInDelta`). δ porte la perturbation — rayonnement
  et diffraction —, pas la houle, qui est à B. Théorie linéaire d'une coque courte devant la longueur d'onde :
  l'eau est lue au centre de la coque. Hauteur au-dessus de la surface et inclinaison se lisent ; la position
  horizontale relative s'**intègre**, `Σ dt·(V − u)`, parce que l'excursion eulérienne de la particule ne se
  lit qu'au second ordre près — une coque qui suit la particule verrait `ξ(x_b) ≠ ξ(α)`, 7 mm ici. La pose
  est arrondie en f32, comme δ la reçoit, et la vitesse de paroi en est la **différence finie** : découpe et
  paroi cohérentes au bit, pose constante pour une coque qui suit l'eau. Prendre la pose absolue compterait
  deux fois le mouvement de la houle.
- **La scène** (`examples/porte_d.rs`) : houle de 25 cm et 6 s vers +x (λ = 56 m) ; coque 4 × 1,6 × 1 m à
  500 kg/m³, proxy 16 × 8 × 4, **lâchée 10 cm au-dessus de son équilibre relatif**, portée par la vitesse de
  l'eau et inclinée comme la surface ; δ en mode linéaire, 24 × 24 m sur 2 m de fond, mailles de 25 cm,
  96 × 96 × 8, chaque paroi laissant 30 % d'eau dans sa maille de bord en `y` (§4) ; 800 pas de 10 ms. La
  force de δ n'anime que le décalage visuel borné (ADR-008 §1). Aucun champ de δ n'est écrit (I-17).

**Pourquoi un lâcher.** Sur une houle longue, une coque qui suit l'eau ne la perturbe qu'à l'ordre `kd` —
un centimètre ici —, et du même ordre que l'erreur de l'approximation de coque courte : invisible, et non
jugeable. C'est le pilonnement relatif qui rayonne.

## 2. Ce qui est mesuré

| critère, écrit avant le code | mesure | verdict |
|---|---|---|
| 1. sur une houle longue (6 s), pilonnement à ± 5 % de `a/(1 − ω²/ωₙ²)` = 1,05767·a | **1,04889·a**, −0,83 % | tenu |
| — le même avec la houle vue par la flottaison, `S = ⟨cos kx⟩` : 1,04892·a | 3·10⁻⁵ ; houle de 3 s : 1,11914·a pour 1,11576 (+0,3 %), là où `1/(1 − ω²/ωₙ²)` seul dirait 1,279 | tenu |
| 1 bis. *(ajouté à la reprise, avant le code)* la coque cavale avec l'eau, excursion à ± 5 % de `a` | **0,99059·ξ** pour `S` = 0,99172 (−0,11 %) ; dérive 4,05 mm/s, de l'ordre de la dérive de Stokes, 7,3 mm/s | tenu |
| 2. au repos relatif, δ au repos au bit | coque qui suit la houle : pose constante, δ **au bit**, 100 pas ; tenue immobile : 0,32 m/s, 8,2 cm | tenu |
| 3. trajectoire de jeu identique au bit avec ou sans δ, scène de la porte D | **identique au bit**, 800 pas | tenu |
| 4. le volume de δ suit celui de la coque plongée à 10⁻⁹ m³ | **3,82·10⁻⁹ m³** | **manqué**, ci-dessous |
| 5. perturbation non nulle, sous l'amplitude de la houle | **9,4 cm** au plus sur les colonnes au couvercle libre (houle : 25 cm) ; 0,51 m/s | tenu |
| 6. images de la surface totale, à plusieurs instants | §3 | fournies ; **verdict de l'utilisateur attendu** |

**Mesurer au bon endroit.** Sous une coque qui perce le couvercle, une face couverte garde une vitesse que
rien ne lit — jusqu'à 3,7 m/s dans la contre-épreuve du critère 2 — et une colonne couverte une hauteur de
pure comptabilité ; les mesures portent sur les faces entièrement ouvertes et les colonnes au couvercle libre.

**Pilonnement libre.** Le proxy n'a pas d'amortissement par rayonnement : le pilonnement que le départ excite
ne s'éteint pas — 0,35 % de `Z` à 6 s, 5,2 % à 3 s dans l'essai du critère 1, que la projection sur la houle
sous la coque écarte. Dans la scène, la coque pilonne de ± 0,12 m relatifs pendant les 8 s.

**Critère 4, manqué au sens strict.** L'écart change de signe au fil des pas, sans dérive. Il vient du
**transport de δ**, pas du couplage : chaque colonne arrondit son incrément de hauteur en f32 et la somme ne se
télescope plus exactement. Borne d'arrondi cumulée `3·2⁻²⁴·Σ|Δη|·dx²` : 1,8·10⁻⁵ m³. **Témoin** (`--temoin`),
même grille, coque immobile, bosse de 10 cm, 200 pas : 3,6·10⁻¹⁰ m³, rapport à sa borne 6,3·10⁻⁴, contre
2,1·10⁻⁴ dans la scène — le même mécanisme, sans aucune paroi qui bouge. La tolérance de 10⁻⁹ venait d'un
domaine de 576 colonnes (S332) ; celui-ci en a 9 216. Un critère de volume **rapporté au plancher du
transport**, ou un transport à somme compensée, reste à décider ; la tolérance n'est pas réécrite après la
mesure.

**Coût** : δ CPU **241 ms par pas**, 110 itérations du gradient conjugué — la référence, pas la production ;
le profil d'ADR-174 D3 n'est pas opposable avant la porte C (ADR-178 D4).

## 3. Les images — pour le verdict

`viewer/captures/s333`, PPM de banc (ADR-124), aperçus PNG à côté, jamais publiés.

| instant | scène, 1600 × 600 | carte de δ, 600 × 600 |
|---|---|---|
| 2 s | `porte_d_02.0s_scene` — `0xb9a974e189acbed2` | `porte_d_02.0s_delta` — `0xa0366797d73e2f97` |
| 4 s | `porte_d_04.0s_scene` — `0x32ca3288ed68258c` | `porte_d_04.0s_delta` — `0x0cba3fe9c000a959` |
| 6 s | `porte_d_06.0s_scene` — `0x4fd48091eeb78821` | `porte_d_06.0s_delta` — `0x984edb9f4bddac25` |
| 8 s | `porte_d_08.0s_scene` — `0x817b5a8d979abda7` | `porte_d_08.0s_delta` — `0xe8a3a75362265bd9` |

- **Scène** : à gauche B + δ **à l'échelle**, à droite B + **5·δ** — B et la coque à l'échelle —, parce que
  les pentes réelles des anneaux, ~0,03, sont sous le seuil de lecture de ce rendu (pratique de l'aperçu
  S297). Pose : œil en (−6 ; −7,5 ; 5) m, visée (0,8 ; 0,8 ; −0,5), champ vertical 52°, 800 × 600 par panneau.
  Coque à sa pose visuelle — physique plus décalage de δ, 1,6 cm au plus.
- **Carte** : `η` de δ vue de dessus, rouge au-dessus, bleu au-dessous, saturée à ± 5 cm ; colonnes au
  couvercle couvert en gris ; un trait tous les 2 m.
- **Couches présentes** : B, une composante ; δ linéaire ; la coque. W absent : ni sillage ni impact.
- **Habillage de banc**, hors système d'eau : ciel en dégradé, lumière de côté sans reflet solaire, couleur de
  l'eau, Fresnel de Schlick ; ni écume, ni réfraction, ni sous-surface ; coque grise sans texture. **Pli droit
  au loin** : le bord de la grille de δ, à murs — le mode linéaire n'a pas d'éponge.

**Questions à l'utilisateur.** (1) Une coque de 4 m qui pilonne de ± 12 cm sur une houle de 25 cm : les
anneaux — espacement ≈ 3 m, dix centimètres au plus près d'elle — sont-ils crédibles ? (2) À l'échelle, la
perturbation est-elle perceptible, ou seulement dans le panneau ×5 ? (3) Le bateau qui monte, descend et
cavale avec la houle, sans rouler, est-il plausible ? **Référence demandée** : une vidéo d'un ponton, d'une
barge ou d'une barque qui pilonne en eau calme ou sur une houle faible, avec la taille de l'objet, la hauteur
et la distance d'observation.

## 4. Ce qui a été trouvé : la paroi et sa maille — A317

Premier placement de la grille, 5,5 cm plus bas en `y` (`--decalage-y 0.055`) : les parois laissent **8 %**
et **52 %** d'eau dans leurs mailles de bord au lieu de 30 et 30 %. Le rayonnement, symétrique à 2 s, devient
**franchement dissymétrique à 8 s** — des vagues fortes du côté de la lamelle de 8 % —, et la perturbation
monte à 13,7 cm. La trajectoire de jeu est la même, au bit, et **ne roule pas** (10⁻¹⁷ rad, `y` = 0 exact) :
c'est δ. Carte à 8 s : `decale/porte_d_08.0s_delta`, `0xb614282e3e5cd161`. L'hypothèse d'un roulis
paramétrique du jeu — le pilonnement à 4,48 rad/s, près du double de la pulsation de roulis, 2,45 — est
écartée : le proxy, symétrique au bit, ne l'amorce pas. **Conséquence** : une coque qui se déplace dans la
grille verra son rayonnement dépendre de sa position sous la maille ; la scène est posée à 30/30.

## 5. Ce qui manque, et ce qui n'est pas prouvé

- **Le verdict visuel de l'utilisateur** (critère 6) : la porte D n'est pas franchie sans lui.
- **A317** et le **critère 4** (§2, §4).
- **W derrière la requête du corps** : B seul ; ni sillage ni impact n'agit sur la coque.
- **L'amortissement par rayonnement** de la coque de jeu : elle pilonne sans fin pendant que δ rayonne.
  δ peut le mesurer hors ligne, comme il a mesuré la masse ajoutée du cube de C10 (S332) — un coefficient
  constant de l'archétype, pas une force de δ au pas.
- **L'approximation de coque courte** : l'eau est lue au centre ; la déformation de l'onde incidente le long
  de la coque — la part non rigide du gradient de vitesse — n'entre pas dans la paroi. Valable pour `kL`
  petit : 0,45 ici.
- **La surface que lit le corps n'est pas celle de l'image de la mer** : `BackgroundWater` lit η eulérien
  linéaire, l'image est CWM (ADR-157, ADR-159) — écart d'ordre `a²k`, 7 mm sur cette houle.
- **δ** : domaine fixe à murs, qui renvoient les anneaux ; mode linéaire seulement ; 12 mailles par longueur
  d'onde rayonnée (3,1 m) ; aucune coque dans la production GPU.
- **Le décalage visuel** : translation seule ; la rotation manque.

---

## 6. S334 — A317 : la paroi de la coque dans sa maille

2026-09-24. Chemin de la porte D, sans verdict visuel : l'utilisateur n'a pas trouvé de référence (R15,
[revue visuelle](REVUE-VISUELLE.md) §20).

### Reproduire

- Commit `cbd1560a` ou plus récent ; machine de référence, CPU, un fil par passage.
- `cargo run -p water-core --release --offline --example a317_lamelle` — la coque 3D en pilonnement imposé,
  cinq placements, ≈ 5 min ; lignes `A317`.
- `… --example a317_lamelle -- --tranche --dx <0.25|0.125|0.0625> --phi 0.05,0.30,0.55,0.80 [--couvercle-partiel]`
  — la convergence ; 7 s, 1 min, 20 min par passage.
- `… --example porte_d -- --couvercle-partiel [--decalage-y 0.055] [--images viewer/captures/s334]` — la scène
  au couvercle partiel ; lignes `PORTE_D flancs`. Sans option, les valeurs du §2 au bit.

### Reproduit hors du jeu, puis attribué

Pilonnement imposé de 5 cm à 4,48 rad/s ; amplitude quadratique moyenne de δ à 2,5–3,5 m de chaque flanc,
entre 3 et 6 s ; `φ`, la part d'eau que la paroi laisse dans sa maille de bord. **En 3D, à 25 cm** : 16,1 mm à
`φ` = 5 %, 10,9 à 30 %, 8,3–8,9 vers 50 %, 16,0 à 80 % — **écart 35 %**, rapport de flancs jusqu'à 1,91.

**Le mécanisme.** δ porte par colonne une hauteur de *remplissage* — l'excès d'eau rapporté à la section
entière — et le couvercle y impose `ρg(η − z₀)`. Sous une coque qui perce le couvercle, cet excès se tient
dans la seule part libre `a` : la vraie surface vaut `(η − z₀)/a`. **La surface d'une colonne en partie
couverte est `1/a` fois trop molle.** Le **couvercle partiel** (`Volume3::set_partial_lid`) y impose
`ρg(η − z₀)/a`, avec deux précautions : l'eau que la coque vient de déposer dans la colonne en est exclue
— le flux de sa paroi la retire pendant le pas (`Base3::deposit`) —, et l'ouverture est bornée par
`dt²·g/dx`, garde du pas explicite de la hauteur.

**Convergence sur une tranche** — la même section, infiniment longue, `ny` = 2 :

| maille | couvercle de S332 : moyenne, écart | couvercle partiel : moyenne, écart |
|---|---|---|
| 25 cm | 18,40 mm, 38,5 % | 26,93 mm, 33,4 % |
| 12,5 cm | 17,04 mm, 44,2 % | 21,99 mm, 10,9 % |
| 6,25 cm | 17,79 mm, 30,5 % | 20,56 mm, **1,8 %** |

**Le couvercle de S332 ne converge pas : A317 est un défaut de structure.** Le couvercle partiel converge,
ordre 1,8 sur la moyenne, **extrapolée à 19,98 mm** ; le couvercle de S332 reste 11 % dessous et dépend du
placement à ± 30–44 % quelle que soit la maille. À 3,125 cm, le gradient conjugué ne converge pas au pas 23
(8 000 itérations), les deux couvercles : de petites cellules sous le fond de la coque.

**Résolution.** Même corrigée, une coque de 6,4 mailles de large rayonne à ± 33 % selon son placement ; il
en faut ~13 pour ± 11 %, ~25 pour ± 2 %.

### Pourquoi le couvercle partiel reste éteint par défaut

Une colonne dont l'ouverture se referme — coque qui glisse — gardait son excès d'eau, que `1/a` change en
pointe : 12,2 m/s dans la contre-épreuve du critère 2 (coque tenue fixe sur la houle), 0,32 avec le
couvercle de S332. **L'eau poussée par la paroi** passe désormais aux colonnes voisines au couvercle ouvert :
5,47 m/s. Par mouvement : glissement pur 0,39 m/s, pilonnement relatif pur 0,34 m/s ; **dès que la coque
tourne par rapport à l'eau**, 1,4 à 5,5 m/s, toujours dans une colonne de coin ouverte à 7–8 % — le résidu de
rotation de S332, vitesse de paroi prise au centre des faces et non au centroïde de leur part couverte, que
`1/a` amplifie. Critère posé avant le code : ≤ 1 m/s. **Manqué : éteint par défaut**, les valeurs S3xx au
bit. Allumé, trois valeurs changent : pilonnement de S332 2,7266 → 2,7434·10⁻¹⁰ m³, décalage 0,0667 →
0,0666 m — couvercles « poussière » du cube —, contre-épreuve de S333 0,32 → 5,47 m/s.

### La scène de la porte D au couvercle partiel

Amplitude à 2,5–3,5 m des flancs longs, 3 à 8 s :

| couvercle | parois à 30 % / 30 % | parois à 8 % / 52 % |
|---|---|---|
| S332 | 34,3 / 34,3 mm | **51,2 / 15,0 mm — rapport 3,42** |
| partiel | 46,5 / 46,5 mm | 55,3 / 45,4 mm — **rapport 1,22** |

Carte à 8 s du placement 8/52 presque identique à celle du 30/30. Critère 3 au bit ; volume 8,3 et
9,3·10⁻⁹ m³, toujours ~3·10⁻⁴ de la borne d'arrondi ; perturbation 13,2 cm (30/30). **Critère posé : les deux
flancs à ± 5 % l'un de l'autre — manqué** : résolution de 25 cm et résidu de rotation, dont le bruit près des
coins se voit à ×5. Images `viewer/captures/s334` : scène / carte 2 s `0xdc67e48d93ed4533` /
`0x6e476e9fd79ea811` ; 4 s `0x4ac9326e7d7003d6` / `0xc8c7f61964126c39` ; 6 s `0x2db0f7edc09d42bf` /
`0x12ed29a898550da5` ; 8 s `0x02c587bba1e7f64a` / `0x67457176f1526445` ; carte 8/52 à 8 s `0xf14d3c45be994d27`.

### Ce qui reste

- **La vitesse de paroi au centroïde de la part couverte** — le remède nommé en S332 —, puis le couvercle
  partiel **allumé par défaut** et la scène de la porte D rejouée.
- **La résolution près des coques** : ~25 mailles de largeur pour ± 2 % ; hors de portée de la référence CPU
  en 3D, c'est une question de production ou de maille locale.
- Le **critère de volume** rapporté au plancher du transport (§2).

