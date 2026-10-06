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

Session : S512 — **en cours**. En autonomie, **6.8, l'impulsion d'entrée dans l'eau** (*slamming*, C20, ADR-023 §2) — absente : une
entrée dans l'eau dure moins d'un tick (17 à 73 ms) ; la flottabilité échantillonnée la rate ou la double selon la phase du tick.

**Ce que la session fait.** Le corps rigide reçoit un archétype d'impact (relèvement `β`, demi-largeur `b`, longueur `L`, seuil de
2 m/s) ; à chaque pas, l'instant **exact** où sa quille passe sous la surface (chute libre résolue dans le pas), et, au-delà du seuil,
l'impulsion de masse ajoutée appliquée à cet instant : `m_a = ½πρb²L`, la quantité de mouvement du corps et de l'eau entraînée conservée —
`v' = m·v/(m + m_a)`, `J = m_a·v'` (→ `m_a·v` d'ADR-023 quand `m_a ≪ m`) ; un événement publié (instant, `J`, `v_rel`, `t_impact`).

**Ordre de grandeur, écrit avant.** La coque de la porte D lâchée à plat (`b` = 0,8 m, `L` = 4 m) : `m_a` = ½·π·1025·0,64·4 ≈ 4 100 kg —
plus que sa masse (3 200 kg) : l'impulsion d'ADR-023 (`m_a·v`) y surestimerait `J` de `1 + m_a/m` ≈ 2,3 ; le bilan (von Kármán) le borne.
Pour un corps lourd (`m_a/m` < 5 %), les deux à 5 % près. `t_impact = 2b·tanβ/(πv)` : 10 à 70 ms.

**Critères, écrits avant (C20).** (1) le bilan : quantité de mouvement corps + eau entraînée conservée à 10⁻¹² près ; `J` à 5 % de
`Δm_a·v_rel` quand `m_a/m` < 5 % ; (2) **l'indépendance à la phase** : la même chute, décalée de vingt phases de tick, rend le même `J` et le
même instant d'impact à 10⁻⁹ près — le témoin échantillonné au tick (l'impulsion appliquée au premier tick où la quille est sous l'eau, à
la vitesse de ce tick) disperse de plusieurs pour cent ; (3) sous le seuil de 2 m/s, aucune impulsion ; (4) les essais du corps inchangés
(archétype absent par défaut). Si (1)–(4), 6.8 validée.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — l'archétype d'impact, la détection exacte, l'impulsion ; essais (1)–(4).
- [ ] **P3** — preuve ; liste 6.8 ; rituel.

### Notes de reprise
