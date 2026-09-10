# Plan de benchmark et de validation

Toute valeur marquée « à calibrer » dans un ADR renvoie à l'un des bancs ci-dessous. Un banc
produit une **décision**, pas un rapport.

---

> **S03** — Le harnais est désormais spécifié : [`SPEC-003-harnais-de-validation.md`](SPEC-003-harnais-de-validation.md),
> et sa batterie physique dans [`CAS-CANONIQUES.md`](CAS-CANONIQUES.md). La section 0 ci-dessous en
> reste le résumé ; en cas de divergence, SPEC-003 fait foi. Deux points ajoutés en S03 s'appliquent
> à **tous** les bancs :
>
> - **iso-qualité, pas iso-résolution** (SPEC-003 §5.2) — obligatoire pour B3, sous peine de
>   désigner le mauvais candidat avec des chiffres à l'appui ;
> - **paire nulle** dans toute comparaison perceptuelle (SPEC-003 §5.3) — sans plancher de bruit,
>   un jury produit du bruit présenté comme une donnée.

## 0. Harnais commun

ADR-003 §3 rend le monde d'eau entièrement reproductible : un scénario est défini par
`(T_sim initial, descripteurs de région, journal d'événements, trajectoires d'acteurs)`.

Le harnais est donc **sans interface, rejouable et comparable image par image**. Il doit exister
avant B1 ; c'est le seul élément d'infrastructure sur le chemin critique.

Métriques produites automatiquement pour chaque exécution :

| Métrique | Définition | Pourquoi |
|---|---|---|
| `cpu_p50 / cpu_p99` | contribution du système d'eau par tick | **le p99 décide**, jamais la moyenne |
| `gpu_p50 / gpu_p99` | idem | |
| `latence_échantillon` | âge de la donnée renvoyée par `sample()` | piège ADR-007 §4.1 |
| `dérive_masse` | (masse mesurée − masse attendue)/masse, par seconde | détecte les solveurs qui fuient |
| `réflexion_frontière` | amplitude renvoyée / amplitude incidente à l'éponge | cible < 1 % |
| `pics_alloc` | allocations à l'exécution | doit rester à 0 (I-06) |
| `écart_hash` | divergence de B entre deux plateformes | doit rester à 0 (I-03) |

**Piège de méthode à éviter.** Un banc qui compare des solveurs sur le coût par cellule
sélectionnera le solveur au meilleur débit et écartera le solveur à faible latence. Or la latence
est structurante (ADR-007 §4.1). Chaque banc de solveur doit donc publier un couple
`(coût, latence)` et la décision se prend sur les deux.

---

## B1 — Champ de fond : nombre de composantes et coût d'évaluation

**Question.** Combien de composantes pour B, et à quel coût par échantillon ?
**Protocole.** N ∈ {32, 64, 128, 256}. Mesure du coût d'un échantillon CPU avec LOD spectral actif
et inactif. Évaluation subjective en double aveugle sur trois états de mer.
**Décision.** N retenu, et courbe `nombre de composantes effectives = f(distance, taille d'objet)`.
**Ajout S05 (écart R09).** Mesurer aussi la **distance à partir de laquelle la répétition d'une
tuile FFT devient perceptible**, en fonction de sa taille. ADR-004 §3 démontre qu'un océan pavé
produit des artefacts ; §6.2 admet néanmoins une tuile FFT pour le détail haute fréquence. La
tension est acceptable — une répétition n'est pas une bande de calme, et ce détail ne porte aucune
donnée gameplay — mais sans ce chiffre, la taille de tuile sera choisie à l'œil.
**À surveiller.** Le LOD spectral ne doit pas modifier la hauteur perçue à l'échelle d'un bateau,
sinon la flottabilité dépendra de la distance de la caméra — défaut subtil et gênant.

## B2 — Couche W : technologie et λ_cut

**S152 : partiellement exécuté**, [BANC-B2-S152](BANC-B2-S152.md), ADR-105.
Impact80m/60s, choix de profils256/512, coût et restauration30s. Technologie globale,
lambda_cut et capacité restent ouverts ; énergie60s est la prochaine mesure S152-1.

