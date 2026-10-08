# Les deux raccords ensemble : la 3D réduite à la bande de déferlement — S693 (liste 4.14 ; ADR-275, étape 1)

*S693, 2026-10-08, en autonomie, vers la v2.* La première étape du LOD de simulation
([ADR-275](../adr/ADR-275-le-lod-de-simulation.md)) : APIC 3D ne couvre plus que la bande où la vague se retourne. Au large, Saint-Venant
pousse la 3D par une zone de colonnes (S650) ; au rivage, il la reprend (S684–S690).

## Reproduire

- `python outils/essai.py both_relays_together_shrink_the_3d_to_the_breaking_band_s693 --ignore` (≈ 10 min) ;
- `… the_offshore_relay_witness_s693 --ignore` (les témoins, ≈ 32 min).

## Ce qui a été fait

- **La zone de colonnes et la sortie à droite, permises ensemble.** Le refus de S682 invoquait « leurs tableaux par particule », mais
  la zone de colonnes n'en a aucun.
- **`RelaisRivage`** :
  - le bord gauche est réglable par l'appelant ;
  - le volume de la 3D est pris par `total_volume` (sans colonnes, le même nombre qu'avant).
- **La vague de S647**, avec APIC 3D sur `[x_r ; 10,775]` m, à 2,5 cm, sur 16 fils.

## Mesuré

| raccord du large `x_r` | ce que Saint-Venant porte de l'onde | particules posées | retournement | air enfermé | masse | calcul (4 s) |
|---|---|---|---|---|---|---|
| **5,0 m** | 1,6 m | **77 664** | 2,524 s, 9,838 m | 2,693 s, 10,245 m | 6,7·10⁻¹⁶ | **10,3 min** |
| 4,0 m | 0,6 m | 104 528 | 2,542 s, 9,863 m | 2,702 s, 10,221 m | 6,6·10⁻¹⁶ | 12,4 min |
| 1,0 m | rien (l'onde naît dans la 3D) | 195 504 | 2,582 s, 9,888 m | 2,759 s, 10,313 m | 4,8·10⁻¹⁶ | 19,8 min |
| le relais au rivage seul (S690) | — | 236 848 | 2,624 s, 9,963 m | 2,777 s, 10,325 m | ≈ 10⁻¹⁶ | 13,6 min |
| le tout-3D (S647–S648) | — | — | **2,642 s, 9,988 m** | 2,817 s, 10,375 m | — | — |

| critère (écrit avant) | mesuré |
|---|---|
| (1) sans colonnes, au bit | tenu (S684 inchangé) |
| (2) le retournement à 0,02 s et 0,15 m du tout-3D | **manqué** : −0,118 s ; la position, tenue (0,15 m) |
| (2) l'air enfermé après lui, en avant | tenu |
| (3) la masse à 10⁻¹² ; la dette sous un quantum | tenu |

## Localisé

- **Deux montages corrigés en route** (ADR-273 D2 enfreint deux fois). La 3D partait de l'onde de S650, à la distance canonique
  (x₁ = 3,488 m), alors que la référence part de 3,4 m. Et `Plage::nouvelle`, repris de S650, centrait lui aussi le Saint-Venant du large
  à 3,488 m : une valeur par défaut cachée dans l'aide. Corrigés, ils ne changeaient presque rien (2,522 → 2,524 s).
- **Le porteur Saint-Venant fait la moitié de l'écart.** Plus il porte l'onde, plus elle se retourne tôt : 2,582 s, puis 2,542 s, puis
  2,524 s. Sans dispersion, il raidit son front ; à 5 cm, la diffusion numérique le masquait (S650 : 0,004 s).
- **Le reste, ≈ 0,04 s, n'est pas départagé.** C'est l'écart entre le raccord à 1,0 m et S690. Il peut venir de la zone de colonnes et
  de son bord ouvert, ou du repère de S650 : le fond posé à z = 0 sous 0,50 m d'eau, et non à 0,05 m sous 0,55 m.

## Ce que cela dit

- La 3D se réduit à la bande de déferlement, avec la masse au bit : trois fois moins de particules.
- Le gain de temps est moindre (10,3 min contre 13,6). Le pas est plus court, et la zone de colonnes pose des particules pour l'eau qui
  entre (77 664 à 92 900).
- **Le raccord au large demande un porteur dispersif.** Pour une vague raide et non linéaire, Saint-Venant ne propage pas l'onde comme
  APIC. Dans le jeu, la houle du large est portée par B (dispersif), qui nourrira la bande 3D (ADR-214). Pour une onde solitaire, il faut
  Serre–Green–Naghdi : la suite (S694), qui servira aussi de référence à la non-linéarité en eau peu profonde (A234).

## Note du 2026-10-08 (S695) — l'attribution corrigée

« Le porteur Saint-Venant fait la moitié de l'écart » est **faux**. Avec SGN, un porteur dispersif, le retournement est le même : 2,524 s
([S695](RELAIS-LARGE-SGN-S695.md)). Les témoins de cette session variaient deux causes à la fois : ce que Saint-Venant porte, et la part de
l'onde qui traverse le raccord du large. C'est la seconde. L'écart vient du raccord du large lui-même : un bord ouvert à vitesse uniforme
sur la verticale, et une zone de colonnes hydrostatique.
