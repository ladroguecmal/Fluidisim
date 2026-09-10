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

Session : S145 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **refaire le bilan d'avancement.** Celui de S69 oriente encore `REPRISE.md` §4 — « ~85 %
comme corpus, ~15 % comme système ; `δ`, `W` et `V` n'existent pas » — et il a **76 sessions**.
C'est l'exemple même de l'état recopié qui se lit au présent alors qu'il ne l'est plus (**A185**).
Le refaire à la même méthode, puis en tirer le fil suivant.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — compteurs **à la source**, comme S69 : sessions, ADR, spécifications, registres,
      angles, leçons, lignes de Rust, tests, cas, bancs. Mécanique, aucune interprétation.
- [x] **P3** — le bloc « **construire le système** » : ce que le code fait aujourd'hui, module par
      module, et ce qui manquerait pour que ce soit la couche `W` du jeu. C'est là que S69 disait
      ~5 %, et c'est l'affirmation la plus susceptible d'avoir vieilli.
- [x] **P4** — le bloc « **savoir mesurer** » : étages du harnais, cas exécutables, bancs exécutés.
      S69 disait 2 étages sur 6, 12 cas sur 23 exécutables, 0 banc sur 11 exécuté.
- [x] **P5** — les deux lectures recalculées, **et ce que S69 recommandait confronté à ce qui a
      été fait**. Une recommandation vieille de 76 sessions a-t-elle été suivie ? Sinon, pourquoi,
      et cela vaut-il décision.
- [x] **P6** — livrable BILAN-S145, rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ 4c463f9 = master ; worktree `886155`. 275 tests/cinq ignorés, 98 ADR, 18 invariants.

Méthode de S69, à reprendre telle quelle pour que les deux bilans soient comparables :
compteurs vérifiés à la source (fichiers, sortie du harnais, `REPRISE.md` §4) ; trois blocs
mesurés séparément — *décider quoi construire*, *savoir mesurer*, *construire le système* ; puis
deux lectures, corpus et système. **Ne pas s'appuyer sur les colonnes « État » des tableaux
d'actions antérieurs à S45** : S69 les a écartées, elles disent « ouverte » là où une note en
prose clôt (A185).

Ce qui est déjà connu et cadre le travail :
- la trajectoire d'ADR-054 (S71) : `WaveEvent` → journal rejouable → impact propagé →
  sillage/intégration → B2 ;
- depuis S71, le dépôt a écrit `RadialImpact`, le journal d'ondes, la composition B+W, la
  pression spectrale, `LiveWater`, l'admission incrémentale — mais **aucune session n'a déclaré
  que `W` existait**. Vérifier lequel des deux est vrai est le cœur de P3.

Piège à éviter : refaire le bilan en relisant le corpus. S69 a mesuré à la source ; un bilan
d'opinion ne serait pas comparable au sien et ne vaudrait rien.

Second piège : conclure « le projet a bien avancé » parce que 76 sessions ont passé. Le fait
central de S69 était que **vingt-deux sessions d'affilée n'avaient produit aucune conception du
système** ; la même question se pose pour les soixante-seize qui suivent, et la réponse peut être
la même.

P2-P6 : compteurs à la source, inventaire par couche, état du harnais, deux lectures recalculées,
BILAN-S145, A211, L228, rituel §6.7, six décomptes corrigés. Aucun code modifié, 275 tests.

Les trois résultats, par ordre d'importance :
1. **B1 n'a jamais été lancé** — recommandé par S69, zéro banc sur onze après 76 sessions ;
2. **`W` existe** et le repère disait le contraire — 9 768 lignes reçues par 275 essais ;
3. **A211** : le chaînage « suite Sxxx » propage la proximité, pas l'importance.

Pour S146 sans relire : **lancer B1**, « champ de fond : nombre de composantes et coût
d'évaluation ». Il ne demande aucune couche manquante — `B` existe (`background.rs`, 587 lignes),
le harnais mesure, la graine produit des réalisations indépendantes depuis S65. Sa question n'est
pas le coût mais la **justesse** : A187 a montré qu'à 256 composantes `Hs` s'écarte de 6,6 % par
un mécanisme de battements, et A188 que la calibration statistique reste à faire. Le protocole est
dans `docs/validation/PLAN-BENCHMARK.md` §B1.

Piège pour S146 : écrire une sonde de plus au lieu de lancer le banc. C'est ce que le dépôt fait
depuis 76 sessions, avec d'excellents résultats — mais une sonde mesure le modèle contre lui-même,
quand B1 mesurerait autre chose que sa propre cohérence.
