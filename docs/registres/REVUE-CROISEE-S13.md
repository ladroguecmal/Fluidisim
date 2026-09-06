# Revue croisée de SPEC-006, ADR-022 et ADR-023 — S13

Trois documents structurants écrits en quatre sessions — SPEC-006 en S09, ADR-022 en S10, ADR-023 en
S12 — et **jamais confrontés à quoi que ce soit**. La revue S05 a porté sur les vingt premiers ADR,
la revue S08 sur les cinq premières SPEC ; ceux-ci n'avaient rencontré personne.

**Douze écarts, dont un de gravité 1 et six de gravité 2.** Deux d'entre eux portent sur des
**invariants**, ce qui est nouveau : jusqu'ici le corpus n'avait été trouvé en défaut que sur des
ADR et des spécifications.

Gravité : **1** = deux documents s'excluent, une décision doit trancher · **2** = incohérence
opérationnelle, corrigeable par précision · **3** = documentaire ou mineur.

> **Règle de conduite de cette session.** J'ai écrit les trois documents audités. Ce que je crois y
> avoir mis n'est pas ce qui y est, et un audit mené de mémoire n'aurait trouvé que ce que
> j'attendais. Chaque contrôle part donc du **texte du corpus**, pas de l'intention. Le rendement le
> confirme : les deux écarts les plus coûteux — E01 sur un invariant, E11 sur la sauvegarde — sont
> précisément ceux que je n'attendais pas.

---

## Tableau

| # | Écart | Documents | Grav. | Résolution |
|---|---|---|---|---|
| E01 | **I-11 impose un mécanisme supprimé en S05** | I-11 ↔ ADR-021 §3 | **1** | **ADR-024** |
| E02 | Deux échelles de rangs de dégradation partagent une numérotation | SPEC-006 §7 ↔ ADR-012 §4 | 2 | renumérotation SPEC-006 |
| E03 | `ring_slots` a deux règles de dérivation contradictoires | SPEC-006 §2.5 ↔ §3.4 | 2 | note corrective SPEC-006 |
| E04 | Les tailles de `struct` annoncées sont fausses, et propagées | 6 documents, 9 endroits | 3 | notes correctives + convention |
| E05 | La charge d'actifs du serveur est énoncée en deux moitiés | ADR-022 §5.1 ↔ SPEC-006 §2.6 | 3 | note corrective ADR-022 |
| E06 | Une cadence « par tick de rendu » contredit la règle du diviseur | SPEC-006 §2.3 ↔ ADR-012 §7 | 2 | note corrective SPEC-006 |
| E07 | **I-12 est faux pour les domaines substitutifs** | I-12 ↔ ADR-013 §4 | 2 | **ADR-024** |
| E08 | Une force autoritaire nouvelle n'a pas de ligne dans la table | ADR-023 §2.4 ↔ ADR-008 §1 | 3 | note corrective ADR-008 |
| E09 | `BreakerVertex` ne peut pas exprimer l'étendue d'un site ponctuel | ADR-023 §4.2 ↔ SPEC-006 §6 | 2 | note corrective ADR-023 |
| E10 | L'équation de coalescence emploie une grandeur inexistante | ADR-023 §5.2 ↔ ADR-015 §3 | 2 | note corrective ADR-023 |
| E11 | **La sauvegarde emporte des événements non répliqués — C19 en devient faux** | ADR-022 §4.2 ↔ SPEC-006 §3.1 | 2 | note corrective ADR-022 |
| E12 | Le seuil de déclenchement de l'impact n'est pas déclaré répliqué | ADR-023 §2.5 ↔ ADR-021 §3 | 3 | note corrective ADR-023 |

---

## E01 — I-11 impose un mécanisme supprimé en S05 *(gravité 1)*

**Constat.** L'invariant I-11 énonce :

> « Aucune énergie ne franchit la frontière client → serveur sans borne validée. **Toute demande
> d'événement issue d'un client est plafonnée par une cause connue du serveur.** »

Et ADR-021 §3.1, écrit en S05 :

> « Le chemin d'énergie client → serveur **disparaît**. L'angle mort A16 […] n'est plus atténué par
> un plafond : il **n'existe plus**, faute de chemin. Le mécanisme de plafonnement d'ADR-009 §3
> devient **sans objet**. »

La seconde phrase de l'invariant décrit donc un plafonnement aboli il y a huit sessions. Son
intention est mieux servie qu'avant — il n'y a plus de chemin du tout, ce qui est strictement plus
fort qu'une borne — mais son **énoncé impose un mécanisme qui n'existe pas**.

