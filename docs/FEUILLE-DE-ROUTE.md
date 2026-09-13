# Feuille de route du système d'eau

> **Ambition finale complète, construction progressive par versions de plus en plus capables.**
> — formulation de référence de l'utilisateur, 2026-09-13, [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md).

**Ce document est le seul qui porte la trajectoire.** REPRISE, l'index et la file active y
renvoient ; ils ne la recopient pas (L137). Il se met à jour au rituel de fin quand une session
change l'état d'un jalon. Chaque état est **daté** (A185). La décision vit dans ADR-127 ; ce
document en tient l'application.

## 1. Ce qui est visé, et ne se négocie pas en cours de route

B, W, **δ général** (solveur volumétrique à surface libre, domaines multiples, interactions
volumiques générales, frontières mobiles, régime substitutif), **V** (réseau hydraulique,
inondations complexes), leur articulation, et la **grande échelle**. Les rôles restent ceux
d'[ADR-001](adr/ADR-001-decomposition-en-couches.md) §2 : δ n'a jamais d'autorité gameplay
(I-04), les conséquences de jeu d'une inondation passent par V, déterministe et autoritaire.

Réduire ce périmètre n'appartient à aucune session : il faut une décision de l'utilisateur qui
nomme ce qui est retiré (ADR-127 §6). Un budget, un ordre ou une priorité ne retirent rien.

## 2. Les jalons

L'ordre suit les **dépendances**, pas une préférence. Une flèche dit « a besoin de ».

```
J1  B/W visible et interactif
 │
 ├──> J2  domaines δ bornés (cas du δ général)  ──> J3  phénomènes, interactions, frontières mobiles ──┐
 │                                                                                                    │
 └──> V-noyau (ouvert au plus tard avec J2) ──────────────────────────────> J4  inondations complexes, V↔δ
                                                                                                      │
                                                                                          J5  ambition complète, grande échelle
```

**V n'attend pas J3.** Son noyau ne dépend ni de δ ni des frontières mobiles (ADR-054 §1, C12
est un cas V seul). Ce que J4 attend de J3 est l'**articulation** — V expose une surface,
déclenche un domaine δ local, C21 compare la masse avec et sans δ.

### J1 — Version visible et interactive avec B et W

*Livre* : une scène représentative — mer de référence, impacts, sillages — **parcourue en temps
réel**, composée B+W sans refus, coût mesuré face au profil 60 images/s / eau 2 ms.

*État au 2026-09-13 (S204)* : B construit (JONSWAP cuit, ADR-100/101) ; W construit — événement,
journal rejouable, impact radial, sources de pression et sillages, composition B+W, préparation
et restauration du service. Images **hors ligne** seulement : B (S201), impact W dans son emprise
(S203, ADR-126). Profil ADR-125 acquis, seuil 2 % acquis. **Aucun hôte interactif.**
*S205* : la composition B+W admet toute mer — le budget de pente ne borne plus que les
perturbations (ADR-128) ; impact rendu sur la mer S201.
*S206* : coût d'image mesuré sur la scène J1 — **incompatible sur CPU** avec 2 ms à toute
densité qui montre l'impact ; chemin d'image de W par table de Bessel décidé (ADR-129) ;
`paquets_W_max` retiré du profil (I-16).
*S207* : **rendu J1 sur GPU par un hôte séparé**, arbitrage de l'utilisateur (ADR-130) ;
`water-core` reste sans dépendance et publie ce que le GPU consomme.
*S208* : **table de Bessel construite** (`RadialTable`, ADR-129) — W à ~0,95 ms par image sur
36 160 sommets, image à un niveau près du chemin direct, pas de réception λ/16 ; pile de l'hôte
recommandée : wgpu 30.0.1 + winit 0.30.13 ([HOTE-GPU-S208](validation/HOTE-GPU-S208.md)).

*Bloquants nommés* :
- ~~**A245**~~ — **levé en S205** (ADR-128) : la mer S201 (Hs 1,5 m) se compose, impact compris,
  zéro refus ; [COMPOSITION-MER-S205](validation/COMPOSITION-MER-S205.md) ;
