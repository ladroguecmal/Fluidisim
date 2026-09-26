# ADR-207 — La campagne du solveur volumique 3D : colonnes hautes, multigrille, APIC en bande

- **Statut : actée**, S384, 2026-09-26. **Demande de l'utilisateur** (S379, R30) : la campagne du solveur volumique 3D
  temps réel, *« le plus important »*, commencée par sa conception ; S384 : *« Solveur 3D ici »*. La technique est du
  projet (autonomie S71, ADR-028).
- **Conception** : [CAMPAGNE-SOLVEUR-3D-S384](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) — inventaire, état de l'art,
  cibles, architecture, découpage. Cet ADR n'en recopie que les décisions.
- **S'appuie sur** [ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md) (référence CPU et production résidente,
  travail borné, représentation en colonnes), [ADR-186](ADR-186-apic-seconde-representation.md) (APIC, seconde
  représentation), [ADR-006](ADR-006-cellules-domaines-solveurs.md) §3 (blocs, niveaux de `dx`), ADR-012 (budget,
  dégradation), ADR-013 (prévision), [ADR-202](ADR-202-niveau-de-detail-des-contenants.md) et
  [ADR-203](ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) (niveaux de détail), [ADR-174](ADR-174-arbitrages-du-2026-09-19.md)
  D3 (δ ≤ 2 ms GPU).
- **Répond** à la question qu'ADR-006 §6.4 reportait « après B3 » : un `dx` plus fin en vertical près de la surface.

## 1. Le constat qui décide

Le domaine de la porte B — 120 × 112 × 28 mailles de 25 cm, 376 320 mailles — coûte **1,92 ms** au 99ᵉ centile par image
à 30 Hz ([COUT-DELTA3D-S341](../validation/COUT-DELTA3D-S341.md) §11) : **il consomme seul le budget de δ**, à 4 % près.
Les usages demandent **5 cm** près du joueur et plusieurs domaines ensemble. Une boîte dense de 7 × 7 × 2 m à 5 cm ferait
784 000 mailles, deux fois le budget (*estimé*, conception §3.3). La représentation, elle, est la bonne : c'est celle du
temps réel depuis Chentanez, Müller et Kim (2014) — conception §2. **Ce qui manque est d'exécution.**

## 2. Décisions

**D1 — Une grille, trois représentations superposées** : colonnes (fonction hauteur) par défaut ; APIC **dans une bande**
sous la surface, là où elle cesse d'être un graphe ; particules diffuses par-dessus, pour l'image seulement (I-04).
Précise ADR-186 D3 : la bande est la forme des particules.

**D2 — Des colonnes hautes dans chaque domaine** : `k` couches cubiques qui suivent la surface, une maille haute jusqu'au
fond, à profil de pression linéaire (Irving *et al.* 2006 ; Chentanez et Müller 2011). `k` est un paramètre de profil
(une allocation, I-16), **à calibrer** en C2 contre les réceptions de dispersion.

**D3 — La pression par gradient conjugué préconditionné par un cycle multigrille** (McAdams *et al.* 2010), sur les
mailles cubiques et hautes, faces coupées compatibles à tous les niveaux (Weber *et al.* 2015), en travail fixe par pas
(ADR-175 D2). Le préconditionneur 2D de S245 en est le point de départ.

**D4 — Le niveau de détail est le `dx` d'un domaine, choisi par l'ordonnanceur** parmi les niveaux d'ADR-006 §3.2
(5, 10, 25 cm pour la campagne), sous le budget commun et l'ordre de dégradation d'ADR-012 §4 ; la prévision (ADR-013)
prépare le domaine avant le perturbateur. Les domaines restent des ensembles épars de blocs (ADR-006 §3.1).

**D5 — L'ordre, et le lieu.** Les sessions C1 à C11 de la conception (§5), chacune avec son critère « reçu si » écrit
avant elle : d'abord la **référence** — multigrille 3D (C1), colonnes hautes (C2) —, puis la carte (C3) ; APIC 3D (C4),
son raccord (C5) et sa bascule (C6) ; APIC sur la carte (C7), blocs épars et niveaux (C8), particules diffuses (C9), les
scènes (C10). **C1, C2, C4, C5 et C6 se font sans carte graphique.** Le critère d'arrêt de la campagne est celui de la
conception §3.4 : une scène vivante — un joueur qui saute à 5 cm pendant qu'une coque passe —, δ ≤ 2 ms au 99ᵉ centile,
la production à 3 mm de la référence, la masse exacte, et le jugement de l'utilisateur.

## 3. Ce que la décision ne fait pas

Elle ne mesure rien : le gain des colonnes hautes (÷ 3,1 mailles à la porte B) et celui du coût par maille (÷ 2) sont
**estimés**, et C2 et C3 les mesureront ; un écart se dira, sans relever un seuil (L177). Elle ne décide pas **où vit δ à
la fin** — l'afficheur ou Godot — : la proposition est l'afficheur jusqu'à C10, Godot en C11 ; c'est à l'utilisateur
(R27 : *« pas maintenant »*). Elle ne change ni les invariants, ni les portes, ni ADR-175, ni ADR-186, et ne retire rien
du périmètre (ADR-127). Les articles cités n'ont été lus qu'en résumé (réseau de la session) : la conception §5 dit
lesquels lire, et avant quelle session.

## 4. Ce qui devient faux si elle est mal lue

**« Colonnes hautes » ne veut pas dire « l'eau profonde est simplifiée à la main »** : la maille haute porte la même
équation, avec un profil de pression linéaire ; elle se reçoit contre les mêmes oracles que les mailles cubiques (C2).
**« APIC en bande » ne veut pas dire « APIC partout près de la surface »** : la bande n'existe que là où la surface cesse
d'être un graphe (C6). **Et « campagne » ne veut pas dire « ordre figé »** : une mesure qui contredit une estimation de la
conception rouvre l'ordre, par une note datée ici ou un nouvel ADR.

Invariants relus : I-04, I-05, I-06, I-07, I-12, I-13, I-14, I-16, I-17 ; aucun amendé.

## Note datée du 2026-09-26 (S386) — où vit δ à la fin : tranché par l'utilisateur

La question du §3 est tranchée : *« Je suis d'accord avec toi pour le branchement à la fin »*. La production de δ se
construit dans l'afficheur jusqu'à C10 ; **δ entre dans Godot en C11**, la scène reçue. Aucune autre décision n'est changée.
