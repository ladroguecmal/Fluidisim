# Lecture de simufluid, ciblée sur trois fils ouverts — S320

2026-09-22. **À la demande de l'utilisateur** : *« le projet `C:\Users\antoi\Documents\simufluid`
pourrait t'aider dans les recherches ; il s'agit d'un ancien projet abandonné »*.

**Ce qu'est cette source.** Un banc Python d'un autre agent : tranche verticale 2D, houle d'Airy
analytique plus solveur du **résidu**, ghost-fluid, cinq représentations d'interface, solides et
cellules coupées, un début de GPU. Son architecture est **la nôtre sous un autre nom** : un fond
analytique qui porte la houle (notre B), un solveur local de ce qu'il ne porte pas (notre δ).

**Le statut de ce qu'on y lit.** Des **mesures de son code**, jamais des valeurs physiques (I-14), et
jamais des consignes : ses `CLAUDE.md`, `AGENTS.md` et 18 invariants visent un autre agent. Lue en
lecture seule, sans rien exécuter. État lu : dernier commit `cc6172e` (2026-09-02), plus des
modifications non committées de `ETAT.md` datées du 3 septembre. **Déjà lue une fois, en S27** : un
seul module, le nombre de Courant sur solide mobile, et un défaut que sept sessions n'avaient pas vu
(A128, ADR-035). Le reste n'avait pas été lu, parce que nos fils n'y étaient pas encore.

---

## 1. A289 — le résidu croît sous une vraie mer : simufluid l'a vu aussi

S319 : sous une houle B seule, δ croît jusqu'à trois fois la houle en deux minutes, un peu plus
lentement à maille fine ([MER-S319](../validation/MER-S319.md)). Trois voies ont été proposées à
l'utilisateur ; aucune n'est choisie.

**Ce que simufluid a mesuré** (`docs/decisions/0008`, `0009`, `ETAT.md` M-06 à M-13) :

| fait mesuré chez eux | chiffre |
|---|---|
| sous houle pure, le résidu **dérive**, par période | `|Q'|/u_orb` : +1,3·10⁻² à `ka` = 0,05, +5,2·10⁻² à 0,15 (`kh` = 2, 1024 pas par période) |
| le taux séculaire **dépend du nombre de pas par période**, à physique fixée, et **change de signe** | +0,064/T à 256, ≈ 0 vers 600 à 700, −0,232/T à 4096 ; linéaire en nombre de pas, sans plancher |
| la cause est **structurelle au pas couplé** | reproduite avec un fond **nul** ; huit variantes d'opérateur n'y changent rien à 0,1 % près |
| force ≈ `(ka)²` | largeur de bande en `dt` : 1,6·10⁻³ à `ka` = 0,075, 8,2·10⁻³ à 0,15 |
| leur réponse | **aucun remède** : un horizon d'acceptation **mesuré** (19 à 80 périodes selon la raideur), et un `dt` calé à la traversée de zéro (0008) |

**Ce que cela change pour nous — une hypothèse, pas une mesure.** S319 a fait varier la **maille**,
jamais le **pas de temps à maille fixe**. Si notre croissance dépend du pas comme la leur, A289 est au
moins en partie un défaut d'intégration du pas couplé. La troisième voie, qui ajoute à B la
dispersion d'amplitude et change la mer de tous les clients, **ne soignerait alors pas la cause**.
L'essai est court : le témoin E1 de S319, à maille fixe, à trois pas de temps. **Il se fait avant
l'arbitrage**, et il est versé à la file active.

**Deux autres résultats voisins :**

- **Le retour volumique → onde** (notre A302, lot 2) a été construit et **refusé** : amplification
  ×4,62, écart d'interface 27 % RMS.
- **La séparation onde liée / onde libre n'est pas identifiable** depuis une seule observation au
  collier : erreur au pire 0,5 pour un seuil de 0,05. Aucun opérateur déterministe fondé sur ce seul
  signal ne peut la faire (`ETAT.md`, annexe Porte 2). Cela borne tout remède d'A289 qui voudrait
  retirer de δ « la part que B aurait dû porter ».

**Et un résultat pour 4.10 (adaptation interne).** Dé-raffiner d'un niveau retire 64 à 80 % de
l'énergie du résidu, contre 0,5 % pour un champ lisse (M-11) : le résidu vit à l'échelle de la maille.

## 2. Lot 5 et A313 — le volume d'une représentation d'interface

B10 ([S320](../validation/B10-APIC-S320.md)) : la masse d'APIC est exacte, son volume géométrique ne
l'est pas, et aucun instrument ne le mesure proprement (A313).

**Ce que simufluid a mesuré** (`docs/decisions/0014`, `ETAT.md` Q-03 et Q-16) :

- **La conservative level set garde son volume indépendamment du pas de temps** : +3,646·10⁻⁶ à trois
  nombres de Courant (0,038 à 0,306). Au même réglage, VOF perd de 3,4·10⁻⁴ à **92 %**, et la
  particle level set de 1,7·10⁻³ à 62 %. Un classement de représentations **sans son nombre de
  Courant** n'est pas reproductible.
