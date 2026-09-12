# ADR-122 — L'ordre en amplitude d'un véhicule non linéaire dispersif

- **Statut : actée**, S193, 2026-09-12 ; autonomie technique S71.
- **Ne remplace aucun ADR.** Complète [ADR-112](ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md)
  en fournissant le véhicule qui manquait, et conserve le seuil de **2 %** d'[ADR-120](ADR-120-b4-tolerance-de-deux-pour-cent.md).
- Ne choisit **aucune** technologie δ. Porte sur les véhicules de banc et sur ce
  qu'un schéma tronqué en amplitude peut revendiquer.
- Mesures : [SURFACE-LIBRE-NL-S193](../validation/SURFACE-LIBRE-NL-S193.md) §7.

## Décision

**Un véhicule non linéaire dispersif de ce dépôt retient l'ordre trois en amplitude.
L'ordre deux est refusé, et il l'est pour une raison qui n'est pas sa précision de
profil.**

Un développement en amplitude tronqué à l'ordre `M` rend le profil d'ordre deux de
Stokes dès `M=2` — mesuré, rapport `b₂` de `0,995269` à `1,002078` en profondeur
infinie, indiscernable de `M=3` à ce titre. Mais **`M=2` rate la moitié du décalage
de fréquence d'ordre trois**, et la fraction qu'il capture **dépend du régime** :
`0,4916 → 0,5004` à `kh=6,2832`, `0,663` à `kh=1,5708`. À `M=3`, le même rapport vaut
`0,997788 → 1,016454` en profondeur infinie, sous les 2 % d'ADR-120.

Conséquence directe et mesurée : sur vingt périodes, l'erreur de profil contre l'onde
de Stokes progressive vaut **21,3 %** à `M=1`, **8,2 %** à `M=2` et **0,33 %** à `M=3`.
Le facteur 25 entre les deux derniers ne vient pas de la forme — leurs profils
instantanés sont presque identiques — mais de la **phase accumulée** par une fréquence
fausse de moitié.

## Ce que la décision énonce, au-delà du choix de `M`

**Un ordre de troncature n'a pas de « taux de fidélité » propre.** L'intuition
naturelle — « l'ordre deux capture les effets d'ordre deux, l'ordre trois affine » —
est fausse sur la grandeur qui compte ici. Le décalage de fréquence en `(ka)²` a deux
sources : la contribution directe des termes quartiques de l'énergie, et la
contribution au second ordre de perturbation des termes cubiques. Une troncature
quadratique perd la première et garde la seconde ; **elle ne rend donc ni zéro, ni le
tout, mais une fraction — et cette fraction est fonction de la profondeur**, donc du
régime, donc non portable. La conclusion naïve « une troncature quadratique ne décale
pas la fréquence » était prédite fausse avant mesure (§3.4) et l'est.

**La règle qui en sort, et qui dépasse ce véhicule.** Tout schéma qui tronque en
amplitude — perturbatif, couche W, candidat δ construit sur un développement — doit
déclarer son ordre **et** la grandeur physique que cet ordre manque. Vérifier un
profil ne suffit pas : deux ordres peuvent rendre le même profil et des fréquences
différentes de moitié, et c'est la fréquence qui gouverne l'erreur au bout d'une
fenêtre de quelques dizaines de périodes.

## Domaine de validité, borné des deux côtés

La décision ne vaut que dans le domaine où l'oracle existe :

- **Cambrure** — `ka ≲ 0,44` en profondeur infinie ; au-delà la crête déferle et la
  surface cesse d'être un graphe.
- **Ursell** — `U = aL²/h³ ≪ 1`. Cette borne est **mesurée comme une falaise** et non
  comme une dégradation : à `U=0,05` le véhicule rend `b₂` à `6,4·10⁻⁵` près sur un
  coefficient de 101,6 ; à `U=65` il est faux d'un facteur 2 ; à `U=130` d'un facteur
  225 ; à `U=261` l'état cesse d'être fini. La divergence n'est pas une instabilité de
  pas — le pas employé est à `0,0967` de sa borne de stabilité — c'est le paramètre du
  développement qui a quitté son domaine.

**Il en résulte qu'aucune réception non linéaire n'est possible en faible profondeur
aux amplitudes qu'un banc emploierait**, faute d'oracle et non faute de candidat : à
`L=8 m` et `h=0,25 m`, `U ≪ 1` impose `a ≲ 2,4·10⁻⁵ m`. Une telle réception exige une
autre famille de références — cnoïdale, ou Boussinesq — qui n'est pas construite ici.
C'est l'objet de **A234**.

## Ce que la décision ne fait pas

1. **Elle ne ferme pas A217.** Disposer d'un véhicule à la fois non linéaire et
   dispersif n'est pas avoir mesuré le couplage de deux trains. ADR-112 conserve
   toute sa portée : une superposition indépendante ne reçoit pas le couplage, et la
   comparaison perturbatif/total reste à faire.
2. **Elle ne choisit pas de solveur δ** et ne fixe aucun seuil de bascule. Aucune
   valeur d'A216 ne s'en dérive.
3. **Elle n'adopte pas la formule de fréquence d'ordre trois en profondeur finie**,
   dont la convention de courant moyen n'est pas déclarée (**A235**) — même si le
   véhicule la reproduit à `0,013 %` près, un accord numérique ne fournit pas une
   convention manquante.
4. **Elle ne branche pas la source B+W** de S190/S191, et n'autorise aucune addition
   des écarts mesurés ici au budget projeté `1,374540 %` : références, champs et
   conditions aux limites diffèrent.
5. **Elle ne revendique aucun contrat runtime.** `f64`, allocations de banc, aucune
   prétention I-03/I-06/I-08 ; la bibliothèque `water-core` est inchangée.

## Ce qu'il faudrait pour l'inverser

Retenir `M=2` redeviendrait licite si l'usage visé **n'observait jamais la phase sur
plus de quelques périodes** — l'erreur de profil instantané y est du même ordre qu'à
`M=3` — ou si une correction de fréquence était appliquée séparément et reçue comme
telle. Passer à `M=4` se justifierait si une réception exigeait le décalage de
fréquence sous `0,1 %` à `ka=0,1`, là où `M=3` mesure `1,6454 %` ; le coût est une
convolution de plus par terme, mesuré nulle part ici.
