# ADR-001 — Décomposition de l'eau en quatre couches (B / W / δ / V)

- **Statut** : proposée — pierre angulaire, conditionne ADR-002 à ADR-012
- **Session** : S01
- **Remplace** : la notion de « trois régimes » de `architecture_globale §5`
- **Résout** : `zones_ouvertes §3, §4, §5, §18, §19, §20, §30`

---

## 1. Problème

Le document source pose trois régimes continus : eau simplifiée → zone de transition → simulation
physique. Ce découpage est incomplet, et son incomplétude produit trois impasses mesurables.

### 1.1 Impasse de dispersion

Une houle analytique (Gerstner / spectre) et un solveur volumétrique ne partagent pas la même
relation de dispersion. En eau profonde, l'analytique impose ω = √(gk) exactement ; un solveur
discret impose ω = √(gk)·f(k·dx), avec une erreur qui croît quand la longueur d'onde approche
la taille de cellule. Une vague qui traverse un domaine physique en ressort **déphasée**. Sur un
domaine de 30 m avec 2 % d'erreur de célérité et une houle de période 8 s, le déphasage atteint
un quart de période en une quinzaine de secondes : la frontière de sortie devient visible même
avec une interpolation parfaite. Le document source considère la transition comme un problème
d'interpolation ; c'est un problème de **cohérence de modèle**.

### 1.2 Impasse d'échelle du sillage

`architecture_globale §3.2` propose d'activer « une région longue et étroite » pour la traînée
d'un bateau. La longueur d'onde des vagues transverses de Kelvin vaut λ = 2πv²/g. À 10 m/s,
λ = 64 m ; à 15 m/s, λ = 144 m. Le demi-angle du sillage est 19,47° et sa longueur utile atteint
plusieurs centaines de mètres. Un solveur volumétrique capable de porter ce sillage devrait
couvrir des dizaines de milliers de mètres carrés à une résolution résolvant λ/20. C'est hors
budget de plusieurs ordres de grandeur. **Le sillage lointain ne sera jamais volumétrique.**

### 1.3 Impasse d'autorité

Si la poussée qui renverse un joueur provient d'un solveur volumétrique, alors ce solveur doit
être déterministe et répliqué. Un solveur GPU ne l'est ni entre deux GPU différents, ni entre
CPU et GPU, ni entre deux versions de pilote. `zones_ouvertes §30` cherche à définir « le niveau
de déterminisme nécessaire pour la flottabilité » ; la réponse honnête est qu'aucun niveau utile
n'est atteignable par cette voie. Il faut donc que l'autorité gameplay **ne passe jamais par le
volumétrique**.

---

## 2. Décision

L'eau est représentée par la **somme de quatre couches indépendantes**, chacune avec son propre
modèle, son propre coût, son propre régime d'autorité réseau et son propre cycle de vie.

```
Surface_visible(x,t) = B(x,t) + W(x,t) + δ(x,t)      (couches de surface libre)
Volumes internes     = V                             (réseau hydraulique séparé)
```

### Couche B — Fond analytique (Background)

| | |
|---|---|
| Nature | fonction fermée, évaluable en tout point sans état |
| Portée | planétaire, permanente |
| Contenu | état de mer spectral, houles longues, marée, niveau moyen |
| Coût | O(1) par point d'échantillonnage, aucune mémoire par cellule |
| Déterminisme | **exact** — fonction pure de (position, temps, descripteur de région) |
| Réseau | rien à répliquer sauf le descripteur (≈128 o/région) et l'horloge |
| Autorité | implicite : identique chez tous par construction |

B est le seul état permanent de l'océan. **Il n'a aucune représentation par cellule.**

### Couche W — Couche d'ondes (Wave layer)

