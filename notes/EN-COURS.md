# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S188 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S187-1/A50, **rejouer la composition de S186 sur un réseau ancré** (ADR-118).
S187 a trouvé que le réseau du dépôt posait son dernier nœud hors du bloc, et que l'ancrer
divise l'erreur spatiale par jusqu'à six, gratuitement. Deux conséquences pour S186, et la
seconde est la vraie :

1. **les magnitudes** — la règle de dimensionnement de S186 §8.5 est d'égaliser les erreurs
   des deux axes pris seuls. L'axe spatial vient de perdre un facteur allant jusqu'à six :
   le point de parité se déplace entièrement, et l'exemple publié (« la cadence devient
   dominante à `c = 32` ») ne tient plus ;
2. **la loi elle-même** — le maximum pour les modes causaux a été mesuré sur une erreur
   **concentrée** sur une tranche unique. Un réseau ancré la **répartit** (S187 §8.5 :
   erreur de tranche haute nulle, maximum déplacé vers le milieu). Rien ne dit qu'une loi
   de composition mesurée sur une erreur concentrée soit celle d'une erreur répartie.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — publier le protocole avant tout chiffre : la famille ancrée à **nombre de
      nœuds identique** à S186, la grille `r × mode × c` rejouée à l'identique, les trois
      mêmes lois avec le **même critère déjà déclaré** `[0,80 ; 1,25]`, les réceptions, et
      ce que le rejeu ne prouvera pas.
- [x] **P3a** — sortir dans `support/` ce que S187 gardait local : les indices ancrés
      uniformes. Rejouer `graded_lattice` et vérifier `0x6cf13183b4a240df`.
- [x] **P3b** — écrire `anchored_composition.rs` : la grille de S186, réseau ancré, même
      référence, mêmes métriques. Réceptions : `14³` ancré = réseau plein en bits, la
      ligne `c = 1` redonne S187 §8.4, la ligne `r` plein redonne les `eU` temporelles de
      S186. Relever.
- [ ] **P4** — recevoir dans un document de validation ; **note corrective datée** si la loi
      change ; ADR seulement si une décision nouvelle en sort. Angles et leçons.
- [ ] **P5** — rituel de fin (REPRISE.md §6).

### Notes de reprise

P3b S188 : `examples/anchored_composition.rs`. Deux exécutions, `diff` identique hors
lignes de cargo ; aucune durée mesurée. Empreinte **0x21bab548c7b9775c**. Workspace
**331 réussis / cinq ignorés** en debug et en release.

**Réceptions, les six passent.** (2) le réseau plein ancré rend la référence **en bits**
pour les trois modes — l'ancré à 14 nœuds *est* `axis_indices(r=1)`, donc la coïncidence
attendue est vérifiée et non supposée. (3) les vingt-et-une erreurs temporelles pures
redonnent **exactement** S186 §6.2 et donc S185 : mnt c=2 0,7700, mnt c=64 33,2115,
ext c=8 0,7754, int c=64 6,7740. (4) l'erreur spatiale à 8 nœuds par axe vaut **1,7160 %**,
la valeur ancrée de S187 §8.4. (5) plancher 0,386 %, celui de S186. (1) et (6) plus haut.

**LA LOI NE CHANGE PAS, MODE PAR MODE.** Verdict global : les trois lois rejetées
(additive 0,468–0,984, quadratique 0,659–1,245, maximum 0,869–1,707), exactement comme
S186. Par mode :

| mode | additive | quadratique | maximum | retenue S188 | retenue S186 |
|---|---|---|---|---|---|
| maintien | 0,500–0,951 | 0,702–0,999 | **0,895–1,060** | maximum | maximum |
| extrapolation | 0,468–0,941 | 0,659–0,998 | **0,869–1,000** | maximum | maximum |
| interpolation | **0,845–0,984** | **1,017–1,245** | 1,018–1,707 | additive, quadratique | additive, quadratique |

Et **mieux satisfaite** qu'en S186 : la plage du maximum se resserre de 0,826–1,155 à
0,895–1,060 pour le maintien, et de 0,860–1,034 à 0,869–1,000 pour l'extrapolation.
L'erreur concentrée du réseau débordant rendait la composition **plus bruyante**, pas plus
propre. Aux cadences hautes la loi est **exacte** : mnt/ext à `c = 64` rendent 33,2115 et
27,3202, les valeurs temporelles pures, rapport 1,000 aux trois lignes.

**Et la métrique ajoutée dit pourquoi.** La tranche qui porte le maximum est **14** dans
tous les cas — spatial seul, temporel seul, et les 84 cases composées. Compte publié :
**39 cases jugées sur 39 où les deux maxima vivent sur la même tranche.** L'ancrage a
changé la **magnitude** de l'erreur spatiale, pas **l'endroit** de son maximum : le champ
lui-même culmine en haut du bloc, parce que c'est là que `|S|` est le plus grand, et aucun
réseau n'y change rien. C'est la première des trois issues déclarées en §4 — *la loi
tient* — et la colonne de tranche montre qu'elle tient pour **la même** raison.

**Les magnitudes, elles, bougent beaucoup.** Erreur spatiale seule, à nombre de nœuds
identique :

| ligne | nœuds | ancré | débordant (S186) | facteur |
|---|---:|---:|---:|---:|
| r=1 | 2744 | 0 | 0 | — |
| r=2 | 512 | **1,7160 %** | 2,5401 % | 1,48 |
| r=4 | 125 | **3,6805 %** | 13,6043 % | **3,70** |
| r=8 | 27 | **13,1488 %** | 32,9593 % | 2,51 |

Deux lectures utiles. **Mesurée, sans interpolation : 27 nœuds ancrés (13,1488 %) valent
125 nœuds débordants (13,6043 %) — même erreur pour 4,6 fois moins de nœuds.** Et le point
de parité entre les deux axes se déplace : à 125 nœuds, l'erreur spatiale débordante 13,60 %
égalait le maintien vers `c ≈ 20` ; ancrée à 3,68 %, elle l'égale vers `c ≈ 6`. **La règle
de dimensionnement de S186 §8.5 tient, son point d'application se déplace d'un facteur ~3.**
Conséquence de conception : sur un réseau ancré, l'optimum va vers **plus** de décimation
spatiale et **moins** de réduction de cadence.

