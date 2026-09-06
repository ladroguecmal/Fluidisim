# Audit des saturations de modèle — S38

**Ce registre recense les saturations des deux solveurs, dit ce que chacune détruit, et prédit son
déclenchement avant de le mesurer.** Il répond à l'action **S34-1** et à l'angle mort **A146**,
reportés quatre fois.

---

## 1. Ce qu'est une saturation de modèle, et pourquoi ce n'est pas un garde-fou

S34 a audité les **garde-fous du harnais** : chacun a-t-il été vu refuser ? Ils protègent une
**mesure** — ils déclarent un montage inexploitable et rendent la main.

Une **saturation de modèle** est autre chose. Elle vit dans le solveur, elle s'applique **en cours de
calcul**, et elle ne rend la main à personne : elle remplace un état physiquement impossible par un
état acceptable, et le calcul continue. Le solveur ne s'arrête pas, ne signale rien, et **produit un
résultat qui a l'air d'un résultat**.

> **Le critère de cet audit.** *Une saturation rare est un filet ; une saturation fréquente est un
> solveur qui produit des états impossibles et qu'on maquille.* Les deux sont indiscernables tant
> que personne ne compte (**A146**).

## 2. Ce que la saturation d'état détruit, exactement

Les deux véhicules portent la même, écrite indépendamment — une convergence de plus entre les deux
lignées (`ADR-043` §1) :

```rust
if h < 0.0 { h = 0.0; hu = 0.0; }              // delta.rs — pas_naif et pas_equilibre
fn saturer(h, hu) { if *h < 0.0 { *h = 0.0; *hu = 0.0; } }   // shallow.rs
```

Elle fait **deux choses, et aucune n'est neutre** :

| geste | effet physique | ce que cela viole |
|---|---|---|
| `h ← 0` quand `h < 0` | **crée de la masse** : `−5·10⁻⁷` devient `0`, soit de l'eau ajoutée | la conservativité, argument écrit d'`ADR-038` §2 et mesuré par `C01-volume` |
| `hu ← 0` | **détruit de la quantité de mouvement**, sans transfert | la conservation de la quantité de mouvement, que rien ne mesure |

**La masse créée est cumulable et se mesure.** C'est la grandeur que cet audit ajoute : non pas
seulement *combien de fois*, mais *combien*.

## 3. Recensement — `delta.rs`

| # | où | expression | catégorie | ce qu'elle empêche |
|---|---|---|---|---|
| **S1** | `configure` ×5 | `(η₀ − b).max(0)` | initialisation | une hauteur négative au repos sur un fond qui perce la surface |
| **S2** | `configure` | `xi.clamp(0, 1)` | initialisation | une abscisse normalisée hors bassin |
| **S3** | `avec_cfl` | `cfl.clamp(0.05, 2.0)` | réglage | un nombre de Courant absurde |
| **S4** | `front` | `f.clamp(0, 1)` | mesure | une interpolation de front hors de sa cellule |
| **S5** | `bords` ×2 | `(h + b_int − b_fant).max(0)` | condition de bord | une cellule fantôme à hauteur négative sur fond montant |
| **S6** | `flux_rusanov` ×2 | `(G·h.max(0)).sqrt()` | protection | une racine de nombre négatif → `NaN` |
| **S7** | `pas_naif`, `pas_equilibre` | `if h < 0 { h = 0; hu = 0 }` | **état** | **le sujet de cet audit** |
| S8 | 6 endroits | `if h > h_sec` | seuil de sec | hors mandat — **A163**, action **S37-1** |

## 4. Recensement — `shallow.rs`

| # | où | expression | catégorie | ce qu'elle empêche |
|---|---|---|---|---|
| **T1** | `maille_en` | `(i.max(0) as usize).min(n−1)` | indexation | un accès hors tableau |
| **T2** | ×4 | `(G·h.max(0)).sqrt()` | protection | une racine de nombre négatif |
| **T3** | reconstruction ×3 | `(h + b − b*).max(0)` | **schéma** | rien : c'est la reconstruction hydrostatique d'Audusse, une étape du schéma et non un filet |
| **T4** | `saturer` | `if h < 0 { h = 0; hu = 0 }` | **état** | **le sujet de cet audit** |
| **T5** | éponge | `(1 − σ·dt).max(0)` | garde-fou | un facteur d'amortissement négatif — **A160** |
| T6 | 8 endroits | `> 1e-10` | seuil de sec | hors mandat — **A163** |
| T7 | `avancer_jusqu_a` | `dt.min(t_fin − t)` | cadence | un dépassement du temps final |