**Question.** Paquets d'ondes lagrangiens, équation d'onde 2D sur pyramide GPU, ou Boussinesq ?
Et où placer la coupure `λ_cut` avec δ ?
**Protocole.** Trois scénarios : sillage de bateau (5, 10, 15 m/s, eau profonde puis 5 m de fond),
anneau d'impact, houle réfractée sur une bathymétrie de plage.
**Métriques.** Erreur d'angle de sillage vs 19,47° (et vs `arcsin(1/Fr_h)` en peu profond) ;
conservation d'énergie sur 60 s ; coût pour 4096 paquets ; déterminisme croisé.
**Décision.** Technologie de W, `λ_cut`, capacité maximale en paquets.
**Priorité 1 du projet.**

> **Mode d'emploi : [`DOSSIER-B2`](DOSSIER-B2.md) *(S16)*.** Scénarios en fichiers concrets, métrique
> d'iso-qualité nommée, procédure de décision, préalables et pièges propres au banc. Trois points en
> sortent qui changent ce protocole : un **cinquième scénario** (B2-05, arrivée en cours de partie,
> que l'ajout S04 ci-dessus réclamait sans qu'aucun scénario ne l'exerce) ; la **métrique d'erreur
> unique** exigée par SPEC-003 §5.2, qui manquait — l'erreur de célérité relative sur C02, intégrée
> sur `[λ_cut, 4·λ_cut]` ; et le fait que **B2 produit un couple et non un nombre**, `λ_cut` avec le
> tableau des taux de décimation admissibles par classe de domaine.

> **Ajout S04 — coût réseau caché, à mesurer et non à supposer.** `advance(t)` doit être une
> fonction pure du journal d'événements (SPEC-004 §5.1). Les paquets lagrangiens le sont ; un champ
> 2D intégré ne l'est pas et impose des points de reprise à stocker, répliquer et transmettre à tout
> joueur qui rejoint. **Le protocole doit donc mesurer, pour chaque candidat : la taille d'un point
> de reprise, sa fréquence, et le volume à transmettre à une arrivée en cours de partie.** Sans
> cela, la comparaison se fera sur la seule qualité visuelle et retiendra l'option la plus coûteuse
> en réseau sans que personne ne s'en aperçoive avant l'intégration.

> **Ajout S05 — critère de recevabilité de `λ_cut`.** ADR-021 §3.2 fait reposer la sécurité et la
> simplicité du modèle réseau sur un argument de fermeture : aucun phénomène de conséquence
> gameplay ne peut naître exclusivement dans δ, parce que tout ce qui dépasse `λ_cut` appartient à
> W. **Une valeur de `λ_cut` qui laisserait tomber un phénomène gameplay dans δ est irrecevable**,
> quelles que soient ses qualités de coût. À vérifier explicitement avant de retenir une valeur.

> **Ajout S15 — le critère de recevabilité porte deux conséquences, pas une.** SPEC-006 §5.6
> s'appuie sur **le même argument de fermeture** pour justifier que le signal de traversabilité
> ignore δ : une perturbation capable de changer une décision de cheminement dépasserait `λ_cut` et
> appartiendrait donc à W, où elle est répliquée. Un relèvement de `λ_cut` remettrait ainsi en cause
> **l'autorité des ondes répliquées et la validité du signal de navigation**. Un banc qui ne
> vérifierait que la première laisserait passer la seconde — et le défaut se manifesterait par des
> PNJ qui traversent un gué chez un joueur et se noient chez un autre.
> *(Action annoncée par SPEC-006 §5.6 en S09, retrouvée non exécutée par l'audit des registres.)*

## B3 — Couche δ : technologie, coût et latence

**Question.** Quel solveur volumétrique ?
**Protocole.** Quatre scénarios imposés, identiques pour tous les candidats :
1. coque en mouvement, `dx` = 0,10 m — mesure du couplage solide et de la stabilité ;
2. impact d'un objet lourd, `dx` = 0,05 m — cavité, jet, refermeture ;
3. domaine substitutif de déferlement, `dx` = 0,25 m — mesure du temps d'établissement réel ;
4. compartiment inondé en référentiel accéléré — validation de `g_eff` injectée.
**Métriques.** `(coût, latence)`, dérive de masse, stabilité CFL, respect du budget imposé,
qualité perçue en double aveugle.
**Décision.** Solveur retenu, ou deux solveurs si aucun ne couvre les quatre cas — l'architecture
l'autorise (ADR-007 §3).

## B4 — Validité du régime perturbatif

**Question.** ADR-001 tient-il ? À partir de quel rapport `|δ|/Hs` la décomposition additive
devient-elle visiblement fausse ?
**Protocole.** Même scène simulée deux fois : (a) perturbative B+W+δ, (b) substitutive intégrale de
référence, à résolution élevée. Comparaison de la surface, des forces sur la coque, et de la
perception en double aveugle.
**Décision.** Seuil de bascule (valeur de départ 0,35·Hs), ou remise en cause d'ADR-001.
**Ce banc est le juge de l'architecture. Il doit être conçu pour pouvoir l'infirmer.**

> **Ajout S04 — la première hypothèse à écarter en cas d'échec.** Un terme source incomplet
> (SPEC-004 §6.1, angle mort A50) produit exactement le symptôme qu'on attribuerait à une faillite
> d'ADR-001 : dérive lente du domaine par rapport au fond, frontière redevenue visible. Le
> protocole doit donc inclure une exécution de contrôle avec `S` calculé sur un réseau non
> dégradé, et une seconde avec `S` volontairement tronqué — l'écart entre les deux mesure la
> sensibilité au terme source avant toute conclusion sur l'architecture.
> Le pas d'échantillonnage du fond (`is_smooth_at`, SPEC-004 §6.2) est un paramètre direct de ce
> banc.

## B5 — Blocs épars et décomposition récursive

**Question.** Taille de bloc 8³ ou 16³ ? La superposition `δ_grossier + δ_fin` fonctionne-t-elle ?
**Protocole.** Coût de fusion/séparation sur 100 événements ; mesure du surcoût de bordure ;
scénario de recouvrement spectral avec et sans filtre passe-bas.
**Décision.** Taille de bloc ; adoption ou rejet des résolutions mixtes.

## B6 — Flottabilité

**Question.** Combien de points d'échantillon, quels coefficients, quelle masse ajoutée ?
**Protocole.** Cinq archétypes (navire, barque, caisse, bouée, débris) sur quatre états de mer.
Mesure de la période propre de roulis et de pilonnement, comparaison aux valeurs attendues ;
vérification de la bascule en mode contraint pour les petits objets (ADR-008 §3).
**Décision.** Table de paramètres par archétype, seuils de sous-cyclage.

## B7 — Budget sur matériel cible

**Question.** Quelles valeurs pour `cpu_sim_ms`, `gpu_sim_ms`, `memoire_blocs`, `domaines_max` ?
**Protocole.** Scène de charge maximale réaliste (bataille navale + littoral + météo) sur chaque
plateforme cible, avec le régulateur d'ADR-012 §5 actif.
**Décision.** Profils de qualité chiffrés ; vérification que le p99 tient.
**À surveiller.** Le pompage du régulateur est plus visible que la dégradation. Mesurer la
fréquence de changement de la manette `q`, pas seulement sa valeur moyenne.

## B8 — Seuils d'activation et de prédiction

**Question.** Toutes les valeurs d'ADR-013 §5.
**Protocole.** Sessions de jeu instrumentées : taux de domaines créés sans jamais être vus,
taux d'événements visibles apparus en retard, fréquence des créations/destructions par minute.
**Décision.** Table de seuils.
**Indicateur clé.** *Taux de gaspillage* = fraction des domaines créés qui n'ont jamais dépassé
2 % de l'écran. Un système bien réglé se situe entre 10 % et 25 % : à 0 %, la prédiction est trop
timide et des événements apparaissent en retard ; au-delà de 40 %, elle brûle du budget.

## B9 — Champ d'écume *(S02)*

**Question.** Résolution, nombre de cascades et demi-vies des deux canaux du champ `F` ?
**Protocole.** Trois scénarios : haute mer à `U10` croissant (vérifier la couverture de moutons
contre la relation de Monahan, SPEC-002 §1), sillage de navire sur 60 s, zone de surf.
**Métriques.** Écart à la couverture attendue ; coût GPU par cascade ; persistance visuelle du
sillage ; apparition ou non des traînées de convergence lorsque l'advection utilise la vitesse
orbitale complète plutôt que le seul courant.
**Décision.** Résolution, cascades, demi-vies, budget particules.
**Piège.** Un réglage d'écume calé à l'œil dérive systématiquement vers le trop-blanc. Le banc doit
comparer à la couverture prédite, pas au goût.

## B10 — Cavité d'entrée dans l'eau *(S02)*, et source d'onde *(S137)*

**Question.** Coefficients de la séquence couronne → cavité → pincement → jet de Worthington.
**Et, depuis S137 :** que le même corps **émet** comme onde de gravité — à quelle longueur
d'onde, et avec quelle part de son énergie.
**Protocole.** Sphères et corps allongés, `Fr` d'entrée de 1 à 15, `dx` = 0,05 m et 0,02 m.
**Métriques.** Profondeur de pincement en diamètres de corps, hauteur de jet, durée, coût.
**Ajout S137, deux métriques de source**, observables sans instrumenter l'entrée :

- **Longueur d'onde dominante.** Chronométrer l'arrivée du maximum d'amplitude à une distance
  `r` du point d'entrée : `λ = 8πr²/(g·t²)`, dérivé de `c_g = ½√(gλ/2π)` (SPEC-001 §1). D'où
  `α = λ/b`, `b` étant la demi-largeur du corps. Deux distances au moins, pour vérifier que le
  résultat n'en dépend pas — trop près le paquet n'est pas dispersé, trop loin l'amplitude passe
  sous le bruit, et **fixer ces deux bornes fait partie du protocole**.
- **Énergie rayonnée.** Énergie du train d'ondes intégrée sur un anneau à distance `r`,
  rapportée à `½ρb³v²` : c'est `η` directement.

**Décision.** Table de coefficients par `Fr`, et seuil de `Fr` en dessous duquel la cavité n'est
pas modélisée. **Et, depuis S137 :** `α` et `η`, avec leur dépendance éventuelle au régime.

> **Critère de réussite, S137.** Ce que la calibration doit resserrer est borné avant la mesure :
> `α ∈ [3,35 ; 6,11]` par dérivation de la forme du modèle (ADR-092), et `η ≤ 2Kgα⁴bs²/v²` par
> sa borne d'énergie. **Une mesure hors de ces bornes ne calibrerait pas le modèle : elle le
> réfuterait** — ce qui est un résultat en soi, et doit être rapporté comme tel plutôt que
> reporté sur les coefficients. Voir [ADR-093](../adr/ADR-093-ou-se-calibre-la-source-d-impact.md).

## B11 — Rendu sous-marin *(S02)*

**Question.** Modèle de diffusion retenu, et coût du profil immergé.
**Protocole.** Trois eaux (océan clair, côtier, estuaire en crue), caméra immergée, caméra à demi
immergée, remontée depuis 30 m.
**Métriques.** Coût du profil immergé vs profil extérieur à budget constant ; stabilité de la
ligne de flottaison sur la caméra à demi immergée ; lisibilité de la fenêtre de Snell par mer
formée.
**Décision.** Modèle de diffusion, répartition du budget en mode immergé.

---

## Séquencement

```
Harnais  →  B1 ─┐
                ├→ B2 (λ_cut) ─┐
                │              ├→ B4 (juge d'ADR-001) →  B6  →  B8
                └→ B3 ─────────┘        │
                                        └→ B5
B7 en continu, sur chaque livraison
```

B2 et B3 peuvent démarrer en parallèle mais convergent sur `λ_cut` : prévoir un point de
synchronisation explicite entre les deux équipes.
