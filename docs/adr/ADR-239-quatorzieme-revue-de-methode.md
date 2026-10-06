# ADR-239 — Quatorzième revue de méthode (S546–S550)

- **Statut : actée**, S551, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-238](ADR-238-treizieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une formule d'analyse fausse au plan, puis un diagnostic posé avant de la relire** (S549) : `tan θ = m_h·Δy/(m·GM)` au lieu de `Δy/GM` (le proxy décalé déplace toute la poussée) ; le critère semblait manqué de 10 %, j'ai accusé le proxy et changé le montage pour rien avant de relire ma formule — ADR-226 D1 dit pourtant de séparer « la référence contre la sienne » | un montage changé et rétabli, une fausse piste écrite | **protection élargie** (D1) ; S550 l'a déjà appliquée (la formule GZ vérifiée par intégration, juste) |
| Le rituel qui exige le plan committé (ADR-238 D1) | — | appliqué à S547–S550 ; **rien à changer** |
| La reprise après une coupure d'usage (S547) : le plan committé suffisait | — | la procédure de reprise à chaud a tenu ; **rien à changer** |

## 2. Décision

**D1 — Une formule d'analyse nouvelle (la valeur attendue, le rapport qu'on mesure) s'éprouve par un calcul indépendant avant la mesure**
— une intégration, une seconde dérivation, un cas limite connu (élargit la protection « une valeur attendue se calcule dans l'essai »,
L378) ; et, devant un écart, c'est elle qu'on relit d'abord (ADR-226 D1, « la référence contre la sienne »).

## 3. La prochaine revue

S556.