**Pourquoi c'est de gravité 1.** Un invariant n'est pas un document de plus : c'est celui qu'on cite
pour refuser une proposition sans discussion de détail. Deux effets, opposés et tous deux mauvais :
quelqu'un implémentera le plafonnement parce qu'un invariant l'exige ; ou quelqu'un, constatant que
le mécanisme n'existe pas, conclura que l'invariant n'est pas tenu et proposera de « rétablir » un
chemin client → serveur — c'est-à-dire de rouvrir l'angle mort A16 que la suppression du chemin
avait fait disparaître.

**Comment il a survécu.** C'est la troisième instance de la classe **A78** : la correction s'est
propagée vers ADR-005 §3 et ADR-009 §3, qui ont reçu leurs notes correctives, et pas vers
l'invariant qui les citait. Ni S05 — qui a pourtant *produit* ADR-021 — ni S08 ni S11 ne l'ont vu.
S11 auditait les listes « ce qui reste ouvert », et un invariant n'en est pas une.

**Trouvé en confrontant** la table d'autorité de SPEC-006 §2.6 aux dix-sept invariants, un par un.

**Résolution → ADR-024.** Un invariant ne se change que par un ADR explicite. La reformulation
proposée supprime le mécanisme et garde — en le renforçant — l'énoncé :

> **I-11 — Aucun chemin d'énergie ne va du client vers le monde répliqué.** Un client ne peut pas
> faire naître un événement W répliqué : le serveur les émet depuis leurs causes, qu'il possède. La
> transduction δ→W d'un client ne produit que du `W_local`, cosmétique et non répliqué. Il n'y a
> donc rien à plafonner et aucune borne à valider.

## E07 — I-12 est faux pour les domaines substitutifs *(gravité 2)*

**Constat.** I-12 : « **Créer et détruire un domaine est visuellement gratuit.** […] Toute
proposition qui la casse est refusée. → ADR-005, ADR-007, ADR-012, **ADR-013**. »

ADR-013 §4 — cité par l'invariant — dit exactement le contraire pour une moitié des cas : un domaine
**substitutif** met **40 s** à s'établir, et ADR-022 §2.6 a chiffré sa restauration depuis une graine
à **4,4 à 8 s**. Seul le domaine **perturbatif** naît gratuitement, à δ = 0.

**Ce que l'omission a coûté.** ADR-022 §2.6 a dû redémontrer que la restauration d'un domaine
substitutif n'est pas gratuite, et en tirer que le rang 5 d'ADR-012 §4 ne s'y applique pas. Un
invariant correctement formulé le lui aurait donné, et l'écart aurait été trouvé en S01 plutôt qu'en
S10.

**Résolution → ADR-024.**

> **I-12 — Créer et détruire un domaine *perturbatif* est visuellement gratuit.** C'est la propriété
> qui rend possibles l'ordonnancement, la dégradation, le repli hors caméra et le remplacement de
> solveur. Un domaine **substitutif** n'a pas cette propriété : son établissement se compte en
> secondes (ADR-013 §4), et toute dégradation qui le détruit doit dimensionner son hystérésis sur son
> temps de restauration (ADR-022 §2.6).

---

## E11 — La sauvegarde emporte des événements non répliqués, et C19 en devient faux *(gravité 2)*

**Constat.** ADR-022 §4.2 fait figurer dans `WaterPersistentState` un
`span<const WaveEvent> events_alive`, sans filtre. SPEC-006 §3.1 — écrite **une session plus tôt** —
a doté `WaveEvent` d'un `EventOrigin` à trois valeurs, dont deux locales et non répliquées.

**Trois conséquences.**

1. La sauvegarde emporterait des événements que le serveur n'a jamais eus et les réinjecterait au
   rechargement comme s'ils étaient autoritaires.
2. Leurs `id` ne sont pas des `server_seq` : l'ordre total et la déduplication d'ADR-009 §2 ne valent
   pas pour eux, et le tri par `id` de SPEC-006 §3.2 les mêlerait aux autres.
3. **Le cas canonique C19 en serait faux.** ADR-022 §6.1 exige un hash *identique* après aller-retour
   de persistance, en régime D1 donc binaire. Un événement `TransductionLocale` vient d'un solveur δ,
   qui n'est jamais D1 : le hash différerait par intermittence, et le banc conçu comme l'argument
   phare d'I-17 échouerait pour une raison étrangère à I-17. C'est le pire cas de figure — un test
   juste qui échoue pour une cause fausse discrédite le test, pas la cause.

