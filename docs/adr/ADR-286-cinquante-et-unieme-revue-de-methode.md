# ADR-286 — Cinquante et unième revue de méthode (S731–S735)

- **Statut : actée**, S736, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-285](ADR-285-cinquantieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S731 | la revue (ADR-285) ; le piège d'ADR-223 D4 (un `\n` dans un *heredoc*) retrouvé deux fois, corrigé par l'outil d'édition | deux corrections | aucune : la règle existe |
| S732 | le sélecteur conçu ; ses nombres par un script | — | — |
| S733 | les seuils publiés hors du retournement, le coût manqué ; un essai ajouté (E3) avec ses critères écrits avant | aucun | la méthode a tenu |
| S733, S734 | deux fermetures refusées par `fermer.py` (le registre de précision, le lot dû) ; l'arbre restauré avant de relancer (ADR-285 D3) | quelques minutes | aucune : l'outil a fait son office |
| S734 | **une durée fixée sans calculer quand l'onde atteint une frontière** : S3 a tourné 12 s, alors que sa crête atteint le mur vers 10,5 s. Le critère du mur ne pouvait que tomber | 67 min dont la fin ne prouvait rien | **D1** |
| S734 | **un critère faux par construction** : « le front à plus de 1 m du mur », pour un mur placé dans 0,12 m d'eau. Vu en route, corrigé avant le calcul | 8 min de calcul | **D1** |
| S734–S735 | **un instrument pris sur une géométrie neuve sans l'éprouver** : le front lu par φ sur une pente de 1:3, en escalier puis lisse. ADR-233 D1 le demandait déjà. S735 a montré qu'il lisait 10 à 30 mm d'eau sans aucune particule. Deux témoins (S4) ont tourné pour rien | ≈ 70 min de calcul, une session de diagnostic | **D2** |
| S734 | **une chaîne de calculs en arrière-plan lancée avec un chemin relatif** : rien n'a tourné | aucun | **D3** |
| S734 | la consigne de l'utilisateur, le 2026-10-09 : *« on ne fait plus en parallèle »* | — | **D4** |
| S735 | le diagnostic par deux lectures indépendantes, son critère écrit avant : la cause nommée sans ambiguïté | — | la méthode a tenu |

## 2. Décisions

**D1 — Un montage borne aussi sa durée.** Pour chaque frontière d'un domaine (un mur, une sortie, un raccord, le sable sec), le plan
calcule, par un script, l'instant où l'onde l'atteint : sa vitesse de crête, la longueur à parcourir. La durée de l'essai reste en deçà,
avec une marge, ou le plan dit pourquoi la frontière peut être atteinte. **Chaque critère d'un témoin se relit contre sa géométrie** : un
critère du front n'a de sens que si la frontière est sur du sec. C'est le complément d'ADR-235 et d'ADR-257, qui bornaient l'espace.

**D2 — Un instrument nouveau sur sa géométrie porte sa seconde lecture.** Quand un instrument est employé sur une géométrie où il n'a pas
été éprouvé (une pente raide, un fond lisse, un jet de rive), l'essai calcule aussi une lecture indépendante de la même grandeur :
- φ et le compte des particules pour une épaisseur ;
- deux seuils pour un front.

L'écart des deux est rapporté **au fil du calcul** (ADR-281 D1). Une divergence arrête l'interprétation avant la fin du calcul. ADR-233 D1
devient ainsi un geste de l'essai, et plus seulement une intention du plan.

**D3 — Un calcul en arrière-plan s'appelle par chemins absolus** : le script et ses fichiers. Le répertoire courant d'une commande en
arrière-plan n'est pas celui de la conversation.

**D4 — Un seul sujet à la fois.** L'utilisateur, le 2026-10-09 : *« on ne fait plus en parallèle »*. Une session porte une question ;
pendant un calcul long, elle prépare la suite de cette même question (le diagnostic, le remède, la preuve), et rien d'autre.

## 3. La prochaine revue

S741.
