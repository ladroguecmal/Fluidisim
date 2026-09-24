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

Session : S340 — **en cours**. **Porte B** : le verdict R16, puis le critère 2 sur la production, cas 1 et 2 ;
chemin de la v1 ([ADR-174](../docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4), porte en cours de §3 bis.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — **verdict R16** de l'utilisateur sur les images de S339 : *« Tout parrait bon visuellement »*. Critère 3
d'[ADR-175](../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4 : reçu. Restent, pour la porte B,
les cas 1 et 2 du critère 1 **sur la production** — critère 2, *« production contre référence, sur les mêmes cas :
écart de hauteur sous 3 mm, pente et phase publiées »* ; S305 n'a mesuré que le cas 3
([CUVE-GPU-S305](../docs/validation/CUVE-GPU-S305.md) §7). Le critère 4, le coût, relève de la porte C.

**Ce que la session doit rendre possible.** La porte B reçue, si les deux cas tiennent. **Ce qui contraint le
cas 1** : la production n'évalue que le fond de B — des composantes progressives, en eau profonde —, et le cas 1
de S297 couple la référence à une onde stationnaire analytique en eau finie (`L` = `h` = 2 m, `k·h` = π). Le fond
du cas 1 sera donc **la même onde stationnaire faite de deux composantes de B** opposées, pour la référence comme
pour la production ; l'écart de ce fond à l'analytique est publié, et la référence sur ce fond est rejouée contre
HOS.

Critères, écrits avant le code :
1. **R16** consigné : critère 3 reçu — registre, preuve, feuille de route, file.
2. **Un fond de B par ses composantes** (`Background::from_components`) : refus d'une liste vide, allocation
   demandée à l'hôte comme `configure` ; essai du cœur.
3. **Cas 2 sur la production** : une composante de B le long de `x` (direction (1, 0) exacte), une crête initiale
   invariante en `y`, `ny` = 8, 2 s. La production reste **invariante en `y`** à quatre ulps du repos près, et
   suit la référence **sous 3 mm** ; pente publiée.
4. **Cas 1 sur la production** : `ny` = 1, géométrie de S297 (murs, boîte de 2,25 m), 5 et 10 cm, `nx` 32, 64 et
   128 si la durée le permet, une période. Publiés : l'écart du fond de B à l'onde analytique ; la référence sur ce
   fond contre HOS (tolérances de S253 : profil < 2 %, harmonique < 20 % à 128) ; **la production contre la
   référence, sous 3 mm**, pente et phase.
5. Si 3 et 4 tiennent : **porte B reçue** (critères 1 à 3 ; le 4 à la porte C) — feuille de route, file, liste.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le verdict R16 : critère 3 reçu ; critère 1.
- [ ] **P3** — `Background::from_components` et son essai ; critère 2.
- [ ] **P4** — le cas 2 sur la production ; critère 3.
- [ ] **P5** — le cas 1 : fond de B stationnaire, chaînon et HOS sur la référence.
- [ ] **P6** — le cas 1 : production contre référence ; critère 4.
- [ ] **P7** — preuve ; la porte B, si elle tient ; critère 5.
- [ ] **P8** — rituel.

### Notes de reprise
