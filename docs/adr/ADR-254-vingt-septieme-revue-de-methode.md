# ADR-254 — Vingt-septième revue de méthode (S611–S615)

- **Statut : actée**, S616, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-253](ADR-253-vingt-sixieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un garde-fou du code que la référence n'avait pas mesuré** (S613) : le plan promettait que `pas` refuse Courant > ½, mais son script ne calculait pas le Courant de sa propre référence — 0,60 dans une maille presque sèche. Découvert quand le code a refusé le pas ; plan amendé avant la mesure (ADR-244 D1), un remède trouvé (Kurganov–Petrova) | un amendement, une relance | **protection élargie** (D1) |
| **Une propriété documentée qu'aucun cas n'éprouvait** (S613 → S614) : « murs aux bords » ; les deux cas de S613 avaient des bords secs, et les murs n'exerçaient aucune pression. Découvert en S614 quand une référence à bord mouillé a divergé | une session ralentie, un défaut latent | **protection nouvelle** (D2) |
| Les échappements d'un script d'édition en heredoc (S613 : `\\n`) — passé par un fichier écrit à l'outil | une relance | rien de nouveau : ADR-245 D3 (les scripts sont des fichiers ou des heredocs cités) |
| Les implémentations indépendantes (numpy), les phrases assertées (ADR-253), la suite mesurée (ADR-252) | — | **rien à changer** : tenues |

## 2. Décisions

**D1 — Un garde-fou du code se mesure sur la référence du plan.** Si le code refuse au-delà d'une borne (un nombre de Courant, une
profondeur, une capacité), le script du plan calcule cette grandeur sur sa propre référence et asserte qu'elle reste dans la borne
(élargit ADR-251 D1 et ADR-253 D1).

**D2 — Une propriété écrite dans la documentation d'un module est éprouvée par un cas, ou marquée « non éprouvé ».** « Murs », « positif »,
« équilibré », « conservatif » : chacune a son cas dans l'essai, ou la documentation dit qu'elle ne l'a pas.

## 3. La prochaine revue

S621.
