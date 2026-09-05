# ADR-017 — Phases : glace et vapeur

- **Statut** : proposée — dépend de l'arbitrage n° 2 (voir `00_INDEX.md`)
- **Session** : S02
- **Comble** : angle mort A19
- **Dépend de** : ADR-004, ADR-010, ADR-015

---

## 1. Décision

**La glace appartient au système d'eau.** `liquid_id` est complété d'une `phase` ∈
{liquide, glace, vapeur}, et la transition est pilotée par la température de `HydroSample`.

L'alternative — traiter la glace comme un objet solide de décor — fonctionne jusqu'à la première
fois qu'il faut la briser, la faire fondre, ou faire flotter un morceau. À ce moment-là, deux
systèmes se disputent la même masse d'eau et la comptabilité de V est fausse.

---

## 2. Formation : une équation, pas un solveur thermique

L'épaisseur de glace suit l'équation de Stefan, qui n'intègre qu'une variable :

```
h = √( 2·k·ΔT·t / (ρ_glace · L_fusion) )        k = 2,2 W/m/K ; L = 334 kJ/kg
```

Sous forme utilisable, avec `FDD` = degrés-jours de gel cumulés (K·jour) :

```
h ≈ 0,035 · √FDD          [m]
```

| Conditions | FDD | Épaisseur |
|---|---|---|
| −10 °C pendant 1 jour | 10 | 11 cm |
| −10 °C pendant 5 jours | 50 | 25 cm |
| −20 °C pendant 10 jours | 200 | 50 cm |
| −5 °C pendant 30 jours | 150 | 43 cm |

Un accumulateur `FDD` par nœud de lac ou par cellule `HydroGrid` suffit. Coût : un flottant et une
addition par tick lent.

### 2.1 La mer agitée ne gèle pas en plaque

Une surface soumise à la houle forme du frasil puis de la glace en crêpes, pas une plaque. Règle
retenue : la formation d'une plaque continue exige `Hs < 0,15 m`. La glace apparaît donc d'abord
dans les eaux abritées — ce que le paramètre `fetch` d'ADR-004 fournit déjà. Encore un
comportement juste obtenu sans code dédié.

---

## 3. Portance : la loi est en h², pas en flottabilité

L'intuition dit que la glace porte par flottabilité. Une plaque de 10 cm ne dépasse la surface que
de 8 mm (`ρ_glace = 917 kg/m³`, donc 8,3 % émergé) et ne « porte » ainsi que ≈8 kg/m².

Ce qui porte réellement, c'est la **résistance en flexion de la plaque**, qui croît comme le carré
de l'épaisseur (formule de Gold, `P ∝ h²`). D'où la table de sécurité usuelle, directement
exploitable comme réglage de conception :

| Épaisseur | Charge admissible |
|---|---|
| 5 cm | rien — dangereuse |
| 10 cm | une personne à pied |
| 20 cm | un groupe, une motoneige |
| 30 cm | une voiture légère |
| 50 cm | un camion léger |

**Conséquence** : doubler l'épaisseur quadruple la charge. Le passage du « ça tient » au « ça
cède » est brutal, ce qui en fait une bonne mécanique de tension.

---

## 4. Rupture

Une cellule de glace cède quand la charge locale dépasse `P(h)`. Le système d'eau :

1. convertit la cellule en eau libre + un ou plusieurs **floes** (objets rigides flottants,
   flottabilité par ADR-008, mode contraint pour les petits) ;
2. émet un `WaveEvent` de type `rupture` (bruit, onde, écume) ;
3. rend la masse correspondante à V ou à la masse d'eau parente.

La glace émet donc des événements comme n'importe quelle autre source. Aucun chemin nouveau.

---

## 5. Vapeur

**Pas de simulation.** La vapeur est :

- un **retrait de masse** dans V, à un débit fonction de l'apport thermique ;
- un **effet volumétrique de rendu** (ADR-007 §4 : la simulation produit des champs, le rendu
  compose) ;
- éventuellement un `WaveEvent` si la vaporisation est assez violente pour perturber la surface —
  tuyère au ras de l'eau, coulée de métal, explosion sous-marine chaude.

Le cas du vide (ébullition puis gel) est traité en ADR-015 §5, et c'est le seul endroit où
vaporisation et solidification se produisent ensemble.

---

## 6. Fonte

Symétrique de la formation : `FDD` négatif fait décroître `h`. La masse fondue **rentre dans la
comptabilité de V** — sans quoi un lac gelé puis dégelé perd ou gagne de l'eau à chaque cycle,
défaut qui n'apparaît qu'après des dizaines d'heures de jeu et qui est très pénible à diagnostiquer
après coup.

Journaliser l'écart de masse au passage de phase, comme pour le couplage V ↔ δ (ADR-010 §6).

---

## 7. Ce qui reste ouvert

1. **Arbitrage humain requis** : le projet veut-il de la glace ? Cet ADR est écrit pour être prêt,
   pas pour imposer le besoin.
2. Granularité de la plaque : cellule `HydroGrid` (64 m) est trop grossière pour une rupture
   crédible. Prévoir une subdivision dédiée, probablement 2 à 4 m.
3. Neige sur glace : isole et ralentit la croissance d'un facteur 2 à 3. À inclure ou à ignorer.
4. Rendu de la glace (transparence, bulles emprisonnées, fractures) — équipe rendu.
5. Interaction glace ↔ navires : brise-glace, coque prise dans les glaces. Ouvre un modèle de
   contact plaque/coque qui n'est pas trivial. À cadrer avant tout engagement de design.
