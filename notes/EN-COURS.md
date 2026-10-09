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

Session : S753 — **en cours**. En autonomie ; session longue. ADR-289 D3.3, ADR-291 D4 : **la 3D corrigée contre une référence extérieure**,
les mesures de laboratoire de Synolakis (l'onde solitaire `H/d` = 0,3 qui déferle sur la plage canonique de 1:19,85 ; S712, `references/synolakis/`).

**L'essai** : le montage de S712 (`d` = 0,5 m, 2,5 cm, le pas plafonné à 2,5 ms comme S713 E2), avec la 3D corrigée (ADR-291 : `Complete`,
consciente du fond, R1). Les profils de la surface à t·√(g/d) = 15, 20, 25, comparés aux mesures par `comparer_synolakis_s712` : l'écart
quadratique moyen, la crête mesurée contre la calculée.

**Le témoin** : S713 E2, la 3D sans correction au même pas :

| t | la crête mesurée | S713 E2, la 3D sans correction | l'écart quadratique |
|---|---|---|---|
| 15 | 0,314 d en 8,38 d | 0,424 d en 8,07 d | 0,044 d |
| 20 | 0,318 d en 3,66 d | 0,316 d en 3,77 d | 0,066 d |
| 25 | 0,190 d en 0,30 d | 0,218 d en −3,03 d | 0,035 d |

**Les critères, écrits avant** :
1. **la crête à t = 15 à 0,05 d de la mesure** (sans correction, 0,11 d de trop : l'onde trop haute de S713) ;
2. **l'écart quadratique plus petit que S713 E2** à au moins deux des trois instants.

**Le repos** (ADR-290 D3) : la 3D corrigée tient le repos sur l'escalier à 1:30 et 1:12 (S750) ; la plage de 1:19,85 est entre les deux.

**Contrôles du plan** (ADR-280, ADR-287, ADR-289, ADR-291)

- **témoin** : les mesures de Synolakis (la référence extérieure, ADR-280 D2) ; S713 E2 (la 3D sans correction).
- **instrument** : `comparer_synolakis_s712`, éprouvé en S712–S713 (le profil par la surface, lissé sur 10 cm).
- **calcul** : ≈ 45 min, en arrière-plan.
- **ADR** : ADR-280 D2 (une référence extérieure tranche) ; ADR-289 D3.3 ; ADR-291 D4.
- **pièges** :
  - les mesures portent une onde générée par un batteur, amortie sur le trajet (le NOAA le dit) : un écart restant n'est pas
    forcément celui de la 3D ;
  - à t = 25, la lame monte sur le sable sec : le pas tombe (S712).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la configuration dans le montage ; l'essai ; (1), (2).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
