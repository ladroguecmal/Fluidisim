# ADR-155 — La queue du spectre de B se rend en pentes par pixel

Actée S256, 2026-09-16, autonomie S71. Premier remède issu d'une revue visuelle
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §7). Complète ADR-100/101 (bande explicite) et
ADR-148 (filtre selon le pas projeté). Ne modifie ni la recette de B ni ses requêtes.

## Constat

Verdict R1 de l'utilisateur : « la mer est trop lisse, on dirait un lac ». La mesure le confirme.
B s'arrête à `4 fp` (`λ` ≥ 3,5 m), et sa pente quadratique moyenne vaut **0,0075**, contre
**0,044** pour une mer réelle de même `Hs` (Cox & Munk, SPEC-001 §1 sexies). La rugosité qui
module la lumière est absente : c'est un manque de physique, pas un réglage d'habillage.

Ajouter ces ondes à la géométrie ne sert à rien : la grille projetée (deux pixels) ne les porte pas
au-delà de quelques mètres, et ADR-148 les filtrerait. Ce qui se voit d'une onde de 10 cm, c'est la
**normale** qu'elle donne au pixel.

## Décision

1. **Même spectre, prolongé.** La queue couvre `[b_B, b_Q]·fp`, où `b_B` est la borne haute de la
   recette. Sa densité est **absolue et identique** à celle de la bande : `S(f) = Hs² q(x)/(16 fp
   ∫_a^{b_B} q)`. Le `Hs` de la bande n'est donc pas renormalisé, et la queue ajoute sa variance
   propre, publiée. Directions, graine et dispersion en eau profonde suivent la recette ; les
   phases sont tirées sur des indices disjoints de ceux de la bande. La cuisson se fait dans le
   cœur (`background_spectrum::bake_tail`), et nulle part ailleurs (SPEC-005 §1).
2. **Premier montage : `b_Q = 32`**, soit `λ` ≥ 5,5 cm en `Tp` 6 s, dans le domaine des ondes de
   gravité. La `mss` du spectre passe de 0,0075 à 0,0195.
3. **Pentes seulement, au pixel.** Le fragment ajoute `Σ w·a·cos(φ)·k` aux pentes interpolées. La
   hauteur, les sommets, les requêtes de jeu, l'admission de pente et les réceptions B/W
   existantes ne changent pas. Détail graphique au sens de la source §15.2 : déterministe, sans
   autorité.
4. **Filtre d'empreinte.** `w = spectral_weight(k, h_px)`, la fonction d'ADR-148, avec `h_px`
   l'empreinte du pixel sur l'eau (dérivées écran de la position). Les composantes sont rangées par
   `k` croissant, et la boucle s'arrête à la première de poids nul.
5. **Réception** : [QUEUE-SPECTRALE-S256](../validation/QUEUE-SPECTRALE-S256.md), écrit avant le code.

## Ce qui reste ouvert, et le dit

- **La queue JONSWAP en `f⁻⁵` n'atteint pas la rugosité observée** : 0,0229 au mieux, à la limite
  gravité-capillarité, soit 52 % de Cox–Munk. Le spectre des ondes courtes (équilibre en `f⁻⁴`,
  capillaires, étalement directionnel plus large) est un modèle manquant, à nommer, choisir et
  recevoir. Ce n'est pas un coefficient à forcer.
- L'écume et les moutons attendus par `Hs` 1,5 m (source §13, ADR-014) sont hors de cette décision.
- La queue n'entre pas dans δ ni dans les forces, et les impacts et sillages restent sans queue.