Indices ancrés obtenus, à noter parce qu'ils ne sont pas ceux qu'on poserait à la main :
14 → [1..14] ; 8 → [1,3,5,7,8,10,12,14] ; 5 → [1,4,8,11,14] ; 3 → [1,8,14]. L'arrondi au
plus proche produit un pas irrégulier (7→8 puis 8→10) ; c'est le prix de l'accrochage aux
centres de mailles, et il est visible plutôt que lissé.

**Ce qui n'est pas mesuré** : la composition sur un réseau **gradué** — celui qui répartit
vraiment l'erreur, puisque S187 y relevait une tranche haute à zéro. C'est la seule
configuration où les deux maxima pourraient cesser de coïncider, et c'est donc le seul
endroit où la loi resterait à éprouver.

P3a S188 : `anchored_indices(n, want)` posée dans `support/perturbative_block.rs` ;
`graded_lattice` y pointe et rend **0x6cf13183b4a240df**, sa valeur publiée. Les deux
autres empreintes du support sont vérifiées par la même occasion : `cadence_error`
**0x39567a1d4bc2ba4c**, `composed_error` **0x0e743846d4656870** et sa sortie entière
identique au `diff`. Les trois réceptions publiées tiennent après le déplacement.

S188 : master 2d05c77 propre, quatre copies au même commit ; 118 ADR / 231 angles /
267 leçons / 18 invariants / 6 SPEC / 23 cas. Démarrage à froid, copie principale.

**Entrées déjà acquises, ne pas les refaire.** S186 donne la grille complète sur réseau
débordant : erreurs spatiales seules 2,5401 / 13,6043 / 32,9593 % à `r = 2/4/8`, erreurs
temporelles par mode et par cadence, et les trois lois avec leur verdict par mode (maximum
pour maintien et extrapolation, quadratique pour l'interpolation). S187 donne les erreurs
ancrées à nœuds égaux, mais **seulement à `c = 1`** : 8³ ancré → 1,6947 %, et le plancher
horizontal de son propre réseau. Le plancher de référence est 0,386 % et ne bouge pas —
même référence, mêmes dénominateurs `max|S| = 1,540547e-4`, `max|u'(T)| = 7,993168e-5`.

**Le réseau ancré à comparer.** Pour que le rejeu soit lisible, la famille ancrée doit
avoir **exactement les nombres de nœuds** de la famille isotrope de S186 : 14³ = 2744,
8³ = 512, 5³ = 125, 3³ = 27. Ce sont les `uniform_indices` de S187 à 14, 8, 5 et 3 nœuds
par axe, appliqués aux trois axes. Une seule variable change entre S186 et S188 : **où le
dernier nœud se pose**. Si l'on changeait aussi la graduation, on ne saurait pas à quoi
attribuer l'écart.

**Piège anticipé.** À 14 nœuds par axe, l'ancré et le débordant coïncident (`axis_indices`
à `r = 1` donne déjà 1..14) : la ligne `14³` doit donc être **identique en bits** à la
référence et à la ligne `r = 1` de S186. C'est la réception la moins chère et la plus
parlante — si elle échoue, le réseau ancré n'est pas ce qu'on croit.

**Second piège.** `scatter_indexed` n'est pas `scatter`. S187 a reçu leur identité en bits
sur les indices uniformes **débordants** ; le rejeu utilise les indices **ancrés**, pour
lesquels aucun `scatter` historique n'existe. L'identité ne peut donc être vérifiée qu'au
cas `r = 1`, et c'est une limite du contrôle, pas une échappatoire.

---

Session précédente : S187 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S186-1/A50, le **réseau gradué en profondeur**. S186 a montré que l'erreur
spatiale globale est **exactement** celle de la tranche la plus haute du bloc, et que les
treize autres sur quatorze sont surrésolues d'un facteur pouvant atteindre vingt-trois.
Un réseau isotrope dépense donc la même densité de nœuds là où le contenu est lisse et là
où il ne l'est pas. Question mesurée : **à erreur égale, combien de nœuds un réseau gradué
économise-t-il ?** Et avant elle, celle que S186 a laissée ouverte : **quelle part de
l'erreur du haut vient de l'axe vertical ?** Si elle est horizontale, graduer en `z` ne
réduit pas l'erreur — il ne réduit que le nombre de nœuds, ce qui reste le but.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — publier le protocole avant tout chiffre : attribution par axe, règle de
      graduation **dérivée** de la mesure (équidistribution de `h²·∂²S`), la courbe
      erreur/nombre de nœuds comme livrable, les réceptions, et ce que la mesure ne
      prouvera pas.
- [x] **P3a** — étendre `support/perturbative_block.rs` **sans toucher au chemin
      isotrope** : réseau à pas par axe, puis réseau à indices quelconques par axe.
      Réception : le chemin général doit reproduire le chemin uniforme **en bits**, et
      S185/S186 doivent rendre leurs empreintes publiées (`0x39567a1d4bc2ba4c`,
      `0x0e743846d4656870`).
- [x] **P3b** — attribuer l'erreur par axe, puis construire la graduation depuis le profil
      mesuré et relever la courbe iso-erreur. Relever.
- [x] **P4** — recevoir dans un document de validation ; angles, leçons, et **ADR si une
      décision de conception en sort** — un réseau d'échantillonnage gradué est un contrat
      pour le consommateur, pas un détail de banc.
- [x] **P5** — rituel de fin (REPRISE.md §6).

### Notes de reprise

