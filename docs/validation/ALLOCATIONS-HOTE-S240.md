# Les allocations par image de l'hôte GPU — S240, 2026-09-15

Traite le dernier reliquat mesurable de **J1-bis** : **I-06 n'a jamais été mesurée pour la pile
graphique**. [CADENCE-HOTE-S225](CADENCE-HOTE-S225.md) §Suite l'écrit — « les allocations de la pile
graphique (I-06, toujours non reçues) » — et la [feuille de route](../FEUILLE-DE-ROUTE.md) la garde
dans les travaux nécessaires de J1 (ADR-131 D6). Les deux autres reliquats de cette ligne —
interaction manuelle représentative, seconde cible — demandent une personne et une machine, et
restent hors de portée d'une session (REPRISE §5).

## 1. Protocole, écrit avant instrument et mesure

### 1.1 Ce que I-06 dit, et ce qui n'a jamais été vérifié ici

[I-06](../01_INVARIANTS.md) : « **Aucune allocation à l'exécution.** Blocs, domaines, paquets, nœuds
et anneaux d'instantanés viennent de pools dimensionnés par profil au démarrage. » Le cœur la reçoit
par un **allocateur compteur** qui refuse après `seal` ; c'est un contrôle de `code/`, exécuté par la
suite. L'hôte `viewer/` n'a jamais eu cet instrument : ses trois dépendances verrouillées
(wgpu 30.0.1, winit 0.30.13, pollster 1.0.1) allouent ou non par image, et personne ne l'a compté.

**Ce qui n'est pas en cause.** I-06 vise l'eau et son état. La question ouverte ici est de savoir si
elle vaut, telle qu'elle est écrite, pour une couche dont le code n'est pas le nôtre — comme
[ADR-139](../adr/ADR-139-volume-et-plan-oriente-des-contenants.md) a dû amender I-08 pour V.

### 1.2 Le consommateur, et pourquoi maintenant

Le budget de 2 ms (ADR-125) et **A265** — « le recouvrement CPU/GPU varie avec la charge sans que la
mesure l'explique ». **Base relevée avant instrument**, ce jour, même machine et même scène que S235
(`--multi --cadence`, 960×540, secteur) :

| grandeur | médiane | p95 | maximum |
|---|---:|---:|---:|
| intervalle de présentation | 4,7062 ms | 5,8529 ms | **16,4724 ms** |
| CPU par image | 3,9828 ms | 4,8242 ms | **15,7814 ms** |
| dont sillage | 3,1185 ms | — | **13,5879 ms** |
| dont transfert | 0,2209 ms | — | 1,2159 ms |
| acquisition | 0,0237 ms | — | 0,1035 ms |
| présentation | 0,5771 ms | — | 2,2853 ms |
| GPU eau | 0,4399 ms | — | 0,4552 ms |

**Le maximum vaut quatre fois la médiane, et le GPU ne bouge pas** (0,4399 → 0,4552 ms) : la gigue
est entièrement du côté CPU, et surtout dans la préparation du sillage. Une allocation — et la faute
de page de son premier contact — est un suspect direct. Ce protocole ne le suppose pas : il le compte.

### 1.3 L'instrument

Un `#[global_allocator]` qui enveloppe `std::alloc::System` et incrémente des compteurs atomiques
(nombre et octets, allocations et réallocations, libérations). **Aucune dépendance nouvelle** ; la
pile reste celle verrouillée en S210/S211.

**Attribution par phase, pas par site d'appel.** La boucle d'image a déjà une décomposition chiffrée
en millisecondes (S225) : `frame.update` (préparation CPU, à nous), `upload`, acquisition de la
surface, `draw` (encodage et soumission), `present`, et le reste — la boucle d'événements. Les
compteurs sont relevés aux mêmes bornes, de sorte que les allocations se lisent **en face** des
millisecondes déjà publiées. Nommer le site exact demanderait une pile d'appels à chaque allocation,
donc une dépendance et un coût qui déplacerait la mesure : c'est une **limite déclarée**, et une
phase qui alloue se resserre ensuite par bissection.

**Ce que l'instrument ne doit pas faire.** Allouer lui-même : les relevés vont dans des tampons
**réservés avant la boucle**. Les tampons de millisecondes existants seront réservés de la même
façon, car un `Vec` qui grandit pendant un banc de cadence fausse ce même banc.

### 1.4 Critères de réception, déclarés avant construction

1. **Allocations par image en régime** : nombre et octets, médiane, p95 et maximum, les dix
   premières images écartées comme le banc le fait déjà.
2. **Attribution par phase**, lisible en face des millisecondes de S225.
3. **Séparation à nous / à la pile** : ce qui tombe dans `frame.update` est à nous.
4. **L'instrument ne déplace pas les chiffres publiés** : `--multi --verify` (3 mm) et
   `--multi --cadence` rejoués avec et sans lui, écarts publiés. Un `VERIFY` qui bouge invalide la
   mesure, pas le chemin.
5. **Ce qui est à nous est supprimé, ou justifié par écrit**, puis re-mesuré.
6. **Coût** (ADR-131) : techniques présentes, absentes, domaine.

### 1.5 Arrêt

I-06 reçue pour l'hôte — aucune allocation en régime dans ce qui est à nous, et le chiffre de la
pile publié —, **ou** requalification datée de la portée de I-06 pour la pile graphique, avec le
nombre, l'octet et la cause. Aucun seuil inventé, aucun banc modifié pour obtenir du vert.
