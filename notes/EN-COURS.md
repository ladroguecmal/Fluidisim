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

Session : S748 — **en cours**. En autonomie ; session longue. ADR-289 D3.1 ; DENSITE-HYBRIDE-S747 : la projection hybride garde le repos et
l'onde solitaire, mais freine la remontée de S645 (−12 %). **La question** : où et quand la projection freine-t-elle la lame ? **Un
diagnostic, aucun remède** (ADR-226 D1 ; ADR-287 D4 : regarder le champ).

**L'essai** (`ressaut_s748`) : le montage de S645 (2,5 cm, l'escalier, l'air balistique), deux fois, sans projection puis avec la
projection hybride consciente du fond. Tous les 0,05 s :
- **le front par les particules** (S737) ;
- **l'énergie de l'eau au-delà du pied moins 0,5 m** : `Σ (½·|v|² + g·z)·quantum` ;
- **le plus grand déplacement imposé par la projection au dernier pas** (`density_projection_shift`) ;
- tous les 0,25 s, **le profil de la surface** (la plus haute particule par colonne), écrit dans `calculs/s748_*.csv` et tracé.

**Le critère du diagnostic, écrit avant** : la cause est localisée si l'on peut dire
- (a) **l'instant** où les fronts divergent de plus de 1 cm ;
- (b) si, à cet instant, **l'énergie** de la lame avec projection est plus basse (de plus de 2 %) ;
- (c) **où** la projection déplace le plus les particules à ce moment : dans le ressaut, ou ailleurs.

Sinon, la question reste ouverte, et les lectures sont rapportées.

**Contrôles du plan** (ADR-226, ADR-286, ADR-287, ADR-288)

- **témoin** : le même montage sans projection (S645 redonné au millimètre en S738).
- **instrument** : le front par les particules, éprouvé en S737 ; l'énergie, une somme sur les particules ; le déplacement, rendu par le
  cœur.
- **calcul** : deux fois ≈ 6 min.
- **ADR** : ADR-226 D1 (localiser d'abord) ; ADR-287 D4 ; ADR-286 D2 (deux lectures du front : les particules, et les profils regardés).
- **pièges** : l'énergie potentielle dépend de la référence de `z` ; seules les différences entre les deux calculs comptent.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'essai ; les deux calculs ; les profils tracés ; (a), (b), (c).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
