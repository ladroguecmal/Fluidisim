# La même onde pour SGN et la 3D — S719 (liste 4.14)

*S719, 2026-10-09, en autonomie ; l'utilisateur dort.* S718 : de bout en bout, la remontée dépasse le tout-3D de 3,3 cm, et le large (SGN)
en est la cause. ADR-278 D3 posait la question : l'onde de départ, un profil de Boussinesq, n'est l'équilibre ni de SGN ni de la 3D. Ici,
les deux partent de l'onde solitaire de SGN : le profil de Rayleigh, `k = √(3a/(4d²(d+a)))`, plus large de 14 %.

## Ce qui est fait

- **`OndeDepart`** : une seule structure pour les trois solveurs du montage (ADR-276 D1), le profil de Boussinesq ou celui de Rayleigh, la
  vitesse `c·η/(d+η)`.
- **`Large::AucunRayleigh`** et **`Large::BoutEnBoutRayleigh`**, ramenés aux modes de base dès l'entrée de la fonction.

## Reproduire

- `python outils/essai.py the_same_wave_for_serre_and_the_3d_s719 --ignore` (≈ 25 min).

## Mesuré

| | retournement | air | la remontée maximale | coût jusqu'à 5 s |
|---|---|---|---|---|
| S717–S718, l'onde de Boussinesq : le tout-3D | 2,637 s, 9,988 m | 2,790 s | 0,3387 m à 3,917 s | 1 191 s |
| S718, l'onde de Boussinesq : de bout en bout | 2,569 s, 9,863 m | 2,747 s | 0,3714 m (+3,3 cm) | 302 s |
| **E1, l'onde de Rayleigh : le tout-3D** | **2,558 s, 9,963 m** | 2,728 s | **0,3504 m à 3,816 s** | 1 158 s |
| **E2, l'onde de Rayleigh : de bout en bout** | **2,521 s, 9,938 m** | 2,663 s | **0,3806 m à 3,779 s (+3,0 cm)** | 310 s |

Le volume rendu à la mort : −3,7·10⁻⁴. La masse : 7,8·10⁻¹⁵. Le volume entré par la gauche de la bande, nourrie par SGN (l'onde de
Boussinesq, S718) : 0,0300 m³. La 3D en fait passer d'elle-même 0,0309 m³ au même plan (S699).

**Critères** : le retournement (−0,037 s, −0,025 m), l'air, le volume : **tenus**. **La remontée échoue**, à +3,0 cm.

## Ce que cela dit

- **L'onde de départ n'est pas la cause** : avec la même onde pour les deux, l'écart de remontée reste de 3,0 cm (3,3 cm avant).
- **Le volume non plus** : nourrie par SGN, la bande reçoit 3 % d'eau de moins que la 3D n'en fait passer, et remonte pourtant plus haut.
- **La cause est la dynamique propre du porteur.** SGN garde l'amplitude d'une onde très non linéaire (H/d = 0,3), que la 3D réajuste vers
  le bas (S704, S715). Née de SGN, l'onde est plus haute (+10 %) et plus raide : elle plonge plus fort et remonte plus haut.
- **Lequel a raison face à la réalité ne se tranche pas ici** : notre 3D surestimait déjà la crête mesurée par Synolakis avant le
  déferlement (S712–S713). Pour la question d'ADR-278 D3, la réponse est : **ni l'onde de départ ni le volume, mais la dynamique propre de
  SGN contre celle de la 3D**, sur une onde très non linéaire.
- **Ramené à l'usage** : 3 cm de remontée sur une pente de 1:12 font ≈ 35 cm de lame. La houle d'un jeu, moins non linéaire au large, sera
  plus près de SGN.

## La suite

- La question reste inscrite (ADR-278 D3). Elle se rouvrira avec la houle (W) au large, quand le raccord en sera nourri.
- La phase A a ses pièces : la vague de bout en bout fonctionne (S718), pour 4 fois moins de calcul. **Son jalon est une séance visuelle**
  (R43) : la houle du large, le déferlement en 3D, la remontée sur le sable. C'est la suite, après la revue S721.
