# Empreintes de non-régression

*Tenu par `python outils/non_regression.py --inscrire` (S483, ADR-222 D3) — ne pas modifier à la main.* Une empreinte se réinscrit
quand un changement voulu modifie le résultat, et le journal de la session le dit. Le pas médian dépend de la carte et de sa
température : le seuil est 1,3 fois la valeur inscrite.

## Reproduire

`python outils/non_regression.py` (vérifier) ; `python outils/non_regression.py --inscrire` (réinscrire). Le détail des trois vérifications
est en tête de l'outil et dans [REFERENCE-PARALLELE-S483](REFERENCE-PARALLELE-S483.md) §3.

| clé | valeur |
|---|---|
| `bulle_empreinte_fils_1` | `079c9a99cc007f48` |
| `bulle_empreinte_fils_16` | `079c9a99cc007f48` |
| `poches_ecarts_detection` | `0` |
| `poches_nombres_differents` | `0` |
| `poches_pire_ecart` | `2.0e-06` |
| `v1_trajectoire_5s` | `pas=269 colonnes=173 n=5370` |
| `v1_masse_ecart_quanta` | `0` |
| `v1_pas_median_ms` | `14.10` |
