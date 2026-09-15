# ADR-143 — La pression f32 de δ s'arrête à sa précision représentable

- Statut : **actée**, S238, 2026-09-15 ; autonomie technique S71.
- Traite **A272**. Précise le contrat de convergence de [PRESSION-F32-S231](../validation/PRESSION-F32-S231.md)
  sans relâcher son seuil ; applique la tolérance physique déclarée par
  [CANDIDAT-DELTA-S199](../validation/CANDIDAT-DELTA-S199.md) §5, critère 4.
- Mesures et réception : [PRESSION-PLANCHER-S238](../validation/PRESSION-PLANCHER-S238.md).

## Constat

Le gradient conjugué f32 s'arrête quand le vrai résidu recalculé vérifie `‖b − Ap‖/‖b‖ ≤ 10⁻⁶`. Ce
nombre vient du candidat f64 ; S231 l'a tenu en f32 jusqu'à 8 192 mailles. Au-delà, S237 a vu un pas
refuser : résidu figé à 1,05·10⁻⁶ jusqu'à 64 000 itérations.

La mesure de S238, au pas refusé et à ses voisins, a établi :

- une **erreur inverse composante par composante d'une unité d'arrondi** (`ω ≈ 6·10⁻⁸ ≈ u`) : la
  pression est la meilleure solution que f32 représente ;
- un **cycle exact** de l'état de pression (période 420 dans un montage), mais de longueur non bornée :
  7 386 itérations, puis plus de 16 000, avant certification dans le banc de S237 ;
- une divergence projetée de **1,45·10⁻⁷**, soixante-dix fois sous la tolérance physique de S199 ;
- l'identité discrète `div u = (b − Ap)/scale` dans toute maille fluide, vérifiée à 7·10⁻⁸ près ;
- que `ω` **ne peut pas être un critère d'acceptation** : 4,7·10⁻⁴ sur des pas convergés, 2·10⁻⁸ sur un
  système sans solution.

## Décision

1. **Le seuil de résidu relatif 10⁻⁶ reste le critère premier**, testé avant tout certificat à chaque
   vrai résidu recalculé. Un pas qui l'atteint **sans relance** suit le chemin antérieur au bit ;
   l'empreinte S232 est inchangée et la suite du cœur passe sans tolérance modifiée.
2. **Arrêt au plancher** à une relance, si le vrai résidu est indiscernable de l'arrondi de son propre
   calcul, `ω ≤ ROUNDOFF_BACKWARD_ERROR = γ₈ = 8u/(1 − 8u)`, `u = 2⁻²⁴` — constante dérivée du modèle
   standard de la virgule flottante pour une ligne à quatre faces, non ajustée —, ou si l'état de
   pression revient au bit (détection de Brent, empreinte de 64 bits, mémoire constante).
3. **À l'arrêt au plancher, et seulement alors**, le pas est reçu si
   `max|div u|·dx/max|u| ≤ PROJECTION_DIVERGENCE_TOLERANCE = 10⁻⁵` (S199), dégradé sinon. Par l'identité
   ci-dessus, cette tolérance est une norme maximale du résidu à l'échelle physique.
4. Plafond d'itérations atteint sans plancher : dégradé, inchangé (ADR-007 §2).
5. `Report` publie `floor` et `backward_error`.

## Réception

Onde de 5 cm de S237 à 128 colonnes, plafond d'origine : **reçue**, profil 0,25 %, harmonique `2k`
0,71 % ; 38 pas au plancher, `ω` de 2,7 à 7,7 `u`, divergence ≤ 1,5·10⁻⁷, pire pas 526 itérations. Contre
la solution f64 du même système : vitesse corrigée à 5,5·10⁻⁸ près, pression à 3·10⁻⁵. Système sans
solution : dégradé par les deux arrêts.

## Ce que la décision ne fait pas

- **Elle ne relâche aucun seuil.** Le résidu relatif garde 10⁻⁶ ; la tolérance de divergence est celle que
  S199 avait déclarée avant toute construction ; `γ₈` n'accepte rien à lui seul.
- **Elle ne rend pas le plancher plus bas** : un système qui s'arrête au plancher au-dessus de la tolérance
  physique est dégradé, comme avant.
- **Elle ne garantit pas la précision de la pression** au-delà de la divergence : l'écart à la solution f64
  du même système est mesuré sur un cas, pas garanti.
- Elle est écrite pour l'opérateur **deux dimensions** à quatre faces : en 3D, six faces donnent `γ₁₀`, à
  recalculer avec l'opérateur, non à transposer.
- Elle ne vérifie pas la divergence mise à l'échelle des pas **convergés** selon le critère premier, qui
  dépasse 10⁻⁵ à 128×64 sur la bosse et à 256×128 dans la famille S231 (A273, déclencheur dans la file).

- **Elle ne garde pas au bit les pas qui convergeaient après relance.** Un pas dont le vrai résidu, à une
  relance, est déjà sous `γ₈` mais au-dessus de 10⁻⁶ s'arrête là, quand l'ancien chemin aurait continué
  une ou deux itérations jusqu'à passer 10⁻⁶ par un tirage d'arrondi. Mesuré sur la trajectoire de 5 cm
  de S237 : 12 pas à 32 colonnes et 39 à 64 changent de bits ; profil et harmonique identiques aux
  chiffres publiés (PRESSION-PLANCHER-S238 §4). Seule la détection de cycle gardait ces bits, à un coût
  non borné.

## Réversibilité

Retirer les deux certificats d'arrêt rend exactement le chemin S231–S237 ; les pas au plancher
redeviennent des relances jusqu'au plafond.

## Note datée — 2026-09-15, S239

Le §3 exigeait la tolérance physique de S199 **au plancher seulement**, et sur toutes les mailles
mouillées. [ADR-144](ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) l'étend à tout
chemin d'acceptation et la restreint aux **lignes franches** — celles sans face fantôme de surface —,
parce que le résidu d'une ligne à fantôme plafonne à un ulp de sa propre magnitude
([mesure](../validation/TOLERANCE-PRESSION-S239.md) §3). Le reste d'ADR-143 est inchangé : le
certificat d'arrondi `ω ≤ γ₈` et la détection de cycle restent les deux arrêts au plancher.
