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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S683 — **en cours**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage (ADR-271).

**Ce que la session fait.** **L'entrée à droite** (`feed_right`, avec la sortie de S682) :

- un volume donné par rangée s'ajoute à un **réservoir** ;
- chaque quantum entier (`dx³/8`) devient une particule posée dans la dernière colonne, sur le réseau au quart de maille, à la place
  la moins occupée sous la surface ;
- la particule reçoit la vitesse d'entrée.

La masse se compte : particules × quantum + réservoir.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-272)

- **témoin** : sans entrée, au bit (S682 et tous les essais d'APIC).
- **instrument** : le bassin de S682, l'eau à 0,2 m, nourri à 0,1 m/s sur la hauteur mouillée du bord droit pendant 1 s (le volume de
  chaque pas lu comme en S682). Ce que rendrait chaque hypothèse :
  - une entrée juste rend le bilan exact (particules × quantum + réservoir = départ + entré, à 10⁻¹² près) et la colonne du bord peu
    tassée ;
  - des particules posées au même endroit (une place mal choisie) donnent une colonne surchargée et une vitesse parasite.
- **calcul** (ce script) : ≈ 2.0 L, 128 particules ; le niveau monte de 10 mm, sous le quantum d'une couche de
  particules (25 mm) : le niveau ne départage pas, le volume oui.
- **ADR** : ADR-271.
- **pièges** :
  - la capacité réservée (une particule refusée est comptée, et son quantum reste au réservoir) ;
  - le fond (une place sous le fond n'est pas une place) ;
  - le réservoir négatif (le volume donné est positif ; la sortie est celle de S682).

**Critères, écrits avant.**

1. Sans entrée, au bit.
2. Le bilan exact à 10⁻¹² près.
3. La dernière colonne à au plus 10 particules par maille mouillée.
4. La vitesse maximale sous 0,5 m/s : l'entrée à 0,1 m/s, plus l'onde qu'elle lance (`√(g·h)·Δh/h` ≈ 0,07 m/s).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'entrée ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
