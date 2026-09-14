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

*État relu S227, 2026-09-13* : **hôte GPU B + impact + sillage présent** (S211–S213),
mer JONSWAP, composition par le cœur reçue S214. Budget de pente resserré S215/S216/S223 ;
**A254/A262 closes**, l'admission ne bloque plus par principe une scène à plusieurs sources.
Le domaine honnête du sillage est publié (ADR-132) ; l'hôte avertit lorsqu'il en sort.

**J1 reste partiel** : passe GPU d'eau 4,16 ms à caméra fixe et 2,85 ms en médiane de balayage
à 960×540 ; cadence complète ≈198 Hz (S225). Ces mesures qualifient cette implémentation,
pas l'objectif. LOD/visibilité/mutualisation restent à construire ; qualité aux angles rasants,
interaction représentative, allocations de la pile graphique et seconde cible non reçues.
Voir [CADENCE-HOTE-S225](validation/CADENCE-HOTE-S225.md) et la table ci-dessous.

A255/A261 restent des limites de bornes, **pas un préalable générique à la mutualisation** :
S222 a déjà admis plusieurs sillages ; ses bornes locales à ≈25 s ne sont pas intégrées.
A258 suit leur réserve numérique, A263 la constante Bessel. Les déclencheurs de reprise vivent
uniquement dans la file active. L'ordre de livraison n'interdit pas de construire les briques
indépendantes de J2 ou V pendant que J1-bis progresse.

#### J1-bis — Espace d'optimisation du rendu et travaux nécessaires (ADR-131, S213)

Un dépassement mesuré qualifie l'implémentation, pas la fonctionnalité. **2 ms (ADR-125) est un
objectif éprouvé sur la combinaison** des techniques ci-dessous, sur des scènes représentatives,
sans présumer qu'elle réussira ou échouera. Aucune demande de réduction d'ambition ne se fonde sur
l'échec d'optimisations prises isolément. La liste est ouverte.

| technique | état au 2026-09-13 | publie avec elle |
|---|---|---|
| phases repliées de B au GPU | **présente** (S211) | écart GPU/cœur |
| table de Bessel d'un impact (ADR-129) | **présente** (S208, S211) | écart au champ direct, pas λ/16 |
| **temps** — sillage : tronçons achevés repliés, modes préconstruits | **présente** (S213) : 1,26 ms forçage / 0,36 ms après à 4 096 nœuds, un fil ; 6e-8 du chemin préparé | écart au chemin préparé, pic aux bornes (2,12 ms), retour arrière (3,10 ms) |
| **espace** — grille locale et transformée | absente, nommée S212 | quadrature d'image, coutures et période |
| **LOD spatial** — densité, emprise selon distance et écran | absente | Nyquist par distance (L283), coutures entre niveaux |
| **LOD spectral** — nœuds par source selon distance et visibilité | absente | écart à la recette pleine, durée et rayon honnêtes (ADR-107, **ADR-132** : les deux lois se recalculent depuis la recette réduite) |
| **LOD temporel** — cadence de mise à jour selon distance, vitesse, régime | absente | erreur de phase, I-09 |
| **visibilité** — frustum, occlusion, hors écran | absente | exactitude au retour dans le champ |
| **mutualisation** — nœuds partagés par sources de même recette, passe/grille communes B/W | absente ; **plus conditionnée par le budget de pente** (S222 : trois sillages et huit impacts passent), désormais par le coût de passe seul | superposition dans son domaine (ADR-123), et part du budget de pente consommée |

**Chaque mesure de coût publie** techniques présentes, techniques absentes et domaine de validité
(scène, recette, sources, formats, instants, machine, grandeur mesurée) — ADR-131 D3.

**Travaux nécessaires de J1, indépendants du coût** (ADR-131 D6) : A251 et composition faites
S214, A254 close S223, cadence complète faite S225. Restent l'interaction manuelle représentative,
angles rasants et poses de caméra, allocations de la pile graphique (I-06), seconde cible (B7).
La part dynamique d'A255 n'est pas un blocage d'admission tant que le scénario reste admis.

*Bancs qui tranchent à ce jalon* : **B1** (nombre de composantes et coût de B, dès qu'un LOD
existe dans l'hôte) ; **B2** partiel (représentation de W, dès que le coût B+W par image est
mesuré sur la scène) ; **B7** partiel (budget sur la machine locale, pas encore la cible).
*Cas* : C02, C07, C18, branche W de C19.

### J2 — Domaines volumiques bornés, comme cas de construction du δ général

*Livre* : un ou plusieurs domaines δ **pris dans le système** — interfaces `Volume`/`Caps`
(ADR-007), ordonnanceur et dégradation (ADR-012, I-05), création et destruction gratuites
(I-12), éponge vers B+W (ADR-005) — sur des cas bornés : cavité et gerbe d'impact, proche-coque.
Un domaine borné est une **étape** du δ général, jamais un produit à part (ADR-127 D3).

