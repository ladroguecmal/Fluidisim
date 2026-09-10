
# ADR-111 — Le critère de bascule s'exprime en profondeur, pas en `Hs`

- **Statut : actée**, S161, 2026-09-10, autonomie technique S71.
- **Corrige le paramétrage** du critère proposé par [ADR-001](ADR-001-decomposition-en-couches.md)
  §3.3, sans remettre en cause la décomposition elle-même.
- **Premier volet exécuté de B4**, le banc qui juge l'architecture.
- **Prolonge :** [ADR-108](ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md) — pas de seuil gelé
  sans tolérance déclarée.
- **Mesure :** [B4-DEBLOCAGE-S161](../validation/B4-DEBLOCAGE-S161.md).

## Problème

ADR-001 §3.3 propose, « à calibrer », un critère de bascule vers le régime substitutif :

> `max|δ| > 0,35 · Hs_local`

B4 devait le calibrer. Il est bloqué depuis toujours par sa **référence substitutive intégrale** —
un solveur qui calcule le champ total sans décomposition. Le dépôt en possède un, et un seul qui
convienne : `shallow.rs`, Saint-Venant 1D **non linéaire**. Un solveur linéaire ne pourrait pas
servir : la superposition y est vraie par construction.

## Ce qui a été mesuré

`‖(η_A + η_B) − η_AB‖ / ‖η_AB‖` : l'écart entre l'addition de deux perturbations simulées
séparément et leur simulation conjointe.

**Le rapport `|δ|/Hs` ne gouverne pas cet écart.** À `A_δ/h0` égal, l'écart est le même que la
perturbation vaille 10 % ou 100 % de l'onde de fond :

| `A_δ/h0` | `A_δ/A_B` | écart |
|---:|---:|---:|
| 0,050 | 0,50 | 1,21e-2 |
| 0,050 | 0,10 | 1,23e-2 |
| 0,100 | 1,00 | 2,04e-2 |
| 0,100 | 0,20 | 2,44e-2 |

**Ce qui le gouverne est l'amplitude rapportée à la profondeur** : `écart ≈ 0,24 · max|δ|/h`,
proportionnalité vérifiée sur cinq décades — jusqu'à `A_δ/h = 8e-6`, où le coefficient vaut encore
0,99 fois sa valeur, ce qui exclut un plancher d'intégration. Le contrôle à pas de temps imposé
place la part numérique entre 0 et 2 %.

## Décision

**1. Le critère de bascule s'exprime en `max|δ| / h`, et non en `max|δ| / Hs`.** Le paramétrage
d'ADR-001 §3.3 est retiré : il fait dépendre la validité de la décomposition d'une grandeur — l'état
de mer — dont la mesure montre qu'elle ne la gouverne pas. Appliqué tel quel, il autorise des
écarts qui varient d'un facteur dix selon `Hs` : 0,8 % pour une houle faible, 8,4 % pour une mer
dont `Hs` approche la profondeur.

**2. Aucun seuil n'est gelé.** La loi est publiée — `écart ≈ 0,24 · max|δ|/h` en régime peu
profond — et le seuil suit la tolérance de celui qui l'applique : 0,21 pour 5 %, 0,45 pour 10 %.
Encoder un nombre gèlerait dans l'architecture une tolérance que personne n'a spécifiée, ce
qu'ADR-108 refuse et qu'A214 attend toujours de B4.

**3. La décomposition additive n'est pas infirmée — son paramétrage l'est.** Elle tient très bien :
moins de 1 % d'écart tant que `max|δ| ≤ 0,04·h`, et 10 % seulement quand la perturbation atteint la
moitié de la profondeur. C'est un résultat favorable à ADR-001, obtenu par un banc conçu pour
pouvoir l'infirmer.

## Ce que cette décision ne fait pas

**Elle ne débloque pas B4 en entier.** Trois volets restent hors d'atteinte : les **forces sur la
coque** — il faut un intégrateur de corps rigide —, la **perception en double aveugle** — il faut
des personnes —, et le **contrôle du terme source** de l'ajout S04 (A50), non exécuté ici.

**Elle ne vaut pas en eau profonde.** Saint-Venant est non dispersif ; `B` est une houle dispersive
en eau profonde, où la profondeur ne joue plus. La variable y serait vraisemblablement la cambrure,
et cela **reste à établir** — c'est la limite principale de ce volet, et elle est structurelle, pas
budgétaire.

**Elle n'explique pas le régime de très faible amplitude de fond.** À `A_B/h ≤ 0,02`, le
coefficient vaut 0,95 au lieu de 0,24. Mesuré, pas compris.

## Réception

299 tests, cinq ignorés — inchangés. `Shallow1D::configure_bosses` est ajoutée au véhicule d'essai
(ADR-043) : additive, elle pose une somme de gaussiennes là où rien ne permettait d'écrire un état
initial à deux perturbations. Sans elle, le montage minimal d'un test d'additivité était
inexprimable — ce qui explique en partie que ce volet n'ait jamais été tenté.

Banc rejouable : `cargo run -p water-core --release --example additivite_b4`.

## Note de portée du 2026-09-10 (S162)

**ADR-112 remplace les conclusions de choix du paramètre de bascule d'ADR-111.** Le montage
S161 compare des évolutions indépendantes, alors que SPEC-004 §6.1 prévoit un résidu couplé
au fond, avec termes croisés et source. Les mesures S161 sont conservées ; elles ne suffisent
pas à recevoir ce couplage ni à choisir sa bascule. Aucun seuil n'est rétabli ou gelé.
Voir [ADR-112](ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md).