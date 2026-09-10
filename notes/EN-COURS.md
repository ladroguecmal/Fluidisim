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

---

## Session en cours

Session : S142 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A209**, ouverte par ma propre migration en S141. `ImpactField::new` compare sa
**borne L1** à `medium.max_slope` quand `RadialImpact` y compare désormais la pente **réelle** :
le même champ de `Medium` signifie deux choses selon le champ qui le lit. Mesurer le rapport de
ce champ-là comme S139 l'a fait pour le candidat radial, puis décider — migrer, séparer les deux
sens dans le type, ou retirer un champ que plus personne ne construit.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — sonde `pente_modale` : rapport `slope_bound / max|∇η|` d'`ImpactField`, avec son
      témoin analytique — la borne somme `a_m·k_m` et la pente vaut `|Σ a_m·k_m·û_m·sin(φ_m)·
      cos(ω_m t)|`, donc le rapport ne peut pas descendre sous 1.
- [x] **P3** — balayer ce dont il pourrait dépendre : longueur d'onde, énergie, instant. Thèse à
      vérifier : `side = 4λ` et les modes sont indexés par des entiers, donc le motif est
      **identique** à toute λ — le rapport devrait être une constante, comme pour le radial et
      contrairement à la pression. Le champ étant périodique et `sample` n'ayant pas d'emprise
      restreinte, le maximum est toujours atteint : c'est ce qui distingue ce cas de A206.
- [x] **P4** — **constater l'état réel du champ** avant d'en décider : qui le construit, quels
      ADR le documentent, ce qu'il porte que le candidat radial ne porte pas. Une décision de
      retrait ne se prend pas sur le seul fait qu'aucun appelant ne subsiste dans le dépôt.
- [x] **P5** — décider et appliquer : ADR, migration ou retrait. Témoins avant/après si des bits
      bougent, comme en S141.
- [ ] **P6** — rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ f40df3a = master ; worktree `886155`. 272 tests/cinq ignorés.

Ce qui est déjà lu et n'est pas à relire :
- `impact_field.rs` — `new` : 40 modes sur une grille cartésienne, `side = 4λ`, amplitude
  `1/(1+(radius−4)⁴)` mise à l'échelle par l'énergie, `k = τ·radius/side`, borne
  `slope += amplitude·k` ;
- `sample` : `slope[axe] -= amplitude·τ·turns[axe]·sin(phase_espace)·cos(ω t)`, et
  `τ·|turns| = k` — même structure que le candidat radial, donc même forme de majoration.
- à `t = birth`, toutes les phases temporelles valent 1 : c'est là que le maximum est attendu,
  comme en S139.

Piège à éviter : conclure « personne ne le construit, donc il est mort ». Le dépôt garde des
choses **exprès** — c'est le troisième fork qui l'a appris (S39, état `archivé`). Constater qui
le documente avant de proposer quoi que ce soit.

Second piège : si le rapport est une constante, la migration est **tentante et facile**. Elle
déplacerait pourtant une frontière de plus, et ce champ n'a aucun essai de réception comparable à
ceux du candidat. Mesurer d'abord, décider ensuite — pas l'inverse.
