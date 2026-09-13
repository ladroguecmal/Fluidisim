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

Session : S223 — terminée
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue avec S223 », même conversation que S222. master et trois copies à 416daa3,
jeton libre, **maillons 1**. Copie principale.
Objectif : **A262** — la somme des majorants d'impact ignore la distance entre champs. Chercher
l'**inégalité** qui en tient compte, et la construire si elle tient.

### Ce que la lecture du champ radial établit avant toute mesure

`RadialImpact::sample` calcule la pente radiale comme

```
dη/dr(r, t) = Σ_n  c_n · k_n · J₁(k_n r) · cos(ω_n t)
```

et le majorant publié est construit à la naissance par une seule ligne :

```rust
slope += node.coefficient * k;   // |J1| <= 1, borne conservative.
```

**Deux faits en découlent, et ils commandent la session.**

1. **La borne employée est `|J₁| ≤ 1`, alors que le maximum de `J₁` vaut 0,5819.** Le facteur
   manquant est précisément ce que `SLOPE_L1_RATIO = 1,795071` corrige — un rapport **mesuré**
   (ADR-094, S141), et `1/0,5819 = 1,7185` en est à 4,5 % près. La constante mesurée du dépôt est
   donc, pour l'essentiel, le pic de `J₁` retrouvé par la mesure.
2. **`|J₁(x)| ≤ √(2/πx)` pour tout `x > 0`** (inégalité classique, `ν ≥ 1/2`). Le majorant d'un
   champ radial **décroît donc en `1/√r`**, et cela se démontre au lieu de se mesurer. C'est
   exactement la conscience spatiale qu'A261 déclare absente de toute enveloppe de modules — mais
   un champ radial **n'est pas** une somme de modules à support infini, et c'est ce qui le rend
   traitable.

### Thèse

**1. Une enveloppe par couronne, prouvée.** Pour une couronne `r ≥ r₀` :

```
|dη/dr| ≤ L(r₀) = Σ_n |c_n| k_n · B(k_n r₀),     B(x) = min(0,5819 ; √(2/πx)).
```

**2. Le majorant retenu par champ est le minimum de deux majorants** —
`min( slope_max_at(t) , L(r₀) )`. Le premier est calibré et reçu (ADR-133), le second est une
inégalité pure ; leur minimum est un majorant sans mélanger preuve et calibration. Près du pic le
premier gagne, loin le second l'écrase.

**3. L'inégalité conjointe vient de l'inégalité triangulaire.** Pour un point `p`, si `r₁ = |p − c₁|`
alors `r_i ≥ |d_{1i} − r₁|`. Donc, en balayant `r₁` sur son domaine :

```
max_p Σ_i F_i(r_i)  ≤  max_{r₁}  [ F₁(r₁) + Σ_{i≠1} F_i(|d_{1i} − r₁|) ].
```

C'est un balayage à une dimension, `O(S · N · nœuds)`, valable pour un nombre quelconque de champs,
et **il tient compte de la position relative** — ce que la somme actuelle ignore.

### Le risque nommé avant de commencer

**La borne doit majorer ce que le code calcule, pas la vraie fonction de Bessel.** `radial_impact`
évalue `J₁` par sa propre approximation (table plus Hermite, ADR-129 pour l'image ; `bessel` pour le
champ). Si cette approximation dépasse `B(x)` où que ce soit, l'inégalité est fausse **pour le
programme** même si elle est vraie en mathématiques. La réception doit donc comparer la borne à
`bessel` **exécutée**, sur tout le domaine admis, et si un dépassement existe, le couvrir par une
garde à provenance mesurée (méthode ADR-133) — ou renoncer.

### Critères, déclarés avant toute mesure

1. **Sûreté d'abord** : `B(x)` n'est jamais dépassée par `|J₁|` **tel que le code le calcule**, sur
   tout le domaine ; et l'enveloppe conjointe n'est jamais sous le maximum réel de la composition,
   à aucune séparation ni aucun instant.
2. **Gain**, par séparation `d` et par âge : enveloppe conjointe contre la somme actuelle des
   `slope_max_at`, et contre le maximum réel échantillonné finement.
3. **Coût** : le balayage à une dimension, mesuré, contre les 2 ms d'image et contre les 25 s qui
   ont disqualifié la borne de pression en S222.
4. **Traduction en admission** : combien d'impacts frais passent, avant et après — c'est le chiffre
   qu'A262 attaque (47,4 % de π/7 pour un seul aujourd'hui).
