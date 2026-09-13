# ADR-127 — Ambition finale complète, construction progressive par versions de plus en plus capables

- **Statut : actée**, S204, 2026-09-13, **clarification explicite de l'utilisateur**.
- **Corrige [ADR-124](ADR-124-image-budget-et-effets-bornes.md)** : son point 3, sa conséquence
  sur V et sa clause de réversibilité, lus comme une réduction du produit. Confirme ses points 1
  et 2 (image de B, budget d'image), déjà réalisés en S201 et S202, et l'autorisation d'images
  locales. **ADR-124 n'est pas réécrit** ; il porte une note de renvoi datée.
- Précise la portée d'[ADR-125](ADR-125-budget-image-60hz-deux-ms.md) (note corrective datée) ;
  laisse [ADR-126](ADR-126-emprise-d-un-impact-visible.md) entier.
- Trajectoire tenue à jour dans [FEUILLE-DE-ROUTE](../FEUILLE-DE-ROUTE.md), seul document qui la porte.

## 1. Ce que l'utilisateur a clarifié

> *Ambition finale complète, construction progressive par versions de plus en plus capables.*

Les ambitions initiales restent **intégralement maintenues** : interactions volumiques
générales, inondations complexes, réseau hydraulique V et système à grande échelle. La
direction « image → budget → effets bornés » est un **ordre de construction**, pas une
réduction du produit final. Les effets bornés sont des **étapes** ; δ général et V restent des
**objectifs obligatoires**.

## 2. Ce qui était faux dans la lecture d'ADR-124

ADR-124 écrivait que « δ est un effet borné par son cas d'usage », qu'« un solveur volumétrique
général ne conditionne plus » l'affichage, que « V attend un besoin gameplay nommé », et que
« revenir à un δ général demande une nouvelle décision ». Les deux premières phrases décrivent
un ordre ; les deux dernières en ont fait un **périmètre**. Rien dans la direction donnée par
l'utilisateur n'autorisait ce passage.

La lecture s'est propagée sans être interrogée : ADR-125 §« Conséquences » (« V attend un besoin
gameplay »), la file active, REPRISE, l'index, PLAN-BENCHMARK B3, BILAN-B4-S176, IMAGE-B-S201,
BUDGET-IMAGE-S202 et IMPACT-W-S203 — **ce dernier écrit par la session qui acte cette
correction**. Aucun travail n'a été supprimé à cause d'elle : elle a déplacé des priorités et
reporté S199-2 et S200-1 « si un effet retient le noyau ». Voir A248.

## 3. Décision

**D1 — L'ambition finale est complète et obligatoire.** Le produit visé comprend B, W, δ
**général** — solveur volumétrique à surface libre, domaines multiples, interactions
volumiques générales, frontières mobiles, régime substitutif d'ADR-001 §3.3 — V et les
inondations complexes, leur articulation avec la représentation volumétrique, et le passage à
la grande échelle. Aucun de ces objectifs n'est conditionnel à un besoin à redécouvrir.

