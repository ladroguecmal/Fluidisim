# ADR-269 — Trente-huitième revue de méthode (S666–S670)

- **Statut : actée**, S671, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-268](ADR-268-trente-septieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S666 | la revue (ADR-268) | — | — |
| S667 | la borne de composition calculée en chaque point (ADR-268 D1 appliqué) ; 180/180 dessous | — | — |
| S668 | la borne du plan (la loi `Δ²` d'ADR-196 D2, une somme d'amplitudes au pire) dépassait la mesure d'un facteur 9 ; l'ordre de croissance (×3,3 puis ×6,2) départageait comme l'instrument l'écrivait | — | — |
| S668 → S669 | **une voie nommée « à mesurer » écartée par un calcul hors du dépôt** : le pas des tables adapté au gradient de `k`, estimé dans le bloc-notes à un gain de ×1,4 seulement sur une plage 1:50 (la zone côtière, où `k` varie, demande le pas fin), puis laissé pour la dissipation au déferlement ; la preuve de S668 la nommait encore « à mesurer » | aucun encore ; une session future l'aurait refaite | **écrit ici** et en note datée de la preuve de S668 ; pas de règle (rien n'a coûté) |
| S669 | **une bissection inversée dans le script du plan** : `Q_b` nul partout, quand `Hrms` chutait. La sortie absurde l'a montrée avant l'écriture du plan ; le chiffre faux est devenu l'exemple de ce que l'instrument départage | quelques minutes | couvert par ADR-253 D1 (une phrase du plan qui dépend d'un nombre est assertée) |
| S669 | le remaniement de la marche protégé par une empreinte prise **avant** (au bit) | — | la pratique d'ADR-259, appliquée |
| S670 | la cuisson de `Cote2D` par la marche spectrale, protégée de même par une empreinte des tables ; 0,40 % contre l'équilibre 1D | — | — |

## 2. Décisions

**Aucune règle nouvelle.** La seule friction qui pouvait coûter, une voie écartée sans trace, n'a rien coûté. Elle est réparée par
écrit :

- la voie écartée, avec son nombre, dans ce tableau ;
- une note datée dans la preuve de S668.

Si une voie écartée sans trace est refaite, la revue suivante en fera une règle.

## 3. La prochaine revue

S676.
