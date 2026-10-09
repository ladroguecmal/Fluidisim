# ADR-287 — Cinquante-deuxième revue de méthode (S736–S740)

- **Statut : actée**, S741, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-286](ADR-286-cinquante-et-unieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S736–S738 | les règles d'ADR-286 appliquées : la durée bornée par les frontières, la seconde lecture, un sujet à la fois. Elles ont trouvé le plafond de S4 (S737) et désigné l'écart par bissection (S738) | — | elles ont tenu |
| S739–S740 | **un solveur pris pour référence sans son essai canonique.** La 3D ne garde pas une onde solitaire sur fond plat : ses particules se tassent sous la crête (S740). Le juge du déferlement (S647, S690–S730), les témoins du sélecteur et les conclusions sur SGN en dépendaient. L'essai qui l'aurait montré, une onde solitaire sur un canal plat, le plus simple d'un modèle de vagues, n'avait jamais été fait. ADR-230 D1 demandait une référence *convergée* ; elle l'était (S713), mais convergée vers un défaut | cent sessions de mesures à relire en partie ; cinq sessions pour le trouver | **D1** |
| S737 | **un critère tiré d'un seuil publié sans vérifier ce qu'il couvre** : « S4 ne déferle pas » d'après le seuil de Synolakis pour la montée (0,818), alors que le reflux déferle dès 0,479 | un critère faux, un essai mal lu d'abord | **D2** |
| S739 | **un résumé qui lit son instrument avant qu'il existe** : la surface à t = 0, avant toute reconstruction (±inf) | aucune mesure perdue, un résumé faux | **D3** |
| S740 | **le champ regardé après quatre suspects.** Le profil de la surface, tracé ensuite, montrait l'onde qui se scinde et désignait le tassement | quatre essais de 2 à 7 min | **D4** |
| S737, S740 | le piège d'ADR-223 D4 retrouvé deux fois de plus (un `\n` dans un *heredoc*), sans dommage ; corrigé par l'outil d'édition | quelques minutes | aucune règle neuve : l'outil d'écriture, toujours, pour un texte qui porte une barre oblique inverse |

## 2. Décisions

**D1 — Un solveur ne sert de référence qu'après ses essais canoniques.** Avant qu'une simulation serve de témoin ou de juge (ADR-273,
ADR-285 D1), elle passe les essais de base de son domaine, contre une solution exacte ou des mesures. Pour la 3D des vagues :
- une onde solitaire sur un canal plat, qui garde sa forme (S739, S740) ;
- la levée sur une pente douce, contre la loi de Green ;
- une onde stationnaire, contre la dispersion linéaire ;
- la remontée d'une onde non déferlante, contre la loi de Synolakis (S645).

Ils forment un banc, rejoué quand le cœur change. Un témoin dont le solveur ne passe pas ce banc est marqué « non éprouvé », et ses
conclusions avec lui. *Convergé* ne veut pas dire *juste* : ADR-230 D1 demandait la convergence ; D1 demande en plus la validité.

**D2 — Un seuil publié se cite avec le phénomène qu'il borne.** Le plan écrit la source, la grandeur et la phase couvertes (la montée ou le
reflux, le déclenchement ou le retournement). Un critère « aucun X » se vérifie contre tous les seuils de X connus.

**D3 — Une grandeur de référence se lit là où son instrument est valide** : après la première reconstruction, après le premier pas. Le
résumé d'une série le dit.

**D4 — Un écart de forme se regarde avant le second suspect.** Quand un essai montre une forme fausse (une onde, une surface, un jet), le
champ est tracé et regardé avant de tester une deuxième hypothèse (le complément d'ADR-281 D1 : montrer au fil du calcul).

## 3. La prochaine revue

S746.
