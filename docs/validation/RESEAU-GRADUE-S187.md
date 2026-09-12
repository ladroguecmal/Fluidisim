# S187 — Le réseau d'échantillonnage gradué en profondeur

2026-09-12. S186-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[COMPOSITION-ERREURS-S186](COMPOSITION-ERREURS-S186.md) a trouvé, sans le chercher, que
l'erreur d'un réseau d'échantillonnage isotrope est **intégralement celle de sa tranche la
plus haute** : 2,54 % à `r = 2` pour le bloc entier, et 2,54 % pour la seule tranche
`z = −0,80 m`, quand la tranche du fond ne vaut que 0,11 %. Treize tranches sur quatorze sont
surrésolues, d'un facteur allant jusqu'à vingt-trois.

Un réseau isotrope dépense donc la même densité de nœuds là où le contenu est lisse et là où
il ne l'est pas. Cette session mesure ce que coûte cette uniformité, et ce qu'un réseau
**gradué** rend.

Comme en S183–S186, les conditions sont publiées **avant** la première exécution (§1–§7) ;
les relevés viennent après (§8).

## 1. Les deux questions, dans cet ordre

**Q1 — quelle part de l'erreur de la tranche haute vient de l'axe vertical ?** S186 n'a mesuré
qu'un réseau isotrope : il ne sait pas si les 2,54 % viennent de `x`, de `y` ou de `z`. La
réponse décide de tout le reste. Si elle est majoritairement **verticale**, graduer en `z`
réduit l'erreur *et* le nombre de nœuds. Si elle est majoritairement **horizontale**, aucune
graduation verticale ne l'abaisse — et le gain sera uniquement en nœuds, à erreur inchangée,
ce qui reste le but mais n'est pas la même annonce.

Cette question passe **avant** la graduation, et non après, parce qu'elle dit comment lire la
courbe qui suit.

**Q2 — à erreur égale, combien de nœuds un réseau gradué économise-t-il ?** C'est le livrable :
une courbe **erreur contre nombre de nœuds**, sur laquelle le réseau isotrope et le réseau
gradué sont deux familles de points. Le gain, s'il existe, est la distance horizontale entre
les deux familles à ordonnée égale — et non une amélioration d'erreur à réseau donné.

## 2. Le véhicule, et l'unique référence

Inchangés de S186, et c'est la condition pour que les chiffres se comparent : `examples/support/`,
bloc côté 16, 2744 mailles intérieures, `dx = 0,25 m`, `dt = 10 ms`, 100 pas, `T0 = 1,5 s`,
même montage d'eau, même pas. Mailles intérieures à `z ∈ [−4,05 ; −0,80] m`. Le solveur du
projet reste à B3 (ADR-007 §5).

**La même unique référence** : réseau plein (`r = 1`), source reconstruite à chaque pas,
chargée sans interpolation. Donc le même plancher, **0,386 %**, et les mêmes dénominateurs
`max |S| = 1,540547e-4 m/s²` et `max |u'(T)| = 7,993168e-5 m/s`. Toute ligne de S186 doit
pouvoir être reproduite par ce véhicule, et la réception 3 l'exige.

**La cadence est tenue à `c = 1` pour toute cette session.** S186 a montré que la composition
suit le maximum pour les modes causaux : l'axe temporel est donc séparable, et le mélanger ici
n'ajouterait que du bruit à une question spatiale. La graduation et la cadence se composeront
selon la loi déjà mesurée.

## 3. Ce qui est ajouté au support, et ce qui n'est pas touché

`nodes_per_axis`, `lattice_points` et `scatter` sont utilisés par S184 (coût) et S186
(composition), et S186 a publié deux empreintes qui en dépendent. **Ils ne sont pas
modifiés.** Deux formes sont ajoutées à côté :

1. **réseau à pas par axe** — `[rx, ry, rz]` au lieu d'un `r` unique. Sert Q1.
2. **réseau à indices quelconques par axe** — trois listes d'indices de mailles où les nœuds
   se posent. C'est la forme générale ; la graduation en est un cas.

La forme générale doit **reproduire la forme uniforme en bits** quand on lui donne des indices
uniformes. C'est une réception, pas une intention : une implémentation générale qui ne retrouve
pas son cas particulier est fausse quelque part, et l'écart se lirait ensuite comme un effet de
la graduation.

**Horizontalement le pas reste uniforme.** Un pas horizontal qui dépendrait de `z` ne serait
plus un réseau mais un arbre, et le `scatter` trilinéaire ne s'y applique pas. Cette limite est
assumée et elle borne le gain annoncé.

## 4. La règle de graduation, dérivée avant d'être codée

L'erreur d'une interpolation linéaire sur une maille vaut, au second ordre,

```
e ≈ (1/8) · Σ_axes  h_axe² · |∂²S/∂x_axe²|
```

Égaliser la contribution d'une tranche à l'autre — c'est-à-dire ne plus laisser une seule
tranche fixer le maximum — demande donc

```
h_z(z) · √|∂²_z S(z)|  =  constante
```