| | |
|---|---|
| Nature | perturbations propagatives à dispersion correcte : paquets d'ondes lagrangiens et/ou champ de hauteur 2D résolu sur GPU |
| Portée | régionale, du mètre au kilomètre |
| Contenu | sillages, anneaux d'impact, ondes d'explosion, tsunamis, déferlement, réfraction bathymétrique |
| Coût | ≈32 o par paquet ; quelques milliers de paquets par région |
| Déterminisme | **exact** — les paquets sont dérivés d'événements horodatés, pas d'un solveur libre |
| Réseau | on réplique les **événements sources**, jamais l'état résultant |
| Autorité | serveur sur les événements ; expansion locale identique chez tous |

W est la couche que le document source ne nommait pas, et dont l'absence rendait §4, §20 et §30
insolubles. C'est elle qui porte tout ce qui est simultanément **grand, lointain et
gameplay-pertinent**.

### Couche δ — Champ perturbatif volumétrique (Delta)

| | |
|---|---|
| Nature | solveur 3D à surface libre, résolvant l'**écart** à (B+W), pas le champ total |
| Portée | métrique — quelques domaines actifs, quelques dizaines de mètres chacun |
| Contenu | proche-coque, gerbe d'étrave, éclaboussure, cavité d'impact, poche d'air, remous sur rocher |
| Coût | budget dominant, borné par l'ordonnanceur (ADR-012) |
| Déterminisme | **aucun** — assumé, revendiqué |
| Réseau | non répliqué |
| Autorité | **jamais autoritaire sur le gameplay** (invariant I-04) |

δ tend vers 0 en s'éloignant de sa source. Ce n'est pas une contrainte imposée de l'extérieur :
c'est la définition de la couche. Un domaine dont δ ne décroît pas vers ses bords est un domaine
mal dimensionné, pas un domaine à agrandir.

### Couche V — Volumes finis et réseau hydraulique

| | |
|---|---|
| Nature | graphe de nœuds (contenants) et d'arêtes (fuites, vannes, débordements, infiltration) |
| Portée | intérieurs de bâtiments, de navires, de vaisseaux ; citernes, piscines, flaques |
| Contenu | quantité de liquide quantifiée, hauteur, pression, débits |
| Coût | négligeable — résolu à basse fréquence (1–10 Hz) en arithmétique entière |
| Déterminisme | **exact** (ADR-010) |
| Réseau | état répliqué, faible fréquence, serveur autoritaire |

V ne partage aucune surface libre avec B/W/δ. Un volume V peut *exposer* une surface visuelle et
*déclencher* un domaine δ local, mais sa comptabilité de masse lui est propre.

---

## 3. Conséquences

### 3.1 Ce que la décision résout

- **§3 « données minimales de la fausse eau »** — la réponse est : *aucune donnée par cellule*.
  L'état simplifié n'est pas stocké, il est **dérivé**. Le coût de la fausse eau devient
  structurellement nul. Détaillé en ADR-004.
- **§4 « algorithme de zone de transition »** — la transition ne mélange plus deux champs
  concurrents. Elle amortit δ vers zéro sur une couche éponge. Plus de réflexion numérique à
  traiter, plus de conservation de phase à assurer : la phase est portée par B, qui traverse le
  domaine sans jamais être discrétisée. Détaillé en ADR-005.
- **§30 « autorité multijoueur »** — on réplique des événements, pas des champs. Détaillé en ADR-009.
- **§20 « très grands événements »** — un tsunami est un objet de la couche W, pas un très grand
  domaine δ. Le seuil recherché n'est pas un seuil de taille : c'est un **changement de couche**.
- **§22 « flottabilité »** — elle s'évalue sur B+W, analytiquement, sans lecture GPU. ADR-008.
- **§18/§19 « choix et changement de solveur »** — le choix ne porte plus sur *un* solveur mais
  sur deux emplacements indépendants (W et δ), chacun remplaçable seul. ADR-007.

### 3.2 Ce que la décision coûte

- **Deux solveurs à écrire au lieu d'un**, plus le graphe V. C'est le prix ; il reste inférieur au
  prix d'un solveur unique censé couvrir six ordres de grandeur spatiaux.