**T3 n'est pas une saturation** et figure ici pour qu'on cesse de la compter comme telle : dans le
schéma d'Audusse, `(h + b − b*)⁺` **est** la définition de la hauteur reconstruite. La retirer ne
retirerait pas un filet, elle changerait de schéma.

## 5. Les prédictions, écrites avant la mesure

*Une prédiction qui se réalise coûte une ligne ; une prédiction qui échoue désigne l'endroit où le
modèle mental est faux* (**L125**).

| saturation | C01 — repos sur pente | C03 — seiche | C04 — rupture sur lit sec |
|---|---|---|---|
| **S7 / T4** (état) | **jamais** | **jamais** | **oui, localisé au front** |
| **S6 / T2** (racine) | jamais | jamais | **jamais** — `h ≥ 0` en entrée de pas, puisque S7 a agi à la fin du précédent |
| **S5** (bord) | **oui, une fois par pas** — le fond monte, la cellule fantôme droite est plus haute que l'eau | jamais — fond plat | jamais — fond plat |
| **S1** (init) | jamais — bassin entièrement mouillé | jamais | **oui** — le lit sec, par construction |

**Trois de ces prédictions sont plus intéressantes que les autres :**

1. **S7 sur C01 et C03.** Un seul déclenchement y serait un défaut, pas un filet : le domaine est
   entièrement mouillé et le régime linéaire.
2. **S7 sur C04, et son étendue.** Si la saturation touchait des dizaines de cellules à chaque pas,
   le front que C04 mesure serait en partie un artefact — et C04 sert de critère d'entrée au banc B3
   (`ADR-031`).
3. **S6 et T2 jamais déclenchées.** Ce n'est pas rassurant : *une saturation jamais déclenchée n'a
   pas été testée* (**L118**), et rien ne dit qu'elle protège de ce qu'elle prétend protéger.

## 6. Mesures

### 6.1 Régime nominal — **aucune saturation, nulle part**

Trois cas, deux véhicules, `CFL = 0,45` :

| cas | cellules | pas (`f32` / `f64`) | **S7 / T4** | S5 (bord) | témoin S6 |
|---|---|---|---|---|---|
| **C01** — repos sur pente, 60 s | 160 | 2 891 / 2 891 | **0** | **0** | 0 |
| **C03** — seiche, 20 périodes | 200 | 17 866 / 35 759 | **0** | **0** | 0 |
| **C04** — rupture sur lit sec, 2 s | 800 | 475 / 454 | **0** | **0** | 0 |

**Deux prédictions du §5 sur quatre sont fausses.**

- *Volet 1 — C01 et C03 ne déclenchent jamais* : **vrai**.
- *Volet 2 — C04 déclenche, localisé au front* : **faux**. Il ne déclenche **jamais**. Le lit sec ne
  suffit pas à produire une hauteur négative : le flux de Rusanov préserve la positivité sous sa
  condition de Courant, et `CFL = 0,45` est largement dedans.
- *S5 sur C01 — une fois par pas* : **faux**, jamais. L'analyse était naïve : `h[n+1] = h[n] − dx·pente`
  vaut `≈ 0,99 m` quand `h[n] ≈ 1 m`. Il aurait fallu un bord presque sec pour que la marche morde.
- *S6 et T2 jamais déclenchées* : **vrai**, et démontré plutôt que supposé — le témoin
  `h_negatif_en_entree` reste à zéro sur les trois cas.

### 6.2 Hors condition de Courant — elle mord, et elle ne sauve rien

*Un garde-fou qu'on n'a jamais vu déclencher n'a pas été testé* (**L118**). Le levier est physique :
au-delà de sa condition de Courant, Rusanov cesse de préserver la positivité. C04, 0,5 s :

