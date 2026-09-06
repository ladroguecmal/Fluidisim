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
- [ ] **P2** — **I-01 à I-05** contre ADR-001, ADR-004, ADR-003, ADR-008, ADR-007 et ADR-012.
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
