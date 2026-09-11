# Frontière autonome et fond décimé — S175

## Protocole déclaré

Suite S174-1/A50. Mode --assembly de fond_mobile, durée12 s au lieu de6 : la crête
part de55m et avance à3√(g(1+a))-2√g, donc dépasse90m pour a0,05 et0,06. L’inversion
analytique vérifie encore l’absence de croisement de caractéristiques. Même fenêtre
[30,90], même total initial, même fond Q amplitude0,05 et sources S173–S174.

Comparer simultanément deux fermetures aux deux étages :

- Analytic : fantômes égaux aux moyennes analytiques du total ; témoin S174.
- Anchored : écart d’invariant sortant calculé depuis T intérieur et Q intérieur,
  réancré sur Q fantôme, selon support open_boundary/S169. Aucun résidu entrant
  inconnu n’est fourni. Les moyennes Q viennent du même interpolant spatial/temporel.

L’écart de champ entre ces deux calculs est mesuré point par point àchaque pas,
puis maximisé ; ce n’est pas une différence de maxima d’erreur.
Le témoin Discrete doit retrouver un solveur total avec la même fermeture. Un témoin
total autonome est donc avancé pour chaque fond Q, avec exactement le même calcul de
frontière àchaque étage ; comparer seulement au total àfrontière analytique serait faux.

Les deux budgets de S174 restent calculés avec le flux numérique effectivement utilisé
par chaque fermeture, et les flux Q interpolés ou analytiques. Les prédictions signées
ne changent pas : changer le bord total ne change pas la différence des flux de Q.
Cela ne prouve pas que le champ ni les échanges totaux des deux fermetures soient égaux.

H0 (spatial exact)/H8m phase0,5 ; cadence continue/1/2s, amplitudes0,05/0,06 ; N120/240
et demi-pas àN240.288 évolutions résiduelles (4 sources ×2 frontières) et54 témoins
totaux (un analytique et deux autonomes par configuration). Mesurer préservation,
transport au-delà de la sortie, écart apparié des bords, identité discrète et deux budgets.
Instantané suivant de Q connu dans le véhicule analytique seulement ; aucun événement
inconnu anticipé. Aucun coût3D, réception perceptive ou seuil is_smooth_at annoncé.

## Réception et portée

Depuis code/ :

```text
cargo test -p water-core --example fond_mobile
cargo run -p water-core --release --example fond_mobile -- --assembly
```

Huit tests reçus, dont deux nouveaux : préservation du fond mobile exact avec bord
ancré ; crête sortante, identité discrète àfrontière identique et budgets propres sur
fond spatial/temporel grossier. Les six tests S173–S174 sont rejoués. Après suppression
d’un constructeur intermédiaire devenu inutilisé, compilation exemple et tests revérifiée.
Supports physiques inchangés ; bibliothèque non rejouée (S163 :299 tests/cinq ignorés).

L’invariant entrant résiduel est fixé àzéro par la fermeture. Cela convient au périmètre
fond connu et perturbation sortante ; aucune information d’un événement extérieur
inconnu n’est récupérée. Comparer une onde entrante inconnue àcette frontière demanderait
une autre interface et ne serait pas un échec de conservation de celle-ci.

## Résultats reçus

Integrated, N240, pas nominal. a : amplitude du total, H0 : fond spatial exact.
E : erreur de hauteur /0,05 m ; Eq : erreur de débit /0,05√g ; Db : écart de hauteur
au calcul apparié avec frontière analytique /0,05 m. V : budget interpolé, Va : budget
de référence analytique, normalisés par le volume initial. Db=0 pour son propre témoin.

