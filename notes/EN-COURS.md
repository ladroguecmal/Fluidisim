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
Session          : S64
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S62-1 — corriger la sommation de variance, puis trancher par ADR la fenêtre
                   d'échantillonnage de Hs.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — **diagnostiquer le `NaN` avant de le corriger** : S62 l'a attribué à l'annulation catastrophique sans le vérifier.
- [x] **P3** — corriger ce que le diagnostic désigne, avec témoin et essai de refus ; vérifier que les valeurs nominales ne bougent pas.
- [x] **P4** — mesurer la loi complète, jusqu'aux fenêtres qui étaient hors d'atteinte.
- [ ] **P5** — ADR : trancher la fenêtre, ou dire pourquoi elle ne se tranche pas.
- [ ] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, jeton, **fusion dans master**.

### Notes de reprise

Départ eacd9d9. Acquis de S62 : `Hs` est aveugle à `hs` (rapport 0,914723 sur un facteur 8) et
gouverné par la fenêtre rapportée à `λ_pic` — 8,53 % à 6,8 λ, **0,28 % à 54,7 λ**. À 12288 m
la mesure rend `NaN`, et le cas échoue alors correctement, sans faux succès.

**La cause du `NaN` est déclarée non établie.** S62 a écrit « la variance en une passe rend
`NaN` » — c'est une hypothèse, pas une mesure, et l'ordre de grandeur ne la soutient pas : à
16,7 millions de points, `somme2/n ≈ 0,09` et `moyenne² ≈ 0`, donc `m0` n'a aucune raison de
passer sous zéro. **Une explication correcte n'est pas une cause tant que son effet n'a pas été
mesuré** (**L75**), et le dépôt a déjà payé cette faute.

**L'autre candidat, plus probable et plus grave.** `eta()` s'écrit
`bg.eval(p, t).map(...).unwrap_or(f64::NAN)` : un point hors du domaine évaluable rend `NaN`, et
la somme le propage. À 12288 m de côté les positions vont de −6144 à +6144 m — l'ancre et la
portée du fond n'ont pas été vérifiées à cette distance. Si c'est cela, **le cas ne dit pas
qu'un point était invalide, il dit seulement que la mesure est `NaN`** : le refus fonctionne,
le diagnostic manque. C'est la famille de **L166** — *c'est l'aval qu'il faut suivre*.

Interdits : ne pas changer la fenêtre nominale sans ADR (elle déplace un chiffre publié), ne pas
toucher aux tolérances, ne pas rendre le cas vert en élargissant la marge.

P2 : **l explication de S62 etait fausse, et l action S62-1 prescrivait donc une correction
inutile** (Welford). Le NaN ne vient pas de l annulation : to_local refuse tout point a plus de
4096 m de l ancre (types.rs:90, LIMIT = 4096 x WORLD_UNITS_PER_METRE), eval rend None, eta rend
NaN par unwrap_or, et la somme le propage. Confirme a six metres pres : demi-fenetre 4092 m rend
0,505 pour cent, demi-fenetre 4098 m rend NaN.

Consequences. La fenetre utilisable va jusqu a 8192 m de cote, soit 145 lambda_pic — trois fois
ce qu il faut. Et le cas ne dit pas pourquoi il rend NaN : un point hors portee ne se compte
nulle part, exactement le defaut corrige en S45 sur C10 (A173, L166). C est l aval du NaN qui
manque, pas la sommation.

Note : la faute que S63 vient de recenser — une prescription ecrite sans etre eprouvee — a ete
commise par S62, la session immediatement precedente, dans l action meme que S64 execute.

P3 : les points hors portee sont comptes et le motif est ecrit dans le libelle ; la mesure reste
refusee (pas de variance sur un domaine ampute — faute corrigee en S45 sur C10). Test de refus
et temoin. 131 tests reussis, deux ignores ; hashs et valeur nominale inchanges.

P4, et il a corrige deux affirmations de S62-1 puis en a produit une troisieme.
 (a) **Le cout annonce etait faux** : mode physics 33,9 s au nominal, 35,9 s avec la fenetre
     64 fois plus grande — **+6 pour cent**, pas 64 fois. La mesure de Hs est marginale devant
     les solveurs. L action S62-1 avancait deux raisons de ne pas elargir, toutes deux fausses.
 (b) **La correction marche sur une mer ou le cas echouait** : tp=9 rendait 15,58 pour cent a la
     fenetre nominale, **0,184 pour cent** a 3072 m. Six configurations mesurees a cette fenetre :
     0,152 / 0,184 / 0,282 / 0,282 / 0,367 pour cent — et **6,612 pour cent a 256 composantes**.
 (c) **La graine du scenario ne commande rien** : six graines, six fois 0,282 pour cent au
     chiffre pres. phase0 = i x 0x9E3779B9, sans PRNG ; SeaState n a pas de champ graine, et
     scenario.graine est lue puis jamais utilisee. **Il n existe qu une realisation par etat de
     mer**, donc aucune barre d erreur n est mesurable et aucune tolerance ne se calibre. A186.
 (d) L ecart a 256 composantes ne vient ni de la fenetre ni du pas : 6,614 et 6,615 pour cent a
     pas 1,5 et 1,0 m, fenetre egale. Cause **non identifiee** — A187, et elle borne D2.
