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

```
Session          : S22
État             : en cours
Battement        : 2026-09-06
Objectif         : C01 — le repos hydrostatique sur pente, et le premier δ
```

### Plan

C01 est décrit dans `CAS-CANONIQUES` comme « le test le moins spectaculaire, le plus rapide, et
celui qui élimine le plus de candidats ». Il n'a besoin d'aucune houle, d'aucun corps, d'aucun
réseau : de l'eau au repos sur un fond incliné, et la question de savoir si elle y reste.

**Ce que cette session ne fait pas.** Elle ne choisit pas le solveur δ du projet — ce choix est le
banc **B3**, et ADR-007 §5 liste cinq candidats sans en privilégier aucun. Ce qui est écrit ici est
un **véhicule d'essai**, étiqueté comme tel, exactement comme `background.rs` l'est pour `B` : il
donne à C01 quelque chose à faire tomber. Le livrable durable est le **cas**, pas le solveur.

*Thèse déclarée avant l'exécution : le schéma évident échoue C01.* Le gradient de pression et le
terme de fond sont deux discrétisations différentes de la même quantité ; sur un fond incliné elles
ne s'annulent pas, et l'eau au repos se met à couler. Si la thèse est fausse, c'est mon montage qui
est trop facile, pas le schéma qui est bon — et il faudra le dire.

- [x] **P1** — plan, jeton.
- [x] **P2** — `delta.rs` : grille 1D, état conservatif `(h, hu)`, flux de Rusanov, pas de temps
      CFL. Fond plat d'abord, où le repos est trivialement exact. Test de repos sur fond plat.
- [x] **P3** — le terme de fond au premier jet, la pente 1:20, et C01 branché dans le mode
      `physics` : `max|u|` et `max|η − η₀|` mesurés sur le champ après 60 s.
- [x] **P4** — exécuter, constater, **mesurer** l'amplitude du courant parasite. Un chiffre, pas
      une impression.
- [x] **P5** — reconstruction hydrostatique (Audusse) : le schéma équilibré. Réexécuter, comparer
      les deux chiffres dans le même rapport.
- [x] **P6** — **ADR-030** : ce que C01 a appris, et pourquoi « équilibré sur fond variable » est un
      critère d'**élimination** pour B3, connu avant le banc et non découvert pendant.
- [x] **P7** — répercussions : `CAS-CANONIQUES`, `cas_en_attente()`, index, angles morts, notes
      correctives, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

#### Le dépôt a forké une seconde fois — constaté à l'ouverture de S22

`git worktree list` et `git branch -a`, les deux commandes qu'`CLAUDE.md` impose, ont montré ceci :

| Ligne | Sessions | Contenu propre |
|---|---|---|
| `master` (et `claude/reprise-projet-s22-715339`) | S08 → **S17** | la fusion S16, qui a importé l'autre ligne jusqu'à S15 |
| `claude/reprise-projet-5134cd` | S08 → **S21** | ADR-027 à ADR-029, `code/`, H1 et H3 |

Point de divergence commun : `8fe1503` (S07) — **le même fork que `FORK-S08-S15.md` décrit**, jamais
refermé du côté git. La fusion de S16 a été faite **par import de contenu**, pas par un merge : la
ligne source ne l'a donc jamais reçue et a continué seule pendant quatre sessions.

**Ce que S22 a fait :** repartir de `a6cfe6f` (S21, la ligne la plus avancée et la seule dont le
`REPRISE.md` annonce S22) sur une branche `claude/s22-suite`, **sans rien réécrire**. `master` est
intact.

**Ce qui reste à trancher, et qui n'est pas à moi :** que faire du travail propre à `master`,
S16-S17 — la carte de renumérotation, la revue de cadence sur les documents importés. Il n'est pas
perdu ; il n'est pas non plus dans la ligne vivante. Angle mort à enregistrer en P7.

#### P4 — la thèse était juste, et le défaut est du premier ordre exact

**C01 tombe au premier passage**, sur le montage nominal `dx = 0,25 m` :

| Grandeur | Mesure | Tolérance | Dépassement |
|---|---|---|---|
| `max\|u\|` après 60 s | **19,6 mm/s** | 1 mm/s | **×20** |
| `max\|η − η₀\|` | **7,3 mm** | 1 mm | **×7** |
| volume | 80,000003 m² | 80 m² | passe à 4·10⁻⁸ près |

**Le volume passe, et c'est ce qui rend le diagnostic sûr** : le solveur ne fuit pas, il *remue*.
L'eau est déplacée d'un bout du bassin à l'autre par un courant qui n'a aucune cause physique.
C'est exactement le symptôme décrit dans `CAS-CANONIQUES` — « un lac qui frissonne sans raison ».

**Balayage en résolution** — `cargo test -p water-core courant_parasite -- --nocapture` :

```
dx = 1,0000 m   max|u| = 0,076033 m/s
dx = 0,5000 m   max|u| = 0,038771 m/s
dx = 0,2500 m   max|u| = 0,019581 m/s
dx = 0,1250 m   max|u| = 0,009799 m/s
dx = 0,0625 m   max|u| = 0,004895 m/s
```

`max|u| / dx` vaut 0,0760 · 0,0775 · **0,0783 · 0,0784 · 0,0783** — constant sur les trois grilles
fines. **Le courant parasite est du premier ordre exact en `dx`**, et le coefficient a une valeur :
`C ≈ 0,0783 s⁻¹`.

