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
Session          : S23
État             : en cours
Battement        : 2026-09-06
Objectif         : C04 — la rupture de barrage (Ritter), et le lit sec
```

### Plan

C01 a testé le solveur sur son état le plus trivial : rien ne bouge. **C04 le teste sur le plus
violent** — une colonne d'eau lâchée sur un lit sec, une discontinuité à `t = 0`, et une solution
analytique complète pour toute la suite (Ritter). Entre les deux, il n'y a pas de degré : ce sont
les deux extrémités de ce qu'un solveur d'eau peu profonde doit savoir faire.

**Ce que C04 attaque et que C01 ne touchait pas** : le front de mouillage sur lit sec. C'est là que
les schémas produisent une hauteur négative, un front trop lent, ou les deux.

*Thèse déclarée avant l'exécution : le front sera trop lent, et la mesure de sa position dépendra
du seuil qui la définit plus que du schéma.* La première moitié est un défaut connu des schémas
d'ordre 1 ; la seconde est le vrai risque de la session — **mesurer la position d'un front, c'est
choisir à quelle hauteur d'eau on décrète qu'il commence**, et ce choix n'a pour l'instant aucune
provenance (`H_SEC = 10⁻⁶` est posé sans justification, action **S22-4**).

- [x] **P1** — plan, jeton.
- [x] **P2** — le montage : canal plat, lit sec à droite, marche à `t = 0`. Les murs sont hors de
      portée du signal à `t = 2 s` — le front avance de 12,5 m, la raréfaction remonte de 6,3 m —
      donc aucune condition transmissive n'est nécessaire, et c'est à vérifier plutôt qu'à supposer.
- [x] **P3** — la solution de Ritter comme référence, et **la définition du front** : à seuil `ε`
      dans le champ, comparée à la position où Ritter vaut `ε` — et non à `2√(gh₀)·t`. Comparer une
      mesure à seuil contre une référence sans seuil mesurerait la définition, pas le schéma.
- [x] **P4** — exécuter, constater, mesurer la sensibilité de la position du front à `ε`.
- [ ] **P5** — **S22-4** : donner une provenance à `H_SEC`, ou le remplacer. C04 est le cas qui le
      met en jeu ; le laisser posé au jugé après l'avoir traversé serait la dette exacte que
      décrit A106.
- [ ] **P6** — ce que la session a appris : ADR-031 si la conclusion engage B3, note datée sinon.
- [ ] **P7** — répercussions : `CAS-CANONIQUES`, `cas_en_attente()`, index, angles morts, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S22 laisse et qui vaut pour ici.** Le témoin `C01-jet` doit rester rouge — s'il passe, le
harnais le signale comme anomalie. Le véhicule δ n'a pas de friction : sans effet sur C04, dont
l'énoncé pose explicitement « canal plat **sans frottement** », donc l'action **S22-3** n'est pas un
prérequis de cette session. Elle le redevient pour C03.

**Branche.** `claude/s22-suite`, issue de `a6cfe6f`. `master` s'arrête à S17 et diverge depuis
`8fe1503` (S07) — voir A107. Vérifié à l'ouverture de S23 : rien n'a bougé ailleurs.

#### P4 — la thèse tenait des deux mains, et la seconde moitié est la plus intéressante

**Le solveur est bon partout, sauf au front.** À `dx = 0,05 m`, `t = 2 s` :

| Grandeur | Mesuré | Ritter | Écart | Tolérance |
|---|---|---|---|---|
| `h` au droit du barrage | 0,45466 m | 0,44444 m | **2,30 %** | 3 % |
| `u` au droit du barrage | 2,0323 m/s | 2,0881 m/s | **2,67 %** | 3 % |
| **erreur L1 sur tout le domaine** | — | — | **0,82 %** | 3 % |
| **front, à `ε = 1 mm`** | 10,013 m | 11,934 m | **−16,09 %** | 3 % |

L'erreur globale vaut **0,8 %** et l'erreur au front **16 %** : un facteur vingt. Le défaut est
entièrement **local au front de mouillage** — ce que C04 est précisément fait pour attraper, et ce
qu'aucune des trois autres mesures n'aurait révélé seule.

**Balayage seuil × résolution** — `cargo test -p water-core retard_du_front -- --nocapture` :

```
nx=200   dx=0,2000  ε=1e-4 : −19,91 %   ε=1e-3 : −21,11 %   ε=1e-2 : −17,92 %
nx=400   dx=0,1000  ε=1e-4 : −19,13 %   ε=1e-3 : −19,34 %   ε=1e-2 : −14,55 %
nx=800   dx=0,0500  ε=1e-4 : −16,58 %   ε=1e-3 : −16,09 %   ε=1e-2 : −10,33 %
nx=1600  dx=0,0250  ε=1e-4 : −13,48 %   ε=1e-3 : −12,54 %   ε=1e-2 :  −6,39 %
nx=3200  dx=0,0125  ε=1e-4 : −10,83 %   ε=1e-3 :  −9,46 %   ε=1e-2 :  −3,46 %
```

**Trois lectures, et la troisième est celle qui compte.**

1. **Le front est toujours en retard, jamais en avance.** C'est cohérent : dépasser `2c₀·t`
   violerait la vitesse de propagation maximale du problème. Le test l'assure explicitement.
2. **La convergence est là, mais elle est très lente.** À `ε = 10⁻³`, seize fois plus de cellules
   font passer l'erreur de 21 % à 9,5 % — un facteur 2,2 pour un facteur 16. L'ordre apparent est
   d'environ **0,3**, alors que le schéma est d'ordre 1 ailleurs et que l'erreur L1 globale, elle,
   se comporte normalement. **Le front a son propre ordre de convergence, et il est mauvais.**
3. **Le retard dépend du seuil**, et pas dans le sens qu'on croirait : il est *plus faible* à
   `ε = 10⁻²` qu'à `ε = 10⁻⁴`. C'est la signature d'un front **étalé** : le profil numérique
   rejoint Ritter dans son corps et traîne une queue mince. Mesurer haut sur le profil donne raison
   au solveur, mesurer bas lui donne tort. **Le choix du seuil ne déplace donc pas seulement la
   référence — il change le verdict**, et rien dans l'énoncé de C04 ne dit lequel prendre.

**Ce que le témoin montre.** Le front mesuré comparé à `2c₀·t` donne −20,07 %, contre −16,09 % au
même seuil. **Quatre points d'écart sont de la pure convention de mesure** — un quart du verdict.
