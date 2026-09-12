# S185 — L'erreur de cadence en 3D, et ce qui est disponible au runtime

2026-09-12. S184-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[CONSOMMATION-S184](CONSOMMATION-S184.md) a montré que la cadence temporelle est l'axe bon
marché : elle divise le coût **exactement** par `c`, et le contenu temporel de la source est
lent là où son contenu spatial ne l'est pas. Restait la moitié qui décide : **ce que la
cadence coûte en justesse.**

Comme en S183 et S184, les conditions sont publiées **avant** la première exécution (§1–§5),
les relevés viennent après (§6). Les deux sessions précédentes y ont chacune gagné une erreur
rendue visible au lieu d'être réécrite ; la discipline se paie toute seule.

## 1. L'écart précis que cette session ferme

S174 a mesuré la cadence temporelle sur un véhicule 1D, par **interpolation linéaire entre
deux instantanés**. Et il a écrit lui-même ce que cela supposait :

> *« L'échantillon futur utilisé pour interpoler est connu dans ce cas analytique ; le
> protocole ne reçoit pas l'anticipation d'un événement extérieur inconnu. »*
> — [CADENCE-FOND-S174](CADENCE-FOND-S174.md), Conclusions

C'est exactement la question laissée ouverte. Interpoler demande de connaître l'instantané
**suivant** ; un runtime qui reçoit un `WaveEvent` imprévu ne l'a pas. La cadence n'est donc
pas un seul régime mais trois, et un seul d'entre eux a été mesuré :

| mode | ce qu'il exige | disponible au runtime ? |
|---|---|---|
| **maintien** | rien : la dernière source est réemployée telle quelle | oui |
| **extrapolation** | les **deux** dernières reconstructions | oui |
| **interpolation** | la reconstruction **suivante** | seulement en payant une période de latence |

S185 mesure les trois sur le fournisseur réel `B + impacts + pression`, en 3D. Le troisième
n'est pas une proposition : c'est le **plafond** que les deux autres essaient d'atteindre, et
c'est le régime que S174 avait mesuré sans pouvoir dire s'il était accessible.

## 2. Le véhicule, et ce qui change depuis S184

Le bloc de S184, côté 16 — 2744 mailles intérieures, `dx = 0,25 m`, même placement, même
montage d'eau. Le pas est le même : advection centrée, laplacien à sept points, `− S`
soustraite (SPEC-004 §6.1).

**Ce qui change : l'instant avance.** En S184 la source était figée à `T0` — le contrôleur ne
publie qu'un instant, et la mesure ne portait que sur le coût. Ici `t_n = T0 + n·dt`, et
chaque reconstruction exige d'**actualiser le contrôleur de pression** avant de requêter.
Cette actualisation coûte 163–178 µs par instant publié (S183 §6.4) et **ne dépend pas du
nombre de points** : à 2744 mailles elle pèse 0,2 % de la reconstruction, mais elle
dominerait pour un consommateur épars. Elle est comptée.

`dt = 10 ms`, 100 pas, soit **1,0 s** de temps simulé, de `T0 = 1,5 s` à `2,5 s` — dans la
fenêtre de pression `0–8 s` et sous l'horizon d'impact `4 s`.

**Les échelles de temps du contenu**, qui sont ce que la cadence doit résoudre :

| contenu | échelle |
|---|---|
| composantes de `B`, périodes `Tp/2` à `2·Tp` | 3 à 12 s |
| onde d'impact, `λ = 4 m` en eau profonde : `T = √(2πλ/g)` | ≈ 1,6 s |
| mode de pression le plus court, `λ_min = 1,081 m` advecté à 2 m/s | **≈ 0,54 s** |

C'est 0,54 s la contrainte, pas 3 s. Les cadences `c ∈ {1, 2, 4, 8, 16, 32, 64}` couvrent des
maintiens de 10 ms à 640 ms, soit de 1/54 à 1,2 fois cette période : la plage est choisie pour
encadrer la rupture, pas pour la flatter.

## 3. Métriques

État initial `u' = 0`, de sorte que `u'(T)` soit **entièrement** ce que la source a produit et
que l'erreur relative ait un dénominateur qui veuille dire quelque chose. Deux grandeurs :

- **erreur de source** `eS` = `max |S_utilisée − S_réf|` sur toutes les mailles et tous les
  pas, rapportée à `max |S_réf|` ;
- **erreur de champ** `eU` = `max |u'_c(T) − u'_réf(T)|`, rapportée à `max |u'_réf(T)|`.

Les deux sont liées, et le lien est une **vérification fermée** plutôt qu'une remarque : avec
`u'` partant de zéro et l'advection d'ordre supérieur, `u'(T) ≈ −∫S dt`, donc
`Δu'(T) ≈ −∫(S_utilisée − S_réf) dt`. L'écart entre le `Δu'` mesuré et cette intégrale
accumulée est publié ; c'est la forme qu'avait déjà la prédiction de S170 §2.3.

Un **contrôle à état non nul** reprend les mêmes cadences depuis un champ initial déterministe
non trivial, pour que la conclusion ne tienne pas à la trivialité de l'état.

## 4. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** L'erreur mesurée ici est déterministe — contrairement à une
   durée. Deux exécutions du binaire doivent rendre **les mêmes bits**. C'est pourquoi ce
   document ne publie pas deux séries comme S183 et S184 : il publie une série et l'exigence
   qu'elle se reproduise.
2. **La cadence 1 est la référence, littéralement.** Le chemin à `c = 1` doit rendre le champ
   de référence bit pour bit — sans quoi les deux chemins ne diffèrent pas que par le réemploi.
3. **La référence est assez convergée pour servir de référence.** Le même calcul à `dt/2` et
   200 pas doit rendre un champ proche ; l'écart est publié. Sans ce contrôle, « erreur contre
   `c = 1` » ne voudrait rien dire.
4. **Tout reste fini**, source et état, sur toutes les cadences et tous les modes.

## 5. Ce que la mesure ne prouvera pas

- **Aucun seuil.** Rien ici ne dit quelle erreur est acceptable : ce serait une réception
  perceptuelle ou un critère B4, et aucun n'est adopté. On mesure une courbe, pas un choix.
- **Un seul montage, donc une seule échelle de temps.** L'erreur de cadence est gouvernée par
  le contenu ; une autre recette de pression, une autre vitesse de source, une autre mer
  déplacent tout. Le ratio `maintien mesuré / période du contenu` voyagera mieux que les
  valeurs absolues, et c'est sous cette forme que les résultats sont donnés.
- **La référence est `c = 1`, pas une intégrale exacte.** Son propre défaut de quadrature
  n'est pas mesuré — seulement borné par le contrôle 3.
- **Le véhicule ne projette toujours pas** (S184 §3), et ne modélise ni surface libre ni bord.
- **L'interpolation n'est pas proposée.** Elle exige une période de latence que le runtime
  n'a peut-être pas ; elle est mesurée comme plafond.

## 6. Relevés

*À recevoir en P3. Aucun chiffre n'est écrit avant l'exécution.*

## 7. Ce qui est reçu, et ce qui ne l'est pas

*À recevoir en P4.*
