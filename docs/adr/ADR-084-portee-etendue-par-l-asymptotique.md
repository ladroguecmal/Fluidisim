# ADR-084 — Portée étendue par l'asymptotique, bornée par la phase

- **Statut : actée**, S124, 2026-09-09, autonomie technique S71.
- **Prolonge :** candidat radial ADR-060, Bessel interpolé ADR-064, portée ADR-083.
- **Résout :** A201.

## Problème

La portée d'un champ d'impact vaut `5,09 λ` parce que la table de Bessel s'arrête à `x = 64`.
ADR-083 a acté que cette limite était un défaut d'outillage : personne n'a décidé qu'un plongeon
humain serait invisible au-delà de trois mètres. La piste identifiée — le développement
asymptotique au-delà de la table — était **nommée mais non retenue**, précisément parce qu'elle
n'avait pas été mesurée.

## Ce que la mesure a montré

`code/water-core/examples/bessel_reach.rs`, contre une référence angulaire dense dont la densité
suit `x`. Trois questions, trois réponses (CAUSES détaillées dans PORTEE-ETENDUE-S124) :

- **L'asymptotique tient largement.** Ordre 2 (A&S 9.2.1) : erreur **3,6 × 10⁻⁸ à x = 64**,
  décroissante ensuite. L'ordre 1 seul donnerait déjà 1,6 × 10⁻⁶, sous la tolérance de
  4 × 10⁻⁶ retenue depuis ADR-064.
- **Le raccord ne fait pas d'anneau.** Écart entre la table en `x = 64` et la formule :
  **1,0 × 10⁻⁷**, soit un millionième de l'amplitude locale.
- **C'est la phase qui borne, pas la formule.** `PhaseQ32::from_distance` forme `k_turns · r` en
  `f32` ; l'erreur relative de ce produit se traduit en erreur de phase proportionnelle à `x`, et
  l'erreur sur `J0` croît comme `√x`. Pire cas mesuré sur balayage fin : **3,8 × 10⁻⁶ à
  x ≤ 2048**, **5,8 × 10⁻⁶ à x ≤ 4096**.

## Décision

**1. Le domaine de `bessel` passe de `[0, 64]` à `[0, 2048]`.** Sous 64, rien ne change : la
table et son interpolation Hermite restent le chemin, au bit près. Au-delà, l'asymptotique
d'ordre 2 :

```
J0(x) ≈ √(2/πx) · [ (1 − 9/(128x²))·cos θ + sin θ/(8x) ]
J1(x) ≈ √(2/πx) · [ (1 + 15/(128x²))·sin θ + 3·cos θ/(8x) ]      θ = x − π/4
```

**2. La borne vaut 2048, et cette valeur est mesurée, pas choisie.** Elle est fixée par la
précision de la phase spatiale en `f32` — la même limite qui borne déjà le fond B (ADR-052) —
et non par l'ordre du développement. Y ajouter des termes ne l'améliorerait pas ; c'est le
produit `k_turns · r` qu'il faudrait porter en précision étendue, ce que cette décision ne fait
pas.

**3. `Reach` devient `hi · radius ≤ 2048`.** La portée d'un champ passe de `5,09 λ` à
**`163 λ`**, un facteur 32. Un plongeon humain devient calculable sur une centaine de mètres au
lieu de trois.

**4. Le refus au-delà reste franc.** `bessel` continue de refuser hors domaine plutôt que
d'extrapoler : la borne recule, elle ne disparaît pas. Un appelant qui demande davantage reçoit
`Reach`, qui nomme depuis ADR-082 les deux paramètres en cause.

## Ce que cette décision ne fait pas

Elle ne touche à rien sous `x = 64` : mêmes valeurs, mêmes hachages de campagne. C'est la
vérification qui accompagne la construction.

Elle n'améliore pas la précision, elle étend le domaine à précision constante — la tolérance de
4 × 10⁻⁶ d'ADR-064 vaut désormais jusqu'à 2048. Elle ne dit rien de la justesse **physique** du
champ à grande distance : un champ calculable plus loin n'est pas un champ validé plus loin, et
la réception physique du candidat reste celle d'ADR-060.

Elle ne lève ni le régime d'eau profonde, ni le contrôle de résolution, qui borneront à leur
tour — c'est mesuré en réception, et pour les petits objets `Resolution` reprend la main.

Elle ne porte pas la phase en précision étendue. Ce serait le prochain gain, il vaudrait un
facteur supplémentaire, et il demanderait de reprendre `from_distance` — donc B aussi.

## Réception

[PORTEE-ETENDUE-S124](../validation/PORTEE-ETENDUE-S124.md). La précision est vérifiée contre la
référence dense au-delà de 64, la continuité au raccord est mesurée, et la sonde
`impact_envelope` est rejouée pour chiffrer ce que les cas de jeu y gagnent.

## Note corrective du 2026-09-09, à la réception (S124, P6)

La décision annonce « un facteur 32 » sur la portée et « un plongeon humain calculable sur une
centaine de mètres au lieu de trois ». **Le premier chiffre décrit la borne `Reach` seule ; le
second est faux.** La mesure donne 3,66 m pour ce plongeon, et de zéro à +82 % selon les cas.

En cause : `Resolution` — `dk · (rayon + c_g · âge) ≤ π/2` — devient partout la borne active dès
que `Reach` recule. La décision l'avait envisagé en une ligne, sans en tirer la conséquence :
**un facteur annoncé sur une borne n'est un facteur sur le résultat que si cette borne est la
seule active.** Vérifier laquelle prend le relais aurait coûté une dichotomie.

Ce que la réception établit en revanche, et qui n'était pas dans la décision : `Resolution`
dépend de `N`, déjà libre entre 64 et 256 depuis ADR-060. À `N = 256`, neuf cas de jeu sur onze
atteignent la portée voulue. **Et les deux leviers étaient nécessaires** : `Reach` ne dépendant
pas de `N`, l'extension sans `N = 256` donnait +20 % sur le plongeon, et `N = 256` sans
l'extension l'aurait laissé à 3,05 m. Le coût de ce profil n'est pas mesuré ; son adoption
n'est pas décidée ici.
