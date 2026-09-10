
# ADR-092 — Ce qu'un objet qui entre dans l'eau donne au modèle

- **Statut : actée**, S136, 2026-09-10, autonomie technique S71.
- **Prolonge :** événement d'impact ADR-055, candidat radial ADR-060, portée ADR-083.
- **Résout :** A200 en grande partie — la longueur d'onde cesse d'être arbitraire.

## Problème

`WaveEvent::impact` exige deux nombres, `wavelength_m` et `energy_j`, dont **aucun document ne
dit comment un objet qui tombe les produit**. ADR-055 confie le transfert d'énergie à « un
générateur physique » qui n'existe pas ; ADR-060 qualifie son λ de 4 m de « paramètre d'essai
uniquement » ; ADR-083 pose `λ = α·b` en laissant α « à calibrer ». Trois sessions — S123, S132,
S135 — ont cité ce manque. C'est A200, sévérité 1.

## Ce que la mesure a montré

**La forme spatiale initiale du candidat est exactement homothétique en λ.** Échantillonnée aux
mêmes `r/λ` pour λ de 0,5 à 32 m, elle donne des valeurs identiques à l'écart nul. C'est
structurel — le modèle ne dépend de λ que par `k0 = 2π/λ` — et désormais mesuré.

**Elle a donc un rayon caractéristique proportionnel à λ**, et α n'est pas un paramètre libre :
c'est le rapport qui fait coïncider ce rayon avec la demi-largeur de l'objet.

| définition du rayon | valeur | α correspondant |
|---|---|---|
| mi-hauteur | 0,1830 λ | 5,46 |
| **premier zéro** | **0,2985 λ** | **3,35** |
| rayon de giration | 0,1636 λ | 6,11 |

**L'énergie que le modèle accepte suit une loi exacte.** À pente et longueur d'onde variables,
la dichotomie donne `E_max / λ⁴` **constant à 8,9401 × 10⁻²** pour λ de 0,5 à 8 m, et un rapport
de **16,00** exactement quand la pente admise quadruple. Soit, en forme homogène :

```
E_max = K · ρ · g · λ⁴ · s²        K ≈ 8,89 × 10⁻⁴ (sans dimension)
```

où `s` est la pente maximale du milieu. La dépendance en `λ⁴` et en `s²` est mesurée ; celle en
`ρ` et `g` est celle qui rend la formule homogène.

## Décision

**La longueur d'onde se dérive** : `λ = α · b`, avec **α = 3,35**, le rapport du premier zéro de
la forme initiale. C'est l'étendue de la perturbation centrale — celle que la cavité creuse — et
c'est la lecture qui correspond à la demi-largeur mouillée de Wagner (SPEC-001 §5 bis), laquelle
vaut `b` à la fin de l'impact.

**L'incertitude est bornée et dite** : selon la définition retenue du rayon, α vaut entre 3,35 et
6,11. Le banc B2 resserrera ; en attendant, ce n'est plus un paramètre libre sur [1, 2π] mais une
valeur avec une fourchette d'un facteur 1,8. **Les valeurs α = 1 et α = 2, utilisées comme
hypothèses en S123, sont hors de cette fourchette** — les portées y étaient donc sous-estimées.

**L'énergie ne se dérive pas, et ne le peut pas.** La fraction de l'énergie d'entrée qui part en
ondes de gravité — le reste allant dans la gerbe, la cavité, la turbulence et la chaleur — est
une propriété physique externe qu'aucune lecture du modèle ne produit. Elle reste **à calibrer**,
et **elle est un paramètre explicite de l'appelant**, jamais une constante enfouie :

```
E = η · ½ · ρ · b³ · v²        η : fraction transférée, À CALIBRER — banc B2
```

où `½ρb³v²` est l'énergie de la masse ajoutée emportée à la vitesse d'entrée (SPEC-001 §5 bis,
`m_a = ½πρc²` par mètre, soit `~ρb³` en trois dimensions, à un facteur d'ordre 1 près).

**Mais η est borné par le modèle**, et cette borne se calcule :

```
η ≤ 2 K g α⁴ b s² / v²
```

Elle dit quelque chose que personne n'avait écrit : **la fraction représentable décroît comme le
carré de la vitesse et croît avec la taille**. Les impacts rapides de petits objets sont ceux que
le candidat peut le moins porter.

**Le générateur rend les deux nombres, pas l'événement.** `impact_from_entry` produit
`(energy_j, wavelength_m)` et refuse si l'énergie demandée dépasse ce que le modèle accepte.
L'identité, le référentiel, la naissance et le TTL restent au gameplay : ce sont ses données,
pas de la physique.

## Ce que cette décision ne fait pas

Elle ne calibre pas η, et ne prétend pas que la valeur de α soit définitive : elle remplace un
arbitraire par une dérivation assortie d'une fourchette. Le banc B2 reste nécessaire pour les
deux.

Elle ne modifie aucun calcul existant : le candidat, ses bornes et ses résultats sont inchangés,
et les hachages de campagne doivent le confirmer. Elle n'ajoute pas de dépendance : le générateur
lit le milieu et rend deux nombres.

Elle ne traite ni l'anisotropie — refusée par ADR-060 — ni l'angle de relèvement `β`, qui
n'intervient dans Wagner que par la **durée** de l'impact, pas par son étendue finale.

## Réception

[GENERATEUR-S136](../validation/GENERATEUR-S136.md). La loi d'énergie est vérifiée contre la
dichotomie sur plusieurs longueurs d'onde et pentes, l'homothétie de la forme est mesurée, et un
impact engendré par le générateur est construit puis échantillonné — le générateur produit des
événements que le candidat accepte, ce qui est le seul critère qui compte pour lui.

## Note corrective du 2026-09-10 (S137)

Cette décision renvoie la calibration **au banc B2**. C'est faux : B2 choisit la technologie de W
et `λ_cut`, et aucune de ses métriques ne mesure ce qu'un objet qui entre dans l'eau émet. La
calibration de la source relève de **B10**, dont le protocole — sphères et corps allongés à
Froude connu — fournit déjà les entrées, et auquel S137 ajoute deux métriques de source.
Voir [ADR-093](ADR-093-ou-se-calibre-la-source-d-impact.md).