**Le document se contredit lui-même.** ADR-022 §1, troisième énoncé : l'état persistant tient en
« `T_sim`, le journal des événements W encore vivants, et les volumes entiers ». Depuis ADR-021 §3,
« W » désigne, s'agissant de ce qui est répliqué, le seul `W_rep`. Le §4.2 est donc plus large que le
§1 du même document.

**Résolution.** `events_alive` ne retient que les événements d'origine `Serveur`. Restriction non pas
ajoutée mais **appliquée** — elle était déjà dans le §1.

---

## Les autres écarts

### E02 — Deux échelles de rangs partagent une numérotation *(gravité 2)*

ADR-012 §4 numérote **sept** rangs de dégradation ; SPEC-006 §7 en numérote **cinq** et se déclare
« cohérent avec les rangs d'ADR-012 §4 ». Dans l'une, « rang 5 » signifie *détruire les domaines non
focaux* ; dans l'autre, *élaguer les événements locaux du bus*.

Aggravant depuis S12 : ADR-022 §2.6 a posé que « le rang 5 ne s'applique pas aux domaines
substitutifs ». Rapportée à SPEC-006, cette règle se lirait « on n'élague pas les événements de
transduction dans un déferlement » — contresens complet, et parfaitement plausible.

**Action** : les rangs du chemin poussé sont préfixés **P1 à P5**, et §7 dit qu'ils ne se confondent
pas avec ceux d'ADR-012 §4.

### E03 — `ring_slots` a deux règles de dérivation *(gravité 2)*

§2.5 le dérive de la **mémoire allouée** au canal ; §3.4 le dérive du **rapport de cadences**
(« au moins 4 emplacements pour un consommateur à 10 Hz »). Rien ne dit laquelle l'emporte si la
mémoire n'en autorise que deux.

**Action** : `ring_slots = max(règle de cadence, 2)`, et la mémoire du canal doit le permettre ; si
elle ne le permet pas, c'est une **mauvaise configuration**, détectée par `validate_config()` au
chargement et jamais à l'exécution (SPEC-004 §1.3). Le mécanisme existe, il suffit de l'invoquer.

### E04 — Les tailles de `struct` annoncées sont fausses, et propagées *(gravité 3)*

| Structure | Annoncé | Somme réelle des champs |
|---|---|---|
| `WaveEvent` d'origine (ADR-009 §2) | 40 o | **45 o** |
| `WaveEvent` étendu (SPEC-006 §3.1) | 45 o | **50 o** |
| `ListenerAggregate` (SPEC-006 §4.2) | 45 o | **50 o** |

Détail du second : `id` 8 + `frame_id` 4 + `origin_local` 6 + `cell` 8 + `t_birth` 8 + `kind` 1 +
`energy` 2 + `dir` 4 + `lambda` 2 + `ttl_hint` 2 = 45, puis `material_id` 2 + `displaced_l` 2 +
`flags` 1 = **50**. SPEC-006 a ajouté cinq octets à une base fausse et retrouvé par coïncidence la
taille réelle de l'original.

Propagation : **six documents, neuf endroits** — ADR-009 §2, ADR-021 §3.1, SPEC-003 §8,
SPEC-006 §3.1 (deux fois) et §4.2, ADR-022 §2.3 et §4.2, ADR-023 §4.1, plus la table de contrôles de
REVUE-CROISEE-S08.

**Aucune conclusion ne change** : 1 000 o/s au lieu de 900 dans une zone chargée, ≈205 Ko au lieu de
180 pour une sauvegarde — tout reste négligeable, et le chiffrage d'ADR-023 §4.1 qui écartait les
émetteurs permanents devient même plus défavorable à ce qu'il écartait.

**Ce qui compte est ailleurs.** C'est la première erreur *arithmétique* du corpus, et elle est dans
une structure. S05 a vérifié les tables numériques dupliquées, S08 a recalculé une quarantaine de
valeurs depuis leurs formules — **personne n'a jamais additionné les champs d'un `struct`**. Deux
structures sur deux sont fausses dans SPEC-006 : ce n'est pas un accident, c'est une classe de
contrôle absente.

**Action** : corriger les neuf endroits, et écrire la convention manquante — les tailles annoncées
sont des **sommes de champs compactés**, l'alignement relevant du langage encore ouvert
(SPEC-004 §10.1).

