# ADR-225 — La tolérance de divergence au point mort d'une oscillation

- **Statut : actée**, S492, 2026-10-06 ; décision technique prise ici ([ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D2).
- **Précise** [ADR-144](ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) (la tolérance physique de la projection, `max|div u|·dx / max|u| ≤ 10⁻⁵`)
  pour la référence δ 3D linéaire. Lève [A327](../registres/ANGLES-MORTS.md), mal attribuée en S490. La preuve :
  [A327-S492](../validation/A327-S492.md).

## 1. Ce qui a été trouvé

En S490, une seiche dans une cuve coupée par une cloison alignée sur la grille faisait refuser la projection à sa demi-période ; la
cloison paraissait en cause. **Ce n'était pas elle.** Au pas refusé, la projection avait convergé (résidu vrai sous le seuil, plancher
d'arrondi atteint, erreur inverse 5·10⁻⁷) ; elle était refusée sur la **mesure** de la divergence, relative à la plus grande vitesse — et
au point mort d'une oscillation, **toutes les vitesses passent ensemble près de zéro** (1,1·10⁻⁴ m/s pour une amplitude de 5 cm/s) : un
bruit d'arrondi de 10⁻⁸ suffit à dépasser 10⁻⁵. La cloison décalée d'un micromètre ne changeait que le pas qui tombait le plus près du
point mort.

## 2. Décision

**D1 — Une projection de la référence δ 3D linéaire arrêtée au plancher d'arrondi, dont la divergence tient la tolérance avec un
plancher de vitesse de 1 mm/s (`DIVERGENCE_VELOCITY_FLOOR`), est acceptée.** Seule la décision finale change : la mesure, la boucle de
résolution et toute projection déjà acceptée restent celles d'avant, au bit (les 651 essais du cœur passent, dont les trajectoires 2D
et 3D comparées au bit). Une vitesse de 1 mm/s est sans effet visible.

**D2 — La 2D, le couplage, le pas mobile et la colonne graduée ne changent pas** : un premier essai du plancher dans la mesure elle-même
y changeait les arrêts de cas à vitesses minuscules (six essais au bit). Ils restent exposés au même point mort ; à porter quand une scène
le rencontrera, avec le même remède à la seule décision finale.

## 3. Ce qui changerait la décision

Une scène où une projection acceptée au point mort laisserait une divergence visible (une perte de volume mesurable) : le plancher
descendrait, ou la mesure deviendrait absolue (`max|div u|·dx` contre une vitesse d'échelle de la scène).
