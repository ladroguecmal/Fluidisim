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
Session          : S10
État             : en cours
Battement        : 2026-09-05
Objectif         : `CondensedState`, la persistance hors caméra, et sa confrontation avec
                   `CoastalState` — configuration L22 signalée par S09
```

### Plan

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — l'analyse, couche par couche : qu'est-ce qui doit réellement persister quand un
  domaine quitte la caméra, quand la partie est sauvegardée, quand un joueur rejoint.
  *Thèse (L03) : la question « quel format pour `CondensedState` » est probablement **mal posée**.
  ADR-013 §6 a dissous la simulation hors caméra — le repli **est** la destruction du domaine. Si
  c'est vrai, il n'y a rien à condenser, et le format cherché n'a pas d'objet.*
- [x] **P3** — `ADR-022` §1–2 : la décision, et la démonstration couche par couche.
  *Forme : un ADR, pas une note. ADR-007 §7.3 appelle explicitement « un ADR à écrire », et une
  décision qui en change une autre ne se corrige pas, elle se remplace.*
- [x] **P4** — `ADR-022` §3 : `SeedState` — ce que `condense`/`restore` échangent réellement, et
  l'unification avec `CoastalState`.
  *Thèse : `condense` est une opération **d'outil de cuisson**, pas d'exécution ; `restore` est une
  opération d'exécution. Ce sont les deux moitiés d'un même mécanisme, écrites à deux sessions
  d'intervalle sous deux noms — la configuration L22 exacte.*
- [x] **P5** — `ADR-022` §4 : ce que l'eau met dans une sauvegarde, et le rechargement, l'arrivée
  en cours de partie, le redémarrage serveur.
- [ ] **P6** — `ADR-022` §5 : la couche V, seule persistance vraie, dans un monde partagé ·
  §6 conséquences sur les interfaces · §7 ce qui reste ouvert. Invariant **I-17** si la
  démonstration de P2 tient.
- [ ] **P7** — répercussions : notes correctives dans SPEC-004 (§10.2 et les signatures),
  ADR-007 §7.3, renvoi depuis SPEC-005 §6.
- [ ] **P8** — index, invariants, angles morts, README si nécessaire.
- [ ] **P9** — rituel de fin (`REPRISE.md` §6) : journal S10, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Contradiction trouvée avant même d'ouvrir le sujet, et elle est interne à S01** : ADR-013 §6
  pose que le hors caméra est une **destruction** de domaine (« il n'existe pas de simulation
  ralentie hors caméra »), tandis qu'ADR-007 §7.3 réclame un format de `CondensedState` « pour la
  persistance hors caméra ». Les deux ADR sont de la même session. La revue croisée S05 a confronté
  les vingt ADR et ne l'a pas vue ; la revue S08 a confronté les cinq SPEC et ne l'a pas vue non
  plus. Hypothèse à vérifier en P2 : **un point inscrit dans une liste « ce qui reste ouvert »
  échappe aux audits**, parce qu'un audit vérifie ce qui est affirmé et qu'un point reporté se lit
  comme une lacune connue, pas comme une contradiction.

#### P2 — l'analyse

**La contradiction est confirmée, et elle est textuelle.** Dans ADR-007, les deux signatures sont
introduites par le commentaire `// persistance hors caméra (ADR-001, architecture_globale §9)`.
Dans ADR-013 §6, écrit la même session : « Cela supprime toute la question §9 “quelle méthode
mathématique pour la simulation hors caméra” ». Le mécanisme a été dissous ; la signature écrite
pour lui est restée, et ADR-007 §5.3 a même inscrit « format exact de `CondensedState` → ADR à
écrire » — une tâche créée pour servir un besoin qui n'existait déjà plus.

C'est **L35 une seconde fois**, une session après avoir été écrite : une décision se propage vers
la prose qui l'explique, pas vers les signatures qui n'ont l'air de rien affirmer.

**Démonstration couche par couche.** Ce qui doit survivre à la destruction d'un domaine :

| Couche | Ce qu'il faut conserver | Pourquoi |
|---|---|---|
| B | **rien** | I-02 — le fond ne stocke rien, il est recalculé |
| W | le journal d'événements, 45 o pièce | ADR-003 §3 — `advance(t)` est fonction pure du journal |
| δ perturbatif | **rien** | naît à δ = 0 (ADR-013 §3) · I-12 destruction gratuite · I-04 aucune autorité |
| δ substitutif tenant la masse d'un nœud V | l'entier `i64`, rendu au nœud | ADR-010 §6 — le transfert δ→V est déjà spécifié |
| δ substitutif établissant un train de vagues | **rien à capturer** — l'état vient d'une donnée cuite | ADR-013 §4, SPEC-005 §6 |
| `F`, `A` | **rien** | ADR-014 §2.3 — ce qui sort de la cascade est perdu ; l'écume permanente est re-dérivée de W |
| V | `volume_ml` des nœuds modifiés | ADR-010 §7 |

**Quatre confirmations indépendantes** que rien de δ ne mérite d'être conservé :