5. Aucun seuil de réussite présumé ; publication avec techniques, domaine et rang de passage.

**Prédiction écrite pour être contredite** : la borne par couronne dépassera l'approximation du code
quelque part — probablement aux très petits `x`, où l'interpolation d'Hermite n'a aucune raison de
respecter une inégalité asymptotique —, et il faudra une garde. Le gain conjoint sera **nul à
séparation nulle** et important dès que `d` dépasse quelques longueurs d'onde ; à `d = 50 m` pour
λ = 3,35 m, j'attends un facteur supérieur à 3 sur le terme d'impact. Le coût sera négligeable
devant 2 ms.

### Plan

- [x] **P1** — jeton, entrée, lecture du champ radial, thèse, risque, critères, prédiction, plan seuls.
- [x] **P2** — sûreté de `B(x)` contre `J₁` **exécutée** : balayage du domaine, dépassement mesuré, garde si nécessaire.
- [x] **P3** — enveloppe par couronne dans le cœur : `min(slope_max_at, L(r₀))` ; test de sûreté et de resserrement.
- [x] **P4** — inégalité conjointe par balayage à une dimension ; sûreté contre le maximum réel de la composition, à plusieurs séparations et instants.
- [x] **P5** — gain et coût ; traduction en impacts admis.
- [x] **P6** — décider : ADR et câblage du budget si la borne tient et le prix passe ; sinon constat motivé.
- [x] **P7** — document de réception (en-tête ADR-131 D3) ; suite complète `code/`.
- [x] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2 : **la prédiction est contredite, et pas là où je l'attendais.** J'annonçais un dépassement aux
petits `x`, dû à l'interpolation d'Hermite. Le dépassement est ailleurs et sa cause est plus grave :
**l'inégalité que j'ai supposée est fausse en mathématiques**.

`|J_ν(x)| ≤ √(2/πx)` vaut pour `ν = 1/2` — où `J_{1/2}(x) = √(2/πx)·sin x`, donc avec égalité — et
**pas pour `ν = 1`**. Mesuré sur `bessel` exécutée, domaine [0 ; 2048], 4 millions d'échantillons
par régime :

| régime | pire rapport à l'asymptote | en | dépassement |
|---|---:|---:|---:|
| table de Hermite (x ≤ 64) | **1,034023** | x = 2,1656 | **3,4 %** |
| asymptotique (x > 64) | 1,000044 | x = 65,18 | 44 ppm |

Le dépassement de 3,4 % est **la fonction elle-même**, pas son approximation : `√(2/πx)` est la
limite en `+∞`, pas un majorant, et `|J₁|` la dépasse aux `x` modérés. Les 44 ppm du régime
asymptotique, eux, viennent bien des termes correctifs de l'expansion (A&S 9.2.1).

**La constante qu'il faut** : `sup_x |J₁(x)|·√x = 0,825031`, atteint en `x = 2,165952`, contre
l'asymptote `√(2/π) = 0,797885` — 3,4 % au-dessus, cohérent avec la ligne précédente. Et le pic
plat : la table rend `0,581865191` en `x = 1,840658` contre `J1_PEAK = 0,581865013`, soit
**0,36 ppm** de dépassement.

**Forme retenue** — allure **prouvée**, constante **mesurée**, et c'est à dire ainsi :

```
B(x) = (1 + g) · min( 0,5818650 ;  0,8250310 / √x ),     g = 1e-4
```

La garde `1e-4` vaut **275 fois** le plus grand dépassement mesuré du palier (0,36 ppm) et couvre
l'égalité par construction sur la branche en `1/√x` ; provenance : `examples/couronne_impact_s223.rs`
(méthode ADR-133).

**Et cela éclaire une constante du dépôt.** `SLOPE_L1_RATIO = 1,795071`, mesuré en S141 pour
convertir la borne L1 en pente réelle, vaut `1/0,5819 × 1,045` : **c'était le pic de `J₁` que la
mesure retrouvait**, à 4,5 % près. Le code posait `|J₁| ≤ 1` en commentaire — « borne
conservative » — et la calibration rattrapait le facteur derrière.