- **hôte interactif** — **tranché S207 par l'utilisateur : GPU, hôte séparé** (ADR-130) ; reste à
  construire, dépendances soumises à autorisation nommée au début du lot ;
- **A247** — **mesuré en S206** ([COUT-IMAGE-S206](validation/COUT-IMAGE-S206.md)) : à la
  densité qui montre l'impact, l'image coûte 280 ms sur un fil et 36 ms sur seize. W n'est plus
  le goulot avec la table de Bessel précalculée (ADR-129, facteur 100) ; **B évalué par sommet sur
  CPU l'est** (1,2–1,3 µs). Incompatibilité posée en arbitrage §4, pas contournée ;

*Bancs qui tranchent à ce jalon* : **B1** (nombre de composantes et coût de B, dès qu'un LOD
existe dans l'hôte) ; **B2** partiel (représentation de W, dès que le coût B+W par image est
mesuré sur la scène) ; **B7** partiel (budget sur la machine locale, pas encore la cible).
*Cas* : C02, C07, C18, branche W de C19.

### J2 — Domaines volumiques bornés, comme cas de construction du δ général

*Livre* : un ou plusieurs domaines δ **pris dans le système** — interfaces `Volume`/`Caps`
(ADR-007), ordonnanceur et dégradation (ADR-012, I-05), création et destruction gratuites
(I-12), éponge vers B+W (ADR-005) — sur des cas bornés : cavité et gerbe d'impact, proche-coque.
Un domaine borné est une **étape** du δ général, jamais un produit à part (ADR-127 D3).