P5 S187 : rituel terminé. Décomptes **vérifiés contre le dépôt** : 118 fichiers ADR,
18 invariants, 6 SPEC (dont SPEC-003 dans `docs/validation/`), 23 cas, 267 leçons, A231
au plus haut — **118 ADR, 231 angles, 267 leçons, 18 invariants, 6 SPEC, 23 cas**, portés
dans README, 00_INDEX et REPRISE, et le décompte daté de la table des registres passé à
231 (S187). Invariants relus : aucun ne mentionne l'échantillonnage ni la décimation,
donc aucun ne devient faux ; ADR-118 cite SPEC-004 §6.1/§6.2 et ADR-007 §5, et la §6.2
reçoit une note corrective datée plutôt qu'une réécriture.
**Action non faite et portée, pas perdue** : le réseau de `support/` déborde toujours.
L'ancrer casse les empreintes publiées de S184 et S186 ; c'est donc l'objectif déclaré
de S188, porté par la ligne `Session suivante` (L55).
Jeton libre ; copie principale, aucune copie ouverte. Les trois worktrees ont été
avancés sur master en fin de session.
Suite S188 : S187-1, ancrer et rejouer la composition de S186.

P4 S187 : RESEAU-GRADUE-S187 §8–§9 reçus. **ADR-118 actée** — premier ADR depuis S181 :
le réseau d'échantillonnage s'ancre sur ses frontières, gradue son pas selon la courbure,
et s'arrête quand son axe cesse d'être le plus grossier. **A231** écrite (le débordement
du réseau, jusqu'à un facteur six), **L267** écrite (une campagne à convention fixée
mesure la convention). Deux suivis datés portés sous **A50** et sous **A229** (la
compensation vaut aussi entre axes d'espace). Quatre suivis datés ajoutés sans rien
réécrire : COMPOSITION-ERREURS-S186 (portée des magnitudes, loi non rejouée),
CONSOMMATION-S184 (le coût n'est pas affecté, l'erreur l'était), BILAN-B4-S176,
SOURCE-DECIMEE-S170. Et une **note corrective datée dans SPEC-004 §6.2** : la contrainte
`dx ≤ λ_cut/N` est nécessaire et insuffisante — `λ_cut` n'est pas la longueur d'onde qui
compte (A230) et une densité ne dit rien du placement (A231).
Décomptes à porter en P5, vérifiés contre le dépôt : **118 ADR, 231 angles, 267 leçons,
18 invariants, 6 SPEC, 23 cas**.

P3b S187 : `examples/graded_lattice.rs`. Deux exécutions, `diff` strict identique hors
lignes de cargo ; aucune durée mesurée. Empreinte **0xf2dfa382290e3c64** puis
**0x6cf13183b4a240df** après l'ajout de la table d'ancrage horizontal — c'est la valeur
finale à publier. Réceptions 1, 2, 4 et 6 passent ; `max|S| = 1,540547e-4` et
`max|u'(T)| = 7,993168e-5` redonnent S186, réception 3.

**Amendement de protocole, à déclarer et non à taire.** `rh = 8` a été ajouté après la
première exécution : le protocole déclarait `rh ∈ {1,2,4}` et la famille graduée n'avait
alors aucun point sous 75 nœuds, ce qui laissait l'isotrope `r = 8` (27 nœuds) sans
comparaison. Extension du balayage, pas affaiblissement du critère.

**Q1 — l'axe vertical domine, et les deux axes se compensent** (`eU` en % de max|u'|) :

| r | verticale seule | horizontale seule | isotrope |
|---|---|---|---|
| 2 | 2,6635 (1568 nœuds) | 1,6947 (896) | **2,5401** (512) |
| 4 | 15,4059 (980) | 6,1256 (350) | **13,6043** (125) |
| 8 | 45,5067 (588) | 13,4919 (126) | **32,9593** (27) |

L'isotrope est **sous** l'axe vertical seul aux trois `r` : même famille que A229, deux
approximations sur le même contenu se compensent partiellement. Rapport vertical/horizontal
1,57 / 2,51 / 3,37.

**Le profil remesuré confirme la dérivation.** `|∂²_z S|` de 3,83e-6 (fond) à 4,84e-5
(haut), rapport extrême **12,63**, donc pas vertical profond jusqu'à **3,55 fois** celui du
haut. Le protocole avait dérivé 12,6 et 3,5 depuis les `k_eff` de S186 : le profil mesuré
ici par le programme qui l'utilise tombe dessus.

**LE RÉSULTAT QUI RENVERSE LA SESSION — l'ancrage pèse plus que la graduation.** Le réseau
historique pose son dernier nœud **hors du bloc** (indice 17, `z = −0,05 m`) alors que les
mailles intérieures s'arrêtent à 14 (`z = −0,80 m`). À nombre de nœuds verticaux **égal**,
pas horizontal 2 :

| Nz | débordante (historique) | ancrée uniforme | ancrée dérivée |
|---|---|---|---|
| 3 | **41,2153** | 6,7887 | 6,5503 |
| 5 | **13,6044** | 2,5458 | **1,7919** |
| 8 | **2,5401** | 1,6947 | 1,6947 |
| 14 | 1,6947 | 1,6947 | 1,6947 (768 nœuds, 12 après fusion) |

**L'ancrage vaut un facteur 6,1 / 5,3 / 1,50** ; la graduation par-dessus ne vaut que
**1,04 / 1,42 / 1,00**. Mécanisme : la métrique est un **maximum** et le maximum vit sur la
tranche la plus haute (S186) ; un nœud posé exactement sur cette tranche supprime le terme
dominant, tandis qu'un réseau qui déborde l'interpole sur 2 m.

**Contre-épreuve horizontale** — l'ancrage ne vaut presque rien là où la source ne pique
pas : 3 nœuds 13,4919 → 13,1488 (−2,5 %) ; 5 nœuds 6,1256 → 3,6805 (−40 %) ; 8 nœuds
1,6947 → 1,7160 (**+1,3 %**, donc légèrement pire). Non monotone, et sans le facteur 6.
Donc ce n'est pas « ancrer est mieux » : c'est **poser un nœud là où vit le maximum**.

**Q2 — la courbe, et le plancher.** À `rh = 2` le plancher est l'erreur horizontale,
**1,6947 %**, atteint dès **6 nœuds verticaux gradués** (384 nœuds) ; au-delà, `Nz = 8` et
`Nz = 14` ne changent plus rien. Lectures à ordonnée égale :
- isotrope `r = 2` (512 nœuds, 2,5401 %) → gradué `rh=2 Nz=5` (320 nœuds, **1,7919 %**) :
  **−37,5 % de nœuds et −29 % d'erreur en même temps** ;
- isotrope `r = 4` (125, 13,6043 %) → gradué `rh=8 Nz=3` (27, 13,4919 %) : **−78,4 %** ;
- isotrope `r = 8` (27, 32,9593 %) → gradué `rh=8 Nz=3` (27, 13,4919 %) : **à nœuds
  identiques, l'erreur est divisée par 2,44**.
À `rh = 4` le plancher est 6,1256 % (atteint à `Nz = 4`, 100 nœuds) ; à `rh = 8`, 13,4919 %
(atteint dès `Nz = 3`, 27 nœuds). Les lignes `rh = 1` montrent le déplacement du maximum :
erreur de tranche haute **0,0000** et erreur globale 0,29 à 6,55 % selon `Nz` — la
graduation chasse l'erreur du haut vers le milieu, ce qui est exactement son but.

**Les deux témoins naïfs sont battus, et la règle dérivée est validée sans être
spectaculaire.** À nœuds égaux et `rh = 2` : 320 nœuds → dérivée 1,7919, uniforme 2,5458,
géométrique 3,4373. À 192 → 6,5503 / 6,7887 / 6,5503 (la géométrique coïncide avec la
dérivée à 3 nœuds). À 512 → 1,6947 / 1,6947 / 3,4373 (les deux premières saturent). La
dérivation gagne donc **là où elle sert**, entre 4 et 6 nœuds, d'un facteur jusqu'à 1,42.

**Ce qui n'est pas mesuré et doit être dit** : la loi de composition de S186 (le maximum
pour les modes causaux) a été établie sur le réseau **débordant** ; elle n'est pas rejouée
sur un réseau ancré. Et l'interpolation d'un réseau gradué coûte un peu plus par maille que
l'uniforme (poids non constants), ce qui n'est pas chiffré.

