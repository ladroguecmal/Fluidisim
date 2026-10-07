# Le rouleau 3D, étape 1 : un fond en pente dans APIC 3D — S639 (listes 4.14, 4.16)

*S639, 2026-10-07, en autonomie, sur la demande de l'utilisateur* (« reprend avec 1 », le plan du rouleau 3D en cinq étapes accepté : « Ok
très bien »). Étape 1 : APIC 3D n'avait ni fond ni pente — seulement des parois et une sphère (S393).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s639 -- --nocapture` (≈ 11 s) ; les 47 essais d'APIC 3D
  passent ; suite du cœur : 823 essais listés.

## 1. Ce qui est construit

`Apic3::set_seabed(fond)` : une hauteur par colonne, mise en **escalier** (les mailles dont le centre est sous elle deviennent `SOLID`) ;
`seabed_height`. À chaque pas : les faces qui touchent le fond sont des parois immobiles ; les particules qui y entrent sont reposées
au-dessus, sans vitesse descendante ; la reconstruction de la surface **reflète** les particules sous le fond, et à travers les
**contremarches** (une colonne voisine dont le fond monte plus haut, à portée du noyau). La vitesse de la sphère ne s'impose plus qu'aux
mailles de la sphère (le fond est aussi `SOLID`). Le réglage alloue, le pas non.

## 2. Mesuré

Un canal de 48 × 4 × 16 mailles de 5 cm ; fond plat à 5 cm puis pente 1:3 depuis 0,805 m ; eau au repos à 0,4 m (rivage à 1,855 m) ; 2 s.

| | critère (écrit avant) | mesuré |
|---|---|---|
| mailles solides ; particules posées, gardées, hors du fond | 852 ; 6 048, aucune perdue, aucune sous le fond | tenu |
| la vitesse parasite sur la pente | ≤ 1 cm/s (le critère de repos de S388, S393) | **1,51 cm/s — manqué** |
| le témoin à fond plat | ≤ 1 cm/s | 2,9·10⁻⁶ m/s |
| refus : longueur, non fini, hors du domaine | | tenu |

**Amendement du plan, avant toute mesure de repos** : la pente partait de 0,8 m, où `(k + ½)·dx` égalait exactement le fond pour certaines
colonnes — f32 et f64 tranchaient ces égalités différemment (6 016 particules contre 5 984). À 0,805 m, aucune égalité (asserté).

## 3. Le critère manqué, localisé

L'écart naît au rivage à t = 0, culmine vers 0,2 s, puis une petite onde repart vers le large en s'amortissant (sous 1 mm/s après 1 s).
Localisé (ADR-226) avant tout remède :

1. **Sans réflexion des contremarches** : 4,7 cm/s — la marche voisine, sèche, rendait la surface plus basse contre elle (le défaut de la
   sphère de S393 avant sa réflexion) ;
2. **avec la réflexion des contremarches de même hauteur** : 1,5 cm/s ; l'image de coin (contremarche puis fond) l'aggravait (1,8 cm/s),
   retirée ;
3. **avec toute contremarche à portée du noyau** : 1,5 cm/s, aux colonnes d'une seule maille d'eau du rivage.

À 2,5 cm, le pic ne baisse que de 1,51 à 1,18 cm/s : c'est la géométrie, non la résolution — un escalier n'est pas une pente, et une marche
à demi mouillée excite l'eau. En hauteur, environ 1 mm de vague. **Le remède est connu : les faces coupées** (un fond lisse), le suivant de
cette campagne. L'essai n'affirme que ce qui a tenu (ADR-244).

## 4. Ce que cela dit — et ne dit pas

APIC 3D a désormais un fond quelconque : la masse exacte, aucune particule dans le sol, un canal plat au repos à 3·10⁻⁶ m/s. Sur une pente en
escalier, un frémissement d'un millimètre au démarrage, au rivage. Pour la suite — une vague qui monte la pente à 0,5–1 m/s — il pèse
quelques pour cent ; il sera levé par les faces coupées.

Manquent : les faces coupées, la vague (étape 2), le déferlement (étape 3), le relais 2D → 3D (étape 4), le rouleau qui agit (étape 5).
