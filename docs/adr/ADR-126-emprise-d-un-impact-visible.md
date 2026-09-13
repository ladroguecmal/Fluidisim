# ADR-126 — L'emprise d'un impact visible se dimensionne par ses coutures

- **Statut : actée**, S203, 2026-09-13, autonomie technique déléguée (S71).
- Applique ADR-124 étape 3 et la suite nommée par ADR-125 (« emprise et observateur
  explicites »). Complète ADR-060/066/085/105 ; ne réécrit aucun ADR.
- Réception : [IMPACT-W-S203](../validation/IMPACT-W-S203.md) §4–5.

## Constat

Le domaine d'un `RadialImpact` — rayon R, horizon A, nombre de nœuds N — était dimensionné
par son **admission** : garde de résolution `dk·(R + c_g,max·A) ≤ π/2`, portée de Bessel,
régime profond. Aucune de ces gardes ne dit ce qui se voit quand l'hôte rend B seul hors du
disque ou après l'horizon. Mesuré en S203 sur un impact λ = 3,35 m, E = 164 J :

- la **couture temporelle** vaut 78 mm à A = 2 s et 4,8 mm à 32 s, quel que soit N — un
  impact linéaire s'étale, il ne s'éteint pas ;
- la **couture spatiale** vaut l'amplitude de l'anneau au franchissement de R, ≈ 1/R ;
- au critère de 2 % du pic central, le premier profil qui passe est **N256, R 52 m,
  A 56 s**, dans une fenêtre étroite ; N512 passe pour R ≥ 55 m et A ≥ 56 s ;
- N256 et N512 concordent à 0,1 µm ; à λ×2 les coutures relatives se reproduisent à
  0,019 point près.

## Décision

1. **Une emprise d'impact destinée à l'image se reçoit par ses deux coutures**, spatiale
   (max|η_W| sur r = R pendant [naissance ; +A]) et temporelle (max|η_W| dans R à +A), et
   non par sa seule admission. Un profil qui passe l'admission sans passer ses coutures
   n'est pas un profil d'image.
2. **Profil sans dimension au critère relatif de 2 %** : R ≥ 15,5 λ, A ≥ 96·√(λ/g),
   N ≥ 256 ; **N512 recommandé** hors d'une recherche de coût, N256 ne passant que dans une
   fenêtre étroite. Ce sont des valeurs **mesurées suffisantes**, pas des minima : R = 14,9 λ
   échoue, A = 82·√(λ/g) échoue ; les minima exacts sont entre les deux. Le profil doit
   aussi passer l'admission, qui borne R + c_g,max·A pour N donné. Le critère absolu de l'hôte (3 mm pour la marche S201) s'ajoute et
   devient liant quand η centre > 0,15 m.
3. **Le budget de pente de l'impact est ce que B laisse** : l'hôte déclare
   `Medium::max_slope = π/7 − steepness_B·π`, afin que la construction refuse l'impact au
   lieu que la composition refuse chaque point.
4. **Hors emprise, B seul** ; la réception d'une image se fait contre un **témoin** qui
   exécute la même marche, les mêmes bornes et le même prédicat `admits` avec B partout :
   aucun pixel différent ne doit être hors emprise.

## Ce que cette décision ne fait pas

Elle ne fixe **aucun seuil de visibilité** : 2 % est emprunté à ADR-120 comme choix de banc,
3 mm est une tolérance de marche. Un protocole perceptuel qui fixerait un autre seuil change
R et A par la même mesure, pas par extrapolation. Elle ne modifie ni les gardes de
`RadialImpact` ni le défaut N64 d'ADR-085 : ce profil vaut pour l'image, pas pour une
requête gameplay qui n'a pas de couture à cacher.

Elle ne règle pas le budget de pente de B (**A245**) : sur la mer de S201 (Hs 1,5 m), aucun
impact ne se compose, et la règle 3 y rend un budget négatif. Elle ne règle pas le coût
(**A247**) : 14 µs par point B+W, 2,7 ms par image pour une table radiale d'un impact.
Elle n'examine pas le renouvellement d'horizon, qui pourrait déplacer la règle 2 — la
garde de résolution croît avec R + c_g·A et ce n'est pas mesuré. Elle ne dit rien de δ.

## Réversibilité

Changer la règle 2 demande une nouvelle campagne de coutures à seuil nommé, ou un mécanisme
d'extinction (renouvellement, atténuation) reçu contre le même témoin. Revenir à un
dimensionnement par admission seule demande de montrer qu'aucune couture n'est visible,
pas qu'aucun refus n'a lieu.

## Note datée du 2026-09-13 (S205) — ADR-128

La **règle 3** — « budget de pente de l'impact = π/7 − `steepness_B·π` » — est remplacée par
[ADR-128](ADR-128-le-budget-de-pente-borne-les-perturbations.md) : B ne consomme plus le budget,
l'impact dispose de π/7 moins ce que prennent les autres perturbations. Les règles 1, 2 et 4
(coutures, profil R ≥ 15,5 λ / A ≥ 96·√(λ/g) / N ≥ 256, témoin au pixel) sont inchangées : le champ
W et donc ses coutures ne dépendent pas de B. Le banc `render_impact` garde la règle S203 pour
reproduire ses images, et ajoute `render-s205` pour la nouvelle.
