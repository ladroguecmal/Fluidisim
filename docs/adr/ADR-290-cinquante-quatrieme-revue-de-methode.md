# ADR-290 — Cinquante-quatrième revue de méthode (S746–S750)

- **Statut : actée**, S751, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-288](ADR-288-cinquante-troisieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S746 | la revue (ADR-288), la vérification du lot par l'outil ; elle a servi en S749 | — | elle a tenu |
| S744–S749 | **six variantes de la projection essayées avant que le mécanisme soit nommé.** Le mécanisme (le déplacement des particules sans leur vitesse) a été trouvé en S750 en raisonnant sur l'opérateur. S748 mesurait l'énergie de la lame, mais non ce que la projection seule retire à chaque pas, et l'a attribué à la dilatation de la surface | cinq sessions de variantes | **D1** |
| S749 | **une formule de cible supposée juste au repos** (ρ* = 0,875) et non vérifiée sur son cas statique : le repos perdu de 4,4 mm, découvert après 20 min de calcul | 20 min | **D2** |
| S749–S750 | **la remontée lancée avant le repos**, contre ADR-288 D1, pour aller plus vite ; le repos ne coûte que 2 min | aucun résultat faussé | **D3** |
| S748 | un tracé raté au premier essai (l'échelle verticale, les noms de fichiers) | quelques minutes | aucune : regardé, puis refait (ADR-287 D4) |
| S747 | le piège d'ADR-223 D4, dans la mémoire elle-même ; corrigé par l'outil d'édition | aucun | la règle existe |

## 2. Décisions

**D1 — Un opérateur suspect se mesure par son propre bilan, à chaque pas.** Avant d'essayer des variantes d'un opérateur (une projection,
une correction, un filtre), l'essai mesure ce que cet opérateur seul change à chaque pas : l'énergie, la quantité de mouvement, le volume,
avant et après son application, dans le même état. Puis il écrit l'hypothèse du mécanisme et la signature qu'elle prédit. Une variante
n'est essayée que contre une hypothèse nommée.

**D2 — Une formule de cible se vérifie sur son cas statique, en quelques secondes, avant tout calcul dynamique.** C'est le prolongement
d'ADR-288 D4 : le repos exact, posé, lu par l'instrument du solveur, doit donner la cible que la formule annonce.

**D3 — Le repos passe en premier, même pressé** (ADR-288 D1, rappelée) : il coûte deux minutes, et sans lui aucun autre essai ne se lit.

## 3. La prochaine revue

S756.
