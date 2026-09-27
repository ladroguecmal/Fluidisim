# ADR-013 — Prédiction, activation et précalcul

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §8, §9, §10, §11, §12, §13, §14`
- **Dépend de** : ADR-005, ADR-006, ADR-012

---

## 1. Le critère unique : espérance de gain

Les documents sources traitent séparément le filtre de prédiction, les paliers de confiance,
l'erreur acceptable et la frontière préparation/avance physique. Les quatre se ramènent à une
seule inégalité.

```
Préparer vaut la peine si :   p · C_évité  >  (1 − p) · C_gaspillé
                    donc si :  p  >  C_gaspillé / (C_évité + C_gaspillé)
```

- `p` : probabilité que l'interaction ait lieu telle que prédite
- `C_évité` : pic de coût que la préparation supprime au moment critique
- `C_gaspillé` : coût de la préparation, perdu si la prédiction est fausse

Ce n'est pas une reformulation cosmétique : elle donne un **seuil calculable au lieu d'un seuil à
choisir**, et elle explique pourquoi la réponse diffère selon le type de préparation.

| Type de préparation | `C_gaspillé` | Seuil `p` requis |
|---|---|---|
| Réserver de la mémoire dans le pool | ≈ 0 | ≈ 0 — toujours faire |
| Allouer les blocs, construire les voisinages | faible | ≈ 0,2 |
| Préparer les proxys de collision | faible | ≈ 0,2 |
| **Avancer la simulation dans le temps** | élevé | **≈ 0,8** |

D'où la frontière que `§14` cherchait : **préparer est presque toujours rentable, simuler le futur
ne l'est presque jamais** — sauf pour les domaines à long temps d'établissement (§4 ci-dessous).

## 2. Paliers de confiance — dérivés, non choisis

`zones_ouvertes §12` demande combien de paliers et selon quels critères. Le critère se déduit
d'une comparaison géométrique.

Un objet contrôlable de capacité de manœuvre `a_max` peut déplacer son point d'impact de
`Δ(t) = ½·a_max·t²` en `t` secondes restantes. Préparer une région d'impact n'a de sens que si
cette enveloppe tient dans le domaine qu'on allait construire de toute façon :

```
Δ(t) ≤ R_domaine     ⟺     t ≤ √(2·R_domaine / a_max)
```

| Objet | `a_max` | `R_domaine` | Horizon utile |
|---|---|---|---|
| Avion de chasse | 20 m/s² (≈2 g latéral) | 20 m | **1,4 s** |
| Avion en perte de contrôle | 3 m/s² | 20 m | 3,7 s |
| Véhicule balistique (sorti d'un pont) | 0 | 20 m | limité seulement par le temps de vol |
| Vaisseau lourd | 5 m/s² | 60 m | 4,9 s |

**Un avion pleinement contrôlable ne peut pas être prédit utilement au-delà d'une seconde et
demie.** Ce n'est pas un défaut du système de prédiction : c'est une propriété de l'objet. Toute
tentative d'enveloppe plus large coûterait plus cher que le démarrage tardif — exactement le cas
que `§12` soupçonnait sans pouvoir le trancher.

Paliers retenus, définis par ce que l'on engage :

| Palier | Condition | Engagement |
|---|---|---|
| **T4 — veille** | objet identifié comme pouvant atteindre l'eau | rien ; réévaluation à 2 Hz |
| **T3 — réservation** | `t_impact < 8 s` et trajectoire bornée | réservation mémoire dans le pool, coût nul |
| **T2 — construction** | `Δ(t) ≤ R_domaine` (formule ci-dessus) | blocs alloués, voisinages et proxys construits, δ reste à 0 |
| **T1 — actif** | `t_impact < 0,3 s` ou contact | le domaine est passé à l'ordonnanceur |

## 3. Erreur acceptable d'un précalcul : la question disparaît

`zones_ouvertes §13` cherche un seuil de position, de rotation et de temps au-delà duquel un
précalcul devient invalide.

En palier T2, **δ vaut identiquement 0**. Le domaine préparé ne contient aucune information
physique : il ne contient que des blocs alloués, des voisinages et des proxys. Un déplacement du
point d'impact prévu se corrige en **translatant le domaine** — c'est-à-dire en ré-indexant un
ensemble épars de blocs (ADR-006 §3.1). L'opération est presque gratuite et **sans erreur**,
puisqu'il n'y a rien à transporter.

Les seuls seuils réels sont donc :

1. la nouvelle position sort de l'empreinte mémoire réservée → réallouer (coût faible) ;
2. le niveau de `dx` requis change (objet plus rapide/plus gros que prévu) → rebâtir ;
3. l'événement n'aura pas lieu → libérer.

Il n'existe **aucun seuil de tolérance physique à calibrer**. C'est un cas où la conception a
supprimé une inconnue au lieu de la résoudre, et il vaut la peine de comprendre le mécanisme : la
question n'existait que parce qu'on supposait le précalcul *physique*.

## 4. Avance temporelle : le seul cas légitime

Un domaine **perturbatif** naît correct à δ = 0 : rien à établir. Un domaine **substitutif** naît
faux — sa zone de relaxation doit se remplir et le train de vagues doit traverser le domaine avant
que l'état soit crédible.

```
t_établissement ≈ 1 à 2 périodes de la houle incidente + L_domaine / c_groupe
```

Pour une zone de déferlement avec `T = 8 s`, `L = 120 m`, `c_groupe ≈ 5 m/s` :
`t_établissement ≈ 16 + 24 = 40 s`.

**Quarante secondes.** Aucune fenêtre de prédiction ne couvre cela. Conclusions :

- les zones de déferlement ne peuvent pas être créées à la demande ;
- soit elles sont **permanentes** aux endroits qui le méritent (une plage de niveau, quelques-unes
  par carte), soit elles sont **initialisées depuis un état précalculé** stocké hors ligne, ce qui
  est précisément ce que `zones_ouvertes §27` envisageait sans en connaître la nécessité.

Le précalcul côtier n'est donc pas une optimisation : c'est une **condition de faisabilité**.
Volume de données à prévoir : N états × (bathymétrie locale × état de mer × marée). Un
échantillonnage grossier (4 états de mer × 4 phases de marée = 16 états par plage) suffit
probablement, avec interpolation par relaxation courte (2–3 s) vers l'état courant. À valider.

## 5. Activation et désactivation : seuils de départ

Score d'activation `s ∈ [0,1]` composé selon ADR-012 §2, avec hystérésis 0,60 / 0,40.

Valeurs de départ, **toutes à calibrer** :

| Grandeur | Valeur de départ | Justification |
|---|---|---|
| Durée de vie minimale d'un domaine | 0,75 s | au-delà du temps de réaction du joueur, en deçà de la mémoire perceptuelle |
| Seuil d'énergie résiduelle de désactivation | 1 % du pic, ou plancher absolu | en dessous, l'amplitude est inférieure au clapot de B |
| Délai avant désactivation après passage sous le seuil | 1,0 s | anti-battement |
| Réévaluation d'un candidat T4 | 2 Hz | |
| Réévaluation d'un candidat T2 | 30 Hz | |
| Fraction d'écran déclenchant `focal` | 8 % | à confronter au ressenti |

## 6. Hors caméra (`§9`) : ce que le client conserve

La hiérarchie du document source est conservée, mais son implémentation est triviale ici :

| Niveau | Traitement réel |
|---|---|
| Visible et important | domaine δ actif |
| Hors caméra mais important | **destruction du domaine**, transduction δ→W. W continue de propager, gratuitement et correctement |
| Potentiellement visible plus tard | l'événement W subsiste ; son amplitude décroît analytiquement |
| Insignifiant | l'événement W est retiré |

Autrement dit : **il n'existe pas de « simulation ralentie hors caméra »**. Le repli hors caméra
n'est pas une simulation dégradée, c'est un changement de couche. Cela supprime toute la
question `§9` « quelle méthode mathématique pour la simulation hors caméra » — et supprime aussi
le risque, très réel, d'un budget consommé par des zones que personne ne regarde.

## 7. Ce qui reste ouvert

1. Calibration de tous les seuils (B8).
2. Table `a_max` par archétype d'objet contrôlable — à obtenir auprès de l'équipe véhicules.
3. ~~Format et volume exact de la bibliothèque d'états côtiers précalculés (§27) → dépend de B4.~~
   **Clos.**
   → **S11** : répondu dès S06, sans banc : SPEC-005 §6 donne le format — condition initiale 2D à 0,5 m,
   quatre champs `f16` — et les volumes : **77 Ko par état, 1,2 Mo par plage, 60 Mo pour cinquante
   plages**. ADR-022 §3 a généralisé le type en `SeedState`. Ce qui dépend réellement de B4 est le
   seuil de tolérance sur les paramètres d'une graine, et ce point existe : ADR-022 §7.1.
4. ~~Cas des rochers turbulents permanents (`§26`) : traités comme **émetteurs W stationnaires**
   dépendant de la houle locale.~~ *(S11 : source de données réglée par SPEC-005 §2, comportement
   non spécifié.)* **Spécifié en S12 → [ADR-023](ADR-023-mecanismes-restes-a-specifier.md) §4, et
   le mot « émetteur » a été écarté.** Deux cents sites émettant un événement par seconde feraient
   9 000 o/s par joueur intéressé — dix fois une bataille navale, en permanence, pour du décor. Un
   site est un **terme stationnaire dérivé**, re-calculé à la demande et jamais répliqué, publié
   comme `BreakerVertex` sur le canal existant de SPEC-006 §6. Sa liste se dérive de
   `h < 1,28·H_local` (McCowan) ; la marée l'allume et l'éteint sans réglage.

**Note du 2026-09-27 (S401) — l'enveloppe du §2 construite, en référence** ([preuve](../validation/DOMAINE-EPARS-S401.md) §5).
`useful_horizon` et `Follow` (`domain_blocks.rs`) : les blocs à moins de `r_c` des disques de centre `p + V·t` et de rayon
`r + ½·a_max·t²`, jusqu'à l'horizon `√(2R/a_max)` ; 100 % de 300 manœuvres bornées dedans. **Mesuré au banc** : revu à chaque pas
ou toutes les 0,5 s à 10 m/s, l'ensemble garde la source par la seule dilatation de l'activité, et l'enveloppe multiplie ses
mailles calculées par 1,7 à 2,6 sans changer l'écart ; revu chaque seconde, la source en sort à 0,62 s sans elle. La référence met
l'enveloppe dans l'ensemble **calculé** ; le §2 la range au palier **T2** — blocs alloués, δ à 0 —, ce que les mesures appuient.
La décision ne change pas ; la séparation T2 / T1 est à la file.

**Note du 2026-09-27 (S405) — l'objet balistique du §2, prédit** ([preuve](../validation/IMPACT-PREVU-S405.md)). `ballistic`
(cœur) : point, instant, vitesse, orientation et rotation à l'impact d'un objet sous gravité et traînée quadratique, en rotation
libre, contre la houle telle qu'elle sera ; la région utile couvre une traînée bornée ; `tier` rend le palier du §2 — lu ainsi :
T2 exige aussi `t < 8 s`, et un objet balistique y entre dès lors. Consommé par le domaine épars en mer, le temps de vol devient
la fenêtre de préparation que la source (§8.2) attend : 1,11 s d'avance, quand le suivi sans prédiction dépend de la cadence de
revue. La décision ne change pas ; la région reste calculée en référence, non réservée (T2 contre T1, note S401).