P3a S187 : le support est étendu **à côté** du chemin isotrope — `axis_indices`,
`axes_indices`, `lattice_points_indexed`, `scatter_indexed`, `indexed_count` dans
`perturbative_block.rs`, et la reconstruction sortie dans
`support/source_snapshots.rs` (S186 en avait une copie locale ; deux copies auraient
divergé, L137). **Les deux vérifications passent** : la sortie **entière** de
`composed_error` est identique au `diff` strict après le déplacement, et
`cadence_error` rend `0x39567a1d4bc2ba4c`. Workspace **331 réussis / cinq ignorés** en
debug et en release.
Réception 2 : le réseau général reproduit l'uniforme — mêmes points **et** champ
identique en bits pour `r ∈ {1,2,4,8}`. C'est exact et non fortuit : les poids valent
`(i−idx)/span` contre `((i−1)%r)·(1/r)`, et `1/r` est exact pour une puissance de deux.

S187 : master beadbf0 propre, quatre copies au même commit ; 117 ADR / 230 angles /
266 leçons / 18 invariants / 6 SPEC / 23 cas. Démarrage à froid, copie principale.

**Entrées déjà acquises, ne pas les refaire.** S186 donne, au même montage et au même
bloc : l'erreur spatiale isotrope (r=2 → 2,54 % ; r=4 → 13,60 % ; r=8 → 32,96 %), son
lieu (intégralement la tranche k=14, z=−0,80), le profil `k_eff` par tranche et par axe
(horizontal 0,37→0,79 rad/m du fond vers le haut, vertical 0,496→1,166), et `max|S|` par
tranche (1,45e-5 → 4,14e-5, longueur d'atténuation 3,1 m). Le plancher de la référence
est 0,386 % et il ne bouge pas : c'est la même référence.

**La graduation se dérive avant de se coder.** L'erreur d'une interpolation linéaire vaut
`≈ (1/8)·Σ_axes h_axe²·|∂²S/∂x_axe²|`. Égaliser la contribution d'une tranche à l'autre
demande donc `h_z(z) = C/√|∂²_z S(z)|`, c'est-à-dire de placer les nœuds à **incréments
égaux de `Φ(z) = ∫ √|∂²_z S| dz`**. Le nombre de nœuds `N` fixe `C` ; balayer `N` donne la
courbe. Avec les chiffres de S186, `|∂²_z S| = k_eff_z²·max|S|` passe de 3,83e-6 (k=2) à
4,84e-5 (k=13), soit un rapport **12,6** : le pas vertical profond peut être **√12,6 ≈ 3,5
fois** celui du haut. À pas horizontal `r = 2` et huit nœuds verticaux uniformes (512
nœuds), la graduation devrait tenir en quatre ou cinq — soit **−37 % de nœuds à erreur
égale**, si la tranche haute reste la contrainte.

**Piège anticipé.** Si l'erreur de la tranche haute est majoritairement **horizontale**,
aucune graduation verticale ne l'abaisse — et le gain est alors uniquement en nœuds, à
erreur inchangée. C'est précisément pourquoi l'attribution par axe passe **avant** la
graduation, et non après : elle décide si la courbe se lit comme un gain d'erreur ou comme
un gain de coût.

**Second piège.** Le réseau gradué ne doit pas changer le chemin isotrope : S184 et S186
en dépendent, et S186 a publié deux empreintes. Écrire à côté, pas dedans, et vérifier
par exécution — c'est ce que S185 et S186 ont fait tous les deux.

---

Session précédente : S186 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S185-1/A50, **composer** l'erreur spatiale et l'erreur temporelle sur le
**même** véhicule 3D. S170 a mesuré la décimation spatiale en 1D sur une source figée,
S185 la cadence temporelle en 3D sur un réseau plein. Les deux erreurs n'ont jamais été
mesurées ensemble, et S170 avertit qu'« un ratio de décimation ne décrit pas à lui seul
la précision ». Un budget conjoint `r × c` n'a de sens que si l'on sait comment les deux
erreurs se composent — additivement, quadratiquement, ou pas du tout.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — publier le protocole avant tout chiffre : véhicule partagé, grille
      `r × mode × c`, **une seule** référence, métriques, les trois lois de composition
      mises à l'épreuve, les réceptions, et ce que la mesure ne prouvera pas.
- [x] **P3a** — écrire `examples/composed_error.rs` : source échantillonnée sur le réseau
      `r` aux seuls instants de cadence, `scatter` trilinéaire, puis le même pas.
      Réceptions : `(r=1, c=1)` identique **en bits** à la référence ; la ligne `c=1`
      redonne l'erreur spatiale pure ; la ligne `r=1` redonne les constantes de S185 ;
      empreinte reproductible.
- [x] **P3b** — chiffrer la composition et éprouver l'équivalence advective `h ↔ v·τ` :
      pour un contenu advecté, décimer en espace et retarder en temps pourraient être
      la **même** erreur, et alors elles ne s'additionnent pas.
- [x] **P4** — recevoir dans un document de validation ; angles et leçons.
- [x] **P5** — rituel de fin (REPRISE.md §6).

### Notes de reprise

P5 S186 : rituel terminé. Décomptes **vérifiés contre le dépôt** et non recopiés —
117 fichiers ADR, 18 invariants (I-01 à I-18), 6 SPEC (dont SPEC-003 qui vit dans
`docs/validation/`), 23 cas, 266 leçons, A230 au plus haut : **117 ADR, 230 angles,
266 leçons, 18 invariants, 6 SPEC, 23 cas**, portés dans README, 00_INDEX et REPRISE.
Un décompte périmé corrigé au passage : la table des registres de 00_INDEX annonçait
« 193 points » pour ANGLES-MORTS, quarante de moins que la réalité ; il est maintenant
**daté** plutôt que recopié (A185). Invariants relus : I-01, I-03, I-06, I-14, I-15 —
aucun ne devient faux, et pour cause, aucun contrat n'a changé.
Jeton libre ; copie principale, aucune copie ouverte. Les trois worktrees ont été
**avancés sur master** en fin de session : ils annonçaient « suivante S186 » avec un
jeton libre, exactement le mécanisme des trois forks.
Suite S187 : S186-1, le réseau gradué en profondeur.

P4 S186 : COMPOSITION-ERREURS-S186 §8–§9 reçus, **A229** et **A230** écrits, **L266**
écrite. Trois suivis datés ajoutés sans rien réécrire : CONSOMMATION-S184 (la
justification de `r = 2` est fausse, la borne tient), CADENCE-3D-S185 (S185-1 réalisée,
empreinte inchangée après déplacement), SOURCE-DECIMEE-S170 (son avertissement §2.2
est chiffré et il avait raison). Décomptes à porter en P5, **vérifiés contre le dépôt**
et non recopiés : **117 ADR, 230 angles, 266 leçons, 18 invariants, 6 SPEC, 23 cas**.
Aucun ADR : aucun contrat n'a changé, et mesurer n'est pas décider.

P3b S186 : synthèse **calculée par le programme**, pas posée à la main. Empreinte
**inchangée, 0x0e743846d4656870** — les blocs ajoutés n'impriment que des grandeurs
dérivées, aucun nouveau `write_f32`, et deux exécutions restent identiques au `diff`.

**Le verdict global est celui déclaré : les trois lois sont rejetées.** Mais séparé par
mode, il devient net et utilisable :

| mode | additive | quadratique | maximum | retenue |
|---|---|---|---|---|
| maintien | 0,529–0,976 | 0,749–0,999 | 0,826–1,155 | **maximum** |
| extrapolation | 0,540–0,976 | 0,760–0,999 | 0,860–1,034 | **maximum** |
| interpolation | 0,803–0,988 | 0,991–1,209 | 0,998–1,489 | **additive, quadratique** |

Donc : **pour les deux modes causaux — les seuls dont le runtime dispose — la loi est le
maximum.** Les deux erreurs ne s'ajoutent pas : la plus grande gagne. Conséquence de
conception directe : l'axe bon marché est **gratuit** jusqu'à la parité avec l'axe
dominant, et raffiner au-delà n'achète rien. C'est la réponse à « un budget conjoint
est-il licite » : oui, au sens du maximum, pour un consommateur causal.
L'additive n'est dépassée **nulle part** (max 0,988 sur les 84 cases) : enveloppe sûre.

**H1 est confirmée, au nombre d'axes près.** `A_temps = 0,0522` (interpolation
temporelle, deux cadences jugées ; S185 mesurait 0,052 stable). `A_espace = 0,1187`
à `r=2`, `0,1590` à `r=4`. Rapport **2,27 et 3,05** — et ce rapport ne dépend pas du
normalisateur, puisque les deux constantes sont du second ordre dans le même `λ`.
`scatter` interpole sur **trois** axes, le temps sur un : le facteur ~3 est le nombre
d'axes. La branche verticale du protocole — un facteur approchant `(2π)² ≈ 39` par la
décroissance `exp(kz)` — est **réfutée**, et la raison est le filtrage par la profondeur.

