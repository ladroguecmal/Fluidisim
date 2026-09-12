# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S203 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S202-1 (ADR-124 étape 3, ADR-125) — rendre visible un **impact porté par W**,
avec **emprise** (rayon, horizon, N) et **observateur** (caméra) explicites et mesurés ;
nommer la part qui exigerait un effet δ borné, sans le construire. V hors sujet.

### Constat d'amorce, avant plan (crate jetable hors dépôt)

Recette S201 (JONSWAP N32, Tp6, bande0,5–4 fp) : plancher de pente **L1** de B
`steepness·π` = 0,1014 (Hs0,25) · 0,2027 (0,5) · 0,4055 (1,0) · **0,6082 (1,5)**,
contre `max_slope` = π/7 = 0,4488. **`compose` refuse donc chaque point de la mer S201**
avant tout impact (SlopeEnvelope). ADR-094 tient `steepness_B·π` pour la pente *exacte*
du fond (facteur 1) : vrai pour une composante, non établi pour32 composantes étalées.
Le lot ne migre pas ce terme (bits changés, lot propre) ; il le chiffre et le signale.

### Plan

- [x] **P1** — amorce : trois copies isolées à e13d212 propres, jeton pris, plan seul.
- [x] **P2** — exemple `render_impact.rs`, partie scène : mer Hs0,5 (recette S201 sinon
 inchangée), impact par `impact_generator` (b=1 m, v=8 m/s, fraction déclarée choix de
 banc), milieu profond20 m, `max_slope` de l'impact = π/7 − plancher B (budget alloué,
 dit). Pente réelle de B échantillonnée sur la fenêtre de l'image : facteur L1/réel chiffré.
- [x] **P3** — emprise mesurée : coutures spatiale (max|η_W|, |∇η_W| sur r=R, fenêtre
 [naissance, +A]) et temporelle (max|η_W| dans R à +A) pour candidats (N, R, A) admis ;
 critères déclarés **avant** mesure : ≤3 mm (tolérance du rendu) et ≤2 % du pic central
 (seuil ADR-120 emprunté comme choix de banc). Tests de la fonction de couture.
- [x] **P3b** *(ajoutée après P3)* — deux contrôles avant de retenir l'emprise : accord
 N256/N512 dans R aux instants rendus (+1/+3/+6 s) ; homothétie à λ×2 sur le critère
 relatif (R×2, A×√2, même N attendu). Si l'un échoue, retenir N512 A64 R60 et le dire.
- [x] **P4** — image : B+W par `Prepared::sample_world_batch` dans R, B seul hors R ;
 bornes de marche B+W conservatrices ; trois instants + témoin sans impact ; zéro rayon
 non résolu ; tout pixel différent du témoin doit avoir échantillonné dans R (contrôle).
- [x] **P5** — observateur et coût : pixels par λ au point d'impact, distance où λ < 2 px
 pour cette caméra ; coût par point B seul vs B+W (4096 points, 3 chauffes/11 mesures),
 points tenant dans 2 ms. Part δ nommée (cavité/gerbe/1−fraction) avec domaine et durée
 bornés, comparée au coût S202 ; aucun δ construit.
- [x] **P6** — publication `IMPACT-W-S203` (+ mesures) ; note corrective datée ADR-094 si
 le facteur de B est confirmé > 1 ; ADR seulement si une règle d'emprise est retenue.
- [x] **P7** — rituel §6 complet : journal, angles morts, leçons, index/README/REPRISE,
 décomptes, `outils/velocite.sh`, file active entière, compteur, jeton libre, copies.

### Notes de reprise

Lus : AGENTS, REPRISE (jeton, §1–3, §4 S188–S202, §5–9), journal S201–S202, ADR-124/125,
BUDGET-IMAGE-S202, IMAGE-B-S201, invariants, file active, radial_impact, composition,
prepared_water, impact_generator, render_background. S202 finie à 00:42, jeton libre.
Admission RadialImpact λ=3,35 m : Regime profondeur > 3,35 m ; cg_max≈1,62 m/s ;
Resolution dk·(R+cg_max·A) ≤ π/2 → R+1,62A ≤ 35,7 (N64), 71,4 (N128), 143 (N256).
Borne de hauteur W = η(0, naissance) = Σ coefficients (tous positifs) : pas d'accesseur.

P2 (`render_impact scene`, release) : recette1,5 = 0x7e5cc32275ccce4e (S201 retrouvée).
Hs0,5 : 0xae7cc08b64111fda, plancher L1 0,202731, budget impact 0,246068. Générateur :
λ=3,35 m, E=164 J (fraction0,005 ; max0,006698 ; E_ref32800 J). RadialImpact N64/128/256
R8 A4 s : slope_max0,212607, L1 0,381646, plancher+slope_max0,415338 ≤ π/7, η centre
0,15482 m. Pente de B, trois grandeurs, homothétiques en Hs (rapports identiques) :
L1 0,6082 / **directionnelle exacte 0,5733** (−5,7 %) / échantillonnée 0,4215 (512 m, pas
1 m, 0–120 s ; L1/échantillon = 1,4428). **Même le majorant directionnel refuse la mer S201** :
aucun budget indépendant du point ne l'admet ; seul un maximum local ou statistique le
ferait, et il ne garantit rien. Hs max composable (marge nulle, L1) : 0,4488/0,4055 ≈ 1,107 m.
Six tests exemple passent (debug). Aucune bibliothèque modifiée.

