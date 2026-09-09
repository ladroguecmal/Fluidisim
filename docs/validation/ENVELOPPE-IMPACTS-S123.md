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

`code/water-core/examples/impact_envelope.rs` construit le candidat pour onze cas de jeu, en
séparant deux questions que sa première version confondait : la **géométrie** — le couple
(taille, portée, profondeur, horizon) est-il admissible ? — ne dépend pas de l'énergie et se
mesure à énergie négligeable ; l'**amplitude** est un plafond, mesuré ensuite.

*(La première tentative prenait 10⁻⁹ J pour « négligeable ». Aux très courtes longueurs d'onde
la pente y dépasse déjà la limite du milieu, et le refus se lisait comme une impossibilité
géométrique qu'il n'était pas. Il a fallu descendre à 10⁻³⁰ J pour isoler vraiment la
géométrie — le genre d'erreur qu'une mesure attrape et qu'un raisonnement laisse passer.)*

### 4.1 À la portée demandée, presque tout est refusé

| cas | b (m) | portée voulue | profondeur | α = 1 | α = 2 | α = 4 | α = 2π |
|---|---|---|---|---|---|---|---|
| goutte de pluie | 0,001 | 0,05 m | 20 m | `Reach` | `Reach` | `Reach` | `Reach` |
| balle d'arme | 0,005 | 0,5 m | 20 m | `Reach` | `Reach` | `Reach` | `Reach` |
| pierre lancée | 0,05 | 3 m | 20 m | `Reach` | `Reach` | `Reach` | `Reach` |
| pas de personnage | 0,15 | 3 m | 1 m | `Reach` | `Reach` | **construit** | **construit** |
| plongeon humain | 0,3 | 10 m | 5 m | `Reach` | `Reach` | `Reach` | `Reach` |
| caisse jetée | 0,5 | 15 m | 10 m | `Reach` | `Reach` | `Reach` | **construit** |
| véhicule léger | 1,5 | 40 m | 20 m | `Reach` | `Reach` | `Reach` | **construit** |
| petit vaisseau | 5 | 100 m | 20 m | `Reach` | `Reach` | `Regime` | `Regime` |
| vaisseau en port | 20 | 200 m | 20 m | `Reach` | `Regime` | `Regime` | `Regime` |
| **vaisseau haute mer** | 20 | 200 m | 2000 m | `Reach` | **construit** | **construit** | **construit** |
| explosion de surface | 3 | 80 m | 50 m | `Reach` | `Reach` | `Reach` | **construit** |

**Le verdict ne dépend pas de la calibration.** C'était l'objet du balayage de α : sur onze cas,
un seul se construit à la portée voulue pour α = 2, quatre autres seulement à α = 2π, et les
trois plus petits sont refusés pour toute valeur de α. Le constat tient donc sans connaître α.

### 4.2 La portée atteignable vaut dix fois la taille de l'objet

| cas | portée voulue | portée atteinte | part | borne au-delà |
|---|---|---|---|---|
| goutte de pluie | 0,050 m | 0,002 m | 3,2 % | `Resolution` |
| balle d'arme | 0,500 m | 0,018 m | 3,7 % | `Resolution` |
| pierre lancée | 3,0 m | 0,508 m | 16,9 % | `Resolution` |
| pas de personnage | 3,0 m | 1,528 m | 50,9 % | `Reach` |
| plongeon humain | 10,0 m | 3,056 m | 30,6 % | `Reach` |
| caisse jetée | 15,0 m | 5,093 m | 34,0 % | `Reach` |
| véhicule léger | 40,0 m | 15,279 m | 38,2 % | `Reach` |
| petit vaisseau | 100,0 m | 50,930 m | 50,9 % | `Reach` |
| vaisseau en port | 200,0 m | **aucune** | — | `Regime` |
| vaisseau haute mer | 200,0 m | 203,718 m | 101,9 % | `Reach` |
| explosion de surface | 80,0 m | 30,558 m | 38,2 % | `Reach` |

Au-dessus d'une quinzaine de centimètres, la portée mesurée vaut exactement **10,18 · b**,
c'est-à-dire `5,09 · λ` : la valeur prévue par `Reach`. En dessous, c'est `Resolution` qui mord
d'abord — les ondes courtes se dispersent vite, et l'horizon demandé les emmène hors du
contrôle de phase.

### 4.3 Le plafond d'énergie

Énergie maximale que le candidat porte avant que la pente dépasse la limite du milieu (0,1),
à α = 2 et à la portée géométrique maximale, rapportée à l'ordre de grandeur de l'énergie mise
en jeu — masse ajoutée `~ρb³` à la vitesse d'entrée, SPEC-001 §5 bis :

| cas | E maximale | E de référence | part |
|---|---|---|---|
| goutte de pluie | 1,43 × 10⁻¹² J | 3,28 × 10⁻⁵ J | 0,0000 % |
| balle d'arme | 8,94 × 10⁻¹⁰ J | 1,03 × 10¹ J | 0,0000 % |
| pierre lancée | 8,94 × 10⁻⁶ J | 1,44 × 10¹ J | 0,0001 % |
| pas de personnage | 7,24 × 10⁻⁴ J | 6,92 J | 0,0105 % |
| plongeon humain | 1,16 × 10⁻² J | 8,86 × 10² J | 0,0013 % |
| caisse jetée | 8,94 × 10⁻² J | 1,60 × 10³ J | 0,0056 % |
| véhicule léger | 7,24 J | 1,73 × 10⁵ J | 0,0042 % |
| petit vaisseau | 8,94 × 10² J | 1,60 × 10⁶ J | 0,0558 % |
| vaisseau haute mer | 2,29 × 10⁵ J | 1,64 × 10⁷ J | 1,3955 % |
| explosion de surface | 1,16 × 10² J | 3,46 × 10⁷ J | 0,0003 % |

**Ce tableau ne dit pas que le candidat est insuffisant.** La fraction de l'énergie d'un impact
qui part réellement en ondes de gravité est petite — l'essentiel va dans la gerbe, la
turbulence, le bruit et la chaleur — et **personne ne l'a établie** : ADR-055 en confie le soin
à un générateur physique qui n'existe pas. Ce que le tableau donne, c'est le **plafond** du
candidat, et donc le seuil que ce générateur devra respecter : entre 10⁻⁶ et 10⁻² de l'énergie
de référence selon les cas. Le seuil de pente valant 0,1 et l'énergie variant comme son carré,
un seuil deux fois plus permissif ne déplacerait ces parts que d'un facteur quatre.

## 5. Ce que la mesure établit

1. **La longueur d'onde n'est reliée à rien** (§1), et c'est le manque premier : tout le reste
   en dépend.
2. **La portée d'un champ d'impact vaut 5,09 λ**, soit une dizaine de fois la taille de
   l'objet. Un plongeon humain n'est calculable que dans un rayon de trois mètres. **Cette
   limite est numérique, pas physique** : elle vient de la table de Bessel, tabulée jusqu'à
   `x = 64`, et ADR-060 la range explicitement parmi les « choix numériques testés, pas des
   paramètres gameplay ».
3. **Le régime d'eau profonde exclut les grands objets en eau peu profonde.** Un vaisseau qui
   se pose dans un port de 20 m n'est modélisable à aucune portée. Cette limite-là, en
   revanche, est physique : le modèle suppose l'eau profonde, et c'est une hypothèse d'ADR-059.
4. **Les très petits objets butent sur la résolution avant la portée**, et leur horizon utile
   est court.
5. **Le plafond d'énergie est de l'ordre de 10⁻⁶ à 10⁻² de l'énergie mise en jeu**, ce qui fixe
   une contrainte chiffrée au générateur physique à venir.


## 6. Décision

Voir [ADR-083](../adr/ADR-083-portee-du-champ-d-impact.md), qui acte le contrat `λ = α·b`, range
la limite de portée parmi les défauts d'outillage, assume le régime d'eau profonde comme limite
du modèle, et fait du plafond d'énergie une contrainte écrite pour le générateur à venir.

## 7. Ce qui n'est pas revendiqué

Les onze cas ne sont pas le catalogue du jeu : ce sont des cas plausibles couvrant six ordres de
grandeur en taille. Un catalogue véritable demanderait des données de contenu qui n'existent
pas, et l'inférence resterait « probable et non vérifiée » (`REPRISE.md` §5).

`α` n'est pas fixé, et les portées mesurées lui sont proportionnelles. Le plafond d'énergie
dépend du seuil de pente, pris à 0,1 comme dans les fixtures ; il varie comme son carré.
L'énergie de référence est un ordre de grandeur, pas l'énergie transférée aux ondes.

Aucune borne n'a été modifiée, aucun code de la bibliothèque n'a changé : la session ajoute une
sonde et deux documents.

## 8. Vérification

`cargo test` : 162 core + 93 harnais = 255 tests réussis, cinq ignorés — inchangé, la
bibliothèque n'ayant pas été touchée. La sonde compile sans avertissement.

## 9. Suite

**S123-1, S124 :** lever la limite de portée, qui est un défaut d'outillage et non une propriété
du modèle. Piste identifiée et **non retenue avant mesure** : le développement asymptotique de
`J0` et `J1` au-delà de `x = 64`, `J0(x) ≈ √(2/πx)·cos(x − π/4)`, dont l'erreur décroît quand
l'argument grandit — exactement le régime où la table s'arrête. À mesurer contre la référence
f64 existante avant toute décision.

**Restent ouverts et non traités ici :** le générateur physique d'ADR-055 (quelle énergie, quel
`α`), les grands objets en eau peu profonde (§5.3), l'admission dynamique dans le contrôleur, le
renouvellement de fenêtre, la profondeur finie de pression (S116-2), le bilan mixte, la
durabilité disque.

83 ADR, 201 angles, 17 invariants, 6 spécifications, 23 cas.