*État au 2026-09-14* : candidat MAC x-z en bibliothèque (S199), sans allocation dans le pas et
à refus atomiques (S200), coût du pas mesuré (S202). **S230 : arrêt coopératif atomique et
temps restant explicite**, reprise reçue ; [budget δ](validation/BUDGET-DELTA-S230.md).
I-05 complet non reçu : retards réels observés, admission/marges encore absentes.
Non admissible B3. **Sur le chemin** : A244 /
S200-1 (anciennes API temps f32, respect temporel I-05). **Pression f32 reçue S231** sur les
domaines éprouvés, résidu réel contrôlé, stockage réduit sans gain de vitesse reçu ;
[preuve](validation/PRESSION-F32-S231.md). **S232 : débit ouvert reçu** sur trois fonds lisses,
ordres1,947/1,957/1,966 ; triangles fluides perdus corrigés. L'ancien≈0,90 mesurait une somme
sans ouvertures ; [preuve et limites](validation/FLUX-COUPES-S232.md).
**S233 : hauteur évolutive linéarisée**, consommée par pression/flux du pas suivant ; durée
entière et compensation f32, onde stationnaire reçue, abandon atomique hauteur comprise.
[Preuve](validation/SURFACE-LINEARISEE-S233.md), ADR-141. Géométrie encore fixe ; restent
ordre local/stabilité aux géométries nouvelles, surface géométriquement mobile, passage à
la 3D et couplage de l'écart à B+W. Part d'un
impact que W ne porte pas nommée en S203 (énergie hors ondes, cavité, gerbe).

*Bancs* : **B3** (famille de δ) quand un candidat atteint ses critères ou qu'un second existe ;
**B4** (régime perturbatif, volets restants) sur les cas livrés ; **B5** (blocs épars) quand
plusieurs blocs existent ; **B10** (cavité d'entrée) avec le premier domaine d'impact.
*Cas* : C01, C03, C04, C05, C06, C08, C09, C20, C22, C23.

### V-noyau — ouvert au plus tard avec J2

*Livre* : graphe de contenants et d'arêtes (fuites, vannes, débordements), arithmétique entière,
pas serveur à basse fréquence, état répliqué et restauré (ADR-010, ADR-022, I-03, I-10).

*État au 2026-09-14* : **ouverte en S224** — un module, `hydro_network`, reçu par **C12**
(vidange en 727,4 s contre 728 s analytiques, 0,0824 %) ;
[NOYAU-V-S224](validation/NOYAU-V-S224.md). Nœuds en millilitres entiers, orifice et déversoir, pas
de 100 ms, report de reste, normalisation par arrondi cumulatif, refus atomiques, répétabilité
locale vérifiée (seconde cible non reçue). *S228* : **géométrie orientée consommée dans le pas**,
prisme, cale et forme non convexe reçus ; **A266 corrigée**, tables historiques limitées à +Z.
Arrivées collectives bornées (S227), gravité dirigée (S226). Voir
[VOLUME-ORIENTE-S228](validation/VOLUME-ORIENTE-S228.md) pour précision, refus et coût complet.
*S229* : **capture/restauration du noyau reçue**, écarts aux valeurs d’auteur et restes conservés,
base géométrique identifiée, continuation C19-V locale identique ;
[RESTAURATION-V-S229](validation/RESTAURATION-V-S229.md).
*Restent* : `liquid_id` (A17),
`sky_exposure` et `absorb_rate`, vannes et pompes, **réseau fermé sous pression** (reporté en v2 par
l'ADR), et l’**intégration réseau/stockage** du codec, l’assemblage B/W/V de C19
et le couplage V↔δ de C21. **A264** : plancher des anciennes tables ; **A269** : précision des
géométries et tailles supplémentaires. Cuisson réelle et budget encore à recevoir selon l'usage.

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

**Une mesure de coût**, à tout jalon, publie les techniques présentes, les techniques absentes et
son domaine de validité ; son verdict porte sur l'implémentation mesurée, jamais sur une
fonctionnalité ; le budget s'éprouve sur la combinaison des techniques (ADR-131).

## 4. Arbitrages explicites

**Aucun accord de dépendances du lot S210/S211 n'est encore attendu.** GPU séparé choisi en
S207 (ADR-130), sources autorisées et récupérées S211. Une dépendance nouvelle se traite selon
son autorisation propre. Le budget eau 2 ms est acquis : on ne le redemande pas comme « budget
GPU » ; une incompatibilité établie selon ADR-131 ferait l'objet d'un arbitrage explicite.
Les faits d'intégration ou le matériel externe inconnus restent à constater.

## 5. Historique de la trajectoire

ADR-053 (S70) : construire, en commençant par W. ADR-054 (S71) : ordre des lots W sans faux
préalable. ADR-124 (S201) : image, puis budget, puis effets bornés — **lu à tort comme une
réduction**, corrigé par ADR-127 (S204). ADR-125 (S202) : profil 60 images/s, eau 2 ms.
ADR-126 (S203) : emprise d'un impact visible. ADR-128 (S205) : le budget de pente borne les
perturbations, pas la mer — A245 levé. ADR-129 (S206) : chemin d'image de W par table de
Bessel ; coût d'image J1 incompatible sur CPU, arbitrage de rendu posé. ADR-130 (S207) : rendu J1
sur GPU par un hôte séparé, choix de l'utilisateur. ADR-131 (S213) : un dépassement qualifie une
implémentation ; espace d'optimisation nommé ; 2 ms éprouvé sur la combinaison — clarification de
l'utilisateur. *Les verdicts « incompatible » de S206 à S212 se lisent désormais comme portant sur
les implémentations mesurées à leur date.*
