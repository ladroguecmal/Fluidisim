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

Session : S259 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-16 23:06) : « continue ». Master propre fdc0f1c, copie unique, maillons 2 : cette
session doit livrer une capacité. Lot recommandé en S257 d'après le verdict R2 (« grand lac soumis
au vent ») et A287 (stries : direction liée au rang de fréquence).

Objectif : B **multimodal** — plusieurs systèmes (houle longue + mer de vent), chacun avec sa
bande, son pic et une **loi d'étalement directionnel** (cos^2s, s selon f/fp, Mitsuyasu ;
s_max de Goda), directions tirées **indépendamment du rang de fréquence**. Queue d'ADR-155 selon la
même loi. **Aucune migration silencieuse** : `bake` V1, son empreinte figée et la scène par défaut
restent au bit ; la scène multimodale est une variante déclarée (`--houle`) de l'afficheur.

Réception écrite avant code (ADR-156, protocole) : amplitudes, `k` et fréquences identiques au bit
à `bake` par système ; `E[cos θ] = s/(s+1)` pour l'inverse de la loi ; décorrélation rang/direction
bornée par 3/√N ; `Hs` total = √Σ Hs² ; hauteurs GPU contre cœur ≤ 3 mm sur la scène `--houle` ;
scène par défaut identique (R2 rejoué au bit, `--tail-verify` inchangé) ; coût GPU ; rendus R3.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — SPEC-001 §1 septies (Mitsuyasu, Goda, moment `s/(s+1)`), ADR-156, protocole
  MER-MULTIMODALE-S259, avant code.
- [x] **P3** — cœur : loi d'étalement et son inverse, `bake_directional`, queue directionnelle,
  assemblage de systèmes ; essais.
- [x] **P4** — hôte : nombre de composantes de B variable, scène `--houle`, vérification CPU/GPU.
- [ ] **P5** — réception : hauteurs, scène par défaut au bit, coût, rendus R3 envoyés.
- [ ] **P6** — rituel §6 : liste du projet fini (2.2), A287, file, journal, jeton.

### Notes de reprise

P3 : `bake_directional`, `bake_tail_directional`, `assemble`, `spread_offset_turns` (table de répartition
1 024 trapèzes, densité coupée au-delà de exp(−32) — `decay` n'accepte que 0..=32, premier jet en
dépassement). Essai `multimodal_sea_and_directional_spreading_s259` : E[cos] 0,23078/0,50000/0,90910/
0,98684 contre 0,23077/0,5/0,90909/0,98684 ; Spearman 0,012 (fixture 1,000, borne 0,530) ; largeur
0,063 tour au pic (7 comp.) contre 0,155 au-delà de 2 fp (11) ; m0 0,390625 exact, Hs 2,5 m. Spectre et
queue au bit ; empreinte V1 et essais du spectre 8/8.

P4 : `Scene::build(houle)`, `B_CAPACITY` 64, nombre de composantes passé au shader (était 32 en dur),
`--houle`, `--b-verify`, `--revue=r3`. `--multi --houle --b-verify` : 64 composantes, max η 0,379 mm
(âges 3/12/29) ; défaut 32 composantes, 0,368 mm. R2 rejoué : sept empreintes identiques ;
`--tail-verify` ligne identique à S256.