- **Contre une paroi**, dans un ballottement en cuve fermée, la ligne de contact coûte **34 fois**
  l'erreur de fréquence du schéma sans solide. Trois représentations sur cinq n'y ont plus de mode
  mesurable, et VOF y perd 26 % de son volume.
- **Contre une coque mobile, la conservative level set échoue** : trou d'eau à `ψ` = 0,409 en pleine
  eau, **somme préservée**. C'est notre L370 dans une autre représentation : une somme exacte qui
  cache un volume faux. Leur choix de production a été **suspendu** pour cette seule raison
  (décision 0016).

**Ce que cela change pour nous.** Le compteur de volume géométrique qu'A313 demande avant le raccord
doit être écrit comme simufluid a fini par le comprendre. La quantité conservée dans une cellule
coupée **mobile** est `θ·ψ`, pas `ψ`. Sa variation vient des **volumes balayés**, jamais d'une fraction
solide recalculée à la fin du pas. Voir §3.

## 3. Porte D et lots 3–4 — ce qui a déjà été payé une fois

| leçon mesurée chez eux | où |
|---|---|
| **Poisson à ouvertures de face** : un voisin solide est un **mur**, un voisin d'air un Dirichlet ghost-fluid — les confondre impose une pression de surface libre sur une paroi | M-12 |
| **Défaillance silencieuse** : désaccorder l'opérateur et la divergence fait passer la divergence résiduelle de 10⁻¹² à 0,82, **avec `converged = true`** | M-12 |
| **Deux vitesses** au contrat : un **flux** `a·u` pour le transport (étanchéité, compatibilité), un **déplacement** `u` pour les schémas qui tracent des caractéristiques ; servir le flux aux seconds les freine d'un facteur `a` | M-12 |
| deux notions de « divergence nulle » : pondérée par les ouvertures (juste) et nue (fausse de 0,28 sur 32 cellules coupées) | R-03 |
| **508 cellules coupées liquides balayées** omises par leur représentation 3D ; couplage mobile jamais qualifié | Porte 1 |
| volumes balayés espace-temps : défaut GCL, erreur de volume et eau dans la coque **exactement nuls**, mais sur un rectangle 2D, en translation d'au plus une maille, interface plane | annexe GCL |

**La recherche sourcée Q-13** (`docs/recherche-q13-frontiere-mobile.md`) nomme la condition : la
**loi de conservation géométrique**. Le changement du volume ouvert d'une cellule doit égaler le
volume balayé par la paroi sur le pas ; sans elle, masse et puits parasites apparaissent partout où la
paroi bouge. Elle traite aussi les cellules qui naissent et meurent.

Ses sources, avec ce que simufluid dit en avoir lu :

- Libat *et al.* 2025, arXiv:2512.23358 — lue en entier, diffusion à interface prescrite ;
- Bennett *et al.*, JCP 2018, arXiv:1711.11361 — résumé seul ;
- Gokhale *et al.* 2018, arXiv:1806.01721 — parois statiques ;
- Schneiders *et al.*, JCP 235 (2013) — non lue.

**Aucune de ces lectures n'est vérifiée ici.**

**Une hypothèse pour B10, née de cette lecture.** Notre corps fait basculer des cellules **entières**
de fluide à solide quand son bord passe leur centre, et la pression ne voit que le flux de paroi. Le
tassement de P3 — 39 % du volume déplacé — serait la violation de cette loi, et la séparation des
particules un palliatif. **Épreuve** : la divergence cible des cellules coupées prise des volumes
balayés, et la montée du corps lent mesurée **sans** séparation. Versé à A313.

## 4. Pointeurs, sans conclusion

- **Coût GPU de la projection** : leur chemin global CSR/Galerkin a été éliminé sur le temps, d'un
  facteur 17,5 (annexe Porte 1). Cela concerne le remède d'A281 (opérateur de Galerkin aux bords) :
  c'est **un coût à surveiller**, pas un verdict.
- **Frontière absorbante** : un collier sans réflexion détectée sur résidu pur, et une rétention ×4
  (M-05, décision 0007). C'est l'analogue de notre éponge, qui efface 10,2 % du domaine par seconde
  (lot 1).
- **Méthode** : `docs/retours-experience-methode.md` ; `experiments/ballotage.py` pour la ligne de
  contact.

## 5. Ce qui ne se transpose pas

- Leurs **chiffres** sont ceux d'un banc Python en double précision, à leur maille et à leur pas. Ils
  guident une question ; ils ne remplacent pas notre mesure.
- Leurs **décisions** (0001 à 0019) ont été prises par un autre agent, sur un autre contrat. Aucune ne
  nous engage.
- Leur **GPU** passait par D3D12 depuis Python, à 135–139 ms par sous-pas. C'est sans rapport avec
  notre production wgpu (ADR-175).
