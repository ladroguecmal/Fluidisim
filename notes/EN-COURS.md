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

Session : S482 — **en cours**. En autonomie (ADR-215, ADR-222), **K2-2b — le coût des poches** ([POCHES-CARTE-S481](../docs/validation/POCHES-CARTE-S481.md)
§4 : 71 ms par pas avec poches contre 16,7 sans, sur `--v1`). Avant le plan, sur la réponse de l'utilisateur (*« non pour la 1 sinon
oui »*) : pas de relance planifiée ; les téléchargements faits (Godot 4.7 mono double de DyingStar, DyingStar `develop`) dans
`C:/Users/antoi/FluidisimExterne/` ; DyingStar est passé à Godot 4.7 (ADR-219 et ADR-222, notes ; boussole).

**Ce que la session fait.** (1) **Rien quand rien n'est enfermé** : les racines enfermées se comptent en parallèle ; sans elles, les
parcours par un groupe (numérotation, listes, faces) ne parcourent rien. (2) **Les parcours à plusieurs groupes** : compte par bloc de
256 mailles, préfixe des blocs, écriture — le même ordre des mailles, donc le même résultat. (3) **Les poches dans la multigrille** :
un préconditionneur par blocs — le cycle en V sur les mailles, la diagonale sur les poches —, défini positif ; les noyaux fusionnés du
gradient conjugué de la multigrille, dans des variantes du module des poches.

**Entrées, et comment elles se vérifient.** Le témoin `--v1` sans poches (S481 : 16,7 ms, 60 s, masse exacte) et la bulle de S481
(`MODE=suivi`, 4·10⁻⁵, 42,47 Hz) : rejoués avant et après chaque étape avec le même binaire ; les étages par `V1_LENTS`.

**Critères, écrits avant.** (1) sans poches, le témoin inchangé (pas médian à 5 %, masse exacte) ; (2) la bulle carte contre référence
toujours à 10⁻⁴ (volume, pression), sa fréquence à 10⁻³ ; (3) `--v1` avec poches : **pas médian à 20 % du témoin** (≤ 20 ms), 60 s
stables, masse exacte ; (4) la détection identique (banc `--apic3d-poches`, 0 écart à étiquettes égales). Ce qui ne tient pas est dit.

### Plan

- [x] **P1** — jeton, plan seul ; les réponses et les téléchargements consignés.
- [x] **P2** — rien quand rien n'est enfermé ; mesure.
- [ ] **P3** — les parcours à plusieurs groupes ; (4) ; mesure.
- [x] **P4** — les poches dans la multigrille ; (2) ; mesure.
- [ ] **P5** — `--v1` 60 s (1), (3) ; preuve.
- [ ] **P6** — rituel (par `rituel.py`).

### Notes de reprise
- **P2** — `H_ANY` : l'aplatissement compte les racines enfermées ; `pk_number`, `pk_lists` sortent aussitôt sans elles, `pk_faces`
  sans poche gardée. Détection inchangée (banc : 0 écart à étiquettes égales, mêmes écarts V/P qu'en S481). `--v1` avec poches, 6 s :
  reconstruction **2,7 ms** en moyenne (≈ 12 avant), projection 10,9 ms ; pas médian 31 ms (71 sur 60 s en S481, à remesurer sur 60 s).
- **P4** (avant P3 : la projection était le gros du coût) — `pk_mg_init_finish`, `pk_mg_update_alpha`, `pk_mg_beta_direction` (deux
  parités) : le cycle en V sur les mailles, la diagonale sur les poches. Bulle, 300 pas : volume 2,9·10⁻⁵, pression 4,1·10⁻⁵, **42,47 Hz
  contre 42,50**, **14 itérations au plus** (151 en diagonale), masse exacte — critère (2) tenu. `--v1` avec poches, 6 s : pas médian
  **14,9 ms** (projection 5,6, reconstruction 2,6). `MULTIGRILLE=0` au banc : la diagonale de S481.
