# S121 — Deux causes sous un seul refus : où le défaut est vraiment

2026-09-09. Suite de S120-1, sur A197.

## 1. Ce que A197 annonçait, et ce que la mesure a trouvé

A197 visait `RadialImpact::sample`, qui rend `Error::Domain` aussi bien pour une position hors
domaine que pour une sortie non finie. La cible était juste sur le principe et **fausse sur le
lieu** : ce bloc de finitude n'est pas atteignable.

### Le bloc de finitude de `sample` n'a pas d'entrée qui l'atteigne

`code/water-core/examples/probe_degenerate.rs` balaie énergie, longueur d'onde, rayon,
profondeur, pente maximale et horizon — chaque paramètre sur sa plage représentable, y compris
`f32::MAX`. Sur **18 719 champs construits et 673 884 échantillons, aucun refus**, et une sortie
maximale de 3,17 × 10²⁶ : douze ordres de grandeur sous le débordement `f32`.

La raison est structurelle, et la sonde la rend visible en suivant la direction où les sorties
croissent — les longueurs d'onde décroissantes, à énergie et pente maximales :

| λ | `slope_bound` | pic des sorties | refus |
|---|---|---|---|
| 10⁻⁴ | 6,17 × 10²⁶ | 3,49 × 10²⁵ | 0 |
| 10⁻⁶ | 6,17 × 10³⁰ | 3,49 × 10²⁹ | 0 |
| 10⁻⁸ | 6,17 × 10³⁴ | 3,49 × 10³³ | 0 |
| 10⁻⁹ | 6,17 × 10³⁶ | 3,49 × 10³⁵ | 0 |
| 10⁻¹⁰ | — | — | **la construction refuse** |

Le pic vaut constamment `slope_bound / 17,7`. Or la construction vérifie que `slope_bound` est
fini : **elle refuse donc avant que `sample` puisse déborder**, et avec plus d'un ordre de
grandeur de marge. Le bloc de finitude est du code défensif que rien n'atteint.

### Le même défaut existe, mais dans le constructeur

À λ = 10⁻¹⁰, la construction refuse par `Error::Steepness`. La pente maximale valait pourtant
`f32::MAX` : ce refus ne dit pas « trop raide », il dit **« la borne a débordé »**. La ligne

```rust
if !slope.is_finite() || slope > medium.max_slope { return Err(Error::Steepness); }
```

réunit deux situations qui n'ont rien de commun :

- **la pente dépasse la limite du milieu** — l'appelant a demandé un impact trop énergique pour
  ce milieu ; il peut réduire l'énergie, et le refus est un verdict physique ;
- **la borne n'est pas représentable en `f32`** — aucune énergie plus faible n'est en cause, le
  champ demandé sort du domaine numérique de la bibliothèque.

Un appelant qui traite `Steepness` comme « je réduis l'énergie et je réessaie » boucle
indéfiniment sur le second cas. C'est le défaut qu'A197 décrivait, à un étage au-dessus de
celui qu'elle désignait, et **celui-là est atteignable** : λ = 10⁻¹⁰, `energy_j = f32::MAX`,
`max_slope = f32::MAX`.

## 2. Décision

Voir [ADR-081](../adr/ADR-081-separer-limite-physique-et-limite-numerique.md).

## 3. Construction et réception

*(à compléter)*
