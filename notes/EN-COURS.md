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

Session : S532 — **terminée**. En autonomie : **ADR-235 D1 appliquée à la carte** — `Linear3::new` s'arrêtait (une assertion, une
panique de wgpu) au-delà de ses limites matérielles ; elle les calcule et refuse avec un nom.

**Ce que la session fait.** Après l'obtention de l'adaptateur : les groupes des noyaux de mailles (un par 64 mailles, en une dimension)
contre `max_compute_workgroups_per_dimension` ; chaque tampon (vitesses, géométrie, mouvement, paires de la dispersion, état, colonnes,
relecture) contre `max_storage_buffer_binding_size` et `max_buffer_size` — `Err` nommant la limite et la grandeur. Un banc
`--lineaire-limites` : un domaine au-delà, un en deçà.

**Ordre de grandeur, calculé.** 512 × 512 × 17 = 4,456,448 mailles → 69,632 groupes (> 65 535) : refusé ; 448 × 320 × 24 =
3 440 640 → 53 760 groupes, le plus gros tampon (les paires, ≈ 16 octets par maille et par face) ≈ 223 Mo : accepté sous les limites de
l'adaptateur (relevées en S529).

**Critères, écrits avant.** (1) Le domaine au-delà est refusé par une erreur nommée, sans arrêt ; celui en deçà accepté. (2) Les bancs de
la carte inchangés au bit (sillage `e41630abd739b189`, S503, S504).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les contrôles, le banc ; (1), (2).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — 4 456 448 mailles refusées (69 632 groupes, nommé) ; 3 440 640 acceptées ; sillage, S503, S504 au bit.
- **P3** — preuve LIMITES-CARTE-S532 ; index ; journal.