**Non-monotonie, par mode** : mnt r=4 −17,44 % à c=8 ; mnt r=2 −13,20 % à c=2 ;
ext r=2 −14,05 % à c=8 ; ext r=4 −12,15 % à c=16. Interpolation : −0,04 à −0,16 %,
donc rien. **La compensation partielle appartient aux modes causaux** et disparaît avec
le mode qui n'a presque pas d'erreur temporelle. À publier comme piège de réglage.

P3a S186 : `examples/composed_error.rs` + `examples/support/reuse_mode.rs`. Deux
exécutions, `diff` strict **identique** — aucune durée n'est mesurée dans ce véhicule,
donc la sortie entière est un résultat. Empreinte **0x0e743846d4656870**.
Les `Mode` et `build_source` de S185 ont été **déplacés** dans `support/reuse_mode.rs` ;
`cadence_error` rejoué, empreinte **0x39567a1d4bc2ba4c inchangée** — le déplacement est
vérifié, pas supposé. Workspace **331 réussis / cinq ignorés** en debug et en release.

**Réceptions, les six passent.** (2) `scatter` à `r=1` identique au chargement direct en
bits. (3) contrôle croisé : `max|S| = 1,540547e-4`, `max|u'(T)| = 7,993168e-5`, et la
ligne `r=1` redonne **exactement** les quatorze `eS/eU` de S185 §6.2 (mnt c=2
1,6894/0,7700 … int c=64 12,3586/6,7740). (4) à `c=1` les trois modes sont identiques en
bits pour chaque `r`. (5) plancher `dt/2` = **0,386 %**, la valeur de S185 à la décimale.
(6) tout fini.