P3 : `RadialImpact::slope_max_beyond(time, radius)` dans le cœur, avec ses trois constantes —
`J1_PEAK_BOUND = 0,5818650`, `J1_DECAY_BOUND = 0,8250310`, `RADIAL_ENVELOPE_GUARD = 1e-4` — et leur
provenance en documentation. Elle rend le **minimum** de `slope_max_at(time)` et de
`(1+g)·Σ |c_n| k_n · min(J1_PEAK, J1_DECAY/√(k_n r))` : deux majorants du même champ, l'un calibré
et reçu (ADR-133), l'autre une inégalité à constante mesurée. Leur minimum ne mélange pas preuve et
calibration, il choisit la meilleure des deux. `radius ≤ 0` rend le global **au bit**. `O(N)`, aucune
allocation.

Test `slope_max_beyond_bounds_the_annulus_and_tightens_s223` : quatre longueurs d'onde (0,75 · 2 ·
3,35 · 5 m), six instants adimensionnés, **25 couronnes** par instant et 1 501 points de rayon
chacune — jamais dépassée ; jamais au-dessus de `slope_max_at` ; égalité au bit à `radius = 0` ; et
resserrement **supérieur à 3** à la naissance sur la couronne `r ≥ 15,5 λ`, là où ADR-133 ne donne
rien (ρ = 1). Passe en 12,4 s.

Les chiffres de resserrement par rayon viennent avec P4 : un seul programme porte la mesure de
l'enveloppe seule et celle de l'inégalité conjointe, qui la consomme.

P4+P5 : `mixed::slope_floor_joint(impacts, pressure, time, samples)` — un seul programme porte les
deux étapes. **Le balayage est sûr entre ses échantillons, pas seulement dessus** : sur une cellule
`[a, b]` de `r₁`, `F₁` est majorée par `F₁(a)` — elle décroît — et `F_i(|d_i − r₁|)` par `F_i(δ)`
avec `δ` la plus petite distance atteignable sur la cellule, **nulle si `d_i ∈ [a, b]`**. Aucune
constante de Lipschitz : les deux termes sont monotones du bon côté. Résultat pris en **minimum**
avec la somme d'origine — jamais plus lâche. Un seul champ ou `samples = 0` rendent `slope_floor`.

**Sûreté** : l'assertion `conjointe ≥ maximum réel` tient sur les 30 configurations — deux et trois
impacts, séparations 0 / 5 / 20 / 50 / 90 m, âges 0 / 2 / 8 s, maximum relevé au pas de 0,25 m sur
**l'intersection des disques**, seule région que la composition admet.

**Gain**, et il est maximal exactement là où ADR-133 ne donne rien — à la naissance :

| impacts | d (m) | âge | somme | conjointe | gain | maximum réel |
|---:|---:|---:|---:|---:|---:|---:|
| 2 | 0 | 0 s | 0,425215 | 0,425215 | **1,0000** | 0,424938 |
| 2 | 20 | 0 s | 0,425215 | 0,258735 | 1,6434 | 0,212491 |
| 2 | 50 | 0 s | 0,425215 | 0,241167 | **1,7632** | 0,212470 |
| 3 | 20 | 0 s | 0,637822 | 0,302775 | 2,1066 | 0,212500 |
| 3 | 50 | 0 s | 0,637822 | 0,269244 | **2,3689** | 0,212471 |
| 3 | 90 | 0 s | 0,637822 | 0,248618 | **2,5655** | ~0 |

**À séparation nulle, le gain est exactement 1,0000** — la prédiction le disait, et c'est la
propriété qui rend la borne crédible : elle ne gagne que là où la géométrie le permet.

**Admission — et c'est A262 dans ses propres termes.** Part de π/7 :

| impacts | d (m) | âge | part avec la somme | part avec l'inégalité |
|---:|---:|---:|---:|---:|
| 2 | 50 | 0 s | 94,8 % | **53,7 %** |
| 3 | 50 | 0 s | **142,1 % — refusé** | **60,0 % — admis** |
| 3 | 90 | 0 s | 142,1 % — refusé | **55,4 % — admis** |

**Trois impacts frais séparés de cinquante mètres dépassent π/7 aujourd'hui et passent avec
l'inégalité.** C'est exactement le blocage que S222 avait nommé.

**Coût** : linéaire en échantillons, et la qualité ne l'est pas.

| échantillons | borne (2 impacts, d = 20, âge 0) | coût |
|---:|---:|---:|
| 4 | 0,287975 | 6,3 µs |
| **8** | **0,267912** | **13,3 µs** |
| 16 | 0,262073 | 28,2 µs |
| 64 | 0,258735 | 121,3 µs |
| 128 | 0,258241 | 262,3 µs |

