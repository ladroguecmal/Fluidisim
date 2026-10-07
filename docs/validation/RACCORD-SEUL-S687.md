# Le raccord seul, entre deux Saint-Venant — S687 (liste 4.14)

*S687, 2026-10-08, en autonomie, vers la v2.* [ADR-273](../adr/ADR-273-quarante-et-unieme-revue-de-methode.md) D1 : un raccord se juge
d'abord entre deux copies du même solveur. S685 avait trouvé la remontée du relais 14 à 15 % sous le tout-Saint-Venant, sans pouvoir
séparer le raccord de l'onde portée par APIC.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s687 -- --nocapture` (≈ 1 min).

## Ce qui a été fait

- **Le bord droit de Saint-Venant à flux imposé** (`pas_avec_flux_droit`). La face droite fait passer le flux de masse donné par rangée,
  avec le flux de quantité de mouvement `F·u + ½·g·h²` de la maille de bord. C'est l'équivalent du bord droit d'APIC.
- **Le raccord seul.** Deux domaines de Saint-Venant, raccordés à 5,35 m par le schéma de `RelaisRivage` : l'état du bord du large
  nourrit le bord caractéristique du rivage, et le flux rendu quitte le large par son bord à flux imposé.

L'onde de S644 et le tout-Saint-Venant de S685, depuis le même état initial.

## Mesuré

| maille | le raccord seul | le tout-Saint-Venant | l'écart | la masse |
|---|---|---|---|---|
| 5 cm | 0,2023 m | 0,2190 m | −7,61 %, **une marche de la lecture** | 6,2·10⁻¹⁵ |
| 2,5 cm | 0,2398 m | 0,2398 m | **0,00 %** | 6,5·10⁻¹⁵ |
| 1,25 cm (en route) | 0,2586 m | 0,2586 m | **0,00 %** | 9,1·10⁻¹⁵ |

| critère (écrit avant) | mesuré |
|---|---|
| (1) Saint-Venant sans flux imposé, au bit | tenu (les essais de Saint-Venant, inchangés) |
| (2) la remontée à 6 % du tout-Saint-Venant, à 5 et 2,5 cm | **tenu à 2,5 cm** ; à 5 cm, **manqué par la borne** (voir ci-dessous) |
| (3) la masse à 10⁻¹² près | tenu |

**La borne était sous le quantum de la lecture.** La remontée est lue comme la plus haute maille mouillée. Sur la pente 1:3, elle monte
par marches de `dx/3` : 1,67 cm à 5 cm, soit 7,6 % de la remontée. L'écart mesuré en est exactement une : une maille de moins est
mouillée au-delà de 1 mm. Le calcul du plan tirait la borne du bord caractéristique (S622), et non du quantum de la lecture : c'est
l'erreur qu'ADR-268 D1 décrit. À 2,5 et 1,25 cm, le raccord redonne le tout-Saint-Venant au dix-millième.

## Ce que cela dit

- **Le schéma de raccord est juste.** Entre deux Saint-Venant, il redonne le domaine entier, la masse au bit.
- **Le déficit de S685 vient de ce qu'APIC porte au large, plus un quantum de lecture à 5 cm.** À 2,5 cm, où la lecture vaut 3,5 %, le
  relais restait 14 % sous le tout-Saint-Venant : l'onde d'APIC, plus lente et plus large, sans le front raidi de Saint-Venant
  (S685).
- **La lecture de S685 a le même quantum.** À 5 cm, ses −15 % comptent une marche de 7,6 %.

**Suite** : juger le relais contre une référence du côté d'APIC (ADR-273 D1) — le tout-APIC à air balistique de S645, sur la même onde.
Une lecture de la remontée sous la maille (le rivage interpolé) ôterait le quantum.