**Chiffres bruts, à publier en P4 :**
*Contenu effectif* — `k_eff` mesuré au bloc : horizontal 0,367 → 0,791 rad/m du fond
(z −4,05) vers le haut (z −0,80), vertical 0,496 → 1,166. Soit `λ_eff` de **17 m à 8 m**
horizontalement et **12,7 m à 5,4 m** verticalement, contre `λ_min = 1,081 m` de la
recette. **La profondeur filtre : le contenu présent est 5 à 16 fois plus lisse que la
coupure.** `max|S|` par tranche passe de 1,45e-5 à 4,14e-5 — facteur 2,86 sur 3,25 m,
donc une longueur d'atténuation de 3,1 m, pas les 0,172 m de `1/k_max`.
*Erreur spatiale seule* — r=2 : eS 3,59 % / eU **2,54 %** ; r=4 : 17,89 / **13,60** ;
r=8 : 27,60 / **32,96**. Constante en `(h/λ_min)²` : 0,1187 / 0,1590 / 0,0963.
*Par tranche* — l'erreur globale est **exactement** celle de la tranche la plus haute
(k=14, z=−0,80) aux trois `r` : 2,5401 / 13,6043 / 32,9593. La tranche du fond ne vaut
que 0,11 / 0,43 / 1,54. **Un réseau isotrope gaspille en profondeur ce qui manque en
surface.**
*Composition* — **les trois lois sont rejetées** par le critère déclaré `[0,80 ; 1,25]` :
additive 0,529–0,988, quadratique 0,749–1,209, maximum 0,826–1,489. Mais l'additive
n'est **jamais dépassée** (max 0,988) : c'est une enveloppe sûre à 1,9× de mou près.
La quadratique vaut 1,000 quand un axe domine (`r=8`, ou mode interpolation) et casse
quand les deux sont comparables.
*Non-monotonie, le résultat inattendu* — **dégrader la cadence réduit l'erreur totale** :
mnt r=4 passe de 13,6043 (c=1) à **11,2323** (c=8), soit **−17,4 %** ; ext r=4 à 11,9516
(c=16), −12,1 % ; mnt r=2 à 2,2048 (c=2), −13,2 %. Visible aussi dans `eS` seule
(int r=4 : 17,89 → 15,16 à c=32), donc ce n'est pas un artefact de l'évolution.
Un balayage à un axe à la fois trouve donc un optimum **faux**.

S186 : master e46de38 propre, quatre copies au même commit ; 117 ADR / 228 angles /
265 leçons / 18 invariants / 6 SPEC / 23 cas. Démarrage à froid, copie principale.
**Entrées déjà acquises, ne pas les refaire :** le support `examples/support/` porte
déjà `lattice_points`, `nodes_per_axis` et `scatter` (trilinéaire, borné au coin
supérieur) — S184 s'en sert pour le **coût**, S186 s'en sert pour l'**erreur**, sans
écrire un second réseau. `RATIOS = [1,2,4,8]`, `DX = 0,25 m`. Les échelles du contenu
sont calculées et non posées : pression `λ_min = 1,081 m` (coupure 6 rad/m, 16 radiaux),
impact `λ = 4 m`, `B` le plus court `λ = 14,05 m` (période `Tp/2 = 3 s`, eau profonde).
Donc **c'est la pression qui fixe les deux échelles à la fois** — spatiale par
`λ_min`, temporelle par `T = λ_min / 2 m/s = 0,5405 s`. C'est précisément pourquoi les
deux erreurs risquent de ne pas être indépendantes.
Piège anticipé : une seule référence pour tout le tableau, sinon les erreurs se
comparent à des choses différentes et leur composition n'a aucun sens.

---

Session précédente : S185 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S184-1/A50, mesurer l'**erreur** de la cadence temporelle en 3D avec le
fournisseur réel — et séparer ce qui est disponible au runtime de ce qui ne l'est pas.
S174 avait interpolé entre deux instantanés en écrivant noir sur blanc que l'échantillon
futur n'est pas disponible pour un événement inconnu. S185 mesure les modes **causaux**.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — publier le protocole avant tout chiffre : véhicule à instant qui avance,
      trois modes de réemploi (maintien, extrapolation causale, interpolation à une
      période de latence), métriques, et ce que la mesure ne prouvera pas.
- [x] **P3a** — partager le véhicule : `support/` pour l'hôte, les paramètres de montage
      et le bloc. S185 doit évoluer **le même pas** que S184, sinon la comparaison ne vaut
      rien et deux copies divergeront (L137). Rejouer S184 et vérifier ses chiffres.
- [x] **P3b** — écrire `cadence_error` : l'instant avance, le contrôleur est actualisé à
      chaque reconstruction. Réceptions : reproductibilité en bits, identité de
      prédiction champ/intégrale de l'erreur de source, cadence 1 identique à la
      référence. Relever.
- [x] **P4** — chiffrer le compromis cadence × mode, et le coût du mode lui-même.
      Recevoir dans un document de validation.
- [x] **P5** — rituel de fin (REPRISE.md §6).

### Notes de reprise

