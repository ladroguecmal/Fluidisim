# Le rouleau 3D, étape 5 : la force du rouleau sur un corps — S652 (listes 6.7, 4.14)

*S652, 2026-10-07, en autonomie, vers la v2.* La dernière étape du plan du rouleau 3D. Le point 6.7 attendait « le rouleau plongeant qui
décolle un nageur ».

## Reproduire

- L'instrument, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_body_force_reads_archimedes -- --nocapture`.
- Le rouleau sur le corps : `… the_plunging_roller_pushes_a_body_s652 -- --ignored --nocapture`, depuis une copie du binaire (ADR-265 D1),
  ≈ 6 min. **Il échoue sur le critère manqué ; l'assertion en a été ôtée après la mesure.**

## 1. L'instrument, éprouvé et corrigé avant de mesurer (ADR-263 D2)

**La force de l'eau sur la sphère d'APIC** (S393), qui imposait sa vitesse mais ne recevait rien. C'est la somme, sur les faces entre une
maille du corps et une maille d'eau, de la pression à la face fois `dx²`, dirigée vers le corps.

**Le cas connu** : une sphère immergée au repos, 32 mailles, `ρgV` = 39,24 N.

| version de l'instrument | `F_z/ρgV` | `F_x`, `F_y` |
|---|---|---|
| la pression lue au centre de la maille voisine | **1,38** (une demi-maille trop loin : `ρg·dx³` de trop par colonne du corps) | nuls |
| **la pression extrapolée à la face**, depuis les deux mailles d'eau | **1,13** — critère (1) tenu (≤ 15 %) | nuls |

Le biais restant (13 %) est celui d'un corps en escalier de 32 mailles. Il est rapporté, non corrigé.

## 2. Le rouleau sur le corps

Le montage est le relais de S650, élargi à 8 mailles (0,4 m). Une sphère fixe `r` = 0,1 m est posée à x = 10,4 m, sur sa marche
(0,40 m), à demi immergée au repos.

| | mesuré, 5 cm |
|---|---|
| retournement ; air enfermé | 2,592 s, 9,825 m ; 2,709 s, 10,125 m |
| pic de `F_x` | **165,6 N à 2,745 s** — deux pas seulement au-dessus de la moitié du pic |
| `F_x` lissée sur 0,1 s (témoin) | **47,0 N** |
| impulsion `∫F_x dt` | **15,3 N·s** |
| vitesse de la sonde (3 mailles avant, mi-hauteur) / de toute sa colonne | 0,77 / 3,0 m/s |
| `h·u` au plus, à la sonde | 0,268 m²/s |
| la masse (volume − entré) | −1,7·10⁻¹⁶ |

**Critères, écrits avant.**

- **(1) Tenu**, après la correction de l'instrument.
- **(2) À moitié.** Le pic de force suit bien le retournement, de 0,15 s. Le coefficient de traînée effectif, lui, est **manqué** : 18 avec
  le pic brut. Lissé, il vaut 5,1 avec la sonde à mi-hauteur et 0,33 avec la vitesse du jet ; selon la vitesse de référence, il passe d'un
  bord à l'autre de [0,5 ; 3].
- **(3) Tenu.**
- **(4) Rapporté**, ci-dessous.

**Le critère manqué, localisé (ADR-259 D1).** Le témoin (la force lissée, la vitesse de toute la colonne) sépare deux choses.

- **Le pic est un choc de deux pas** : le jet qui frappe, ou la pression qui saute quand des mailles basculent entre eau et air contre le
  corps. Rien ne départage les deux ici (A334).
- **La vitesse de référence d'une traînée n'a pas de sens unique sous un rouleau** : le jet va à 3 m/s, l'eau sous lui à moins de 1 m/s.

Le critère comparait donc un pic de choc à une traînée en régime établi. L'erreur était dans le critère autant que dans la mesure.

## 3. À l'échelle de la nature (Froude, ×20)

La sphère devient un corps de 2 m de rayon. Sa force lissée vaut environ 380 kN, son impulsion 15,3 × 20⁴ ≈ 2,4 MN·s. Le produit `h·u`
vaut 0,268 × 20^1,5 ≈ **24 m²/s**, contre 1 m²/s, le seuil d'ADR-018 qui emporte un adulte : le rouleau emporte un nageur, et de loin.

## 4. Ce que cela dit — et ne dit pas

Le rouleau d'APIC 3D **pousse** un corps, au bon moment, avec une impulsion et un produit `h·u` qui disent, à l'échelle de la nature, qu'il
emporte un homme : la règle d'ADR-018 est franchie par un facteur 24. La force instantanée n'est pas encore une grandeur fiable. Son pic
est un choc de deux pas, non départagé entre l'impact réel du jet et un artefact de pression (A334). La sphère est **fixe** : la force ne
revient pas encore au corps, et le couplage deux sens reste à faire.

Le plan du rouleau 3D en cinq étapes (S639–S652) est parcouru. Restent ouverts :

- la surface en eau mince au repos (S640) ;
- la quantité d'air (S648) ;
- le relais dans les deux sens (S650) ;
- la force instantanée (A334) ;
- le corps libre poussé par le rouleau ;
- le coût (4.19).
