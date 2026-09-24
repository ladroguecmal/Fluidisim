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

Session : S348 — **en cours**. **Porte C, le pas étalé sur deux images** ; chemin de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — **verdict R17** de l'utilisateur sur les images de S347 : *« Continue je valide »* — la cadence de 30 Hz
d'ADR-012 §7 est validée à l'œil ; l'écart d'amplitude d'A318 n'y fait pas obstacle.

**Ce que la session doit rendre possible.** Le critère de la porte C
([ADR-175](../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.4) : **δ ≤ 2 ms GPU au 99ᵉ centile de la
contribution par image**, sur la scène de la porte B, techniques présentes et absentes publiées. À 30 Hz, un pas pour
deux images de 60 Hz : le pas de 3,68 ms (S343) coupé en **deux parts**, l'une par image — la première passe (fond,
prédiction, couplage, 1,30 ms), la mise en route de la projection et `k` cycles ; puis les `32 − k` autres, le
résidu et la troisième passe (0,29 ms). À 0,062 ms le cycle, `k` = 7 équilibre les parts autour de 1,83 ms.

Critères, écrits avant le code :
1. **R17** consigné : registre, preuve, A318, file.
2. **Le pas coupé, au bit** : sur la scène de B à 30 Hz, 60 pas exécutés en deux parts donnent la surface publiée
   identique au bit à 60 pas d'un seul tenant.
3. **Le coût par image** : 1 000 pas, chaque part horodatée seule, secteur relevé, témoin ; **99ᵉ centile de chaque
   part ≤ 2 ms** ; techniques présentes et absentes (ADR-131 D3).
4. Si 3 tient : **la porte C reçue** sur ce banc, avec le domaine du chiffre — sans rendu concurrent, sans
   l'interpolation du rendu qu'ADR-012 §7 demande, qui reste à faire. Feuille de route, file, liste.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le verdict R17 ; critère 1.
- [x] **P3** — le pas en deux parts, et l'identité ; critère 2.
- [x] **P4** — le coût de chaque part ; critère 3.
- [ ] **P5** — preuve ; la porte C si elle tient ; critère 4.
- [ ] **P6** — rituel.

### Notes de reprise
- **P3, critère 2 tenu.** `encode_split` : partie 0 — première passe, copies, mise en route de la projection, `k`
  cycles ; partie 1 — le reste, le résidu, la troisième passe ; `step_part` et `timed_part`. **60 pas à 30 Hz en
  deux parts : surface identique au bit** au pas d'un seul tenant (13 440 colonnes) ; empreintes du pas entier de
  S343 inchangées (`0x9325cf58781f8b74` à 600 pas).
- **P4, critère 3 tenu** (`--delta3d-deux-parts`, secteur 97 % avant et après). `k` = 7, 1 000 pas : **partie 0
  médiane 1,801 ms, q99 1,848, max 1,859 ; partie 1 médiane 1,874, q99 1,918, max 1,934**. Balayage, 200 pas :
  k 5 → 1,760 / 2,040 (q99) ; 6 → 1,872 / 2,059 ; **7 → 1,899 / 1,924** ; 8 → 1,968 / 1,857 ; 9 → 2,021 / 1,797 —
  7 équilibre.