P5 S184 *(exécuté avant P4 : la mesure devait exister pour que le document puisse la
recevoir ; le plan et git disent la même chose, dans cet ordre)*.
`examples/lattice_phase.rs`, deux exécutions. **Résultat négatif, et c'est l'utile.**
La phase et la trigonométrie ne pèsent que **15–18 %** du différentiel de B, et une
récurrence de réseau n'en retire que **12–15 %**. Le coût n'est pas la trigonométrie :
c'est produire 26 scalaires par composante — 140 ns par composante et par nœud, dont
21 seulement de phase. Changer la traversée ne sauve rien ; il faudrait changer ce
qu'on calcule. Le levier non testé est la vectorisation, pas le parcours.
Chiffres du contenu, calculés et non posés à la main : coupure de pression
k_max=5,8125 rad/m → **λ_min = 1,081 m**. Points par λ_min : H=0,25 → 4,32 ;
H=0,50 → 2,16 ; H=1,00 → 1,08 ; H=2,00 → 0,54. **La décimation spatiale est donc
bornée à r=2 par le contenu**, soit 5,3× seulement. La cadence, elle, divise
exactement par c et le contenu temporel est lent (périodes 3–12 s, segments 2 s) :
c'est l'axe bon marché. Asymétrie à publier.
Note : ce montage utilise le spectre JONSWAP (`from_spectrum`, 32 composantes minimum),
pas le `configure` historique à 16 de S183 ; les parts sont donc internes à ce montage.

P3 S184 : `examples/perturbative_step.rs`, deux exécutions concordantes. Les quatre
réceptions passent, **zéro composante exemptée par ±0**. Chiffres, à publier en P4 :
le pas coûte **14–17 ns par maille**, la source **34–35 µs par maille** — rapport
**~2100**, le pas est **0,047–0,049 %** du total. Décimation spatiale : le gain est
exactement le rapport de nœuds (r=2 → 6,0–8,7 µs ; r=4 → 1,3–1,8 ; r=8 → 0,36–0,41) ;
l'interpolation trilinéaire ajoute ~20 ns/maille, du même ordre que le pas. Cadence :
divise exactement par c (34,5 / 17,4 / 8,6 / 4,3 / 2,2). **r=8 et c=16 combinés
ramènent la source à ~21 ns/maille/pas**, soit la parité avec le pas — donc 8192× de
décimation pour égaler un pas explicite nu. Or la source contient des modes de
pression jusqu'à la coupure 6 rad/m, soit λ≈1,05 m : à dx=0,25 m, r=2 met déjà la
plus courte longueur d'onde à deux points. Le contenu interdit la décimation dont le
coût aurait besoin. **Correction de protocole** : la réception 2 telle que publiée en
P2 était fausse (la différence des deux pas ne vaut pas −dt·S en flottant à état non
nul) ; elle est remplacée par un contrôle exact — source forcée à zéro, le chemin
« avec » rejoint le chemin « sans » en bits. À noter dans le document, pas à réécrire
en silence.

S184 : master 0643cfb propre, quatre copies alignées ; 117 ADR/226 angles/263 leçons.
Démarrage à froid, copie principale. **Entrées déjà acquises, ne pas les refaire :**
S183 donne le coût de produire la source — 37 µs par point différentiel au montage de
référence (B16, 1 impact, 192 créneaux), 30 µs pour la seule pression. S170 donne
l'erreur de la décimation spatiale en 1D et dit explicitement qu'elle ne prouve « ni un
gain de temps, ni le facteur 64 en 3D » ; S174 fait de même pour la cadence temporelle.
S184 n'a donc **pas** à remesurer l'erreur de décimation : elle mesure le temps.
Le dépôt n'a aucun véhicule 3D ; `delta`/`shallow`/`dispersif` sont 1D. Le véhicule de
S184 est un exemple, pas de la bibliothèque — précédent `source_decimee.rs` (S170).
Piège anticipé : un pas sans projection de pression sous-estime le coût du solveur et
**sur**-estime donc la part de la source ; le dire, et borner la projection.

P5 S183 : rituel terminé. **117 ADR, 226 angles, 263 leçons, 18 invariants, 6 SPEC,
23 cas**, vérifiés contre le dépôt (117 fichiers ADR, 18 invariants, 6 SPEC, 23 cas)
et non recopiés. Jeton libre ; copie principale, aucune copie à refermer, les trois
worktrees étaient au même commit que master à l'amorce. Une correction de fond au
passage : la formule « BILAN-S145/S176 portés » circulait depuis plusieurs sessions
alors que **BILAN-S145 est soldé depuis S147** (B1 lancé S146, S63-1 close S147) ;
seul BILAN-B4-S176 reste actif, et il reçoit un suivi daté. Suite S184 : S183-1,
consommation perturbative.

P4 S183 : COUT-DIFFERENTIEL-S183 §6-§8 reçus, **A226** et **L263** écrits. Décomptes
à porter en P5 : **117 ADR, 226 angles, 263 leçons, 18 invariants, 6 SPEC, 23 cas**
(README, 00_INDEX, REPRISE). Aucun ADR : aucun contrat n'a changé. Deux pourcentages
recalculés avant publication — l'écart lot1→lot256 vaut +13 à 16 % en différentiel et
+39 à 43 % en surface, pas les valeurs posées de tête au premier jet.

P3 S183 : `examples/differential_cost.rs`, deux exécutions concordantes, aucun
changement de bibliothèque ; workspace 331 réussis/cinq ignorés debug et release.
Chiffres tenus hors document tant que P4 n'a pas été committé — les voici :
rapport différentiel/surface **3,0 à 4,3** selon lot et montage, médiane ~3,4 ;
il est **le même couche par couche** (B 3,4 ; impact 3,1 ; pression 3,4), donc il
suit le nombre de scalaires publiés (31 contre 10), pas la nature du calcul.
Préparation et actualisation sont **identiques** aux deux chemins : le contrôleur
de pression coûte 168-178 µs par instant publié à 192 créneaux, 41 µs à 48, 355 µs
à 384 — linéaire, ~0,9 µs par créneau, et c'est le poste dominant du cycle.
Refus de montage 0,025-0,097 µs. **Refus au dernier point d'un lot 64 : 2 232-2 400 µs**,
soit le prix du lot entier ; au premier point : 2,0 µs. Un point hors domaine paie
quand même le différentiel de B (2,0 µs à 16 composantes, 7,6-8,4 à 64), parce que
le test de rayon des impacts vient après. Allocations hôte : 512 octets/1 appel
(2048 à 64 composantes), zéro refus après seal. Inspection de source : le seul tas
du chemin d'exécution est `Background.components`, déclaré à l'hôte.