*État au 2026-09-13* : candidat MAC x-z en bibliothèque (S199), sans allocation dans le pas et
à refus atomiques (S200), coût du pas mesuré (S202). Non admissible B3. **Sur le chemin** : A244 /
S200-1 (précision f64, respect temporel I-05), S199-2 (flux des faces coupées), surface libre
mobile (couvercle imposé aujourd'hui), passage à la 3D, couplage de l'écart à B+W. Part d'un
impact que W ne porte pas nommée en S203 (énergie hors ondes, cavité, gerbe).

*Bancs* : **B3** (famille de δ) quand un candidat atteint ses critères ou qu'un second existe ;
**B4** (régime perturbatif, volets restants) sur les cas livrés ; **B5** (blocs épars) quand
plusieurs blocs existent ; **B10** (cavité d'entrée) avec le premier domaine d'impact.
*Cas* : C01, C03, C04, C05, C06, C08, C09, C20, C22, C23.

### V-noyau — ouvert au plus tard avec J2

*Livre* : graphe de contenants et d'arêtes (fuites, vannes, débordements), arithmétique entière,
pas serveur à basse fréquence, état répliqué et restauré (ADR-010, ADR-022, I-03, I-10).

*État au 2026-09-13* : **aucun module** ; conception acquise (ADR-010, SPEC-004/006). Rien ne
l'empêche de commencer en parallèle de J2.

*Cas* : C12 ; branche V de C19.

### J3 — Phénomènes étendus, interactions entre domaines, frontières mobiles

*Livre* : parois et corps mobiles, flottaison, plusieurs domaines et leurs raccords, régime
substitutif et sa restauration depuis graine (ADR-001 §3.3, ADR-013, I-17), référentiel accéléré,
aération et bulles, écume, vue sous-marine.

*État au 2026-09-13* : véhicules d'essai Saint-Venant 1D à paroi mobile et corps flottant simple
(S21–S58), qui ne sont pas le système ; rien dans le candidat δ.

*Bancs* : **B6** (flottabilité), **B8** (seuils d'activation et de prédiction), **B9** (écume),
**B11** (rendu sous-marin), B4 forces et perception. *Cas* : C10, C11, C13, C14, C16, C23.

### J4 — V et inondations complexes, articulées avec la représentation volumétrique

*Livre* : compartiments en réseau, brèches, poches d'air (ADR-015), inondation visible — V expose
sa surface et déclenche δ local — avec une comptabilité de masse qui reste celle de V.

*Dépend de* : V-noyau et J3. *Cas* : C17, C21, C19 complet.

### J5 — Ambitions initiales complètes

*Livre* : grande échelle — référentiels multiples, coordonnées lointaines, bathymétrie et
hauts-fonds, conformité multiplateforme (A98), matériel cible (B7 complet), glace (C15) — puis
approfondissement.

## 3. Comment un banc entre dans la trajectoire

Un banc s'exécute **quand les composants construits permettent de trancher une décision
concrète**, et il la nomme avant de mesurer. On ne ferme pas tous les bancs avant une version
utilisable ; une version ne revendique aucune réception qu'un banc n'a pas rendue (ADR-127 D6).
Les protocoles restent ceux de [PLAN-BENCHMARK](validation/PLAN-BENCHMARK.md).

## 4. Arbitrages explicites ouverts

| arbitrage | pourquoi il est explicite | qui tranche |
|---|---|---|
| ~~Chemin de rendu et hôte de J1~~ — **tranché S207 : (A) GPU, hôte séparé** ([ADR-130](adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md)) *(fusionnait « hôte interactif » et A247, S206)* | Mesuré : B sur CPU coûte 42 ms par image à la densité qui montre l'impact (2 px), 36 ms sur 16 fils ; W est ramené à 27 µs par impact (ADR-129). Options : **(A)** hôte séparé qui évalue B et les tables W sur **GPU** — prévu par ADR-003 et I-08 (« seules des phases repliées passent au GPU ») et par `gpu_sim_ms = 2,5` d'ADR-012 ; `water-core` reste sans dépendance, l'hôte en a (téléchargement) ; **(B)** CPU seul sans dépendance — exige B vectorisé (non mesuré), un groupe de fils persistant, et que les 2 ms se comptent en **temps mur sur tous les cœurs**, ce qu'ADR-125 ne dit pas ; **(C)** changer le profil ADR-125 (fréquence ou temps eau) ; **(D)** grille à 8 px — **perd les anneaux**, donc retire l'impact visible | **l'utilisateur** — a retenu (A) |
| **Dépendances de l'hôte GPU** *(S207 ; recommandation S208)* | ADR-130 : aucune bibliothèque téléchargée sans autorisation nommée. S208 recommande wgpu 30.0.1, winit 0.30.13, pollster 1.0.1 ; demande en deux temps — résolution de l'arbre (index), puis sources ; vendoring ou non | **l'utilisateur**, au début de S209 |
| **Budget GPU de l'eau** *(S207)* | ADR-125 ne dit pas où l'eau s'évalue ; ADR-012 déclarait `gpu_sim_ms = 2,5` ; aucune valeur inventée | la première mesure de l'hôte GPU ; arbitrage explicite si incompatible |
| ~~A247 — coût d'un impact visible~~ | **mesuré S206** ; part technique tranchée par ADR-129, part d'arbitrage fusionnée ci-dessus | — |
| ~~A245 — mer composable~~ | **tranché S205, ADR-128** : B hors du budget de refus, bits publiés inchangés | — |

## 5. Historique de la trajectoire

ADR-053 (S70) : construire, en commençant par W. ADR-054 (S71) : ordre des lots W sans faux
préalable. ADR-124 (S201) : image, puis budget, puis effets bornés — **lu à tort comme une
réduction**, corrigé par ADR-127 (S204). ADR-125 (S202) : profil 60 images/s, eau 2 ms.
ADR-126 (S203) : emprise d'un impact visible. ADR-128 (S205) : le budget de pente borne les
perturbations, pas la mer — A245 levé. ADR-129 (S206) : chemin d'image de W par table de
Bessel ; coût d'image J1 incompatible sur CPU, arbitrage de rendu posé. ADR-130 (S207) : rendu J1
sur GPU par un hôte séparé, choix de l'utilisateur.
