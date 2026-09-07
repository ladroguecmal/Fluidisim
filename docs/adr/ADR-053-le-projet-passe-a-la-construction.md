# ADR-053 — Le projet passe à la construction, et il commence par W

- **Statut** : **ACTÉE** — arbitrage de l'utilisateur, 2026-09-08
- **Session** : S70
- **Tranche** : le point 4 de [`BILAN-S69`](../registres/BILAN-S69.md) §6, remonté comme hors de
  portée d'une session
- **Corrige** : rien n'est réécrit. Le chemin critique de `REPRISE.md` §4 est complété, pas
  remplacé.
- **Applique** : [`ADR-001`](ADR-001-decomposition-en-couches.md) §2 — les quatre couches
- **Clôt** : rien. **Ouvre** : l'écriture de `W`, et avec elle **S63-1** et l'urgence `WaveEvent`

---

## 1. La question, et qui l'a tranchée

`BILAN-S69` a mesuré un écart que soixante-huit sessions avaient laissé implicite : le dépôt est à
**~85 % comme corpus de conception** et à **~15 % comme système utilisable**. `δ`, `W` et `V`
n'existent pas ; onze cas canoniques sur vingt-trois et **onze bancs sur onze** attendent une
couche non écrite.

Le bilan a posé la question sans y répondre, parce qu'elle relève de l'ambition du projet et non
de la conception : *le projet passe-t-il à la construction, ou reste-t-il un corpus de conception ?*
Il notait que **ne pas choisir revient à choisir le second par défaut**, ce qui s'était produit
pendant vingt-deux sessions.

**L'utilisateur a tranché le 2026-09-08 : construction.**

## 2. La décision

### D1 — Le projet construit le système, et l'écriture de code cesse d'être exceptionnelle

Depuis S20, `code/` existait comme **instrument de mesure** : un harnais et des véhicules d'essai,
dont l'en-tête de chaque module rappelle qu'ils *ne sont pas le système*. Cette réserve tombe pour
les couches du projet : **ce qui sera écrit désormais est le système lui-même**, et il est jugé
comme tel — par les cas canoniques, pas par sa ressemblance à un véhicule.

### D2 — On commence par `W`, et le choix n'est pas ouvert : il converge

`ADR-001` §2 définit `W` comme *« des perturbations propagatives à **dispersion correcte** :
paquets d'ondes lagrangiens »*, au déterminisme **exact** parce que dérivés d'**événements
horodatés** et non d'un solveur libre. Quatre besoins indépendants la désignent :

| | Ce que `W` débloque |
|---|---|
| **S63-1** | La couche dispersive cherchée depuis **S22**, jamais planifiée. Elle rend C02 exécutable, donc `λ_cut` mesurable, donc **B2** lançable — le banc décisif |
| **Cas en attente** | **C07** (sillage profond et peu profond) et **C19** (aller-retour de persistance) |
| **Coût d'écriture** | `W` est **analytique** — des paquets, pas un solveur. `δ` est un solveur 3D à surface libre : la pièce la plus lourde du projet, et elle ne commence pas ici |
| **Urgence de format** | **`WaveEvent`** (SPEC-006 §3.1) est *la seule urgence de format du corpus* : structure répliquée à arrêter **avant** que le réseau ne fige son protocole. Écrire `W` force à la trancher |

C'est le seul endroit du graphe où quatre besoins sans rapport entre eux tombent sur la même
pièce. `B` existe déjà ; `V` n'a aucun cas exécutable sans `δ` ; `δ` est le plus lourd.

### D3 — Les règles de tenue du dépôt ne changent pas

La conception reste en Markdown, jamais en page HTML ni en artefact publié. Un ADR n'est jamais
réécrit. Le plan se déclare avant le travail, une étape par commit. Le rituel de fin reste la
dernière étape du plan. **Le code s'ajoute au corpus, il ne le remplace pas** — et la leçon de S21
vaut plus que jamais : *le code est un instrument de mesure de la conception*, et une propriété
numérique revendiquée coûte moins cher à écrire qu'à relire.

### D4 — Ce que la construction ne dispense pas de faire

- **B1 reste le seul banc exécutable aujourd'hui**, et il n'a jamais été lancé. Il tranche le
  nombre de composantes de `B`, question devenue de **justesse** et non de coût depuis **A187**.
  Il ne dépend d'aucune couche à écrire, et il informe `W`, qui se superpose à `B`.
- Les actions ouvertes du harnais ne disparaissent pas ; elles cessent d'être prioritaires.

## 3. L'ordre retenu

```
B1  (mesure, exécutable aujourd'hui)
 └─> WaveEvent arrêté  ──>  W écrit  ──>  C02 ──> λ_cut ──> B2
                             ├─> C07
                             └─> C19
puis  H4 ──> B3          et  δ 3D ──> C05, C06, C09, C21
puis  V                  et  intégrateur de corps rigide ──> C11, C20, C10*
```

**B1 avant `W`** pour une raison de dépendance, pas de commodité : `W` se superpose à `B`, et le
nombre de composantes de `B` conditionne le coût d'évaluation que `W` paiera à chaque point.

## 4. Ce qu'il faudrait pour inverser cette décision

| Inverser | Ce qu'il faudrait |
|---|---|
| **D1** (revenir au corpus seul) | une décision de l'utilisateur, comme celle-ci. **Ce n'est pas une décision de session** — c'est une question d'ambition, et `BILAN-S69` l'a remontée pour cette raison |
| **D2** (commencer par une autre couche) | que `δ` devienne prioritaire pour un besoin de gameplay que le corpus ne connaît pas, ou que `WaveEvent` soit figé ailleurs sans nous. Les quatre motifs de D2 sont indépendants : il en faudrait quatre pour l'inverser |

## 5. Ce que cette décision ne dit pas

- **Elle ne fixe aucune technologie pour `W`.** Paquets lagrangiens, champ 2D sur GPU, ou les deux
  — `ADR-001` §2 laisse les deux ouverts et **B2** est précisément le banc qui tranche. Écrire `W`
  commence donc par ce que B2 exige de comparable, pas par un choix d'implémentation.
- **Elle ne promet pas de calendrier.** Le dépôt n'en a jamais porté, et l'état réel du projet
  reste hors de portée d'une session (`REPRISE.md` §5).
- **Elle ne referme pas les onze bancs.** Elle dit par où on commence à les rendre lançables.
