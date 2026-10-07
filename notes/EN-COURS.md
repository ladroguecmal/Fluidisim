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

Session : S667 — **terminée**. En autonomie vers la v2 ; 2.7. `Cote2D` (S664–S665) n'a été jugée que sur une composante. **Une mer de huit
composantes** (périodes 7 à 12 s, directions −20° à +20°, `Hs` = 2.19 m) sur la plage de S364 : la composition jugée contre la côte 1D,
la mémoire et le temps de cuisson mesurés, extrapolés à une vraie côte.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : sans objet au départ.
- **instrument** : la côte 1D de S364 (`Cote::eval`), la même mer. Ce qui départagerait : **une composition juste** rend un écart de `η`
  sous la borne calculée par point depuis les écarts de chaque composante (`Σ a_c·f_c·(|Δφ_c| + |Δf_c|/f_c)`) ; **une diaphonie** entre
  composantes (un indice de table décalé, une direction mal tournée) la dépasse.
- **calcul** : la borne par point, calculée par l'essai (son ordre, ce script : ≈ 19.1 cm — le plancher de l'instrument,
  ADR-268 D1) ; `Hs` < 2,5 m (asserté : au large du déferlement à 2 m).
- **ADR** : ADR-196 (§3 : la mémoire d'une bathymétrie 2D, ses voies), ADR-264, ADR-268.
- **pièges** : une composante qui s'éloigne de la côte ou à plus de 45° (refusée) ; la mémoire (`ns·nn·20` octets par composante) ; le
  temps de cuisson qui croît avec la marge… absente désormais (les bords périodiques).

**Critères, écrits avant.** (1) À 60 points × 3 instants, sur la plage, `|η₂D − η₁D|` sous la borne de composition **en chaque point**.
(2) Rapportés : la mémoire et le temps de cuisson par composante ; leur extrapolation à 1 km² et 32 composantes au pas de 2 m — et la
voie de réduction qu'ADR-196 §3 nomme (des transformations partagées entre composantes voisines), à mesurer ensuite.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1)–(2).
- [ ] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
- **P2 fini** — (1) 180/180 sous la borne (rapport 0,78), `|Δη|` 0,5 cm ; (2) 1,5 s pour 8 composantes ; 160 Mo/km² pour 32 composantes au pas de 2 m.
