# ADR-281 — Quarante-septième revue de méthode (S711–S715)

- **Statut : actée**, S716, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-280](ADR-280-quarante-sixieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S711 | la revue (ADR-280) | — | — |
| S712 | **des résultats lus seulement à la fin d'un calcul long.** Le premier passage de la plage de Synolakis s'est figé à t ≈ 26,6 (la remontée sur sable sec fait tomber le pas) ; les photos de t = 15, 20, 25, déjà prises, n'étaient comparées qu'à la fin | ≈ 35 min de calcul perdues | **D1** |
| S713 | des lignes de résultat masquées : `essai.py` ne montrait que la marque tirée du nom de l'essai (S713), et la fonction partagée écrivait « S712 photo » | quelques minutes à chercher la photo dans le journal | **D1** (l'outil corrigé) |
| S713 | le calcul à 1,25 cm, estimé à 45 min jusqu'à t = 15, en a pris 65, puis le pas est tombé sous 1 ms au déferlement | aucun : le coût était montré (ADR-274 D1), le calcul arrêté dès sa question tranchée | aucune |
| S715 | un long *heredoc* rejeté par bash ; ADR-267 D2 le défend déjà (un script de plus de 20 lignes passe par l'outil d'écriture) | aucun : rien n'avait été exécuté | ADR-267 D2 le dit déjà |
| S715 | **une lecture qui ne pouvait pas changer** : le plancher de l'instrument, pris à 0,4 s juste après la renaissance, lisait un φ reconstruit avant elle (φ ne se refait qu'au pas suivant). Le critère du plancher passait donc trivialement | aucun résultat faux : les autres critères portaient le verdict ; mais un critère qui ne pouvait pas échouer | **D2** |

## 2. Décisions

**D1 — Un calcul long montre chaque résultat dès qu'il est mesuré**, non à sa fin seulement : chaque photo, chaque comparaison, chaque
critère intermédiaire. Un calcul qui se fige ou qu'on arrête laisse ainsi ce qu'il a déjà mesuré. `outils/essai.py` montre toute ligne
marquée d'une session (`S\d{3} `), quelle que soit la fonction qui l'écrit. C'est le complément d'ADR-274 D1 (la progression montrée).

**D2 — Un critère doit pouvoir échouer.** Pour chaque lecture qui fonde un critère, le plan vérifie qu'elle est recalculée après
l'opération qu'elle juge. Un champ reconstruit au pas suivant (φ, les étiquettes) se lit après ce pas. Un plancher de bruit (ADR-280 D1)
se prend entre deux lectures qui *pouvaient* différer.

## 3. La prochaine revue

S721.
