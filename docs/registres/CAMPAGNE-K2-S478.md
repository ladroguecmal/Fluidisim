# La campagne K2 — la surface non graphe

*Conçue en S478, 2026-10-04*, deuxième campagne du [plan de complétion](PLAN-COMPLETION-S475.md)
([ADR-218](../adr/ADR-218-le-systeme-de-l-eau-complet.md) : la liste à 100 %). Décisions techniques :
[ADR-220](../adr/ADR-220-la-campagne-k2.md).

**Ce que K2 doit fermer** — dix points, sur leur **périmètre final** : 4.16 (surface non graphe : déferlement, éclaboussures
détachées), 4.1 (solveur volumique à surface libre), 4.12 (cavité et gerbe d'impact, C20), 4.20 (changement de solveur, ADR-007),
7.2 (spray), 7.3 (microbulles visuelles), 7.4 (grosses bulles, C13), 7.5 (air comprimé, vide, eau dans le vide), 3.3 (explosions),
3.1 (anneaux d'impact, dont l'eau peu profonde — qui attend aussi 2.7, K3). Pour le jeu (ADR-219) : un MMO spatial — les vaisseaux
percés, l'eau dans le vide et l'air enfermé dans une coque y sont des cas de jeu, pas des curiosités.

## 1. Le point de départ

| point | ce qui est reçu | ce qui manque |
|---|---|---|
| 4.16 | APIC 3D dans le cœur (S388) ; la cavité qui se referme, reçue en 3D (S393, pincement 2,08 √(R/g), convergé à 0,6 %) ; le déferlement porté sur un banc (S410) ; la bande APIC sur la carte, en production, au temps réel (C7, S416–S470) ; le saut de la v1 (R38) | les **éclaboussures détachées** ; le déferlement **dans le système** ; un jet et une couronne qui ne dépendent pas de la maille (A312) |
| 4.1 | la référence, la production GPU, la porte B (S340) | sa surface non graphe dans la production couplée (4.16) |
| 4.12 | la cavité, en 2D et en 3D | **la bulle n'est pas de l'air (A311)** — à maille fine le calcul s'emballe à la fermeture ; la gerbe suit la maille (A312) ; C20 |
| 4.20 | ADR-007 : refusé comme transfert d'état, remplacé par transduction → destruction → création ; la bascule colonnes ↔ particules (C6) dans un domaine | la voie d'ADR-007 construite et éprouvée |
| 7.2–7.5, 3.3 | rien | tout |
| 3.1 | les anneaux en eau profonde, le générateur calibré | l'eau peu profonde (K3), la gerbe (4.12) |

**Aucun modèle d'air n'existe** dans APIC (relu en S478 : ni étiquette d'air enfermé, ni pression de poche). A311 commande donc
tout ce qui suit le pincement — le jet de Worthington, la bulle, les explosions — et, à maille fine, la possibilité même de calculer.

## 2. Les sessions

L'ordre suit les dépendances : l'air (K2-1 à K2-3) avant ce qui en a besoin ; la nappe et le spray (K2-4, K2-5) avant le déferlement
(K2-6) ; le jeu (vide, vaisseaux) avec V (K7) pour la part de 7.5 qui en dépend. Chaque session : sa référence publiée, ses critères
écrits avant, référence CPU puis carte (ADR-213 : une réception par la référence suffit à avancer, la carte suit).

| | session | ce qu'elle construit | critère de réception (écrit ici, précisé au plan de la session) | points |
|---|---|---|---|---|
| **K2-1** | **L'air enfermé, référence** | les composantes d'air enfermé (remplissage depuis l'air libre), leur volume (la surface reconstruite), leur pression par la loi adiabatique, posée comme condition de la projection | B10 3D à `D/dx` = 16 et 24 **va au bout** (A311 : 13 h sans finir) ; la bulle oscille — sa fréquence contre celle de Minnaert, `f = (1/2πR)·√(3γp/ρ)`, à 15 % ; masse exacte | 4.12, 7.4 |
| **K2-2** | **L'air enfermé, carte** | K2-1 sur le GPU (une réduction par composante) | la carte contre la référence (le volume et la pression de la bulle à 1 %) ; le saut de `--v1` stable 60 s avec une poche | 4.12, 7.4 |
| **K2-3** | **Les grosses bulles libres** | une poche qui se détache remonte selon `−g_eff`, se déforme, se fragmente ou se résorbe en champ `A` sous une taille ; C13 | la vitesse terminale contre Davies et Taylor (`0,707·√(g·d_e)`, calotte sphérique) à 15 % ; `g_eff` d'un vaisseau qui accélère (ADR-002) | 7.4 |
| **K2-4** | **La nappe et sa rupture** | une nappe plus mince qu'une maille se rompt en gouttes (modèle sous-maille, ADR-014 §4 : Weber) ; les gouttes, particules balistiques avec traînée, rendent leur masse en retombant | **A312** : la hauteur du jet et le rayon de la couronne de B10 à trois mailles, à 15 % entre elles — ou contre une mesure publiée du jet de Worthington ; masse exacte, gouttes comprises | 4.12, 7.2, 4.16 |
| **K2-5** | **Le spray dans le système** | K2-4 sur la carte ; les gouttes dans l'export et le direct de Godot | le saut de la v1 avec sa gerbe et ses gouttes, au temps réel ; banc visuel contre V3 (l'impact sur les rochers) | 7.2, 4.16 |
| **K2-6** | **Le déferlement dans le système** | la vague de Chen (`ka` = 0,55) dans la bande de la carte, raccordée à B ; le rouleau, l'éclaboussure de retombée | le retournement à 0,72 √(λ/g) (Chen 1999) à 5 % ; le jet plongeant dans la plage publiée ; masse exacte ; banc visuel contre V2 | 4.16, 4.1 |
| **K2-7** | **Les microbulles visuelles** | l'entraînement d'air (pincement, déferlement, retombée) en un nuage de bulles fines, ascension et dissolution ; rendu | la loi de taille de Deane et Stokes (2002) ; la durée de vie du nuage ; banc visuel contre V2 et V3 | 7.3 |
| **K2-8** | **L'air comprimé et le vide** | la poche T2 dans un contenant (avec V, K7) ; l'arête vers le vide d'ADR-015 §5 : débit au col, poussée de réaction, ébullition, glace qui obture | les chiffres d'ADR-015 : 14 % vaporisés, 86 % gelés ; une coque retournée qui flotte puis coule passé son point de non-retour (§3) | 7.5 |
| **K2-9** | **Les explosions** | une bulle de gaz qui naît sous pression (Rayleigh–Plesset), sa période, son affleurement ; le champ lointain vers W | la période de la première oscillation contre la loi de Willis (`T = K·W^(1/3)/(z + 10)^(5/6)`) à 15 % ; une explosion de surface et son anneau | 3.3 |
| **K2-10** | **Le changement de solveur** | la voie d'ADR-007 : transduction δ → W, destruction, création à δ = 0 sous un autre solveur ; mesurée | l'énergie partie dans W, la surface de B+W inchangée au bit, aucun saut visible (banc visuel : le mouvement entre deux images) | 4.20 |
| **K2-11** | **La réception** | C20 en production ; le saut de DyingStar (un corps du jeu d'essai) ; les quatre points fermés sur le système | C20 passe ; 4.16, 4.1, 4.12 cochés avec leur preuve ; le verdict de l'utilisateur sur la gerbe (A312, déclencheur) | 4.16, 4.1, 4.12 |
| **K2-12** | **Les anneaux en eau peu profonde** | après K3 (2.7) : la dispersion en profondeur finie, la gerbe de K2-4 comme source | C07 en eau peu profonde | 3.1 |

**≈ 30 sessions** : la plupart des lignes en demandent deux ou trois (la référence, la carte, la reprise d'un défaut).

## 3. Ce qui peut faire dérailler

- **K2-1 est le pivot.** Si la pression de poche ne suffit pas à tenir le calcul à la fermeture (A311 : « une instabilité n'est pas
  exclue »), il faut d'abord trouver l'instabilité — la session le dira, sans la contourner.
- **A312 peut ne pas converger.** Sans tension de surface, une nappe s'amincit sans fin ; le modèle de rupture (K2-4) la coupe — le
  critère porte alors sur la mesure publiée et l'image, pas sur la convergence seule (ce que disait A312).
- **Le coût.** Chaque pièce s'ajoute au pas de la carte (2,07 ms reçus, C7e) ; une pièce qui dépasse est mesurée et inscrite
  (ADR-131), et le budget se règle en K8.
