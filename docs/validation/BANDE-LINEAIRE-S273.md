# Bande du fond en quadrature linéaire — S273

Réception d'[ADR-166](../adr/ADR-166-quadrature-lineaire-de-la-bande.md) dans le transport réel
de `step_perturbation_mobile`, puis nouveau passage du banc temporel
[S272](RESIDU-TEMPOREL-S272.md). Critères écrits **avant** le code et la campagne.

## Critères de construction

1. **Quadrature** — fixture S272 (h = 1 m, k = π/2, phase 0,37, a = 0,01 m, repos aligné),
   surfaces du pas réel (demi-somme des colonnes à l'intérieur, colonne voisine plus fond au
   bord). Flux de bande de **toutes** les faces, calculés par le code produit, contre l'intégrale
   analytique `q·cosθ·(sinh k(h+ζ_f) − sinh kh)/(k cosh kh)`. Erreur L2 relative :
   ≤ 0,5 / 0,1 / 0,025 % à dx = 0,125 / 0,0625 / 0,03125, et rapport ≥ 3,5 à chaque
   raffinement (ordre deux ; diagnostic S272 : 0,419 / 0,089 / 0,016 %). Témoin rectangle
   recalculé dans le test : ≥ 1,5 % à la maille fine, pour prouver que le test distingue le défaut.
2. **Bandes signées et coupées** — fond affine `U = U0 + S·(z − repos)`, pour lequel la règle est
   exacte : surface sous et sur le repos, bande couvrant trois couches, surface sur une face de
   couche, fond solide au-dessus du repos à la face extérieure. Écart à l'intégrale exacte
   ≤ 2e-7 m²/s (arrondi f32 de S270).
3. **Identités** — fond nul S253 au bit, fond uniforme S270 ≤ 8 ε, témoin sans résidus,
   transaction et expiration : tests existants, inchangés.
4. **Suite complète** sans échec ; toute valeur imprimée qui bouge est expliquée par le fond
   non uniforme. Zéro allocation dans le pas (tests d'exécution existants).

## Critères de la campagne

Les cinq passages S272, mêmes commandes, traces `fluidisim-s273-*.log`. **Critères S272
inchangés** : erreur L2 de `η'` ≤ 2 % à la maille fine et décroissante ; demi-pas ≤ 0,5 % ;
champs divisés par a² ≤ 1 %. Aucun seuil relevé, aucune fenêtre raccourcie.

Qualification des deux biais, **diagnostic seulement**, déclarée ici avant mesure :

- **amplitude** : si `η' = a²η₂ + a³η₃ + …`, alors `N* = 2·N(a/2) − N(a)`, avec `N = η'/a²`,
  élimine le terme d'ordre trois. L'écart de `N*` à l'oracle `η₂` sépare l'erreur numérique
  (écart qui reste) de la troncature de l'oracle (écart qui disparaît) ;
- **temps** : la différence au demi-pas estime l'erreur temporelle (le double si ordre un).

Arrêt : verdict, ou diagnostic chiffré avec le prochain correctif concret. Pas de nouvelle
campagne de raffinement dans cette session.

## Réception de la construction

`band_layer` (`delta_coupling.rs`) porte la règle ; les faces intérieures l'appellent avec un
plancher nul et l'ouverture d'avant, les faces extérieures avec le fond de leur colonne.

| critère | résultat |
|---|---|
| 1. quadrature, flux produit de toutes les faces | **0,4190 / 0,0892 / 0,0162 %** (rapports 4,7 et 5,5), identiques au diagnostic S272 ; témoin rectangle 8,41 / 3,86 / 1,60 % |
| 2. fond affine, 9 bandes au bord × 3 couples (U0, S), 17 faces intérieures | écart maximal **3,9·10⁻⁸** m²/s |
| 3. identités | fond nul S253 au bit, fond uniforme S270, témoin sans résidus, refus et expirations : inchangés, verts |
| 4. suite complète | **480 réussis, 0 échec**, 21 ignorés (368 cœur, 17 intégrations, 95 harnais) ; release : S253 128 colonnes et harmonique reçus |

Valeurs déplacées, toutes par un fond non uniforme :

- S271, cinématique initiale : **3,048 / 1,237 / 0,585 % → 1,509 / 0,445 / 0,137 %** ; il reste
  la différence finie de la divergence, d'ordre deux.
- S253, harmonique `2k` couplée à 32 colonnes, une période : 2,17 → **2,14 %** (seuil 20 %) ;
  témoin sans résidus 99,56 % inchangé (bande éteinte).
- S254, intégration du B de production : 590 faces mouillées au-dessus du plan moyen, comme
  avant ; `u'` maximal 5,36·10⁻³ m/s aux murs.