### E05 — La charge d'actifs du serveur, en deux moitiés *(gravité 3)*

ADR-022 §5.1 conclut « un serveur sans assets n'est pas une option » en ne nommant que les
`shape_lut`. Mais SPEC-006 §2.6 lui fait évaluer la traversabilité et rendre des franchissements de
seuil autoritaires, ce qui exige en plus la **bathymétrie** et les **champs de courant C1**. Chaque
document est juste, aucun n'est complet, et c'est une contrainte de déploiement (A77) qu'une équipe
serveur lira dans l'un **ou** dans l'autre.

**Action** : note corrective dans ADR-022 §5.1, qui porte la question.

### E06 — Une cadence « par tick de rendu » *(gravité 2)*

SPEC-006 §2.3 pose que chaque cadence est un « **diviseur entier du tick de simulation** (30 Hz,
ADR-012 §7) », puis inscrit dans la table qui suit : « Champ `F`, poignée GPU | **par tick de
rendu** ». Or ADR-012 §7 dit du rendu qu'il est « indépendant du taux d'images » et qu'« il
interpole ».

Conséquences : une publication cadencée sur le rendu n'est pas reproductible d'une exécution à
l'autre, l'ordonnanceur ne peut pas la budgéter, et sur une machine à 144 Hz elle ferait presque cinq
fois le travail d'une machine à 30 Hz — pour un champ qu'ADR-014 §6 déclare « toujours actif, c'est
le socle ».

**Action** : la poignée GPU est publiée sur le tick de simulation ou un diviseur ; le rendu interpole
comme pour tout le reste. La règle existait ; la table l'a contredite une ligne plus bas.

### E08 — Une force autoritaire sans ligne dans la table *(gravité 3)*

ADR-023 §2.4 établit que le terme d'impact est autoritaire et affirme qu'il « relève de la ligne
*poussée d'Archimède* » de la table d'ADR-008 §1. Il n'y a pas de ligne pour lui.

C'est **l'écart R08 de la revue S05 à l'identique** — « les poches d'air n'ont pas de ligne dans la
table d'autorité » — qui avait fait ajouter deux lignes à cette même table. Le mécanisme d'ajout est
connu, appliqué une fois, non réappliqué la fois suivante.

### E09 — `BreakerVertex` ne peut pas exprimer l'étendue d'un site ponctuel *(gravité 2)*

ADR-023 §4.2 publie un site turbulent comme un `BreakerVertex` de `surf_width_m` petit. Mais
SPEC-006 §6 définit `surf_width_m` comme la **largeur de la zone de déferlement**, dimension
perpendiculaire à la côte (ADR-005 §4.1) — et non l'étendue du site le long de la crête. Le flux
publié étant en **kW/m de crête**, un consommateur ne peut pas retrouver les 77 kW qu'ADR-023 §4.4
annonce pour un rocher de 5 m.

**Résolution sans changer de type** : un site est publié comme **deux sommets** encadrant son
étendue — une polyligne dégénérée à deux points — ce que `BreakerLineView` accepte déjà. Le champ
garde son sens et le consommateur intègre entre deux sommets comme sur n'importe quel segment.

### E10 — L'équation de coalescence emploie une grandeur inexistante *(gravité 2)*

ADR-023 §5.2 écrit `V_air(z) = n_fusion·R·T / P(z)`. La structure `AirPocket` d'ADR-015 §3 ne porte
**aucune température** : l'équation n'est pas évaluable avec l'état dont le corpus dispose.

**Résolution, et elle simplifie.** L'hypothèse isotherme de §5.3 pose que les deux poches sont à la
même température ; `T` s'élimine :

```
P_fusion · V_fusion = P_a·V_a + P_b·V_b     →     V_air(z) = (P_a·V_a + P_b·V_b) / P(z)
```

Mêmes champs, aucune constante physique à introduire, `n_fusion = n_a + n_b` conservé comme énoncé
de conservation, dichotomie sur le `shape_lut` inchangée. **La correction retire une grandeur au lieu
d'en ajouter une.**

### E12 — Le seuil d'impact n'est pas déclaré répliqué *(gravité 3)*

ADR-023 §2.5 pose `v_rel·n > 2 m/s`, « à calibrer ». Le terme est autoritaire (§2.4) et le serveur
émet l'événement correspondant depuis la cause (ADR-021 §3). Le seuil doit donc être **le même des
deux côtés**, faute de quoi un client voit un choc sans qu'aucun son ne parte, par intermittence et
près du seuil. Il manque une ligne : ce seuil est une donnée **répliquée**, comme `E_cause` et `K`
(ADR-021 §7.2).

