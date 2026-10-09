# Le sélecteur des domaines — conception (S732)

*S732, 2026-10-09.* [ADR-284](../adr/ADR-284-r43-recu-les-domaines-places-selon-la-situation.md) (les domaines placés selon la situation),
[ADR-285](../adr/ADR-285-cinquantieme-revue-de-methode.md) D4 (ses juges). Les nombres : `python outils/selecteur_nombres.py`.

L'utilisateur, le 2026-10-09 : *« ce système qui choisit quel type de simulation doit être le plus peaufiné et solide, car c'est grâce à
lui que l'on aura le meilleur compromis entre réalisme et performance »*. Tout le gain du LOD (ADR-275) passe par lui ; tout défaut visible
d'un raccord aussi (R43).

## 1. Ce qu'il décide

À chaque instant, pour chaque région d'eau :

- **la représentation** : SGN ou Saint-Venant (la 2D), la bande 3D, la boîte 3D autour d'un corps (ADR-275 D1) ;
- **les frontières** de chaque domaine 3D : où elles commencent et où elles finissent ;
- **les instants** de chaque naissance et de chaque mort.

Il ne simule rien lui-même : il lit, prévoit, décide, et journalise.

## 2. Ce qu'il lit

- **Le porteur bon marché**, déjà calculé : l'état de SGN au large, de Saint-Venant au rivage (la hauteur, la vitesse, leurs dérivées).
- **La bathymétrie** (le fond, la pente) et la côte cuite (Cote2D, `Q_b`, S669–S677) pour la houle périodique.
- **Les corps** : la position, la vitesse, la présence (B5, S729).
- **La 3D vivante elle-même**, pour la surveillance (§4) : son mur `J` à chaque raccord (ADR-284 D4), le jet le plus avancé, l'air enfermé.

## 3. Ses règles

**R1 — La prévision par le porteur.** Le porteur 1D (SGN) coûte quelques millisecondes pour plusieurs secondes simulées. Le sélecteur le fait
donc tourner **en avance**, sur un horizon de 2 à 3 s, avec un critère de déclenchement du déferlement. Il en tire :
- **s'il y aura déferlement** : si non, aucune 3D (la remarque de l'utilisateur, S707). Synolakis donne le seuil pour une onde solitaire
  sur une plage plane, `H/d > 0,818·cot^(-10/9)` : 0,052 à 1:12 ;
- **où et quand** : `x_b`, `t_b` ;
- **le type** : glissant, plongeant ou frontal (Grilli, `S₀`, S647) ;
- **où retombe le jet** : `x_b + L_jet`, avec `L_jet = c_b·√(2·H_b/g)`. Sur R43, cela donne **10,344 m**, contre 10,375 m mesurés pour
  l'air enfermé du témoin.

Le critère de déclenchement dans SGN est une pièce à éprouver : la vitesse de montée de la surface rapportée à la célérité (Kennedy et al.,
2000), le rapport `H/h`, le nombre de Froude de la crête. Il se calibre sur les témoins (§6) : un critère, plusieurs scènes.

**R2 — Les frontières (ADR-284 D2).** Aucune frontière entre deux modèles là où agit une physique que l'un des deux n'a pas.
- **L'amont de la bande 3D** : avant le point où SGN cesse d'être juste (la crête trop raide), avec une marge (aujourd'hui 5,0 m pour
  `x_b` ≈ 9,96 m, ADR-278).
- **L'aval** : au-delà de `x_b + 3·L_jet`, plus une marge. Les trois longueurs couvrent le jet, son rebond (S730 : le second jet vers
  11 m) et l'éclaboussure. Si cela dépasse le rivage, la 3D couvre la plage jusqu'à sa mort.
- **La boîte d'un corps** : son empreinte, plus la marge, sur l'horizon de sa vitesse (B5).
- **Les contraintes communes** : pas de frontière dans une région qui change vite (une crête, un ressaut) ; une profondeur suffisante côté
  2D, ou le sable sec.

**R3 — Les instants.** Naître avant `t_b`, assez tôt pour que la 3D ait pris l'onde. La mesure de cette marge est une pièce à faire (S707 :
la naissance au repos ; ici, sous une onde). Mourir quand le ressaut est formé et loin de toute frontière, avec hystérésis (S717 : 3,2 s).

**R4 — Le budget.** Si les besoins dépassent le budget, le sélecteur les classe : un joueur présent, puis un déferlement visible proche,
puis un déferlement lointain. Ce qui ne passe pas reste en 2D, et c'est journalisé.

## 4. Sa solidité

