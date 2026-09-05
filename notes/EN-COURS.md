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
Session          : S11
État             : en cours
Battement        : 2026-09-05
Objectif         : auditer les listes « ce qui reste ouvert » de tous les documents (L40)
```

### Plan

Trois questions par point, toujours les mêmes :
**(a) a-t-il encore un objet ?** — une décision ultérieure l'a-t-elle dissous, comme ADR-013 §6
avait dissous celui d'ADR-007 §5.3 sans que personne ne le voie pendant neuf sessions.
**(b) sa formulation tient-elle encore ?** — chiffres périmés, renvois cassés, prémisse changée.
**(c) qui attend, et quoi ?** — une mesure, une réunion, une décision humaine, du code.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — inventaire mécanique : extraire les ≈110 points des 26 documents, les numéroter,
  produire la table brute. *Thèse : sans inventaire exhaustif écrit, l'audit portera sur ce qu'on
  se rappelle, c'est-à-dire sur les documents récents — précisément ceux qui en ont le moins besoin.*
- [x] **P3** — passe sur **ADR-001 à ADR-008** (socle).
- [x] **P4** — passe sur **ADR-009 à ADR-013** (réseau, hydraulique, ordonnanceur, prédiction).
- [x] **P5** — passe sur **ADR-014 à ADR-022** (phénomènes secondaires, construction, corrections).
- [x] **P6** — passe sur les **six SPEC** et le harnais.
- [x] **P7** — rédiger `docs/registres/AUDIT-POINTS-OUVERTS-S11.md` : le verdict par point,
  la synthèse par catégorie, et **qui attend quoi**.
- [x] **P8** — appliquer : clôtures marquées et notes correctives dans les documents concernés.
- [ ] **P9** — index, angles morts, `METHODE.md` (la passe d'audit devient une phase du protocole).
- [ ] **P10** — rituel de fin (`REPRISE.md` §6) : journal S11, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Volume mesuré avant de commencer** : ≈110 points répartis sur 26 documents. Aucun dans
  SPEC-001, SPEC-002, `CAS-CANONIQUES`, `PLAN-BENCHMARK` ni les registres — ce sont des documents
  de référence et d'audit, ils n'ont pas de dette de ce type. Les plus chargés sont SPEC-006 (8),
  SPEC-004 (6), puis SPEC-005, ADR-014, ADR-015, ADR-017 et ADR-022 (5 chacun).
- **Attente sur la répartition des verdicts** : la valeur de l'exercice est dans les catégories
  (a) et (b). Si tout revient « encore valide », l'audit aura quand même produit une chose utile —
  la liste de qui attend quoi, qui n'existe nulle part.

#### P2 — inventaire, et un ajustement de plan déclaré

**110 points, 26 documents.** Registre créé avec la méthode et les six verdicts (A dissous,
B clos ailleurs, C formulation périmée, D dupliqué, E valide, F pas une question).

*Ajustement du plan, déclaré avant le travail* : les passes P3 à P6 écrivent leurs verdicts
**directement dans le registre**, section par section, plutôt que dans ces notes puis dans le
registre. P7 devient la synthèse et le tableau « qui attend quoi ». Motif : recopier 110 verdicts
d'un fichier à l'autre est exactement le geste qui périme les décomptes (leçon apprise deux fois,
S07 et S10).

#### P3 — socle : 22 E, 4 C, 2 D, 1 A

Trouvaille la plus lourde : **le critère d'I-16 n'est pas opérationnel tel qu'il est écrit.**
ADR-006 §7.3 demande « un nombre maximal de blocs par profil », ce que R04 a interdit — mais le
profil d'ADR-012 §3 déclare encore `paquets_W_max` et `v_noeuds_actifs`, qui sont des tailles de
pool, donc ressources ET capacités à la fois. Le critère qui marche : *une valeur peut figurer dans
un profil si elle est allouée directement ; pas si elle doit être cohérente avec deux autres valeurs
déjà déclarées.* C'est ce qui condamnait `domaines_max`, contradictoire avec la mémoire ET le temps.

#### P4 — 10 E, 4 C, 3 D, 2 B

Deux points **répondus depuis longtemps** et jamais marqués : ADR-009 §2 (anticipation locale —
prémisse morte en S05, mécanisme spécifié en S09) et ADR-013 §3 (format et volume de la
bibliothèque côtière — répondu par SPEC-005 §6 en S06, cinq sessions).

Et une **cinquième dépendance inter-équipes découverte** : l'équipe gameplay spatial, citée deux
fois pour `to_vacuum` (ADR-010 §8.3, ADR-015 §7.2) et absente de la liste des quatre interfaces de
00_INDEX.md.

#### P5 — 21 E, 7 D, 5 C, 4 B, 1 F

**Troisième instance de la même classe de défaut** : ADR-017 §7.2 réclame encore « une subdivision
dédiée » alors qu'ADR-006 §2 porte le mécanisme depuis S05 (écart R07) et que SPEC-006 §5.3 le
réutilise explicitement. Après ADR-007 §5.3 et ADR-006 §7.3. Une correction se propage vers le
document corrigé, jamais vers les points ouverts qui réclamaient la correction.

Symptôme à retenir, ADR-022 §7.4 : *« comme partout, dépend du langage »*. Quand un point ouvert se
justifie par le fait que d'autres documents le posent aussi, il ne devrait pas exister.

#### P6 — 16 E, 2 A, 2 C, 1 B, 1 D, 1 F. Total : 110 points, comptés.

**Quatrième instance**, et c'est la plus instructive : SPEC-004 §10.3 propose encore « un point sur
quatre » alors que la note corrective de S08 (écart E08) est dans **le même document**, quatre
sections plus haut. Le défaut n'est donc pas une affaire de distance entre documents.

**Délai le plus court observé** : SPEC-004 §10.5 était clos par SPEC-006 §2.5 au moment même où S09
l'écrivait. Le défaut se produit à l'instant où la réponse est écrite ailleurs, pas avec le temps.

*Correction apportée à ma propre section P3* : l'en-tête annonçait 31 points pour 30, et le bilan
omettait le verdict B d'ADR-003 §3. Corrigé. Un décompte recopié se périme même dans le document
qui le produit.

#### P7 — synthèse : trois résultats qui débordent l'audit

1. **Onze destinataires extérieurs**, là où l'index en listait quatre. Les sept nouveaux sont plus
   légers (une table de valeurs, un cadrage) mais ils ne se rattrapent pas tard non plus.
2. **B2 débloque quatre points ouverts**, plus que tout autre banc — confirmation indépendante du
   chemin critique, établi jusqu'ici sur les dépendances et non sur un décompte.
3. **Quatre points disent « à spécifier » et aucune session ne l'a jamais pris en charge** :
   terme de slamming, modèle du nageur, rochers turbulents, coalescence des poches T2. Quatre
   sujets courts et indépendants — c'est un objectif de session, et probablement S12.

Et un cinquième arbitrage humain remonté au même rang que les trois connus : **qui possède le
harnais** (SPEC-003 §11.4), rangé jusqu'ici parmi des questions de format de fichier alors que
SPEC-003 §1 dit que la qualité de toutes les décisions à venir est plafonnée par la sienne.

#### P8 — 34 marques appliquées, aucun échec

8 clôtures (B), 15 notes correctives (C), 10 renvois de doublons avec porteur désigné (D),
1 requalification (F). Les scripts rapportent les non-appliquées ; il n'y en a eu aucune.