- **Le régime perturbatif a un domaine de validité** (§3.3).
- **La bathymétrie devient une donnée de simulation streamée**, pas seulement un maillage de
  collision — parce que W doit calculer réfraction, levée et déferlement.

### 3.3 Point de rupture assumé : régime substitutif

Quand |δ| n'est plus petit devant |B+W| — rouleau de déferlement, piscine, coque qui émerge
entièrement, cavité traversante — la décomposition additive perd son sens : additionner une
houle analytique à un champ où il n'y a plus d'eau produit une aberration.

Le domaine bascule alors en **mode substitutif** : il devient propriétaire du champ total dans
son emprise, et B+W ne l'alimentent plus que par ses frontières (générateur de vagues en entrée,
absorbeur en sortie).

Critère de bascule proposé, à calibrer : `max|δ| > 0,35 · Hs_local`, ou domaine substitutif par
nature (volume fini, intérieur, zone de déferlement identifiée par la bathymétrie).

Les deux modes coexistent. Le substitutif est plus cher et plus fragile ; il est réservé aux cas
où le perturbatif est **faux**, pas aux cas où il est simplement imprécis.

---

## 4. Alternatives écartées

| Alternative | Motif de rejet |
|---|---|
| Solveur unique multi-échelle (AMR d'un champ unique) | l'AMR ne franchit pas six ordres de grandeur ; le pas de temps est dicté par la cellule la plus fine et pénalise tout le domaine (SPEC-001 §2) |
| Les trois régimes du document source | ne résout ni la dispersion (§1.1) ni le sillage (§1.2), et laisse l'autorité gameplay dans le volumétrique (§1.3) |
| Tout en champ de hauteur 2D | interdit rouleaux, cavités, poches d'air et immersion, explicitement exigés |
| Tout en volumétrique | hors budget de plusieurs ordres de grandeur |

---

## 5. Ce qui reste ouvert

1. Technique de W : paquets d'ondes lagrangiens vs champ de hauteur GPU vs hybride → benchmark B2.
2. Technique de δ : FLIP/APIC, MPM, eulérien, hybride → benchmark B3. Sans incidence sur cet ADR grâce à ADR-007.
3. Calibration du seuil perturbatif/substitutif → benchmark B4.
4. Comportement de B et W en référentiel non inertiel → ADR-002.

## Note corrective du 2026-09-10 (S161)

Le critère de bascule proposé au §3.3 — `max|δ| > 0,35 · Hs_local` — est **retiré comme
paramétrage**. La première mesure du premier volet de B4 montre que le rapport à `Hs` ne gouverne
pas la validité de l'addition : à `max|δ|/h` égal, l'écart d'additivité est le même que la
perturbation vaille 10 % ou 100 % de l'onde de fond. Ce qui le gouverne est **l'amplitude rapportée
à la profondeur**, `écart ≈ 0,24 · max|δ|/h` en régime peu profond.

**La décomposition, elle, n'est pas remise en cause** : elle tient à moins de 1 % tant que
`max|δ| ≤ 0,04·h`. C'est le paramètre qui était mal choisi, pas la décision.

Voir [ADR-111](ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md) et
[B4-DEBLOCAGE-S161](../validation/B4-DEBLOCAGE-S161.md). Le régime d'eau profonde reste ouvert :
Saint-Venant est non dispersif, et la variable y serait vraisemblablement la cambrure.

## Note de portée du 2026-09-10 (S162)

**ADR-112 remplace les conclusions de choix du paramètre de bascule d'ADR-111.** Le montage
S161 compare des évolutions indépendantes, alors que SPEC-004 §6.1 prévoit un résidu couplé
au fond, avec termes croisés et source. Les mesures S161 sont conservées ; elles ne suffisent
pas à recevoir ce couplage ni à choisir sa bascule. Aucun seuil n'est rétabli ou gelé.
Voir [ADR-112](ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md).