# ADR-045 — La saturation d'état n'est pas un filet, c'est un détecteur de divergence — et il était muet

- **Statut** : proposée
- **Session** : S38
- **Tranche** : angle mort **A146**, action **S34-1**, reportée quatre fois ; action **S37-3**
- **Corrige** : rien n'est réécrit. **A146** est requalifié, pas retiré — sa formulation visait le
  mauvais danger, et le §4 dit lequel.
- **Produit** : les compteurs de saturation des deux solveurs ;
  [`AUDIT-SATURATIONS-S38`](../registres/AUDIT-SATURATIONS-S38.md) ; la mise hors de cause de la
  saturation dans le résidu de S37
- **Clôt** : actions **S34-1** et **S37-3**
- **Ouvre** : **A165**

---

## 1. Ce qui était craint, et ce qui a été mesuré

**A146**, ouverte en S34 : *« les saturations de modèle ne sont comptées nulle part, et une
saturation fréquente est un défaut »*. Le danger énoncé était un solveur qui produirait des états
impossibles à chaque pas derrière un filet complaisant — `h.max(0)` sur le lit sec de C04 était
nommément suspecté.

Les deux véhicules portent la même saturation, écrite indépendamment :

```rust
if h < 0.0 { h = 0.0; hu = 0.0; }
```

Elle **crée de la masse** — remonter `h` de `−5·10⁻⁷` à `0` ajoute de l'eau — et **détruit de la
quantité de mouvement** sans transfert. Ni l'une ni l'autre n'était comptée.

**Mesure, `CFL = 0,45`, trois cas, deux véhicules :**

| cas | pas | déclenchements |
|---|---|---|
| C01 — repos sur pente, 60 s | 2 891 / 2 891 | **0** |
| C03 — seiche, 20 périodes | 17 866 / 35 759 | **0** |
| **C04 — rupture sur lit sec**, 2 s | 475 / 454 | **0** |

**Zéro, partout, y compris là où le soupçon portait.** Le flux de Rusanov préserve la positivité
sous sa condition de Courant, et `0,45` est largement dedans.

## 2. Mais une saturation qu'on n'a jamais vue mordre n'a pas été testée

C'est la leçon de S34 (**L118**), et elle s'applique ici sans changement : à ce stade, deux lectures
sont indiscernables — soit le schéma ne produit jamais d'état impossible, soit la condition est
écrite de telle façon qu'elle ne peut pas être vraie.

Le test d'une saturation est **le cas qu'elle doit attraper**. Le levier est physique : au-delà de sa
condition de Courant, Rusanov cesse de préserver la positivité.

| `CFL` | pas (`f32` / `f64`) | déclenchements (`f32` / `f64`) | masse créée / volume | qdm détruite |
|---|---|---|---|---|
| 0,45 | 100 / 100 | 0 / 0 | 0 | 0 |
| **0,95** | 50 / 49 | **0 / 0** | 0 | 0 |
| **1,20** | 77 / **6 890** | **267 / 11 252** | **1,4·10¹⁷** | `NaN` |
| 1,80 | 44 / 1 070 | 188 / 2 651 | 4,1·10¹⁷ | `NaN` |

**La saturation fonctionne, et elle a désormais son témoin** — à `0,95` elle ne mord pas, sans quoi
une condition « toujours vraie » passerait le même test (**L119**).

**Et la frontière tombe exactement sur la condition de Courant** : rien à 0,95, tout à 1,20. Un
compteur qui ne connaît pas cette condition la retrouve.

## 3. Ce que la saturation fait quand elle mord : rien de bon

C'est le point qui requalifie l'objet. Quand elle se déclenche, elle **ne rattrape rien** :

- la masse créée dépasse le volume initial d'un facteur **10¹⁷** côté `f32`, **10¹⁵⁰** côté `f64` ;
- la quantité de mouvement détruite déborde vers `NaN` ;
- le nombre de pas explose — 6 890 au lieu de 100 pour la même demi-seconde — parce que `dt_cfl` se
  recalcule sur des vitesses divergentes.