---

## Ce qui a été vérifié et tient

Un audit qui ne rapporte que des défauts n'est pas vérifiable.

**Invariants.** Les dix-sept ont été passés un à un contre SPEC-006 §2.6 et contre ADR-022 §1.
Quinze tiennent. Les deux qui ne tiennent pas — I-11 et I-12 — sont E01 et E07, et **aucun des trois
documents audités ne les viole** : ce sont les invariants qui ont vieilli, pas les documents.

**I-06 et I-16** : `ring_slots` et le nombre d'auditeurs sont dérivés du profil et non déclarés
(SPEC-006 §2.5, §4.2), et §4.2 nomme l'écart R04 pour dire pourquoi. Exemplaire — sous réserve
d'E03, qui porte sur la *règle* de dérivation et non sur son principe.

**I-13** : SPEC-006 §4.1 remet au rendu une poignée de texture GPU. Ce n'est pas un partage de
structure de calcul au sens d'ADR-006 §5 — la texture est un produit publié, immuable sur le tick, et
le rendu ne peut ni y écrire ni influencer la simulation par elle. Contrôle passé, **mais la
clarification est à écrire** : quelqu'un invoquera I-13 pour refuser §4.1.

**I-15 et I-04** : la table d'autorité de SPEC-006 §2.6 est complète, chaque ligne justifiée par ses
entrées, et §4.4 dit ce qu'il faudrait faire si un besoin gameplay venait au champ `F` — scinder à la
source, jamais moyenner.

**I-17 ↔ SPEC-005 §6** : le `SeedState` est bien une donnée cuite ; aucune capture d'exécution n'est
demandée nulle part, y compris dans le chemin de sauvegarde.

**Arithmétique de SPEC-006, hors structures.** Cascade 1024² en RG16F = 4,19 Mo ; quatre cascades à
30 Hz = 503 Mo/s. Zone de 4 km à 64 m = 3 906 cellules, 130 évaluations/s contre 2 000 — rapport
15,4. Tuile de 16×16 = 1 024 m, 5 Ko, seize tuiles ≈ 78 Ko. `P = E·c_g` = 15,3 kW/m à `Hs = 2 m` et
337 kW/m à `Hs = 8 m`, ce qui confirme le passage au kW/m. `πHs/T` = 0,63 m/s. **Toutes justes.**

**Cadences** : diviseurs entiers du tick à 30 Hz, vérifiés un à un — 10 Hz = 3 ticks, 5 Hz = 6,
1/2 s = 60, 1/30 s = 900. Une seule exception, E06.

**Rapprochements entre les trois documents** : `SeedState` ↔ `CoastalState` fidèle et sans dérive de
volumes ; « autoritaire » et « persistant » correctement distingués entre ADR-022 §4.5 et
SPEC-006 §2.6 ; la revendication « aucune interface nouvelle » d'ADR-023 §6 tient, l'événement
d'impact empruntant `push_events` qui existe.

**ADR-021 §4** : le rang le plus bas de SPEC-006 §7 n'élague que les événements locaux, jamais
`Serveur`. Transposition exacte.

---

## Suite

| Action | Où | Statut |
|---|---|---|
| **ADR-024** — reformulation d'I-11 et d'I-12 | `adr/`, `01_INVARIANTS.md` | à écrire |
| `events_alive` filtré aux événements `Serveur` | ADR-022 §4.2 | note corrective |
| Rangs du chemin poussé préfixés P1–P5 | SPEC-006 §7 | note corrective |
| `ring_slots = max(cadence, 2)` + `validate_config` | SPEC-006 §2.5 et §3.4 | note corrective |
| Cadence de la poignée GPU d'écume | SPEC-006 §2.3 | note corrective |
| Tailles de `struct` corrigées, convention écrite | 9 endroits | notes correctives |
| Site turbulent publié comme deux sommets | ADR-023 §4.2 | note corrective |
| Coalescence en `P·V`, sans température | ADR-023 §5.2 | note corrective |
| Seuil d'impact déclaré répliqué | ADR-023 §2.5 | note corrective |
| Ligne « terme d'impact » dans la table d'autorité | ADR-008 §1 | note corrective |
| Charge d'actifs du serveur complétée | ADR-022 §5.1 | note corrective |
| Clarification I-13 pour la poignée de texture | SPEC-006 §4.1 | note corrective |