- **La surveillance** : chaque domaine 3D rapporte, à chaque pas, son mur `J` à chaque raccord, et la distance du jet le plus avancé à sa
  frontière d'aval.
- **Le rattrapage** :
  - un jet qui approche d'une frontière : la frontière recule (le geste de B4b, S728), ou la mort est retardée ;
  - un déferlement que le porteur annonce hors de toute 3D (une prévision manquée) : une naissance tardive sur place, le défaut mesuré.
- **Le repli prudent** : quand la prévision est incertaine, entre deux critères ou près d'un seuil, la 3D est plus large et naît plus tôt.
  Le coût monte, le réalisme est gardé.
- **Aucun clignotement** : l'hystérésis (ADR-275 D1) ; une naissance et une mort par événement, au plus ; le nombre de bascules compté par
  domaine.
- **Le journal** : chaque décision est écrite (l'instant, la règle, les nombres lus), et rejouable. Une décision se rejoue sans la 3D, à
  partir du journal et du porteur.

## 5. La batterie de scènes

Chaque scène a un témoin tout-3D. **Ses frontières restantes sont mesurées** (ADR-285 D1) : son mur `J`, une image du jet.

| scène | `d`, `H/d`, pente | `S₀`, type | ce qu'elle éprouve | le témoin |
|---|---|---|---|---|
| **S1** — R43 | 0,5 m, 0,30, 1:12 | 0,231, plongeant ; `x_b` ≈ 9,96 m, le jet à 10,34 m | la frontière d'aval près du rivage | **fait** (S730, E2 : raccord à 12,0 m) |
| **S2** — loin du bord | 0,30 m, 0,30, 1:19,85 (Synolakis) | 0,140, plongeant ; `h_b` ≈ 0,14 m (Green et McCowan, à mesurer) | la bande loin du rivage ; des mesures de laboratoire existent (S712) | à faire (S712 en a un, son raccord à mesurer) |
| **S3** — glissante | 0,30 m, 0,50, 1:90 | 0,024, glissant ; le rivage à 27 m du pied | faut-il la 3D pour un déferlement glissant ? Saint-Venant et la dissipation peuvent suffire, ou non | à faire |
| **S4** — sans déferlement | 0,5 m, 0,03, 1:12 | 0,73, aucun ; la remontée exacte 6,1 cm (Synolakis) | la 3D ne naît pas ; la 2D seule contre le tout-3D **et** contre la loi exacte | à faire |
| **S5** — prévision fausse | S1, le porteur sous-estimant `H` de 30 % | — | le rattrapage : la 3D naît trop tard ou trop étroite, et le sélecteur corrige | S1 |
| **S6** — deux vagues | S1, puis une seconde sur le reflux | — | la continuité des frontières, l'hystérésis, aucun clignotement | à faire |
| **S7** — un corps dans la lame | B5 dans le jet de rive de S1 | — | la boîte d'un corps et la bande ensemble | à faire |

Le piège de S722 : le déclencheur au plus simple se déclenchait dès le départ, parce que l'onde des plages de référence part trop près du
pied. Chaque scène laisse donc au prédicteur au moins 2 s de propagation avant `t_b`.

## 6. Les juges (ADR-285 D4)

Par scène, contre son témoin :

- **le réalisme** :
  - le retournement dans la tolérance d'ADR-278 D2 (0,15 m ; 0,1 s) ;
  - le mur `J` sous 1 cm à chaque raccord, sur toute la durée ;
  - la remontée, à la tolérance mesurée de son témoin (ADR-279 D1) ;
  - l'air enfermé ;
  - une image au jet ;
- **le coût** : le rapport au tout-3D, rapporté ; R43 en est aujourd'hui à 308 s contre 1 152 s ;
- **la solidité** :
  - les bascules comptées ;
  - S5 rattrapée, le défaut et le surcoût mesurés ;
  - chaque décision au journal.

## 7. Les pièces, une par session

| pièce | ce qu'elle fait | son essai |
|---|---|---|
| **P1 — le prédicteur** | SGN en avance, le critère de déclenchement, `x_b`, `t_b`, `S₀`, `L_jet` | sur S1 (le témoin existe) : `x_b` à 0,15 m, `t_b` à 0,1 s, le jet à 0,15 m ; puis sur chaque témoin à mesure qu'il arrive |
| **P2 — les témoins** | S2, S3, S4 en tout-3D, frontières mesurées ; des calculs longs, par `calcul.py` | chaque témoin contrôlé (ADR-285 D1) ; S4 contre la loi de Synolakis |
| **P3 — les règles** | R2 et R3 : la bande placée et datée par la prévision, dans le montage de bout en bout | S1 à S4 : les trois juges |
| **P4 — la surveillance et le rattrapage** | §4 : le mur, le jet près d'une frontière, la naissance tardive, le repli | S5 et S6 |
| **P5 — le journal** | les décisions écrites et rejouées | rejouer S1 à S6 depuis le journal : les mêmes décisions, au bit |
| **P6 — le banc du sélecteur** | toute la batterie en une commande, ses trois juges en un tableau | la batterie entière tient ; le banc entre au rituel (`non_regression.py`), en version courte |

Plus tard, dans le même registre :
- les frontières qui bougent pendant le calcul (ADR-284 D3) ;
- la 2D horizontale : une côte quelconque, des vagues obliques, par rayons ;
- la houle périodique, par Cote2D et `Q_b`.

## 8. Ce qui n'est pas décidé ici

- **Le critère de déclenchement de SGN** : il sera choisi par la mesure, en P1, entre les critères publiés.
- **Le déferlement glissant (S3)** : la 3D ou la 2D, à trancher par le juge du réalisme.
- **Le budget de R4** : à fixer avec la scène du jeu (ADR-262 : le budget est un plafond).

*Note datée du 2026-10-09 (S733), après la question de l'utilisateur sur le coût et le découpage* :
- **Le serveur** ne porte aucun domaine : la 3D, SGN et Saint-Venant locaux sont des détails (δ), calculés par chaque client autour de ce
  qu'il voit, sans autorité de jeu (liste 10.2, 10.4). Le sélecteur tourne sur chaque client.
- **Son coût propre** est mesuré au banc (P6) comme un critère : la prévision, les naissances, les déplacements et les morts, rapportés au
  pas de la 3D.
- **Le découpage** : ADR-006 (S01) l'a décidé. Une grille d'adressage fixe et hiérarchique (64, 512, 4 096 m, codes de Morton) sert au
  réseau, à la persistance et au routage. Chaque domaine de calcul est **un ensemble épars de blocs de 8³ mailles**, pas une boîte
  subdivisée : la forme est portée par les blocs, une fusion est une union, une séparation une partition, et le coût se compte en blocs.
  Notre 3D travaille encore en une boîte par domaine. **Pièce P7** : le sélecteur raisonne en blocs ; une frontière qui bouge ajoute ou
  retire des blocs (le geste de B4b).

*Note datée du 2026-10-09 (S733)* : **P1, le prédicteur**, fait en partie ([preuve](../validation/PREDICTEUR-S733.md)) :
- la place de la vague et son jet sont prévus à 0,1 m, pour 68 ms ;
- l'instant du déclenchement ne l'est pas encore : les seuils publiés tombent de 0,5 à 0,85 m à côté. Le calibrage attend les témoins de
  P2.

*Note datée du 2026-10-09 (S734)* : **P2, les témoins**, en partie ([preuve](../validation/TEMOINS-SELECTEUR-S734.md)).
- S2 est sûr : le retournement à 3,29 s.
- S3 déferle à 3,49 s, mais sa durée dépassait son mur : à refaire.
- S4 n'a pas de témoin valable : le front se fige sur la pente de 1:3, que le fond soit en escalier ou lisse. À diagnostiquer en S735,
  avant tout calibrage.

*Note datée du 2026-10-09 (S735)* : S4 diagnostiqué ([preuve](../validation/DIAGNOSTIC-S4-S735.md)).
- Le front figé était une lecture fausse de φ.
- La 3D remonte réellement trop haut : +37 % au moins, comme sa crête trop haute de S713 et de S1.
- Le juge du retournement compte un vide d'une maille à φ ≈ 0.

Les témoins attendent deux choses : une lecture par les particules, et un juge robuste.

*Note datée du 2026-10-09 (S737)* : les instruments des témoins sont prêts ([preuve](../validation/LECTURE-PARTICULES-S737.md)) :
- la remontée lue par les particules ;
- le juge du retournement robuste au bruit.

**Correction** : S4 (`H/d` = 0,2, 1:3) déferle au reflux (Synolakis : dès 0,141 à 1:3). Il reste un témoin du déferlement pendant la
montée ; il n'est pas « sans déferlement ». La scène sans aucun déferlement demande `H/d < 0,141` à 1:3.

*Note datée du 2026-10-09 (S738)* : la remontée trop haute de S4 vient de la distance parcourue sur le fond plat
([preuve](../validation/DISTANCE-PARCOURUE-S738.md)). L'onde de départ (une vitesse uniforme sur la verticale) se transforme dans la 3D.
Les témoins de la batterie en dépendent ; l'onde de départ exacte vient avant le calibrage.