**Elle ne convertit pas une instabilité en résultat acceptable.** Elle convertit une divergence
franche — qui aurait produit des `NaN` visibles et arrêté tout le monde — en une suite de nombres
finis qui ressemblent à un résultat.

## 4. La décision de fond : ce n'est pas un filet

> **La saturation d'état est un détecteur de divergence, et il était muet.**
>
> Elle ne se déclenche que lorsque le calcul a **déjà** cessé d'être valide. Un premier
> déclenchement est donc l'événement le plus informatif que ce solveur puisse produire — et jusqu'à
> S38, rien ne permettait de le voir.

**A146 avait raison de s'inquiéter, et tort sur la raison.** Le danger n'est pas une saturation
fréquente en régime nominal — elle ne se déclenche jamais. Le danger est **une seule** saturation en
régime dégradé, qui passe inaperçue et laisse le harnais rendre un rapport.

## 5. Les décisions

**D1 — la saturation d'état est requalifiée en *détecteur de divergence*.** Son compteur est exposé
au rapport du mode `physics`, et **un compteur non nul est un échec du cas**, non un avertissement.
Ce n'est pas une tolérance : au-delà de la condition de Courant, aucune grandeur mesurée n'a de sens.

**D2 — chaque saturation garde son couple de tests** : le cas qu'elle doit attraper, et le témoin
qu'elle ne doit pas attraper (**L119**, **A147**). Les deux existent désormais pour S7 et T4.

**D3 — les compteurs d'événements détectent, ils ne quantifient pas.** À `CFL = 1,2`, `masse_creee`
vaut `10¹⁵¹` et `qdm_detruite` vaut `NaN` : la grandeur mesurée est réellement absurde, et ces deux
nombres ne servent qu'à alerter. Le compteur d'événements, lui, reste exact — c'est lui qui porte le
verdict.

**D4 — tout point de saturation passe par une fonction unique et nommée.** Le recensement écrit à la
lecture avait manqué l'étage intermédiaire de RK2 ; il n'a été vu que parce que le compilateur a
refusé l'appel restant après la transformation de `saturer` en méthode. *Ce qui n'est pas forcé de
passer par un point unique échappe à l'inventaire* — même conclusion que S34 sur G10.

## 6. Ce que l'audit a trouvé en passant, et qui n'est pas une saturation

Le résidu de `10⁻¹⁰ m` que `shallow.rs` laisse derrière le front (action **S37-3**) **ne vient pas
de la saturation** : les compteurs sont à zéro sur C04. Il vient du seuil de sec — encore **A163**.

Et une découverte que rien n'annonçait :

> **Un seuil de sec n'assèche pas une cellule : il l'empêche seulement de bouger.** Le seuil coupe
> la **vitesse** — `u = 0` sous `h_sec` — et non le **flux de masse**. La diffusion de Rusanov,
> `α·(h_R − h_L)`, continue à déposer de la matière dans une cellule déclarée sèche, même quand les
> deux vitesses sont nulles.

`delta.rs` porte trois cellules de film sous son propre seuil, `shallow.rs` en porte dix-huit — dans
le rapport qu'on attend de seuils séparés par quatre ordres de grandeur. Angle mort **A165**.

## 7. Ce qui reste ouvert

1. **S5, la saturation de bord, n'a jamais été vue mordre** et n'a pas encore de cas de
   déclenchement. Sa condition est atteignable en principe — un bord presque sec sur fond montant —
   mais personne ne l'a exercée.
2. **Le seuil de sec reste à trancher** (**A163**, action **S37-1**). Cette session l'a croisé trois
   fois sans le toucher, et le §6 ajoute une raison de plus de le décider : il gouverne aussi la
   longueur du film derrière le front.
3. **La conservation de la quantité de mouvement n'est mesurée par aucun cas canonique.** C01 mesure
   la dérive du volume ; rien ne surveille `hu`. La saturation en détruit, et personne ne le verrait
   sans le compteur ajouté ici.
4. **Le rapport de C04 n'a pas d'assertion de conservation**, contrairement à C01. Si une saturation
   s'y déclenchait, aucune grandeur du cas ne le refléterait — seul le compteur de D1 le dirait.