1. **I-04** — δ n'a aucune autorité gameplay. Le perdre ne coûte rien qui compte.
2. **ADR-021 §3.2**, argument de fermeture — δ ne contient, par construction, que ce qui est plus
   court que `λ_cut` : du court, du local et du bref.
3. **I-12** — créer et détruire un domaine est visuellement gratuit. C'est la propriété que toute
   l'architecture défend ; un état à sérialiser la contredirait.
4. **SPEC-003 §8**, et c'est la plus convaincante parce qu'elle vient d'un document écrit pour
   autre chose : le harnais rejoue **une session de jeu entière** à partir de
   `(T_sim, descripteurs, journal d'événements)`. Si une session se rejoue sans état δ, l'état δ
   ne fait pas partie de l'état du monde. Le rejeu est une preuve, pas une analogie.

**Contre-épreuve — le cas qui ne passe pas, et il est réel.** Un domaine **substitutif** n'est pas
gratuit à recréer : 40 s d'établissement, ou 4,4 à 8 s depuis une condition 2D (SPEC-005 §6 corrigé
en S08). Or ADR-012 §4 **rang 5** détruit les domaines non focaux, et §5 engage toute décision de
dégradation pour « au moins 30 frames », soit **1 s à 30 Hz**.

```
fenêtre d'engagement de la dégradation :   1 s
coût de restauration d'un domaine substitutif : 4,4 à 8 s (depuis une graine)
                                                40 s     (depuis rien)
```

Un déferlement non focal peut donc être détruit puis redemandé quatre à huit fois plus vite qu'il ne
se rétablit. **Rang 5 ne s'applique pas aux domaines substitutifs**, ou leur hystérésis se
dimensionne sur le temps de restauration et non sur 30 frames. C'est un écart nouveau, chiffré, et
la règle qu'il produit se généralise : *une dégradation dont la fenêtre d'engagement est plus courte
que le coût de restauration de ce qu'elle détruit est un générateur de pompage.*

**Deux points que le corpus implique sans les dire.**

- **V est la seule couche que le serveur exécute.** I-10 lui interdit nommément W et δ, pas V. Et V
  porte du gameplay (une coursive qui s'inonde), est en arithmétique entière et déterministe à
  10 Hz — c'est-à-dire exactement ce qu'il faut pour être autoritaire au sens d'I-15. La forme de la
  couche V n'était pas un choix de commodité.
- **La sauvegarde force un règlement δ→V.** Si un nœud est gelé et sa masse remise à un domaine
  (ADR-010 §6) au moment où le joueur sauvegarde, il n'y a rien à écrire tant que le transfert
  inverse n'a pas eu lieu. Sauvegarder **provoque** le règlement, l'écart `M' − M` est journalisé
  comme d'habitude, et ce qui est écrit est un entier.

**Ce que `condense`/`restore` deviennent** : `condense` est l'opération par laquelle **l'outil de
cuisson** capture ce qu'il vient de simuler pour en faire une graine (SPEC-005 §7.1) ; `restore` est
l'opération par laquelle **l'exécution** amorce un domaine substitutif depuis cette graine. Les deux
moitiés d'un même mécanisme, écrites à deux sessions d'intervalle sous deux noms — `CondensedState`
en S04, `CoastalState` en S06. La configuration L22, confirmée.

#### P4 — deux rapprochements non prévus

- La contrainte que SPEC-004 §10.2 posait sur `CondensedState` (« se relire sur une machine
  différente, donc pas de disposition mémoire brute ») est celle d'un **actif cuit**, et n'a aucun
  sens pour une condensation en mémoire. Le document disait déjà ce qu'il était, dès S04.
- La graine referme **E07** : produite par δ qui n'est jamais D1, elle n'a pas à être reproductible
  puisqu'elle est identifiée par l'empreinte de son contenu. Les deux résolutions — S08 et S10 — se
  rejoignent sans avoir été conçues ensemble.
- Contraste noté avec ADR-013 §3 : la graine est **le seul endroit du système où un seuil de
  tolérance physique existe**, parce que c'est le seul précalcul qui contienne de la physique.

#### P5 — deux résultats non prévus

- **Le fichier de sauvegarde et la charge utile d'une arrivée en cours de partie sont le même
  objet.** Conséquence d'ADR-003 : quand l'état du monde se réduit à un temps et à un journal, le
  destinataire n'y change rien. Un seul format, et le harnais (SPEC-003 §8) le teste déjà sans
  qu'aucun test de sauvegarde ne soit écrit.
- **Le TTL d'ADR-010 §7 est la borne supérieure de la persistance de l'eau**, pas un nettoyage
  cosmétique. Sans lui, chaque flaque jamais revisitée d'un monde persistant resterait dans l'état
  du monde. À dire dans toute discussion qui proposerait de l'allonger.
- Chiffres : événements vivants ≤ ≈180 Ko (borné par paquets_W_max) ; 100 000 nœuds V modifiés à
  20 o = 2 Mo. Une sauvegarde d'eau est petite, et il faut le dire avant que quelqu'un ne conçoive
  un découpage dont personne n'a besoin.
