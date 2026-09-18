# ADR-166 — Quadrature linéaire de la bande du fond

Actée S273, 2026-09-18, autonomie S71. Remplace la **formule de quadrature** de la bande
d'[ADR-152](ADR-152-surface-mobile-couplee.md) (faces intérieures) et
d'[ADR-165](ADR-165-bande-du-fond-aux-frontieres.md) (faces extérieures). Le reste de ces deux
décisions — débit `Q_v` S237, géométrie totale, lecture de `ζ^n`, ouverture, fermeture de `v` au
bord, ordre de sommation et compensation — est inchangé.

## Constat

La bande d'une couche vaut `U(z_c)·(e − s)`, où `[s, e]` est la partie de la couche comprise entre
le repos et la surface, et `z_c` le centre de la couche, point de l'échantillon. Quand la bande
ne couvre qu'une partie de la couche, son milieu n'est pas `z_c` : la règle est d'ordre un.
S272 l'a isolé sur le fond progressif de profondeur finie, avec les surfaces du pas réel, contre
l'intégrale analytique : **8,41 / 3,86 / 1,60 %** d'erreur sur le flux aux trois mailles
([RESIDU-TEMPOREL-S272](../validation/RESIDU-TEMPOREL-S272.md)). C'est précisément la couche où
vit la bande qui est partielle, donc ce biais n'est pas marginal : la divergence de la bande est
le forçage cinématique d'ordre deux de `η'` (ADR-152, `ζ_t + U(ζ)ζ_x − W(ζ) = ∂x∫_0^ζ U dz`).

## Décision

Dans chaque couche `k`, avec `z_c = (k + ½)·dx`, `[s, e]` le segment signé du repos à la surface
tronqué aux bornes de la couche (et au fond solide aux faces extérieures, comme ADR-165) :

```text
bande_k = (e − s) · ( U + ∂zU · ((e + s)/2 − z_c) )
```

`U` et `∂zU = grad_u[0][2]` sont ceux de l'échantillon de la face, déjà fournis (ADR-113/154).
C'est l'intégrale exacte de la reconstruction linéaire de `U` autour de `z_c`. Une seule fonction
porte la règle, appelée par les faces intérieures (multipliée par l'ouverture, comme avant) et
par les deux faces extérieures. Aucun nouvel échantillon, paramètre, stockage ni allocation.

## Conséquences

- **Ordre deux** pour un fond régulier sur chaque couche traversée ; **exacte** pour un `U`
  affine en `z`.
- **Identités conservées** : fond nul (bande nulle, pas S237 au bit), fond uniforme (`∂zU = 0`,
  règle d'origine), témoin sans résidus S253 (bande éteinte).
- Les valeurs des cas à fond non uniforme bougent ; les reçus antérieurs qui en dépendent se
  rejouent (S253, S254, S271, S272), leurs anciennes valeurs restent dans Git.

## Limite

Le prolongement de production (ADR-154) a un pli à `z = 0` : `∂zU` y saute. Si le repos tombe
**strictement à l'intérieur** d'une couche, l'échantillon de cette couche peut être de l'autre
côté du pli que la bande, et cette couche seule reste d'ordre un. Tous les appelants actuels
alignent le repos sur une face de couche. Rien ici ne reçoit l'exactitude du prolongement
lui-même, ni une houle incidente transparente, ni `W(b) ≠ 0`.

[Critères et réception](../validation/BANDE-LINEAIRE-S273.md).