| a | tau (s) | H (m) | frontière | E | Eq | Db | V | Va |
|---:|---:|---:|---|---:|---:|---:|---:|---:|
| 0.05 | 0 | 0 | Analytic | 3.188561e-12 | 3.668910e-12 | 0.000000e0 | 9.732445e-16 | 9.732445e-16 |
| 0.05 | 0 | 0 | Anchored | 3.188561e-12 | 3.668733e-12 | 4.440892e-15 | 9.726731e-16 | 9.726731e-16 |
| 0.05 | 0 | 8 | Analytic | 7.794680e-2 | 8.360757e-2 | 0.000000e0 | 9.893571e-16 | 9.893570e-16 |
| 0.05 | 0 | 8 | Anchored | 7.806218e-2 | 8.358932e-2 | 1.459787e-4 | 9.155675e-16 | 9.155675e-16 |
| 0.05 | 1 | 0 | Analytic | 1.829662e-2 | 1.957317e-2 | 0.000000e0 | 1.070283e-15 | 1.043234e-4 |
| 0.05 | 1 | 0 | Anchored | 1.876888e-2 | 1.914801e-2 | 5.599371e-4 | 7.936829e-16 | 1.043234e-4 |
| 0.05 | 1 | 8 | Analytic | 8.478266e-2 | 9.090021e-2 | 0.000000e0 | 8.070922e-16 | 1.043234e-4 |
| 0.05 | 1 | 8 | Anchored | 8.505015e-2 | 9.075307e-2 | 6.287948e-4 | 9.853235e-16 | 1.043234e-4 |
| 0.05 | 2 | 0 | Analytic | 5.845345e-2 | 6.234918e-2 | 0.000000e0 | 9.332689e-16 | 2.588824e-4 |
| 0.05 | 2 | 0 | Anchored | 6.046921e-2 | 6.083902e-2 | 2.297254e-3 | 9.783879e-16 | 2.588824e-4 |
| 0.05 | 2 | 8 | Analytic | 9.748254e-2 | 1.044043e-1 | 0.000000e0 | 8.924611e-16 | 2.588824e-4 |
| 0.05 | 2 | 8 | Anchored | 9.794658e-2 | 1.041611e-1 | 2.262339e-3 | 9.953397e-16 | 2.588824e-4 |
| 0.06 | 0 | 0 | Analytic | 5.096302e-2 | 5.514543e-2 | 0.000000e0 | 8.258407e-16 | 8.258407e-16 |
| 0.06 | 0 | 0 | Anchored | 5.100709e-2 | 5.519590e-2 | 6.978624e-5 | 9.043195e-16 | 9.043195e-16 |
| 0.06 | 0 | 8 | Analytic | 1.241046e-1 | 1.345420e-1 | 0.000000e0 | 7.183973e-16 | 7.183973e-16 |
| 0.06 | 0 | 8 | Anchored | 1.243269e-1 | 1.345805e-1 | 2.412344e-4 | 9.696680e-16 | 9.696680e-16 |
| 0.06 | 1 | 0 | Analytic | 6.853175e-2 | 7.420350e-2 | 0.000000e0 | 9.341446e-16 | 1.040803e-4 |
| 0.06 | 1 | 0 | Anchored | 6.910607e-2 | 7.371692e-2 | 5.891860e-4 | 8.206872e-16 | 1.040803e-4 |
| 0.06 | 1 | 8 | Analytic | 1.306170e-1 | 1.415557e-1 | 0.000000e0 | 8.538863e-16 | 1.040803e-4 |
| 0.06 | 1 | 8 | Anchored | 1.312199e-1 | 1.412992e-1 | 6.835952e-4 | 7.801532e-16 | 1.040803e-4 |
| 0.06 | 2 | 0 | Analytic | 1.069971e-1 | 1.157191e-1 | 0.000000e0 | 8.160081e-16 | 2.582792e-4 |
| 0.06 | 2 | 0 | Anchored | 1.086604e-1 | 1.144485e-1 | 2.323513e-3 | 7.553434e-16 | 2.582792e-4 |
| 0.06 | 2 | 8 | Analytic | 1.419886e-1 | 1.537059e-1 | 0.000000e0 | 1.004312e-15 | 2.582792e-4 |
| 0.06 | 2 | 8 | Anchored | 1.431271e-1 | 1.530795e-1 | 2.287146e-3 | 8.659875e-16 | 2.582792e-4 |

Transport perturbé (a0,06), H8m/tau1s, frontière autonome, Integrated.

| N | facteur dt | E | Db | V | Va |
|---:|---:|---:|---:|---:|---:|
| 120 | 1 | 1.880479e-1 | 7.325299e-4 | 7.622798e-16 | 1.040803e-4 |
| 240 | 1 | 1.312199e-1 | 6.835952e-4 | 7.801532e-16 | 1.040803e-4 |
| 240 | 2 | 1.312307e-1 | 6.843748e-4 | 9.509072e-16 | 1.040803e-4 |

## Lecture et suivi

288 évolutions et54 témoins reçus. Les72 cas Integrated ferment leur budget interpolé
à<=1,21e-15 relatif ; les prédictions signées des deux budgets sont reçues à<=1,35e-15
sur les288 cas.72 cas Discrete retrouvent leur témoin total à<=2,23e-16. Courant<=0,219722.
Le fond exact mobile seul reste préservé àN240, E3,19e-12 avec la frontière autonome.

ÀN240/a0,06/H8/tau1s, Integrated/Anchored donne E0,131220, mais l’écart apparié des
bords vaut seulement0,0006836. En unités de perturbation initiale0,01 m, l’erreur
reste65,61 % ; le bord ajoute un écart mesuré0,342 % de cette amplitude. Cela situe
le défaut dominant dans ce montage, sans transformer le bord en garantie universelle.
Àtau2s, Db atteint0,002287 pour le même fond ; une cadence grossière affecte aussi
la frontière qui lit ce fond, même si l’effet reste ici plus faible que l’erreur totale.

Va reste1,040803e-4 àtau1s/a0,06 pour les deux frontières : le biais des flux de Q
est indépendant de la fermeture totale. Les flux totaux et les champs peuvent néanmoins
différer ; seule la prédiction de la différence des budgets est commune.

L’essai dure12s et la crête a effectivement quitté [30,90] ; la réception ne se limite
pas àun fond avant son arrivée au bord. Il ne prouve ni absence de réflexion dans
tous les régimes ni comportement en eau sèche/supercritique ; la fermeture contrôle
explicitement son régime subcritique. Aucun résidu entrant inconnu fourni.

**S174-1 réalisée sur véhicule subcritique àfond connu ; A50 partielle.** La source
et le bord autonomes sont assemblés avec décimation spatiale et temporelle. Aucun
nouvel angle, ADR, seuil physique ou schéma runtime adopté. A225 reste qualifiée,
A216/A217 inchangées.

**Suite S176 : S175-1 — bilan de réception B4.** Consolider ce que S163–S175 ont
réellement reçu (source, représentation, temps, bord) face aux exigences de B4 et
SPEC-004 ; distinguer véhicule1D, runtime, forces/perception et coûts. Identifier le
prochain lot de construction exécutable àpartir de cet écart, sans ajouter une nouvelle
variante locale par défaut. BILAN-S145 relu en S175 : B1 et S63-1 ont leurs réceptions
antérieures, poursuivre la construction après ces contrôles ciblés, pas seulement
le chaînage des sondes. Porteur : prochaine session ; aucun arbitrage humain requis.
