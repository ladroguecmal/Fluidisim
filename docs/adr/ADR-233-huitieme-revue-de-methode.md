# ADR-233 — Huitième revue de méthode (S517–S520)

- **Statut : actée**, S521, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-232](ADR-232-septieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un instrument de mesure appliqué sans être éprouvé sur un cas de sa famille** : trois instruments d'angle essayés sur le sillage de la coque sans réponse connue (S517 — le premier, le maximum des rayons, ne lit même pas Kelvin sur la théorie, S519) ; le bord d'Airy, éprouvé sur une source gaussienne (un lobe), appliqué à une coque (plusieurs lobes), où il lit un creux d'interférence — et sa stabilité d'une fenêtre à l'autre a été prise pour une validité (S520) — malgré la protection « un instrument s'éprouve sur un cas de réponse connue » | une session sans conclusion (S517), deux critères manqués (S520) | **protection changée** (D1) — une erreur répétée sous une protection trop étroite |
| Une limite matérielle (65 535 groupes de dispatch) trouvée au premier domaine dix fois plus grand (S520) | un calcul relancé | une seule fois ; le code la vérifie et la nomme désormais ; **rien à ajouter** |
| Un script d'édition écrit dans la ligne de commande, cassé par une apostrophe (S520) ; un remplacement de texte non vérifié (S517) | rien d'écrit (S520) ; un format à refaire (S517) | les protections existent (L380, L386) et ont arrêté le dégât ; **rien à changer** |
| Le profil d'abord (S518), une référence indépendante du modèle mesuré (S519), un critère remplacé par écrit avant la mesure qu'il juge (S520) | — | appliqués ; **rien à changer** |

## 2. Décision

**D1 — Un instrument s'éprouve sur un cas de réponse connue de la même famille que l'objet mesuré** (élargit la protection
« en construisant l'instrument » de [METHODE](../../notes/METHODE.md)) : même forme de source, même régime. Et on vérifie, sur le profil ou
le champ qu'il lit, qu'il prend **le trait voulu** — une stabilité d'une fenêtre à l'autre ne le prouve pas. Sans cas de sa famille, il
ne conclut pas.

## 3. La prochaine revue

S526.
