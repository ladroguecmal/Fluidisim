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

```
Session          : S09
État             : en cours
Battement        : 2026-09-05
Objectif         : écrire le chemin poussé — SPEC-006. Débloque trois des quatre accords inter-équipes.
```

### Plan

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — SPEC-006 §1–2 : principe du chemin poussé et règles générales — cadences propres,
  instantané immuable à N lecteurs, contrat de fils, anneau sans allocation, âge obligatoire,
  et la règle d'autorité qui découle d'I-15.
  *Thèse : le chemin poussé n'est pas « l'inverse » du chemin tiré. Il a ses propres règles, et
  c'est de ne pas les avoir écrites que naissent trois implémentations divergentes.*
- [x] **P3** — SPEC-006 §3 : le bus d'événements. `WaveEvent` complet avec ses trois champs audio,
  publication à N lecteurs, délai de propagation acoustique.
  *Thèse : `drain_outgoing_events()` de SPEC-004 est un canal à consommateur unique — le premier
  qui appelle vide la file pour les autres. C'est un défaut, pas un détail de nommage.*
- [ ] **P4** — SPEC-006 §4 : écume `F` et aération `A`. Deux publications d'un même champ.
  *Thèse : le rendu veut la texture, l'audio veut une intégrale. Publier la même chose aux deux
  impose un readback à celui qui n'en a pas besoin, ou une texture à celui qui n'en veut pas.*
- [ ] **P5** — SPEC-006 §5 : traversabilité. Tuiles, quatre cadences, événements de franchissement,
  et l'autorité du signal au titre d'I-15.
  *Thèse : le signal de navigation doit être calculé depuis les seules couches répliquées, sinon
  deux clients ne prennent pas la même décision de pathfinding.*
- [ ] **P6** — SPEC-006 §6 polyligne de déferlement · §7 dégradation du chemin poussé · §8 ce que
  l'interface rend impossible · §9 ce qui reste ouvert.
- [ ] **P7** — SPEC-004 : migrer `WaveEvent` en §2 avec ses trois champs audio, renommer
  `WaterSample.u` en `u_total` (écart E05), enrichir §9, marquer le point ouvert n°6 résolu.
- [ ] **P8** — index, README, statut des actions du registre S08, angles morts trouvés en écrivant.
- [ ] **P9** — rituel de fin (`REPRISE.md` §6) : journal S09, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Décision de forme prise avant d'écrire** : un document neuf, `SPEC-006`, plutôt qu'un §11 de
  SPEC-004. Motif : SPEC-004 est déjà à 481 lignes et son unité de propos est « ce que l'hôte
  branche et ce que l'appelant demande ». Le chemin poussé a d'autres consommateurs (audio, IA,
  rendu) et d'autres règles (cadences, âge, dégradation en fréquence). Les mêler rendrait les deux
  moins lisibles pour les équipes qui n'en lisent qu'un.
- **Quatre objets publiés recensés**, pas trois : le bus d'événements (ADR-016 §2), le champ
  d'écume `F` et le champ d'aération `A` (ADR-014 §2 et §5), le `TraversabilitySample` (ADR-018 §1),
  et — trouvé en relisant ADR-016 §2 — la **polyligne de déferlement**, qu'ADR-016 exige que le
  système publie et que SPEC-005 §2 liste déjà comme donnée dérivée à consommateurs multiples.

#### P3 — deux trouvailles en écrivant la structure

- `displaced_ml` est impossible sous ce nom : un `half` en millilitres sature à 65 L, dépassé par
  toute claque de coque. Publié en **litres** — mêmes 2 octets, plafond 65 m³.
- `drain_outgoing_events()` est un **résidu de la conception qu'ADR-021 §3 a remplacée**. Le chemin
  δ→serveur n'existe plus depuis R03 ; la fonction qui le servait a survécu à la décision qui la
  vidait de son objet. Supprimée, pas renommée.
- Troisième point, trouvé en pensant aux consommateurs : l'anticipation locale d'ADR-009 §7.2 ferait
  jouer deux fois le même impact à 100–300 ms d'intervalle. D'où le bit de **rétractation**.