P3, écrit **avant** la campagne : décisif = max|η_W| ≤ min(3 mm ; 2 % de η centre
0,15482 m = 3,10 mm) sur la couture spatiale (r = R − 1 mm, t ∈ [0 ; A] pas 10 ms) **et**
temporelle (t = A, r ∈ [0 ; R) pas 2 cm). Pente de couture rapportée, **non décisive**.
Candidats N ∈ {64,128,256,512} × A ∈ {2,4,6,8,12,16,24,32} s, R = plus grand rayon admis
par Resolution/Reach (décrément 0,5 m jusqu'à admission). Retenir le plus petit N, puis le
plus petit R, qui passe les deux coutures pour l'horizon visé ; si aucun ne passe, le dire.

P3 résultats (`seams`, 138 lignes, sortie archivée scratchpad puis MESURES en P6) :
**aucun candidat A ≤ 32 s ne passe** — couture temporelle 78,4 mm (2 s) · 20,0 (4) · 13,0 (8)
· 10,1 (12) · 8,3 (16) · 6,2 (24) · 4,8 (32), indépendante de N ; la spatiale est nulle dès
que R dépasse le front. Horizons longs : 3,25 mm (48 s) · 2,80 (56) · 2,44 (64) · 2,16 (72)
· 1,62 (96). Balayage N512 : couture spatiale = amplitude de l'anneau au franchissement,
5,05 mm (R30) · 3,84 (40) · 3,06 (50) · 2,77 (55) ≈ 1/R, indépendante de A. **Premier passant
selon la règle : N256, A56 s, R52 m** (bord 2,937 mm, horizon 2,255 mm, marge 2 %) ; N256
échoue à A48 (R65 : horizon 3,246) et A64 (R39 : bord 3,935) → fenêtre étroite. N512 passe
pour tout R ≥ 55 m et A ≥ 56 s. Lecture : seuil relatif 2 % ⇒ R ≈ 15,5 λ ; énergie et λ
sortent du critère relatif si l'homothétie S136 tient (non vérifiée ici → P3b).

P3b, écrit **avant** exécution : (1) N256 et N512 à A56 R52, r ∈ [0 ; R) pas 2 cm, instants
+1/+3/+6 s : accord si max|Δη| ≤ 3 mm (tolérance du rendu). (2) b = 2 m ⇒ λ = 6,70 m, même
fraction : coutures **relatives** à R = 104 m, A = 56·√2 s (N256) et R = 110 m, A = 64·√2 s
(N512) ; homothétie reçue si chaque rapport couture/η centre diffère de moins de 0,1 point
de pourcentage de celui mesuré à λ = 3,35 m.

P3b résultats (`controls`) : (1) max|Δη| N256/N512 = 0,0000 mm à +1/+3/+6/+30 s, 0,0001 mm
à +56 s — **accord**. (2) λ' = 6,70 m, E' = 1312 J, η₀ 0,15482 → 0,21895 m (= √2, η ∝ √E/λ) ;
N256 bord 1,8971 → 1,8971 %, horizon 1,4563 → 1,4753 % ; N512 (R55→110, A64→90,51 s) bord
1,7900 → 1,7904 %, horizon 0,6334 → 0,6420 % — **homothétie reçue** (écart ≤ 0,019 point).
**Emprise retenue : N256, R = 52 m, A = 56 s.** Forme sans dimension, pour le critère relatif
2 % : R ≈ 15,5 λ, A ≈ 96·√(λ/g), N ≥ 256 ; le seuil absolu 3 mm devient liant quand
η₀ > 0,15 m (η₀ ∝ √E/λ). Aucun contrôle ne porte sur l'observabilité réelle de 3 mm.

P4 : caméra/marche/PPM S201 sortis dans `examples/support/ray_view.rs` (une implémentation) ;
**S201 reproduite au bit** après extraction : t12 Hs1,5 0xa52ff81902b150c3, 10 021 895 évals
(11 787 ms). `render <dir> <âge>` rend impact (B+W par `Prepared::sample_world_batch` dans
R, B hors R) et témoin (même marche, mêmes bornes B+W, même prédicat, B partout). Neuf tests
exemple. **+3 s** (t = 15 s) : impact 289 260 rayons eau, 0 non résolu, 0 refus, 9 495 898 évals
dont 5 062 984 B+W, 125 979 px touchés, 56 739 ms, 0x3dba0d3acf15447a ; témoin 10 583 ms,
0x14271a7145740ba1 ; **8 640 px différents, 0 hors emprise** ; écart max 18 niveaux (L),
1 902 px ≥ 3 niveaux. Anneaux lisibles à l'œil au centre (≈320,220), zoom ×3 inspecté.
**+1 s** (t = 13 s) : impact 289 269 eau, 0/0, 9 499 461 évals dont 5 075 829 B+W, 55 191 ms,
0xae92143729673447 ; témoin 10 508 ms, 0xdb298dee39248f0d ; 6 780 px différents, 0 hors R.
**+6 s** (t = 18 s) : impact 289 245 eau, 0/0, 9 490 157 évals dont 5 064 027 B+W, 58 172 ms,
0x7e057b5f0ccac53e ; témoin 10 621 ms, 0x590a5964b0a93cf0 ; 15 737 px différents, 0 hors R.
Résidu max 3 mm partout ; 125 979 px touchent l'emprise (27,3 % de l'image). Anneaux plus
discrets à +1 s, étendus et déformant le reflet solaire à +6 s (zooms inspectés).
Coût image B+W ≈ 5,3× le témoin : ≈ 9 µs par évaluation B+W N256 sur ce CPU (brut, P5 mesure).

