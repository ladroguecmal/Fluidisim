# ADR-282 — Quarante-huitième revue de méthode (S716–S720)

- **Statut : actée**, S721, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-281](ADR-281-quarante-septieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S716 | la revue (ADR-281) | — | — |
| S717 | la mort de la 3D (M1) : un essai, deux critères, tenus | — | — |
| S718 | la vague de bout en bout ; un témoin à une seule cause (sans la mort) a désigné le large | — | — |
| S719 | **une chaîne `;` qui commet un état partiel, pour la seconde fois** (S702, puis S719) : le script de fin s'est arrêté sur une assertion (une chaîne présente deux fois dans le fichier des essais), et la suite de la ligne, séparée par `;`, a lancé le rituel puis commis. ADR-279 D2 le défendait déjà | un commit amendé ; une règle écrite qui n'a pas suffi | **D1** |
| S720 | la police par défaut de PIL n'a pas les accents | un rendu refait | aucune : `rendu_cote.py` avait déjà la bonne police ; le nouveau rendu la reprend |

## 2. Décisions

**D1 — Une session se ferme par `outils/fermer.py`, et non par une chaîne de commandes écrite à la main.** L'outil exécute le script de
fin, compile les essais, lance le rituel, puis commet seulement si les trois ont réussi et si le rituel ne signale aucun manque. Un échec
arrête tout, sans commit. Une règle enfreinte deux fois devient un outil : ADR-279 D2 reste vrai, mais il n'est plus confié à la vigilance.

**D2 — Le script de fin vise des chaînes uniques.** Le fichier des essais grandit (plus de 2 000 lignes), et une assertion ou un message
reviennent d'un essai à l'autre. Le script de fin remplace une chaîne qui porte la marque de sa session (le nom de l'essai, la docstring),
non un fragment commun. Le contrôle de compte (`count == 1`) reste ; il a fait son office deux fois.

## 3. La prochaine revue

S726.
