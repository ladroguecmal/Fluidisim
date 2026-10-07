# ADR-259 — Trente-deuxième revue de méthode (S636–S640)

- **Statut : actée**, S641, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-258](ADR-258-trente-et-unieme-revue-de-methode.md). Elle relit aussi l'audit des intentions initiales que l'utilisateur a demandé
  en S640 ([AUDIT-INTENTIONS-INITIALES-S640](../registres/AUDIT-INTENTIONS-INITIALES-S640.md)).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| S636 (revue), S637, S638 (5.10, V qui déclenche δ, son cycle de vie) : bornes assertées, références indépendantes ; tenu du premier essai | — | rien à changer |
| S639 : un montage à égalités exactes (`(k + ½)·dx` = le fond), que f32 et f64 tranchaient différemment ; vu par l'assertion de bornes d'ADR-257 D1, le plan amendé **avant** la mesure | aucun | **la protection a servi** |
| **S639–S640 : un critère manqué, mal attribué.** En S639, le repos manqué au rivage, peu sensible à la maille, est attribué à la géométrie en escalier ; le remède désigné (les faces coupées) occupe S640. La projection à faces coupées s'avère exacte sous l'eau (8·10⁻⁶ m/s) et s'emballe au rivage (0,26 m/s) : la cause était le film d'eau plus mince que le noyau. La variation de maille ne pouvait pas trancher — l'escalier et le film s'affinent ensemble. Une pente entièrement immergée, dix secondes de calcul, aurait séparé les deux causes en S639 (L419) | une session de remède sur une fausse cause ; six reconstructions explorées | **protection élargie** (D1) |
| **S609, vu par l'audit : une décision retirée, rétablie sans le savoir.** Le seuil `0,35·Hs` est pris dans ADR-001 §3.3 ; ADR-111 et ADR-112, qui nomment ADR-001, l'avaient retiré (« proposition historique non reçue ») ; 4.11 l'affiche reçu. Le registre généré des décisions en vigueur portait la réponse (colonne « nommé par ») ; rien n'obligeait à la lire | un point de la liste faux depuis trente sessions | **protection nouvelle** (D2) |
| **L'audit : des intentions fondatrices sans point, des invariants qui ont dérivé.** Onze intentions des documents fondateurs n'étaient portées par aucun point (versées en S641) ; I-08 (le calcul en `f32`) et I-14 (l'origine des nombres) ne sont plus tenus par une partie du code, sans amendement. La liste, recomptée et actualisée maintes fois, ne l'avait jamais été **contre ses sources** depuis S350 | inconnu tant que l'utilisateur n'a pas demandé | **protection nouvelle** (D3) ; la correction en S642 |

## 2. Décisions

**D1 — Un critère manqué : avant de nommer un remède, un témoin qui supprime une cause.** La variation de deux paramètres d'ADR-256 D2
attribue un écart à la physique ou au numérique ; elle ne départage pas deux causes numériques qui varient ensemble. Avant d'écrire « le
remède est … », la session lance le montage où l'une des causes candidates est absente (ici : la pente immergée, sans rivage), et la
preuve en donne la mesure.

**D2 — Une valeur ou une règle tirée d'un ADR se cite avec ceux qui le nomment.** Le plan qui implémente une valeur, un seuil ou une règle
écrite dans un ADR cite aussi les ADR plus récents qui le nomment (la colonne « nommé par » de
[DECISIONS-EN-VIGUEUR](../registres/DECISIONS-EN-VIGUEUR.md)), après les avoir lus : un ADR ne se réécrit jamais, ses suites sont
ailleurs.

**D3 — Le bilan global relit la liste contre les documents fondateurs.** Le prochain bilan global (et chacun ensuite) refait la
comparaison de l'audit S640 : intentions sans point, intentions changées sans décision, intentions caduques, invariants non tenus. Les
ajouts et corrections techniques se font seuls (ADR-218 D2) ; ce qui touche l'ambition va à l'utilisateur.

**Les corrections qui suivent** : S642 instruit 4.11 (le critère de bascule, ou le seuil rangé en paramètre non reçu), I-08 et I-14
(amendés par ADR, ou le code mis en conformité). Les arbitrages de l'audit — le sens terrain–eau, HEALPix ou cube-sphère, les plages, la
glace, l'air respirable, les grandes zones de déferlement, B3 et le jury, la cible d'I-03 — sont soumis à l'utilisateur
([BOUSSOLE](../../BOUSSOLE.md)) ; rien n'est tranché à sa place.

## 3. La prochaine revue

S646.
