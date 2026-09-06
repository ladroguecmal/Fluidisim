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

*(Rempli en P5.)*

## 7. Verdict

*(Rempli en P5.)*