S183 : master 541ebc5 propre, trois copies alignées ; 117 ADR/225 angles/262 leçons.
Démarrage à froid, copie principale, aucune copie nouvelle. Précédent de méthode :
COUT-PROFIL-IMPACT-S125 (deux exécutions, blocs alternés, black_box, mise en régime,
construction séparée de l'évaluation). Les deux chemins à comparer sont strictement
parallèles : `sample_world_batch` (WaterSample) et `differential_world_batch`
(DifferentialSample), mêmes entrées, même `classify`. Ne pas annoncer de budget :
S125 rappelle que les ~49 ms de S118 sont un contexte historique, pas une enveloppe.

S182 : master3541390 propre, quatre copies alignées ;117 ADR/225 angles/262 leçons.
Suite S181-1. Les contrôleurs existent ; éprouver le consommateur S181 sur leurs
publications contre préparation directe et rejeu. Ne pas inventer un nouvel ADR si
le contrat reste inchangé. Aucun coût ou solveur reçu par ces comparaisons.

P3 S182 : trois tests nouveaux debug/release, workspace331/cinq ignorés ; C18/C02
inchangés.34 scalaires en bits par point, source comprise. Actualisation, admission
incrémentale/intercalée, saturation/reprise, renouvellement et restauration reçus.
Aucune correction runtime. Suite S182-1 coût complet du consommateur différentiel.

P4 S182 : rituel terminé ;117 ADR/225 angles/262 leçons/18 invariants/6 SPEC/23 cas.
Aucun nouvel ADR, angle ou leçon. Jeton libre ; suite S183, mesure du coût complet.

S181 : master a77ea78 propre, quatre copies alignées ;116 ADR/225 angles/262 leçons.
Le montage mixed_water possède déjà classify (contexte/temps/perte), BoundBackground
et les vues de journaux. Réutiliser ces contrôles plutôt qu'inventer une association
locale sans instant. Pression imposée déjà incluse ; source après somme des champs.
Poursuite BILAN-S145/S176. Aucune copie nouvelle ; notes antérieures conservées.

P3 S181 : quatre tests nouveaux debug/release ; workspace328/cinq ignorés, C18/C02
inchangés. Réutilisation classify et helper de pente partagé. WorldPos ancré à1e9m,
deux impacts et deux pressions ; termes croisés >1e-6 reçus à1e-7m/s². Fixture
corrigée : confirm reçoit l'époque0, pas le numéro de séquence. Pas de seuil déplacé.
P4 :117 ADR/225 angles/262 leçons ; L260/L262 appliquées, pas de nouvelle
leçon nécessaire. Suite S181-1 cycle vivant avec ce consommateur différentiel.

S180 : master3b7cae0 propre, quatre copies alignées ;115 ADR/225 angles/261 leçons.
Le champ spectral conserve déjà pression et vitesse modales dans Slot. Dériver
depuis phi_t=-g eta-P/rho ; ne pas oublier la pression imposée dans p_dyn profond.
Suite BILAN-S145/S176, code de bibliothèque ; notes antérieures conservées ci-dessous.

P3 S180 : cinq tests nouveaux debug/release, workspace324/cinq ignorés ; C02/C18
inchangés. Source forcée reçue dès la naissance, contre-épreuve sans gradient de
pression détectée. Slot64 octets (+16), chemins incrémental et reliaison reçus.
P4 : rituel, L262,116 ADR/225 angles/262 leçons ; suite S180-1 composition
différentielle B+impacts+pressions, contexte et instant communs.

S179 : master ae28d98 propre, quatre copies alignées ;114 ADR/225 angles/260 leçons.
P4 S179 :115 ADR/225 angles/261 leçons,18 invariants,6 SPEC,23 cas vérifiés.
Suite S180 : S179-1 pression forcée W ; rituel terminé, jeton libre.
P3 S179 : six tests nouveaux debug/release ; workspace319/cinq ignorés ; C18/C02
inchangés. Référence angulaire512/1024, centre et lot reçus ; mutation isotrope rejetée.
Différences finies àh0,002 échouent pression, h0,01/0,005 passent sans changer seuil.
Poursuite de construction B4 (BILAN-S145/S176). Conserver phases, valeurs et refus
historiques ; origine radiale régulière. Notes S178 ci-dessous conservées comme entrée.

Master 4314dc8 propre ; quatre copies alignées, branche historique archivée conservée.
113 ADR,225 angles,259 leçons,18 invariants,6 SPEC,23 cas. S177 :307 tests/cinq ignorés.
Source continue distincte du résidu discret. Conserver eval et phases historiques.
B linéaire profond seulement ; pression relative au plan moyen ADR113, rho explicite.
Dériver avant de coder ; pas de zéro pour un terme non démontré nul.
BILAN-S145 porté par construction B4 après B1/S63-1. Aucune copie créée.
P2 : ADR114 ; gradients et Laplacien représenté, source S à soustraire. cargo check reçu.
P3 : six nouveaux tests, quatorze tests différentiels reçus ; workspace313/cinq ignorés.
Contre-épreuve : advection neutralisée, échec 0 contre0,2578228700 m/s² ; original restauré.
C18/C02 reçus, hashs inchangés ; source mono-mode quadratique et croisements reçus.
P4 : ADR114/L260 ;114 ADR,225 angles,260 leçons,18 invariants,6 SPEC,23 cas.
Suite S179 : différentiel RadialImpact puis composition B+W. Jeton libre après clôture.
