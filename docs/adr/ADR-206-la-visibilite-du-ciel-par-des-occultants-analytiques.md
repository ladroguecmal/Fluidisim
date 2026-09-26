# ADR-206 — La visibilité du ciel : des occultants analytiques, pour l'image et pour V

- **Statut : actée**, S382, 2026-09-26. **Demande de l'utilisateur** (verdict R30) : *« Je valide, ajoute l'occultation
  du ciel puis continue »* ; la technique est du projet (ADR-028).
- **S'appuie sur** [ADR-205](ADR-205-la-pluie-complete.md) (pièce 10 : l'exposition calculée depuis les objets posés,
  « lancer de rayons vers le ciel, occultants mobiles »), [ADR-204](ADR-204-la-pluie-arete-de-v.md) D3 (l'exposition, 0 à
  1 000, commande l'arête de pluie), la preuve [CIEL-PLUIE-S381](../validation/CIEL-PLUIE-S381.md) §5 (sans occultation,
  le bloc de la piscine se confond avec le sol sous le ciel couvert).

## 1. Le problème

Une surface reçoit le ciel qu'elle voit : `E = ∫ L(ω)·V(ω)·max(n·ω, 0) dω`. Nos nuanceurs de scène posaient `V = 1`.
Sous le ciel couvert, où rien ne porte d'ombre, `V` est **tout** le modelé : le pied d'un mur, un angle, le dessous d'une
bâche. Par ciel clair, `V` vers le soleil fait les ombres portées. La même fonction, pondérée par la direction de la pluie
au lieu de la luminance du ciel, est l'**exposition** que V attend (ADR-204 D3, ADR-205 pièce 10).

## 2. Décisions

**D1 — `V` se calcule sur les objets, pas sur l'image.** Des **occultants analytiques** — des boîtes alignées d'abord,
orientées ensuite (bâches, objets posés) — que la scène déclare. Écartés : l'occultation en espace écran de Godot (SSAO) —
nos matériaux de scène sont non éclairés par Godot (ils portent notre lumière, dans les unités de l'eau, S374), et l'espace
écran oublie ce qui est hors champ ou caché ; le précalcul (cartes de lumière) — les occultants bougent : le joueur pose et
retire une bâche en temps réel (ADR-203). Un territoire entier demandera une représentation de plus grande échelle (champ
de distance, voxels) : la fonction et son contrôle restent, la source des occultants change.

**D2 — La discrétisation** : par azimut (32), l'intervalle d'élévation que masque chaque occultant, réuni dans un masque de
32 bandes **d'égal angle solide** (uniformes en `sin h`) ; la part masquée, pondérée par `max(n·ω, 0)` et la luminance du
ciel (ciel couvert de la CIE ; uniforme pour la part diffuse du ciel clair), est rapportée au **total exact** — le ciel
couvert incliné a une forme close, `E(β)/Lz = [π(1 + cos β)/2 + (4/3)((π − β)·cos β + sin β)]/3`. Un occultant trop bas
pour atteindre la première bande est écarté avant la boucle : **loin des occultants, `V` vaut 1 exactement**, et l'image
d'avant est rendue au bit.

**D3 — Le soleil est une direction du même calcul** : `V` vers le soleil, sur quelques sous-échantillons du pixel, donne
les ombres portées par ciel clair.

**D4 — Une seule liste d'occultants, deux consommateurs.** L'image (nuanceurs) et V (l'exposition d'une ouverture, dans le
cœur) lisent la même déclaration ; chacun a sa pondération — la luminance pour la lumière, la direction de la pluie (vent
compris, quand la météo le donnera) pour l'exposition. La pièce 10 d'ADR-205 réalisera le côté V.

## 3. Conséquences et limites connues

- Les images **par ciel clair changent** là où le ciel est masqué (ombres, pieds de murs) : c'est l'objet de la décision.
  Sans occultant, elles restent identiques au bit — contrôle à chaque modification.
- Réfraction ignorée pour les surfaces immergées (le fond d'un bassin voit le ciel par la fenêtre de Snell, pas en ligne
  droite) ; la lumière renvoyée par le sol n'est pas occultée ; pas d'interréflexions.
- Coût proportionnel au nombre d'occultants proches de chaque pixel : mesuré à chaque étape.