| `CFL` | pas (`f32` / `f64`) | **S7** (`f32`) | **T4** (`f64`) | masse créée / volume (`f32`) | qdm détruite |
|---|---|---|---|---|---|
| 0,45 | 100 / 100 | 0 | 0 | 0 | 0 |
| 0,95 | 50 / 49 | **0** | **0** | 0 | 0 |
| **1,20** | 77 / **6 890** | **267** | **11 252** | **1,4·10¹⁷** | `NaN` |
| **1,80** | 44 / 1 070 | **188** | **2 651** | **4,1·10¹⁷** | `NaN` |

**Trois choses se lisent dans ce tableau, et la troisième est la plus importante.**

1. **La saturation fonctionne.** Elle est vue mordre, elle n'est pas du code mort, et sa condition
   n'est pas inatteignable. Elle a maintenant son **témoin** — à `CFL = 0,95`, elle ne mord pas —
   sans quoi une condition « toujours vraie » passerait le même test (**L119**).
2. **La frontière est nette et elle est à la bonne place** : rien à 0,95, tout à 1,20. C'est la
   condition de Courant elle-même, retrouvée par un compteur qui ne la connaît pas.
3. **Elle ne rattrape rien.** La masse créée dépasse le volume initial d'un facteur **10¹⁷** côté
   `f32` et **10¹⁵⁰** côté `f64` ; la quantité de mouvement détruite déborde vers `NaN`. Le nombre
   de pas explose — 6 890 au lieu de 100 pour la même demi-seconde — parce que `dt_cfl` se
   recalcule sur des vitesses divergentes.

## 7. Verdict

**Aucune des saturations recensées n'est un maquillage, et l'inquiétude d'A146 est levée dans la
forme où elle était posée.** Il n'y a pas de solveur qui produirait des états impossibles à chaque
pas derrière un filet complaisant : en régime nominal, le filet ne touche jamais rien.

**Mais A146 avait raison sur le fond, et pour une autre raison que celle qu'il donnait.**

> **La saturation d'état n'est pas un filet : c'est un détecteur de divergence, et il était muet.**
> Elle ne se déclenche que lorsque le calcul a déjà cessé d'être valide — la frontière mesurée est
> exactement la condition de Courant — et à ce moment-là elle ne répare rien : elle transforme une
> divergence franche, qui aurait produit des `NaN` visibles, en une suite de nombres finis. **Un
> premier déclenchement est l'événement le plus informatif que ce solveur puisse produire**, et
> jusqu'à S38 personne ne pouvait le voir.

Classement, selon la grille annoncée au §1 :

| saturation | classement | ce qu'il faut en faire |
|---|---|---|
| **S7 / T4** (état) | **détecteur de divergence**, ni filet ni maquillage | l'exposer au rapport : un compteur non nul doit **rougir** le cas |
| S5 (bord) | jamais déclenchée, condition atteignable en principe | garder, et lui écrire un cas de déclenchement |
| **S6 / T2** (racine) | **jamais atteignable** — démontré par le témoin | garder : c'est une protection de type, pas un filet de modèle |
| T3 (reconstruction) | **pas une saturation** — une étape du schéma d'Audusse | cesser de la compter comme telle |
| Étage RK2 | jamais déclenchée sur les trois cas | garder, comptée à part depuis S38 |

### 7.1 Ce que l'audit a trouvé par accident

**Le recensement écrit du §4 avait manqué un point de saturation** : l'étage intermédiaire de RK2,
dans `shallow.rs`. Il n'a été vu que parce que le compilateur a refusé l'appel restant après la
transformation de `saturer` en méthode.

*Un recensement à la lecture en manque.* Ce qui l'a rattrapé n'est pas une relecture plus attentive
mais un **changement de signature qui oblige chaque appel à se déclarer**. C'est la même leçon que
S34 sur G10 : ce qui n'est pas forcé de passer par un point unique échappe à l'inventaire.

### 7.2 Une limite de l'instrument, à dire

À `CFL = 1,2`, `masse_creee` vaut `10¹⁵¹` et `qdm_detruite` vaut `NaN`. **Les compteurs eux-mêmes
débordent.** Ce n'est pas une faute de mesure — la grandeur mesurée est réellement absurde — mais
cela veut dire que ces deux nombres ne servent qu'à **détecter**, jamais à quantifier une dérive
modérée. Le compteur d'événements, lui, reste exact.