**Le chiffre qui décide.** Atteindre 1 mm/s par raffinement seul demanderait
`dx = 10⁻³ / 0,0783 = 12,8 mm`, soit **19,6 fois plus fin** que le montage nominal. En 2D, le coût
va comme `dx⁻²` en cellules et `dx⁻¹` en pas de temps (CFL) : **×7 500**. Il n'y a pas de
raffinement qui rachète un schéma non équilibré — c'est un défaut de *nature*, pas de *finesse*.

**Ce que la cause n'est pas.** Le terme de fond centré n'est pas coupable ici. Sur un fond
**linéaire** et une surface plane, `h` varie linéairement, et la différence centrée du flux de
pression `g·h²/2` égale exactement `g·h·∂b/∂x` — les deux se compensent par construction. Le
coupable est la **diffusion de Rusanov** : `−α/2·(h_R − h_L)` porte sur la hauteur, qui varie le
long d'une pente **même quand l'eau est parfaitement immobile**. Le schéma diffuse un saut qui
n'est pas un saut d'écoulement mais un saut de géométrie.

C'est le point non anticipé de la session : j'attendais le terme source, et c'est le flux.

#### P5 — deux défauts se superposaient, et le plus gros était dans la condition aux limites

**Correction à porter sur ce qui est écrit plus haut : le diagnostic de P4 était faux.** Il
attribuait les 19,6 mm/s à la diffusion de Rusanov. La vraie cause principale était ailleurs.

`bords()` recopiait la **hauteur d'eau** dans la cellule fantôme — `h[0] = h[1]`, le miroir évident.
Sur un fond en pente, le lit de la fantôme n'est pas à la cote de sa voisine : recopier la hauteur
y installe une surface libre décalée de `dx·pente`, c'est-à-dire **une marche d'eau permanente
contre chaque mur**, qui se vide dans le domaine dès le premier pas. Le miroir juste porte sur la
**surface libre** : `h_fantôme = η_interne − b_fantôme`.

Le symptôme qui l'a révélé n'était pas le courant : c'est le **schéma équilibré qui perdait 1,1 %
de volume** alors que son intérieur est exact par construction. Une propriété exacte qui donne un
résultat faux ne laisse qu'une possibilité — l'erreur est en dehors de ce qu'elle couvre.

> **Un intérieur équilibré et un bord qui ne l'est pas donnent un solveur non équilibré.** La
> propriété ne se découpe pas, et c'est le genre de chose qu'on n'écrit dans aucun ADR parce qu'elle
> paraît évidente une fois dite.

**Après correction du bord — les deux schémas, à 60 s :**

| `dx` | jet : `max\|u\|` | jet : `max\|η−η₀\|` | équilibré : `max\|u\|` | équilibré : `max\|η−η₀\|` |
|---|---|---|---|---|
| 1,0000 m | 1,98 mm/s | 85,0 mm | 0,0018 mm/s | 0,00048 mm |
| 0,5000 m | 1,04 mm/s | 43,0 mm | 0,0026 mm/s | 0,00048 mm |
| **0,2500 m** | **0,53 mm/s** | **21,6 mm** | **0,0068 mm/s** | **0,00072 mm** |
| 0,1250 m | 0,29 mm/s | 10,8 mm | 0,0062 mm/s | 0,00119 mm |
| 0,0625 m | 0,10 mm/s | 5,5 mm | 0,0070 mm/s | 0,00131 mm |

**Trois constats, dans l'ordre d'importance.**

1. **Le premier jet passe `max|u|` et échoue `max|η−η₀|`.** 0,53 mm/s contre 1 mm/s admis — il
   aurait été déclaré conforme par un cas qui n'aurait mesuré que la vitesse. C'est la seconde
   assertion de C01 qui le fait tomber, avec 21,6 mm pour 1 mm. **Les deux assertions de C01 ne
   sont pas redondantes**, et rien dans l'énoncé ne le disait.
2. **Le défaut résiduel du premier jet est du premier ordre exact en `dx`** : `max|η−η₀|/dx` vaut
   0,0850 · 0,0860 · 0,0864 · 0,0862 · 0,0875. Atteindre 1 mm par raffinement seul demanderait
   `dx = 11,4 mm`, soit **×21,9**, soit **×10 500** en coût 2D. Le raffinement ne rachète pas
   l'équilibrage — conclusion inchangée depuis P4, sur une autre grandeur.
3. **L'erreur du schéma équilibré ne dépend pas de `dx`** : elle reste entre 0,0005 et 0,0013 mm,
   et l'ulp d'un `f32` à 3 m vaut 0,00024 mm. **C'est le bruit d'arrondi, pas une erreur de
   discrétisation.** Le repos est préservé algébriquement, et il le serait sur trois cellules.

**Ce qui reste à savoir, et que cette session ne sait pas.** Le montage de C01 a un fond à pente
**constante**. Sur un tel fond, `h` varie linéairement, le saut de hauteur aux interfaces est le
même partout, et sa divergence est donc presque nulle — le premier jet y est *presque* équilibré
par accident de géométrie. Un fond **courbe** (la bosse parabolique classique) le ferait tomber bien
plus lourdement. **C01 tel qu'énoncé est moins discriminant qu'il n'en a l'air** : angle mort à
enregistrer, et proposition d'un C01-bis à fond courbe.

**Le témoin.** Le premier jet reste exécuté à chaque passage, sous le statut `TÉMOIN`, et **n'est
pas compté dans les échecs** — une batterie rouge en régime nominal est une batterie que personne
ne lit. Sa sémantique est inversée, pas suspendue : le verdict est **agrégé** — s'il venait à passer
*toutes* les assertions de C01, la batterie le signale comme anomalie. C'est le seul moyen de savoir
que le montage a cessé de discriminer.