Les nœuds se posent alors à **incréments égaux de** `Φ(z) = ∫ √|∂²_z S| dz`, et le nombre de
nœuds `N` fixe la constante. C'est l'équidistribution classique, et elle a ici une conséquence
chiffrable **avant** la mesure : avec le profil de S186, `|∂²_z S| = k_eff_z² · max|S|` passe
de `3,83e-6` à la tranche 2 à `4,84e-5` à la tranche 13, soit un rapport **12,6**. Le pas
vertical profond peut donc valoir **√12,6 ≈ 3,5 fois** celui du haut.

**Le profil n'est pas repris de S186 : il est remesuré par le programme**, au même instant
`T0`, par différences finies sur le réseau plein. Recopier un profil d'une session à l'autre
est exactement ce que ce dépôt a déjà payé (A185) ; et un profil mesuré ici est un profil dont
la provenance est le programme qui l'utilise.

Trois précisions qui font partie du protocole :

- les nœuds sont **accrochés aux indices de mailles** — le réseau ne peut pas se poser entre
  deux centres, puisque la référence, elle, est aux centres. L'accrochage est arrondi au plus
  proche, les doublons sont fusionnés, et le nombre de nœuds réellement obtenu est publié à
  côté du nombre demandé ;
- la tranche haute et la tranche basse **portent un nœud**, sinon les mailles du bord seraient
  extrapolées au lieu d'être interpolées ;
- `Φ` est calculé sur `|∂²_z S|` **agrégé par tranche en maximum**, cohérent avec la métrique
  du dépôt, qui est un maximum et non une moyenne quadratique.

## 5. Ce qui est mesuré

Trois campagnes, toutes à `c = 1`, toutes contre la même référence.

**Q1 — attribution par axe.** Pour `r ∈ {2, 4, 8}` : décimation **verticale seule**
(`[1, 1, r]`), **horizontale seule** (`[r, r, 1]`), et **isotrope** (`[r, r, r]`, qui doit
redonner S186). Erreur globale, erreur de la tranche haute, et nombre de nœuds pour chacune.

**Q2 — courbe iso-erreur.** Deux familles :

- **isotrope**, `r ∈ {1, 2, 4, 8}` — les quatre points de S186 ;
- **gradué**, pas horizontal `rh ∈ {1, 2, 4}` et nombre de nœuds verticaux demandés
  `Nz ∈ {3, 4, 5, 6, 8, 14}`.

Pour chaque point : nœuds, `eS`, `eU`, et l'erreur de la tranche haute. La lecture se fait à
ordonnée égale.

**Un témoin qui peut invalider la règle.** À nombre de nœuds verticaux égal, la graduation
dérivée est comparée à **deux graduations naïves** : le pas uniforme, et un pas géométrique de
raison fixe 2 posé à la main. Si la règle dérivée ne bat ni l'un ni l'autre, c'est la
dérivation de §4 qui est fausse — et il faudra le dire, non l'omettre.

## 6. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** Deux exécutions, mêmes bits ; une empreinte est publiée. Ce
   véhicule ne mesure aucune durée, donc sa sortie entière est un résultat.
2. **Le général reproduit le particulier.** Réseau à indices uniformes = réseau à pas uniforme
   = `scatter` de S186, **bit pour bit**, pour `r ∈ {1, 2, 4, 8}`.
3. **S186 est redonné.** Les trois erreurs isotropes — 2,5401 / 13,6043 / 32,9593 % — et le
   plancher 0,386 % doivent réapparaître, et les empreintes publiées de S185
   (`0x39567a1d4bc2ba4c`) et de S186 (`0x0e743846d4656870`) doivent être **inchangées** après
   l'extension du support. Vérifié par exécution des deux programmes.
4. **Le réseau couvre le bloc.** Aucune maille intérieure n'est extrapolée : sur chaque axe, le
   premier nœud est au plus bas indice intérieur et le dernier au plus haut, ou au-delà.
5. **Tout reste fini**, source et champ, sur toutes les configurations.

## 7. Ce que la mesure ne prouvera pas

- **Aucun seuil de justesse.** A50 attend une décision, pas un chiffre de plus.
- **Un seul montage, une seule profondeur de bloc.** Le profil `∂²_z S` est celui de ce
  contenu à cette profondeur ; un bloc affleurant la surface donnerait un autre profil, et
  probablement un gain plus grand. La **règle** voyage, ses nœuds non.
- **Aucun gain de temps mesuré.** Le nombre de nœuds est un coût géométrique. S184 a établi
  que le coût suit exactement le nombre de nœuds pour la reconstruction — c'est donc une
  conversion légitime, mais elle est citée, pas remesurée, et l'interpolation d'un réseau
  gradué coûte un peu plus par maille qu'un réseau uniforme, ce qui n'est pas chiffré ici.
- **Pas de réseau horizontalement gradué**, ni d'arbre. Voir §3.
- **Le profil est mesuré à un seul instant.** Un contenu dont la structure verticale évoluerait
  fortement demanderait une graduation réactualisée, et le coût de la réactualisation n'est pas
  mesuré.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur (S184 §3).
