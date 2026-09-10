# S137 — Le banc qui devait calibrer la source ne la mesure pas

2026-09-10. [ADR-093](../adr/ADR-093-ou-se-calibre-la-source-d-impact.md) actée. S136-1.

## 1. La session commence par une prémisse fausse

La suite S136-1, que j'avais rédigée, disait : « le banc B2 est mentionné depuis ADR-060 sans
avoir jamais été spécifié ». **C'est faux.** B2 est spécifié dans PLAN-BENCHMARK §B2, et détaillé
dans un dossier entier — `DOSSIER-B2`, écrit en S16 : scénarios en fichiers concrets, métrique
d'iso-qualité, procédure de décision, préalables et pièges.

Écrire un « banc B2 » à partir de rien aurait produit un second protocole à côté du premier. Deux
documents voisins divergent, et ce dépôt a forké trois fois pour cette raison (**L137**). La
première chose à faire était donc de lire, pas d'écrire.

## 2. Ce que la lecture montre : le renvoi est faux depuis l'origine

**B2 n'est pas le banc de la source.** Il choisit la **technologie de W** — paquets lagrangiens,
équation d'onde 2D ou Boussinesq — et fixe `λ_cut`. Ses métriques : erreur d'angle de sillage,
conservation d'énergie sur 60 s, coût pour 4096 paquets, déterminisme croisé. Aucune ne mesure
ce qu'un objet qui entre dans l'eau **émet**.

**B10 est plus proche, mais mesure autre chose.** « Cavité d'entrée dans l'eau » : sphères et
corps allongés, Froude d'entrée de 1 à 15. Ses métriques portent sur la cavité — profondeur de
pincement, hauteur du jet de Worthington, durée — c'est-à-dire le phénomène proche et visuel, pas
l'onde de gravité qui en part.

**Aucun banc ne mesure la source d'onde d'un impact.** Le renvoi « à calibrer B2 » a masqué ce
trou depuis S77, et trois décisions successives — ADR-060, ADR-083, ADR-092 — l'ont recopié sans
le vérifier. La mienne y compris, la semaine dernière.

## 3. Décision

[ADR-093](../adr/ADR-093-ou-se-calibre-la-source-d-impact.md) : **la calibration de la source
relève de B10**, dont le protocole fournit déjà les entrées — des corps entrant à Froude connu,
c'est exactement le montage dont `α` et `η` ont besoin. Créer un douzième banc dupliquerait un
protocole existant.

**Deux métriques y sont ajoutées**, observables sans instrumenter l'entrée :

- **Longueur d'onde dominante** : chronométrer l'arrivée du maximum d'amplitude à distance `r`
  donne `λ = 8πr²/(g·t²)`, dérivé de `c_g = ½√(gλ/2π)` (SPEC-001 §1). D'où `α = λ/b`.
- **Énergie rayonnée** : énergie du train d'ondes intégrée sur un anneau à distance `r`,
  rapportée à `½ρb³v²` — c'est `η` directement.

**Et le banc reçoit un critère de réussite, ce qu'il n'avait pas.** Ce que la calibration doit
resserrer est borné *avant* la mesure : `α ∈ [3,35 ; 6,11]` par dérivation (ADR-092), et
`η ≤ 2Kgα⁴bs²/v²` par la borne d'énergie. **Une mesure hors de ces bornes ne calibrerait pas le
modèle : elle le réfuterait** — résultat en soi, à rapporter comme tel plutôt qu'à absorber dans
les coefficients.

## 4. Ce qui a été écrit

- `PLAN-BENCHMARK` §B10 : les deux métriques, la décision étendue, le critère de réussite.
- **Trois notes correctives datées**, dans ADR-060, ADR-083 et ADR-092, à l'endroit où chacune
  porte le renvoi erroné. Un ADR ne se réécrit pas ; une erreur factuelle reçoit une note visible.
- ADR-093, qui porte la décision et dit ce qu'elle ne fait pas.

Aucun code n'a changé : 269 tests réussis, cinq ignorés, inchangés.

## 5. Ce qui n'est pas revendiqué

**Rien n'a été mesuré.** Un banc ne se reçoit qu'en l'exécutant, et le bilan S69 le rappelle :
onze bancs sur onze attendent une couche non écrite. Ce qui est vérifiable ici est la cohérence
documentaire, pas une propriété physique.

La formule `λ = 8πr²/(g·t²)` n'est pas mesurable sans précaution : trop près le paquet n'est pas
dispersé, trop loin l'amplitude passe sous le bruit. Fixer ces deux distances fait partie du
protocole et n'est pas fait ici.

B2 et son dossier ne sont pas touchés : leur objet reste le choix technologique de W.

## 6. Suite

**S137-1, S138 :** trois ADR portaient le même renvoi erroné, recopié de l'une à l'autre sans
vérification. Ce n'est probablement pas le seul : le corpus compte 93 décisions et 204 angles
morts, et les renvois entre eux n'ont jamais été vérifiés systématiquement. **Un audit des
renvois** — chaque « voir X », chaque « à calibrer Y », chaque « traité en Z » confronté à ce que
X, Y et Z disent réellement — est le prolongement direct, et le dépôt a déjà payé ce genre
d'audit deux fois : S11 sur les points ouverts, S15 sur les actions annoncées en prose.

Restent ouverts : l'extension de fenêtre, la profondeur finie de pression (S116-2), le bilan
mixte, la durabilité disque, et les deux calibrations que B10 devra produire.

93 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
