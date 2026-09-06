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
Session          : S13
État             : en cours
Battement        : 2026-09-05
Objectif         : confronter SPEC-006, ADR-022 et ADR-023 au corpus — trois documents
                   structurants écrits en quatre sessions, jamais audités
```

### Plan

La revue S05 a confronté les vingt premiers ADR, la revue S08 les cinq premières SPEC. **Ces trois
documents-là n'ont jamais rencontré personne**, et ils ont été écrits vite : SPEC-006 en S09,
ADR-022 en S10, ADR-023 en S12.

Règle de conduite pour cette session : **ne pas auditer de mémoire.** J'ai écrit les trois ; ce que
je crois y avoir mis n'est pas ce qui y est. Chaque contrôle part du texte du corpus, pas de
l'intention.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — SPEC-006 contre les **17 invariants** et contre les ADR qu'il sert
  (ADR-012 budgets et dégradation, ADR-014, ADR-016, ADR-018, ADR-021 autorité).
  *Thèse : un document d'interface écrit vite viole d'abord les invariants de ressources — I-06
  allocation, I-16 profil — parce qu'ils ne se voient qu'en additionnant des tailles.*
- [x] **P3** — SPEC-006 contre SPEC-001 à SPEC-005 : chiffres, cadences, unités, renvois.
- [x] **P4** — ADR-022 contre le corpus : I-17 tient-il partout, la couche V, le harnais, SPEC-005.
- [x] **P5** — ADR-023 contre le corpus : les quatre mécanismes contre ADR-008, ADR-010, ADR-013,
  ADR-015, et contre les chiffres de SPEC-001/002.
- [x] **P6** — les trois **entre eux** : ils se citent mutuellement (ADR-023 §4 publie sur
  SPEC-006 §6 ; ADR-022 §3 recoupe SPEC-005 §6 ; ADR-022 §4 recoupe SPEC-006 §2.6).
  *Thèse : le risque le plus élevé est là. Trois documents écrits par la même session-mère à quatre
  sessions d'écart se citent avec confiance et sans vérification.*
- [x] **P7** — rédiger `docs/registres/REVUE-CROISEE-S13.md` : écarts, gravité, résolution, et la
  liste des contrôles passés — sans elle la revue n'est pas vérifiable.
- [x] **P8** — appliquer les résolutions : notes correctives datées, nouvel ADR si une décision
  change.
- [x] **P9** — index, angles morts, décomptes.
- [ ] **P10** — rituel de fin (`REPRISE.md` §6) : journal S13, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Volume** : SPEC-006 fait 808 lignes, ADR-022 et ADR-023 environ 370 et 460. C'est plus que ce
  que S08 avait confronté d'un coup, d'où le découpage en quatre passes plutôt qu'en deux.
- **Attente** : S08 avait trouvé dix écarts dont deux de gravité 1 sur un corpus plus relu que
  celui-ci. Si cette session en trouve nettement moins, la première hypothèse à écarter est que
  l'audit a été mené de mémoire.

#### P2 — SPEC-006 contre les 17 invariants et les ADR qu'il sert

**E01, gravité 1 — l'invariant I-11 décrit un mécanisme supprimé depuis S05.**
I-11 énonce : « Aucune énergie ne franchit la frontière client → serveur sans borne validée. **Toute
demande d'événement issue d'un client est plafonnée par une cause connue du serveur.** »
Or ADR-021 §3.1 : « Le chemin d'énergie client → serveur **disparaît**. […] Le mécanisme de
plafonnement d'ADR-009 §3 devient **sans objet**. »
La seconde phrase de l'invariant décrit donc un plafonnement qui n'existe plus. L'intention est
mieux servie qu'avant — il n'y a plus de chemin du tout — mais l'énoncé impose un mécanisme aboli.
Trouvé en confrontant la table d'autorité de SPEC-006 §2.6 aux invariants.
C'est la **troisième instance** de la classe A78 : la correction s'est propagée vers ADR-005 et
ADR-009, pas vers l'invariant qui les citait. Et un invariant est le document qu'on cite pour
refuser une proposition. Ni S05 — qui a pourtant produit ADR-021 — ni S08 ni S11 ne l'ont vu : S11
auditait les points ouverts, et un invariant n'en est pas un.
**Un invariant ne se change que par un ADR explicite → ADR-024.**

**E02, gravité 2 — deux échelles de rangs de dégradation partagent une numérotation.**
ADR-012 §4 numérote sept rangs (rang 5 = détruire les domaines non focaux). SPEC-006 §7 en numérote
cinq (rang 5 = élaguer les événements locaux du bus) et se dit « cohérent avec les rangs
d'ADR-012 §4 ». Deux échelles indépendantes, mêmes numéros, dans deux documents qui se citent.
Aggravant depuis S12 : ADR-022 §2.6 a ajouté « **le rang 5 ne s'applique pas aux domaines
substitutifs** », règle qui porte sur l'échelle d'ADR-012. Un lecteur qui la rapporte à SPEC-006
comprendrait qu'on n'élague pas les événements de transduction dans un déferlement — contresens
complet.

**E03, gravité 2 — `ring_slots` a deux règles de dérivation qui peuvent se contredire.**
SPEC-006 §2.5 : « `ring_slots` se calcule à l'initialisation à partir du **coût mesuré d'un
instantané et de la mémoire allouée** au canal ».
SPEC-006 §3.4 : « `ring_slots ≥ (cadence_bus / cadence_du_consommateur_le_plus_lent) + 1`, soit au
moins **4 emplacements** pour un consommateur à 10 Hz ».
Les deux règles sont dans le même document, à une section d'écart, et rien ne dit laquelle l'emporte
si la mémoire n'en autorise que deux. Le défaut se manifesterait par des pertes d'événements
silencieuses — que `sequence` rend constatables, ce qui est une consolation et non une réponse.

**E04, gravité 3, mais il a traversé quatre documents — `WaveEvent` ne fait pas 45 octets.**
Somme des champs déclarés en SPEC-006 §3.1 : `id` 8 + `frame_id` 4 + `origin_local` 6 + `cell` 8 +
`t_birth` 8 + `kind` 1 + `energy` 2 + `dir` 4 + `lambda` 2 + `ttl_hint` 2 = **45**, puis
`material_id` 2 + `displaced_l` 2 + `flags` 1 = **50 octets**, et non 45.
En remontant : la structure d'origine d'ADR-009 §2, annoncée à **40 octets**, en sommait déjà **45**.
SPEC-006 a donc ajouté 5 octets à une base fausse et retrouvé par coïncidence la taille réelle de
l'original.
Propagation : ADR-009 §2 (40 o, 800 o/s) → SPEC-003 §8 (« 40 o pièce ») → SPEC-006 §3.1 (45 o,
900 o/s) → ADR-022 §4.2 (« 45 o pièce », ≈180 Ko). Valeurs justes : **50 o**, 1 000 o/s, ≈205 Ko.
**Aucune conclusion ne change** — tout reste négligeable — mais c'est la première erreur
*arithmétique* trouvée dans le corpus, et elle est dans une structure. S08 avait revérifié les
formules et les tables ; personne n'avait additionné les champs d'un `struct`.

**E05, gravité 3 — la charge d'actifs du serveur est énoncée en deux moitiés.**
SPEC-006 §2.6 : le serveur peut évaluer la traversabilité, « toutes les entrées sont analytiques ou
cuites » — ce qui suppose bathymétrie et courants C1 chargés, sans le dire.
ADR-022 §5.1 : « le serveur charge des données cuites », mais ne nomme que les `shape_lut`.
Chacun est juste, aucun n'est complet, et c'est une contrainte de déploiement (A77) qu'une équipe
serveur lira dans l'un ou dans l'autre.

**Contrôles passés — six, sans écart.**
· **I-06 et I-16** : `ring_slots` et le nombre d'auditeurs sont tous deux *dérivés* du profil et non
déclarés (§2.5, §4.2), et §4.2 nomme R04 pour dire pourquoi. Exemplaire.
· **I-15** : la table d'autorité de §2.6 est complète et chaque ligne est justifiée par ses entrées ;
la contrainte « δ exclu » sur la traversabilité est posée comme conception, pas comme observation.
· **I-04** : aucun canal ne fait remonter δ vers une décision ; le champ `F` est déclaré d'autorité
locale et §4.4 dit ce qu'il faudrait faire si un besoin gameplay lui venait — scinder à la source,
jamais moyenner.
· **I-13** : §4.1 remet au rendu une poignée de texture GPU. Ce n'est **pas** un partage de structure
de calcul au sens d'ADR-006 §5 : la texture est un produit publié, immuable sur le tick, et le rendu
ne peut rien y écrire ni influencer la simulation par elle. Contrôle passé, mais la clarification
mérite d'être écrite — quelqu'un invoquera I-13 pour refuser §4.1.
· **Cadences** : toutes diviseurs entiers du tick à 30 Hz — 10 Hz = 3 ticks, 5 Hz = 6, 1/2 s = 60,
1/30 s = 900. Vérifié une à une.
· **ADR-021 §4** : le rang le plus bas de SPEC-006 §7 n'élague que `TransductionLocale` et
`AnticipationLocale`, jamais `Serveur`. Transposition exacte, sans dérive.

#### P3 — SPEC-006 contre SPEC-001 à SPEC-005

**Arithmétique revérifiée, et elle tient sauf sur les structures.** Cascade 1024² en RG16F =
**4,19 Mo** ; quatre cascades à 30 Hz = **503 Mo/s** ✓. Zone de 4 km à 64 m = **3 906** cellules,
soit 130 évaluations/s contre 2 000 pour 200 agents à 10 Hz — rapport **15,4** ✓. Tuile de
16×16 cellules = 1 024 m, 256 échantillons × 20 o = **5 Ko**, seize tuiles ≈ **78 Ko** ✓. Flux
dissipé `P = E·c_g` : 15,3 kW/m à `Hs = 2 m`, et **337 kW/m à `Hs = 8 m`** — au-delà des 65 kW/m
que saturerait un `half` en W/m, ce qui confirme le choix du kW/m ✓. Vitesse orbitale
`πHs/T = 0,63 m/s` à `Hs = 1 m`, `T = 5 s` ✓.

**E04 est plus large que je ne l'avais écrit.** La taille de `WaveEvent` est reprise dans **six
documents et neuf endroits** — ADR-009 §2, ADR-021 §3.1, SPEC-003 §8, SPEC-006 §3.1 et §3.1 *(coût
réseau)*, ADR-022 §2.3 et §4.2, ADR-023 §4.1, REVUE-CROISEE-S08 — et non quatre. Correction de ma
propre note de P2.

**E04 bis — la même erreur, une seconde fois dans SPEC-006.** `ListenerAggregate` (§4.2) est annoncé
à **45 octets**. Somme des champs : `foam_active[3]` 6 + `foam_residual[3]` 6 +
`aeration_sector[16]` 32 + `immersion` 2 = 46, plus `ListenerId` → **50 octets**. Deux structures
sur deux mal comptées dans le même document. Ce n'est donc pas un accident : **aucune taille de
structure du corpus n'a jamais été vérifiée**, et les deux revues croisées précédentes ont contrôlé
des formules et des tables, jamais une somme de champs.

**E06, gravité 2 — une cadence du chemin poussé n'est pas un diviseur du tick.**
SPEC-006 §2.3 pose : « Chaque canal a sa cadence propre, **diviseur entier du tick de simulation**
(30 Hz, ADR-012 §7). » Et la table qui suit immédiatement inscrit : « Champ `F`, poignée GPU |
**par tick de rendu** ».
Or ADR-012 §7 dit exactement le contraire du rendu : « Tick de simulation **fixe à 30 Hz,
indépendant du taux d'images**. Le rendu interpole. » Le tick de rendu n'est pas un diviseur du tick
de simulation ; il n'a aucun rapport fixe avec lui.
Conséquences réelles : une publication cadencée sur le rendu n'est pas reproductible d'une exécution
à l'autre, l'ordonnanceur ne peut pas la budgéter, et sur une machine à 144 Hz elle produirait
presque cinq fois plus de travail que sur une machine à 30 Hz — pour un champ dont ADR-014 §6 dit
qu'il est « toujours actif, c'est le socle ».
**Résolution** : la poignée GPU d'écume est publiée sur le **tick de simulation** ou un diviseur, et
le rendu interpole comme il le fait pour tout le reste (ADR-012 §7). Aucun mécanisme nouveau — la
règle existait, la table l'a contredite une ligne plus bas.

**Contrôles passés.** `flow_speed` (SPEC-006 §5.1) ↔ `u_total` (SPEC-004 §2, renommé en S09) :
cohérent, et §5.5 porte le chiffre qui le motive. `TraversabilitySample` = 19 octets de champs
alignés à 20 : conforme à l'annonce, **mais** le document ne dit nulle part si ses `struct` sont
supposés compactés ou alignés — convention absente, à écrire (cf. E04). Portées d'invalidation
(§5.3 sous-cellule) ↔ ADR-006 §2 : réutilisation explicite, sans doublon. Polyligne de déferlement
(§6) ↔ SPEC-005 §2 : la donnée est bien cuite et republiée, pas recalculée.

#### P4 — ADR-022 contre le corpus

**E07, gravité 2 — l'invariant I-12 est faux pour les domaines substitutifs, et il cite le document
qui le contredit.**
I-12 : « **Créer et détruire un domaine est visuellement gratuit.** […] Toute proposition qui la
casse est refusée. → ADR-005, ADR-007, ADR-012, **ADR-013**. »
Or ADR-013 §4 — cité par l'invariant — établit qu'un domaine **substitutif** met **40 s** à
s'établir, et ADR-022 §2.6 a chiffré sa restauration depuis une graine à **4,4 à 8 s**. Créer un
domaine substitutif n'a donc jamais été gratuit, depuis S01.
L'invariant vaut pour les domaines **perturbatifs**, qui naissent à δ = 0 — c'est ce que dit
ADR-013 §4 dans la même phrase. Tel qu'il est écrit, il ferait refuser ADR-013 §4 lui-même, et il a
manqué à ADR-022 §2.6 : cette section a dû redémontrer que la restauration n'est pas gratuite alors
qu'un invariant correctement formulé le lui aurait donné.
**Deuxième invariant défectueux de la session, même classe qu'E01 → tous deux portés par ADR-024.**

**Contrôles passés — cinq.**
· **I-17 ↔ SPEC-005 §6** : le `SeedState` est bien une donnée cuite ; aucune capture d'exécution
n'est demandée nulle part, y compris dans le chemin de sauvegarde de §4.2.
· **ADR-022 §4.4 ↔ ADR-010 §6** : le règlement δ→V forcé à la sauvegarde emploie exactement le
transfert déjà spécifié, avec sa perte contrôlée journalisée. Pas de second mécanisme.
· **I-03 amendé ↔ SPEC-003 §2** : la couche V figurait dans le régime D1 depuis S03 ; l'amendement
S10 rattrape l'invariant sur la spécification, dans le bon sens.
· **C19 en mode `check`** : le cas ne mobilise que B, W et V — ni GPU, ni δ, ni assets lourds. Il
tient donc dans les contraintes de SPEC-003 §4, comme ADR-022 §6.1 l'affirme. Vérifié, pas supposé.
· **§4.1 — les quatre situations** (sauvegarde, arrivée en cours de partie, reconnexion, redémarrage
serveur) : la citation d'ADR-003 §3 est exacte, et le rapprochement tient.

**E05 confirmé et précisé** : ADR-022 §5.1 ne nomme que les `shape_lut`, alors que le serveur a
besoin en plus de la **bathymétrie** et des **champs de courant C1** pour évaluer la traversabilité
(SPEC-006 §2.6) et les franchissements de seuil qu'il rend autoritaires. La contrainte de
déploiement A77 est donc sous-évaluée dans le document qui la porte.

#### P5 — ADR-023 contre le corpus

**E08, gravité 3, mais c'est la deuxième fois — une force autoritaire nouvelle n'a pas de ligne dans
la table d'autorité.**
ADR-023 §2.4 établit que le terme d'impact est **autoritaire** au titre d'I-15 et dit qu'il « relève
de la ligne *poussée d'Archimède* » de la table d'ADR-008 §1. Il n'y a pas de ligne pour lui, et la
table est ce qu'on lit pour savoir ce qui peut changer une issue de jeu.
C'est **exactement l'écart R08 de la revue S05** — « les poches d'air n'ont pas de ligne dans la
table d'autorité » — qui avait justement fait ajouter deux lignes à cette même table. Le mécanisme
d'ajout est donc connu, appliqué une fois, et non réappliqué la fois suivante.

**E09, gravité 2 — `BreakerVertex` ne peut pas exprimer l'étendue d'un site ponctuel.**
ADR-023 §4.2 publie un site turbulent comme un `BreakerVertex` de `surf_width_m` petit. Mais
SPEC-006 §6 définit `surf_width_m` comme la **largeur de la zone de déferlement** — la dimension
perpendiculaire à la côte (ADR-005 §4.1) — et non l'étendue du site le long de la crête. Or le flux
publié est en **kW/m de crête** : sans étendue de crête, un consommateur ne peut pas retrouver les
77 kW qu'ADR-023 §4.4 annonce pour un rocher de 5 m.
**Résolution sans changer de type** : un site est publié comme **deux sommets** encadrant son
étendue — une polyligne dégénérée à deux points — ce que la structure `BreakerLineView` accepte déjà
telle quelle (`first_vertex` par région, sommets consécutifs). Le champ `surf_width_m` garde son
sens, et le consommateur intègre entre les deux sommets comme pour n'importe quel segment.

**E10, gravité 2 — l'équation de coalescence emploie une grandeur qui n'existe pas.**
ADR-023 §5.2 écrit `V_air(z) = n_fusion·R·T / P(z)`. La structure `AirPocket` d'ADR-015 §3 porte
`node`, `volume_ml`, `n_moles`, `pressure_pa`, `centroid` — **aucune température**. L'équation n'est
donc pas évaluable avec l'état dont le corpus dispose.
**Résolution, et elle simplifie** : l'hypothèse isotherme de §5.3 dit que les deux poches sont à la
même température. On peut donc éliminer `T` entièrement :

```
P_fusion · V_fusion  =  P_a·V_a + P_b·V_b        →     V_air(z) = (P_a·V_a + P_b·V_b) / P(z)
```

Mêmes champs, aucune constante physique à introduire, et `n_fusion = n_a + n_b` reste vrai comme
énoncé de conservation. La dichotomie sur le `shape_lut` est inchangée. **La correction retire une
grandeur au lieu d'en ajouter une** — même forme que la résolution d'E07 en S08.

**Contrôles passés — quatre.**
· **§2 ↔ I-15** : les entrées du terme d'impact sont l'état du solide et B + W ; aucune ne vient de
δ. L'autorité est correctement justifiée, seul le tableau manque (E08).
· **§3 ↔ ADR-008 §3** : le mode contraint est employé tel quel, avec sa projection sur la surface et
son amortissement vers la vitesse orbitale — pas de variante, une condition d'entrée de plus.
· **§4 ↔ ADR-014 §2.3** : le « terme stationnaire dérivé » est bien le mécanisme déjà retenu pour
l'écume permanente, cité et non réinventé.
· **§5 ↔ ADR-010 §5** : l'hystérésis d'ouverture reprend la forme de celle des flaques, avec un
seuil propre. Cohérent.

#### P6 — les trois documents entre eux

C'est là que j'attendais le plus, et c'est là que se trouve l'écart le plus coûteux.

**E11, gravité 2 — la sauvegarde emporte des événements non répliqués, et cela casse C19.**
ADR-022 §4.2 fait figurer dans `WaterPersistentState` un `span<const WaveEvent> events_alive`, sans
filtre. Or SPEC-006 §3.1 — écrite **une session plus tôt** — a doté `WaveEvent` d'un `EventOrigin` à
trois valeurs : `Serveur`, `AnticipationLocale`, `TransductionLocale`. Les deux dernières sont
locales, cosmétiques et non répliquées (ADR-021 §3).

Trois conséquences, la troisième étant la plus gênante :

1. une sauvegarde emporterait des événements que le serveur n'a jamais eus, et qu'elle réinjecterait
   au rechargement comme s'ils étaient autoritaires ;
2. leurs `id` ne sont pas des `server_seq` : l'ordre total et la déduplication d'ADR-009 §2 ne
   valent pas pour eux, et le tri par `id` de SPEC-006 §3.2 les mêlerait aux autres ;
3. **le cas canonique C19 en serait faux.** ADR-022 §6.1 exige que le hash après aller-retour de
   persistance soit **identique** à celui d'une simulation continue, et le revendique en régime D1,
   donc binaire. Un événement `TransductionLocale` provient de la transduction d'un solveur δ, qui
   n'est jamais D1 : le hash différerait par intermittence, et le banc conçu comme l'argument phare
   d'I-17 échouerait sans que sa cause soit dans I-17.

**Résolution.** `events_alive` ne retient que les événements d'origine `Serveur`. Ce n'est pas une
restriction ajoutée : c'est la stricte application d'ADR-022 §1, dont le troisième énoncé dit que
l'état persistant tient en « `T_sim`, le journal des événements W encore vivants, et les volumes
entiers » — le mot *W* y désignant, depuis ADR-021 §3, le seul `W_rep`. Le document se contredisait
entre son §1 et son §4.2.

**E12, gravité 3 — le critère de déclenchement de l'impact n'est pas déclaré répliqué.**
ADR-023 §2.5 pose `v_rel·n > 2 m/s`, « à calibrer ». Le terme d'impact est autoritaire (§2.4) et le
serveur émet l'événement correspondant depuis la cause (ADR-021 §3), puisqu'il possède la physique
des objets. Il faut donc que **le même seuil soit appliqué des deux côtés**, faute de quoi un client
verrait une gerbe et un choc sans qu'aucun son ne parte, par intermittence et près du seuil.
Le point n'est pas faux ; il lui manque une ligne disant que ce seuil est une **donnée répliquée**,
au même titre que `E_cause` et `K` (ADR-021 §7.2).

**Contrôles passés — quatre, et ce sont les rapprochements que je me méfiais le plus de croire.**
· **ADR-022 §3 `SeedState` ↔ SPEC-005 §6 `CoastalState`** : la généralisation est fidèle, les
volumes sont repris sans dérive (77 Ko par état, 1,2 Mo par plage), et le `kind = Cotier` couvre
exactement l'usage d'origine.
· **ADR-022 §4.5 ↔ SPEC-006 §2.6** : « le signal de traversabilité n'est jamais écrit, il est
entièrement dérivé » est cohérent avec « autoritaire » — autoritaire et persistant sont deux
propriétés distinctes, et les deux documents ne les confondent pas.
· **ADR-023 §6 « aucune interface nouvelle » ↔ SPEC-004 §3** : l'événement d'impact emprunte
`push_events`, qui existe. La revendication tient.
· **ADR-023 §4 ↔ SPEC-006 §6** : le canal est réutilisé sans modification de type — sous réserve de
l'écart E09, qui porte sur l'expressivité de `BreakerVertex` et non sur le canal.

#### P7 — registre écrit

`docs/registres/REVUE-CROISEE-S13.md`. Douze écarts, un de gravité 1, six de gravité 2, cinq de
gravité 3. **Deux portent sur des invariants** — c'est nouveau : le corpus n'avait jamais été trouvé
en défaut ailleurs que dans des ADR et des spécifications.

Constat qui vaut d'être noté : **aucun des trois documents audités ne viole un invariant.** Ce sont
les invariants qui ont vieilli. Un document récent confronté à une règle ancienne révèle la règle
autant que le document.

#### P8 — coupé en deux, l'étape dépassait le quart d'heure

**P8a** : `ADR-024` écrit, I-11 et I-12 amendés dans `01_INVARIANTS.md` avec la mention de
l'amendement et sa date. **P8b** : les onze notes correctives.

Découpage déclaré ici avant de le faire, conformément à la règle « aucune étape ne dépasse une
quinzaine de minutes ; si elle est plus grosse, la découper ».

**P8b** : 18 marques appliquées, aucun échec. Les douze écarts sont traités — deux par ADR-024, dix
par notes correctives datées. `ADR-021 §3.1` et `REVUE-CROISEE-S08` gardent leur « 40 octets » : ce
sont des textes historiques (un raisonnement de S05, une table de contrôles de S08), et un registre
d'audit ne se réécrit pas. La correction vit dans ADR-009 §2, qui porte la structure.