**D2 — La construction progresse par versions de plus en plus capables.** Jalons, ordonnés par
dépendances (la feuille de route les détaille et en tient l'état) :

| jalon | ce qu'il livre |
|---|---|
| **J1** | une première version **visible et interactive** avec B et W |
| **J2** | des **domaines volumiques bornés**, comme cas de construction et de validation du δ général |
| **J3** | l'extension des phénomènes, les **interactions entre domaines** et les **frontières mobiles** |
| **J4** | **V et les inondations complexes**, puis leur articulation avec la représentation volumétrique |
| **J5** | les ambitions initiales complètes ; approfondissement ensuite |

**D3 — Un domaine borné est une étape du δ général, pas un produit séparé.** Il passe par les
interfaces du système — `Volume`/`Caps` d'ADR-007, masques de couches, ordonnanceur
d'ADR-012, chemin tiré et poussé — et ses défauts connus (S199-2, S200-1/A244) sont **sur le
chemin** : on les traite quand J2 emploie le noyau, pas « si un effet le retient ».

**D4 — V n'est pas repoussée à la fin.** Son noyau — graphe de contenants et d'arêtes,
arithmétique entière, serveur autoritaire, C12 — ne dépend ni de δ ni de J3 (ADR-054 §1). Il
**s'ouvre au plus tard avec J2**, en parallèle des domaines bornés. Ce que J4 attend de J3,
c'est l'articulation : exposer une surface V, déclencher un domaine δ local, C21.

**D5 — Les responsabilités et les garde-fous ne bougent pas.** ADR-001 §2 reste la table des
rôles. δ n'a jamais d'autorité gameplay (I-04) ni d'état sérialisé (I-17) : les conséquences de
jeu d'une inondation passent par V, déterministe et autoritaire (I-03, I-10, ADR-010). Aucun
chemin d'énergie du client vers le monde répliqué (I-11) ; autorité testée contre I-15.
Interfaces SPEC-004/006, conservation et critères physiques inchangés.

**D6 — Les bancs servent les décisions, ils ne précèdent pas les versions.** Un banc s'exécute
quand les composants construits permettent de **trancher une décision concrète**. On ne cherche
pas à fermer tous les bancs avant de construire une version utilisable ; en retour, une
version ne revendique aucune réception qu'un banc n'a pas rendue.

**D7 — Le budget est une cible, pas un ciseau.** 60 images/s et eau 2 ms (ADR-125), seuil
numérique 2 % (ADR-120) : **acquis**. Le budget se confronte à des **scènes représentatives** ;
il n'autorise pas à retirer une fonctionnalité en silence. Toute incompatibilité mesurée — A247
en est une — donne lieu à un **arbitrage explicite** : scène, mesure, options, ce que chacune
dégrade. Quand l'option retire une part de l'ambition, l'arbitrage revient à l'utilisateur. La
dégradation prévue par ADR-012 et I-05 (sous-résoudre pour tenir le budget) reste un
mécanisme ; elle n'est pas une suppression.

## 4. Ce que cette décision ne fait pas

Elle ne fixe **aucun calendrier** et **aucune technologie** : famille de δ (B3), représentation
de W (B2), coupure W/δ restent à trancher par leurs bancs, au moment où ils tranchent. Elle ne
change aucun invariant, ne rouvre pas ADR-027, ne modifie ni ADR-125 (profil) ni ADR-126
(emprise). Elle ne déclare aucun jalon atteint.

## 5. Arbitrages explicites ouverts à sa date

- **Hôte interactif de J1.** `water-core` est sans dépendance (ADR-020) et le workspace l'est
  entièrement. Une fenêtre temps réel demande soit un hôte séparé avec dépendances (téléchargement,
  infrastructure : **décision de l'utilisateur**), soit un hôte sans dépendance par appels système
  de la plateforme. Non tranché.
- **A245** : la composition B+W refuse toute mer au-delà de Hs ≈ 1,1 m — bloquant pour J1.
- **A247** : un impact visible coûte plus que 2 ms sur CPU — incompatibilité à arbitrer selon D7.

## 6. Réversibilité

**Réduire l'ambition finale n'appartient pas à une session.** Une décision d'ordre, de budget ou
de priorité ne peut restreindre le périmètre du produit ; une restriction demande une décision
explicite de l'utilisateur qui **nomme ce qui est retiré**. Changer l'ordre des jalons demande de
montrer une dépendance, pas une préférence.

Invariants relus : I-03, I-04, I-05, I-10, I-11, I-15, I-17 ; aucun amendé.

## Note datée du 2026-09-13 (S213) — D7 précisé par ADR-131

Sur clarification de l'utilisateur : une incompatibilité mesurée qualifie **l'implémentation
mesurée**, jamais la fonctionnalité ; l'espace d'optimisation (temps, espace, LOD spatial,
spectral et temporel, visibilité, mutualisation) est nommé dans la trajectoire ; chaque mesure
publie techniques présentes, absentes et domaine de validité ; 2 ms s'éprouve sur leur
combinaison ; aucune demande de réduction ne se fonde sur l'échec d'optimisations isolées. D7
n'est pas réécrit. Voir [ADR-131](ADR-131-un-depassement-qualifie-une-implementation.md).
