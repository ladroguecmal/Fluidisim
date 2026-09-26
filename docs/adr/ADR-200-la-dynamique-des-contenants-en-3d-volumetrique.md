# ADR-200 — La dynamique de l'eau des contenants se calcule en 3D volumétrique

- **Statut : actée**, S374, 2026-09-26, **décision de l'utilisateur** : *« La dynamique de fluide doit se faire en 3D
  volumétrique »* — reçue pendant S374, qui visualisait la piscine de V (niveaux et débits) et allait dessiner la lame du
  déversoir et le jet de la pompe par des trajectoires balistiques tirées des débits.
- **Précise** [ADR-025](ADR-025-propriete-de-la-masse-entre-V-et-delta.md) (la masse reste au nœud, δ y est forcé) et
  [ADR-186](ADR-186-apic-seconde-representation.md) (APIC, la seconde représentation) pour les contenants de V ;
  [ADR-199](ADR-199-vannes-et-pompes-dans-v.md) n'est pas changé. Porte **E** de la feuille de route, liste **5.10**.
- Preuve de l'état de départ : [PISCINE-V-S374](../validation/PISCINE-V-S374.md).

## 1. Ce qui est décidé

**D1 — Le mouvement de l'eau d'un contenant se calcule, en trois dimensions, par δ.** Dans un contenant qu'on voit ou
avec lequel on interagit — la piscine d'abord —, la surface, le clapotis, le choc du jet, l'écoulement vers le seuil sont
ceux d'un domaine δ 3D (`Volume3`, [ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md)), et **jamais** un
habillage cinématique déduit des débits de V. L'habillage balistique préparé en S374 (P4) est écarté comme dynamique ; il
n'est pas construit.

**D2 — V garde la masse, δ la dynamique** (ADR-025, I-04, I-15). V reste l'autorité répliquée : volumes entiers, débits
des arêtes, commandes. Le domaine δ d'un contenant est amorcé au niveau de V et **forcé vers son volume** (relaxation,
`τ ≈ 1 s`) ; les arêtes de V deviennent pour lui des **sources et puits locaux**, à leur position : le refoulement d'une
pompe injecte son débit avec sa quantité de mouvement, le seuil d'un déversoir retire ce qui le franchit. La comptabilité
de masse est identique avec et sans δ — le critère de la porte E (C21).

**D3 — Ce que les colonnes de δ ne portent pas.** La lame qui tombe du déversoir et le jet dans l'air sont de l'eau
**au-dessus d'une autre eau ou dans l'air**, plusieurs couches sur une verticale : c'est la seconde représentation,
**APIC** (ADR-186), aujourd'hui reçue sur un banc 2D seulement (liste 4.16, A316). Jusque-là, ces deux écoulements ne
sont **pas dessinés** comme dynamique ; le bassin et le bac tampon, eux, le sont par δ.

**D4 — L'ordre.** (1) δ 3D dans le bassin : domaine sur l'intérieur, masse asservie à V, source du jet et puits du
seuil, surface publiée et rendue dans Godot ; (2) le bac tampon, de même ; (3) APIC en 3D pour la lame et le jet (lot 5) ;
(4) la commande en direct (V et δ dans le processus de Godot, ou un lien local) — l'intégration native attend l'accord de
téléchargement de godot-rust.

## 2. Ce que cela ne change pas

V reste le seul état autoritaire et répliqué des contenants (I-10 : le serveur exécute V, jamais δ) ; ADR-199 et la
sauvegarde WVST v2 inchangés. La mer (B, W, δ de la haute mer) inchangée. Rien n'est retiré du périmètre.

> **Note du 2026-09-26 (S375).** D4 (1) fait sur la référence CPU ([PISCINE-DELTA-S375](../validation/PISCINE-DELTA-S375.md)) :
> le bassin en δ 3D, masse à V (0,1 µm), jet et seuil comme sources et puits, surface rendue dans Godot. À la maille de
> 20 cm que la référence permet, la dynamique tient en quelques millimètres et ne se voit pas : la suite passe par δ sur
> GPU (5 à 10 cm). Deux défauts de la référence mobile corrigés en chemin ([ADR-201](ADR-201-plancher-de-l-echelle-de-vitesse-de-la-projection.md), `shift_rest`).

> **Note du 2026-09-26 (S376).** D1 est **corrigé** par [ADR-202](ADR-202-niveau-de-detail-des-contenants.md), décision de
> l'utilisateur : un contenant **vu de loin** n'est pas calculé par δ — V et des effets factices ; δ seulement près d'un
> joueur ou d'un perturbateur (ou prévu), en zones selon la taille du contenant. D4 n'est plus pressé : δ sur GPU dans
> Godot, *« pas maintenant »* (R27).