**Huit échantillons rendent 96,5 % du gain pour 11 % du coût de soixante-quatre.** À 13–28 µs,
c'est 0,7 à 1,4 % du budget d'image de 2 ms — négligeable, comme la prédiction l'annonçait, mais
**seulement à cet échantillonnage** : à 64 le coût monte à 6 % et à 128 à 13 %. Le défaut par
défaut doit donc être bas, et c'est une décision de P6.

P6 : **ADR-138 actée et câblée.** `RadialImpact::slope_max_beyond`, `mixed::slope_floor_joint`,
`JOINT_SLOPE_SAMPLES = 8`. `slope_floor` aiguille vers l'inégalité **dès deux champs**, et
`sample_world_batch` calcule son budget **une fois, hors de la boucle des points**, par
`slope_floor` — sans quoi l'annonce et le refus liraient deux quantités différentes et la garantie
d'ADR-128 dans les deux sens tomberait. `steepness` publiée continue de sommer `slope_max()` : seul
le **budget de refus** emprunte l'inégalité, aucun bit publié ne change.

**Zéro attente de test à changer**, et la raison est structurelle : à un seul champ le chemin est
celui d'avant **au bit** (l'aiguillage ne se déclenche qu'à deux), et le résultat est pris en
minimum avec la somme d'origine, donc jamais plus lâche. Suite complète : **371 réussis
(273+4+1+93), 5 ignorés** — un de plus qu'en S222, le test de la couronne.

*Incident de route, consigné pour ne pas le refaire* : mes blocs de formules en commentaire de doc
étaient compilés par `rustdoc` comme du Rust — sept erreurs de jeton sur des caractères
mathématiques. Un bloc nu dans un `///` est du code ; il faut `` ```text ``. Les doctests le disent
tout de suite, la suite unitaire non.

Hôte : la scène J1 ne porte qu'**un** impact, donc l'aiguillage ne s'y déclenche pas — `floor`
0,167438 et 0,162917 aux deux âges, `d_eta_m = 0,000000000`, `VERIFY` 7,2271e-5 m : **identiques à
S222**. C'est le contrôle qu'il fallait : la voie neuve ne perturbe pas la voie existante.

P7 : [COURONNE-IMPACT-S223](../docs/validation/COURONNE-IMPACT-S223.md) — en-tête ADR-131 D3 ;
§1 l'inégalité supposée est fausse, et ce que cela dit de `SLOPE_L1_RATIO` ; §2 l'enveloppe par
couronne et sa réception ; §3 l'inégalité conjointe et sa sûreté **entre** les échantillons ;
§4 ce qu'elle rend, en gain et en admission ; §5 le coût et le choix de huit intervalles, comparé
aux 25 s de S222 ; §6 les contrôles. Suite nommée : ce qui reste d'A254 est le terme de pression,
et les deux termes ont désormais des limites de nature différente.

P8 : rituel §6 exécuté. Journal S223 ; **A263** (sévérité 3 — une constante calibrée rattrapait un
facteur calculable que personne n'avait nommé) ; suivi **A262** (traitée) ; **L304, L305**. Index,
README, REPRISE (§3 « 138 décisions », §4, file active, jeton), feuille de route (bloc J1, ligne
A254 désormais close), file plurielle de QUESTIONS-OUVERTES.
**Invariants relus** — **I-18** : c'est lui que la décision touche, et il est **mieux servi** qu'avant
(la couronne majore une pente **réelle**, pas une borne L1, et son minimum avec `slope_max_at`
conserve la conversion mesurée) ; **I-06** (aucune allocation : le balayage est une boucle sur deux
accumulateurs) ; **I-14** (deux constantes neuves, `J1_PEAK_BOUND` et `J1_DECAY_BOUND`, chacune avec
son banc ; la garde aussi) ; I-04 et I-08 inchangés. Aucun devenu faux, aucun amendé, aucun ADR
réécrit.
**Règle des deux maillons : compteur remis à 0**, et par le code — `outils/velocite.sh` donne
**W = S223** (contre S221 avant la session). Le compteur était à 1 en entrant ; S223 l'a fait
retomber en construisant.
**Recommandation portée** : la ligne `Session suivante` ne nomme plus un reliquat — elle renvoie à
la **file**, et nomme V-noyau, zéro module en 223 sessions, avec la consigne de ne pas le laisser
glisser si l'hôte passe d'abord.
Décomptes vérifiés : 138 fichiers dans `docs/adr`, 305 leçons, 263 angles.
Jeton libre, battement 18:30. Copies de travail avancées sur master après ce commit.
