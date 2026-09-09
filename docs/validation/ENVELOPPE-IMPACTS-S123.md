# S123 — Le couloir du candidat radial face aux impacts du jeu

2026-09-09. Suite de S122-1, sur A199. **Session de conception** : la mesure y sert une
décision, elle n'en tient pas lieu.

## 1. D'où vient la longueur d'onde ? De nulle part.

Le candidat radial est paramétré par `wavelength_m`, portée par l'événement d'impact. Le
spectre est la bande `[k0/2, 2k0]` autour de `k0 = 2π/λ` (ADR-060). Tout le comportement du
champ — son étendue, sa vitesse de propagation, les bornes qui l'acceptent ou le refusent —
découle de ce seul nombre.

**Aucun document ne dit comment le choisir.** La lecture est sans ambiguïté :

- ADR-055 valide `wavelength_m` comme « longueur d'onde positive en mètres », et rien de plus.
  Pour l'énergie, il ajoute que « le générateur physique devra établir ce transfert » — le
  même report vaut implicitement pour la longueur d'onde, et ce générateur n'existe pas.
- ADR-060 pose la bande autour de `k0` et précise que ce choix de forme « est **à calibrer
  B2**, pas une loi physique de contact », puis que « l'événement apporte son énergie, pas la
  forme ». Le λ de ses résultats — 4 m — est explicitement un « paramètre d'essai uniquement ».
- Le registre des angles morts ne contient rien sur ce point ; c'est un trou, pas une décision.

Autrement dit : **le candidat est piloté par une grandeur que personne ne sait produire.** Le
couloir mesuré en S122 est un couloir en λ ; tant que λ ne se rattache à rien de physique, dire
qu'un cas de jeu « tombe dedans » ou « tombe dehors » n'a pas de sens. C'est le premier
résultat de la session, et il précède la confrontation demandée.

## 2. Ce que le corpus permet d'écrire, sans inventer de nombre

I-14 interdit tout nombre sans provenance, et autorise l'étiquette « à calibrer » avec le banc
qui le fixera. Le corpus fournit de quoi caractériser l'**étendue** de la perturbation, à
défaut de sa longueur d'onde :

- **SPEC-001 §5 bis** (Wagner, migré depuis ADR-023 en S14) : une carène de relèvement `β`
  entrant à vitesse `v` mouille une demi-largeur `c(t) = (π/2)·v·t / tan β`, et met
  `t_impact = 2·b·tan β / (π·v)` à mouiller une demi-largeur `b`. À la fin de l'impact,
  **l'étendue perturbée vaut la demi-largeur de l'objet**, et cela ne dépend ni de `v` ni de `β`.
- **SPEC-001 §5** : `λ = 2πv²/g` pour les vagues transverses d'un sillage. Cette formule
  concerne un objet en **mouvement horizontal établi**, pas une entrée verticale ; l'employer
  ici serait un détournement, et elle n'est pas retenue.

D'où la seule écriture défendable aujourd'hui :

```
λ = α · b        b = demi-largeur de l'objet à la fin de l'impact
                 α : sans dimension, À CALIBRER — banc B2
```

**La valeur de α n'est pas devinable depuis le corpus**, et cette session ne la fixe pas. Elle
fait autre chose, qui ne demande pas de la connaître : établir **la sensibilité du verdict à
α**. Si un cas de jeu est refusé pour tout α d'un facteur 1 à 2π, le refus ne dépend pas de la
calibration et le constat tient. C'est ce que mesure §4.

## 3. Ce que les bornes deviennent en fonction de la taille

En substituant `λ = α·b` dans les trois bornes couplées d'ADR-082 :

| borne | condition | en fonction de la taille |
|---|---|---|
| `Reach` | `hi · radius ≤ 64`, `hi = 4π/λ` | **`radius ≤ 5,09 · α · b`** |
| `Regime` | `depth > π/lo = λ` | **`depth > α · b`** |
| `Resolution` | `dk·(radius + c_g·âge) ≤ π/2` | dépend en plus de l'horizon |

Deux conséquences se lisent directement, avant toute mesure :

- **La portée du champ est proportionnelle à la taille de l'objet.** Un petit objet ne peut pas
  produire un champ étendu : à `α = 2`, une pierre de 10 cm ne peut être calculée que dans un
  rayon de 1 m. Ce n'est pas absurde physiquement — ses ondes sont courtes et s'amortissent —
  mais c'est une contrainte de conception que rien n'énonçait.
- **La profondeur requise est proportionnelle à la taille de l'objet.** Un objet de 20 m de
  demi-largeur exige, à `α = 2`, plus de 40 m d'eau. Un vaisseau qui se pose dans un port en
  est exclu, quel que soit le soin apporté au reste.

## 4. Mesure

*(à compléter)*
