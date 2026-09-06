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
Session          : S14
État             : en cours
Battement        : 2026-09-05
Objectif         : le contrôle inverse — pour chacun des quinze invariants restants, l'ADR qu'il
                   cite dit-il encore ce que l'invariant résume ?
```

### Plan

S13 a vérifié que trois documents récents **respectent** les dix-sept invariants. C'est l'exercice
courant. Le contrôle **inverse** — l'invariant résume-t-il encore fidèlement l'ADR qu'il cite ? —
n'a jamais été fait, et il a rapporté deux écarts sur deux tentatives, dont un de gravité 1.

I-11 et I-12 ont été traités en S13 par ADR-024. Restent **quinze**.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — **I-01 à I-05** contre ADR-001, ADR-004, ADR-003, ADR-008, ADR-007 et ADR-012.
- [ ] **P3** — **I-06 à I-10** contre ADR-006, ADR-002, ADR-003, ADR-004 et ADR-009.
- [ ] **P4** — **I-13 à I-17** contre ADR-006, ADR-012, ADR-021, ADR-022, et le cas particulier
  d'I-14, qui ne cite aucun ADR.
- [ ] **P5** — rédiger `docs/registres/AUDIT-INVARIANTS-S14.md` : le verdict par invariant, et la
  liste de ceux qui tiennent — sans elle le contrôle n'est pas vérifiable.
- [ ] **P6** — appliquer : notes correctives, amendements, et un ADR si une décision change.
- [ ] **P7** — index, angles morts, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6) : journal S14, leçons, index, jeton libéré.

### Notes de reprise

**Quatre hypothèses vérifiées avant de déclarer ce plan**, pour ne pas planifier sur une intuition :

- **I-16 n'a jamais reçu la précision que S11 avait décidée.** Le registre
  `AUDIT-POINTS-OUVERTS-S11` §2.6 conclut « précision du critère d'I-16 dans `01_INVARIANTS.md` »,
  et sa table « Suite » l'inscrit. La note a été posée dans ADR-006 §7.3 ; l'invariant, lui, est
  inchangé. **Une action décidée dans un registre et jamais exécutée** — exactement la classe que ce
  registre dénonçait.
- **I-13 cite un objet qui n'existe pas** : « il peut demander une priorité au `WaterManager` ».
  SPEC-004 §3 nomme la classe `WaterSystem`, et `WaterManager` n'apparaît nulle part ailleurs.
- **I-02 est contredit par son propre ADR source.** « B n'a aucune représentation par cellule, ni en
  mémoire, ni **sur disque**, ni sur le réseau » — or ADR-004 §2.2 définit la grille `HydroSample`,
  par nœud, que SPEC-005 §2 range parmi les données d'auteur, et qu'ADR-022 §4.2 **écrit dans la
  sauvegarde** sous le nom de `RegionDescriptor`.
- **`HydroSample` est une quatrième structure mal dimensionnée** : annoncée à 40 octets, ses champs
  en somment **34** (onze `f16` = 22, deux `f16[2]` = 8, deux `u8` = 2, `_pad[6]` = 6). L'angle mort
  A85 devient quatre structures sur quatre.

#### P2 — I-01 à I-05

**I-01 — défaut, gravité 2.** « Tout consommateur passe par `EvalWater(x, t, LayerMask)`. »
C'est faux depuis S09, et faux **par décision** : ADR-018 §1 interdit nommément à la navigation
d'interroger `EvalWater` (« sans quoi la navigation devient un consommateur majeur »), l'audio lit
des agrégats (SPEC-006 §4.2), le rendu reçoit une poignée de texture (§4.1). SPEC-006 §1.1 chiffre
même l'écart : 130 évaluations/s publiées contre 2 000 interrogées.
Le principe — il n'existe pas « la » surface produite par un système unique — tient entièrement. Le
**mécanisme nommé** est devenu l'un des deux, et l'invariant ne connaît que l'autre.

**I-02 — défaut, gravité 2.** « B n'a aucune représentation par cellule, ni en mémoire, ni **sur
disque**, ni sur le réseau. »
Contredit par son propre ADR source : ADR-004 §2.2 définit la grille `HydroSample`, **par nœud**,
que SPEC-005 §2 range parmi les données d'auteur et de cuisson. Et ADR-022 §4.2 l'**écrit dans la
sauvegarde** sous le nom de `RegionDescriptor` — un document que j'ai écrit en S10 en citant I-02
approbativement trois sections plus haut.
La distinction voulue est celle entre un **état** et des **paramètres** — c'est exactement I-09, qui
la formule proprement. I-02 ne la dit pas, et sans elle son énoncé est faux au pied de la lettre.

**I-03 — défaut, gravité 3.** Amendé en S10 pour inclure la couche V, mais sa flèche ne cite
qu'ADR-003, qui ne traite ni de V ni de son arithmétique entière. La source du déterminisme de V est
ADR-010 §4 (« débits quantifiés en millilitres avec report de reste […] le résultat est
reproductible »). L'invariant couvre une couche dont il ne cite pas la source.

**I-04 — défaut, gravité 1, et il ne vient pas de l'invariant.**
I-04 tient contre ADR-008, ADR-014 §5.2, ADR-021 §5 et ADR-023 §2.4 : vérifié un à un, aucune force
de gameplay ne vient de δ. **Mais ADR-010 §6 le contredit**, et personne ne l'avait rapproché :

```
V → δ :   le nœud est gelé, sa masse M est remise au domaine […]
δ → V :   le domaine rend M' = masse mesurée ; l'écart M' − M est reporté comme perte contrôlée
```

C'est un **transfert de propriété de masse** vers δ, et ADR-010 §6 le dit ainsi. Trois conséquences :

1. pendant l'épisode, **la masse d'eau d'un compartiment est déterminée par δ** — donc par un
   solveur non déterministe (SPEC-003 §2 : δ n'est jamais D1). Le volume d'une cale décide d'un
   chavirement : c'est une issue de jeu, et I-04 l'interdit ;
2. **le serveur n'a pas d'histoire.** Il exécute V (ADR-022 §5.1) et n'exécute jamais δ (I-10). Il
   ne peut donc ni geler le nœud, ni recevoir `M'`. Serveur et client divergent pendant toute la
   durée de l'épisode, et rien ne les réconcilie ;
3. l'écart `M' − M` est journalisé comme diagnostic — ce qui est juste — **et** appliqué comme perte
   de masse réelle — ce qui ne l'est pas.

ADR-010 §6 date de S01, avant qu'I-10 n'ait ses conséquences travaillées et huit sessions avant
qu'ADR-022 §5.1 n'établisse que le serveur exécute V. Aucune revue ne l'a rapproché d'I-04 parce que
les revues confrontent des documents entre eux, jamais un invariant à un ADR qui ne le cite pas.
**Résolution → ADR-025**, en P6.

**I-05 — tient.** `step(dt_target, budget_ms)` et `StepResult` de SPEC-004 §4 portent exactement ce
que l'invariant exige, et §4.1 en fait un point de contrat non négociable.
*Observation de portée, sans défaut* : une part croissante du coût lié à l'eau vit **côté hôte** —
la flottabilité (ADR-008 §2) et depuis S12 le terme d'impact (ADR-023 §6) — et n'entre pas dans le
budget que I-05 protège. L'invariant parle des solveurs, et il a raison de s'y tenir ; mais
« l'eau ne peut structurellement pas provoquer un pic de frame » se lit plus large qu'il n'est vrai.
