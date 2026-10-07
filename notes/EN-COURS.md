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

Session : S635 — **terminée**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S632–S634), puis **7.6 — un
dégel physique (le bilan d'énergie)** : le dégel de S575 était une rampe posée à la main.

**Ce que la session fait.** Dans `glace.rs` : `flux_de_fonte(T_air, α, albédo, S)` — `q = α·T_air + (1 − albédo)·S` (W/m², nul s'il est
négatif : alors c'est Stefan qui gèle) ; `epaisseur_fondue(h₀, q, t)` = `max(0, h₀ − q·t/(ρ_glace·L))` ; `duree_de_fonte(h₀, q)`. Le lac de S575
fond heure par heure par `ajuster_glace` — l'état exact est le temps de fonte cumulé (ADR-245), non la glace quantifiée. Ne fait pas : le
rayonnement infrarouge, le flux de l'eau sous la glace, la neige, le regel nocturne.

**Références, calculées avant** (ce script). `α` = 20 W/m²/K, `T_air` = +5 °C, albédo 0,6, `S` = 200 W/m² : **q = 180.0 W/m²**. La glace de S575
après 30 jours : 61021 quanta, **h₀ = 0.61021 m** ; la fonte dure **1038299.435444 s** (12.0174 jours) : la glace s'annule à
l'heure **289**. L'énergie d'un quantum de glace : 306278 J. **Borne du montage** (ADR-257 D1, assertée) : la fonte tient dans la
fenêtre de 20 jours.

**Quantum** : 1 000 ml de glace (917 g), soit 306278 J. **Critères, écrits avant.** (1) le flux 180 W/m² ; la durée de fonte égale à la
référence à 10⁻⁶ s ; (2) le lac : la masse à l'entier à chaque heure, la glace nulle à l'heure 289 et pas avant ; (3) le bilan d'énergie :
à chaque heure, l'énergie reçue `q·A·t` et la chaleur latente de la glace fondue diffèrent de moins d'un quantum (306278 J) ; (4) refus :
`α` ou `S` négatifs, albédo hors de [0, 1], une valeur non finie.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — le dégel et son essai ; (1)–(4).
- [x] **P3** — preuve ; liste 7.6 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — (1)–(4) tenus du premier essai. Suite : 820 essais listés.
- **P3** — preuve DEGEL-S635 ; ligne 7.6 ; index ; journal ; le lot.