P5, écrit **avant** mesure : coût par point sur 4096 points dans R (B seul, W seul, B+W par
`sample_world_batch`), 3 chauffes/11 mesures, médiane et max. Piste hôte, pas bibliothèque :
le champ étant radial, une **table η(r), η'(r)** au pas λ_min/8 = 0,209 m reconstruite par
image, puis Hermite cubique par point ; recevable pour le rendu si max|Δη| ≤ 3 mm contre
l'échantillon direct aux âges 1/3/6/30 s (20 000 rayons). Observateur : pixels par λ et λ/2
(le long de la visée et en travers) au point d'impact et distance où ils passent sous 2 px.

P5 résultats. **Observateur** : impact au pixel (320,0 ; 224,1), 28,86 m ; λ = 11,05 px le long
de la visée, 45,10 px en travers ; λ/2 = 5,51 / 22,55 px. Sous 2 px le long : λ dès 67 m,
λ/2 dès 47,5 m (horizontal, même azimut) ; en travers : λ jamais avant 600 m, λ/2 dès 325,5 m.
**L'observateur est dans l'emprise en plan** (28 m < 52 m) : sa projection n'est pas bornée,
première boîte englobante fausse retirée ; 27,3 % des pixels touchent R (P4).
**Coût** (4096 pts dans R, âges 1/3/6/30 s, médianes) : B 6,50–6,70 ms = 1,6 µs/pt ;
W N256 50,0–56,2 ms ≈ 12–14 µs/pt ; B+W `sample_world_batch` 57,1–59,3 ms = 13,95–14,49 µs/pt
→ **138–143 points dans 2 ms**. Table radiale M = 249, pas 0,209 m : construction 2,66–2,76 ms
(> 2 ms à elle seule pour un impact), évaluation 62–83 µs les 4096 = 15–20 ns/pt, erreur max
0,0014–0,0062 mm ≪ 3 mm → **recevable pour le rendu**, pas au budget. B seul : ~1250 points
dans 2 ms sur ce CPU, un fil — l'évaluation CPU par point ne tient pas 60 Hz, même sans W.
**Part non portée par W** : Fr = 1,806 (< 5, pas de cavité franche, SPEC-002 §3) ; 32 636 J
sur 32 800 hors ondes (f à calibrer B2) ; majorant balistique 3,262 m / 1,631 s ;
b = 1 m = 0,2985 λ. SPEC-001 §2.4 dimensionne déjà un δ impact 6×6×4 m à dx 0,05 :
1,15 M cellules, ≈37 Mo — rapport 2 246 aux 512 cellules x–z de S202 (0,59 ms) ; **aucun
coût transposé**, noyau 3D ou axisymétrique absent.

P6 : IMPACT-W-S203 (+ MESURES, 232 lignes, BOM/CRLF des sorties PowerShell retirés) ;
**ADR-126 actée** — emprise d'image reçue par ses coutures, profil R ≥ 15,5 λ, A ≥ 96√(λ/g),
N ≥ 256 (N512 recommandé), budget de pente d'impact = π/7 − plancher B, témoin au pixel.
Valeurs suffisantes, pas minima (R 14,9 λ et A 82√(λ/g) échouent). Notes correctives datées
ADR-094, ADR-095 et note S139 d'ADR-062 : `steepness_B·π` n'est pas la pente exacte d'un B
à plusieurs directions. Angles à écrire au rituel : A245 (plancher B refuse Hs > 1,107 m),
A246 (emprise dimensionnée par admission, jamais par couture), A247 (coût d'un impact
visible contre 2 ms ; B seul 1,6 µs/pt). Décomptes attendus : 126 ADR, 247 angles.

P7 : journal, A245–A247, L280–L281, index/README/REPRISE (§4, file, §3), file active
(A245, A247, S202-1 réalisée ; autres lignes relues, conservées). velocite.sh : B/W S181 en
code, δ S202, V jamais ; chaînage 37/44 ; part système S190–S199 6,9 %. Décomptes vérifiés
126 ADR / 247 angles / 281 leçons / 18 invariants / 6 SPEC / 23 cas. Workspace rejoué
343/cinq ignorés. I-18 relu : juste, enfreint par le terme de B (A245). Compteur 0 par
ADR-126 (l'outil dirait 1 : pas de code src). Jeton libre ; copies à avancer après commit.
