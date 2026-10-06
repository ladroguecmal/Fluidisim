# ADR-227 — La poussée du proxy au centre de la part immergée

- **Statut : actée**, S500, 2026-10-06 ; décision technique prise ici ([ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D2,
  [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D2).
- **Précise** [ADR-008](ADR-008-flottabilite-et-autorite.md) §2 (le proxy de flottabilité, la rampe `sat`) et répond en partie à son
  §5.1 (le nombre de points, B6). La mesure : [B6-PROXY-S499](../validation/B6-PROXY-S499.md) ; la preuve :
  [POUSSEE-S500](../validation/POUSSEE-S500.md).

## 1. Ce qui a été trouvé

Un point du proxy représente une tranche verticale d'épaisseur `e` ; partiellement immergé (fraction `f`), il poussait **en son milieu**.
La hauteur des poussées, qui fixe le centre de carène et donc la stabilité de forme (`GM = KB + BM − KG`), était fausse de
`(1 − f)·f·e²/(2d)` — c'est tout le surcoût des couches que B6 a mesuré (S499 : 560 points pour le navire sans compensation, et des
proxys à une couche qui ne tenaient que parce que deux erreurs se compensaient).

## 2. Décision

**D1 — La poussée d'un point partiellement immergé s'applique au centre de sa part immergée** : son milieu abaissé de `(1 − f)·e/2` le
long de l'axe du corps. La force ne change pas — ni la translation, ni le pilonnement droit ; le moment seul. Un point noyé ou sec n'est
pas touché.

**D2 — B6, ce qui en suit** : à l'équilibre droit, la hauteur des poussées est celle du centre de carène à l'arrondi pour toute grille ;
le proxy ne paie plus que l'inertie de flottaison discrète (`BM·(1 − 1/n²)`) et la houle vue par la flottaison. **Une couche suffit** ;
le plus petit proxy d'un pavé (GM à 5 %, houle à 1 % pour `λ = 2L`) : `7 × max(7, ⌈√(BM/(0,04·GM))⌉) × 1` — 70 points pour le navire de
60 m, 49 pour la barque et la caisse.

**D3 — Une bouée d'essai est plate** (hauteur ≤ 0,4 × le côté) : une bouée haute et étroite a un `GM` de quelques millimètres que
l'erreur `1/n²` d'un proxy grossier rend négatif.

## 3. Ce qui changerait la décision

Un corps dont la section n'est pas un pavé (carène en V, sphère) : le centre de la part immergée d'une tranche n'y est plus à mi-hauteur
de cette part ; il faudrait des points portant leur profil, ou plus de couches.
