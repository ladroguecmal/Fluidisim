# ADR-083 — La portée d'un champ d'impact, et ce qui la borne

- **Statut : actée**, S123, 2026-09-09, autonomie technique S71.
- **Prolonge :** événement d'impact ADR-055, impact régional ADR-059, candidat radial ADR-060.
- **Résout :** A199 — et déplace ce qu'elle demandait.

## Problème

A199 demandait de confronter le couloir d'acceptation du candidat radial aux impacts que le jeu
produira. La confrontation bute d'abord sur un manque plus élémentaire : **la longueur d'onde
qui pilote tout le candidat n'est reliée à rien**. ADR-055 la valide comme « positive en
mètres » et confie le reste à un générateur physique qui n'existe pas ; ADR-060 précise que sa
bande spectrale est « à calibrer B2 » et que son λ de 4 m est un « paramètre d'essai
uniquement ». Aucun document ne dit comment un objet qui tombe produit une longueur d'onde.

Une fois ce lien posé — même approximativement — la mesure (ENVELOPPE-IMPACTS-S123 §4) donne un
résultat net : **sur onze cas de jeu, un seul se construit à la portée voulue**, et le verdict
ne dépend pas de la calibration.

## Décision

**1. Le lien taille–longueur d'onde est un contrat d'auteur, écrit et à calibrer.**

```
λ = α · b        b : demi-largeur de l'objet à la fin de l'impact (SPEC-001 §5 bis)
                 α : sans dimension, à calibrer — banc B2
```

`b` est la seule grandeur que le corpus permette d'invoquer : la théorie de Wagner donne
l'étendue mouillée à la fin de l'impact, et elle vaut `b` indépendamment de la vitesse et du
relèvement. `α` n'est pas devinable et **cette décision ne le fixe pas** ; elle acte que le
producteur d'événements devra le fournir, et que sa valeur relève d'une calibration, pas d'un
choix d'auteur au cas par cas. `λ = 2πv²/g` est écartée : elle décrit un sillage établi, pas
une entrée.

**2. La portée d'un champ d'impact vaut `5,09 λ`, et c'est une limite de tabulation.**

Mesurée exactement à `10,18 · b` pour α = 2, sur tous les cas au-dessus d'une quinzaine de
centimètres. Elle vient de `hi · radius ≤ 64` — la table de Bessel s'arrête à `x = 64` — et
ADR-060 range cette limite parmi les « choix numériques testés, pas des paramètres gameplay ».

**Elle est donc actée comme un défaut d'outillage, pas comme une propriété du modèle.** Un
plongeon humain calculable dans un rayon de trois mètres n'est pas une décision de conception
que quiconque ait prise ; c'est la conséquence non examinée d'une table. La lever est une
action ouverte — S123-1 — et la piste identifiée est le développement asymptotique de `J0` et
`J1` au-delà de la table, qui converge d'autant mieux que l'argument est grand. **Cette piste
n'est pas retenue par cette décision** : elle sera mesurée avant d'être choisie.

**3. Le régime d'eau profonde est une limite du modèle, et elle est assumée.**

Un objet de 20 m de demi-largeur exige plus de 40 m d'eau à α = 2 ; un vaisseau qui se pose
dans un port n'est modélisable à aucune portée. Contrairement à la portée, ce n'est pas un
défaut d'outillage : ADR-059 pose l'eau profonde comme hypothèse, et la lever demanderait un
autre noyau — dispersion en profondeur finie, `ω² = gk·tanh(kh)`. Cette décision **ne l'ouvre
pas** et enregistre le trou tel quel : les grands objets en eau peu profonde ne sont pas
couverts, et devront l'être par un autre modèle.

**4. Le plafond d'énergie devient une contrainte écrite pour le générateur à venir.**

Le candidat porte, avant que la pente dépasse la limite du milieu, entre 10⁻⁶ et 10⁻² de
l'énergie de référence d'un impact — la masse ajoutée `~ρb³` emportée à la vitesse d'entrée.
Ce n'est **pas** un constat d'insuffisance : la fraction réellement transférée aux ondes de
gravité est petite et personne ne l'a établie. C'est le seuil que le générateur physique
d'ADR-055 devra respecter, et il est désormais chiffré au lieu d'être ignoré.

## Ce que cette décision ne fait pas

Elle ne fixe pas `α`, ne lève aucune borne, ne modifie aucun code. Elle ne choisit pas la
méthode d'extension de portée, et n'ouvre pas le noyau en profondeur finie. Elle ne dit pas
quelle fraction d'énergie part en ondes.

Elle ne prétend pas non plus que les onze cas mesurés soient le catalogue du jeu : ce sont des
cas plausibles, choisis pour couvrir six ordres de grandeur en taille. Un catalogue véritable
demanderait des données de contenu qui n'existent pas — et l'inférence resterait « probable et
non vérifiée », au sens de `REPRISE.md` §5.

## Réception

[ENVELOPPE-IMPACTS-S123](../validation/ENVELOPPE-IMPACTS-S123.md). La sonde balaie `α` sur un
facteur 2π précisément pour que les constats ne dépendent pas de sa valeur, et sépare la
géométrie de l'amplitude — la première version les confondait, et lisait un refus de pente
comme une impossibilité géométrique.
