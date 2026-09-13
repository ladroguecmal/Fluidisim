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

Session : S223 — en cours
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
- [ ] **P4** — inégalité conjointe par balayage à une dimension ; sûreté contre le maximum réel de la composition, à plusieurs séparations et instants.
- [ ] **P5** — gain et coût ; traduction en impacts admis.
- [ ] **P6** — décider : ADR et câblage du budget si la borne tient et le prix passe ; sinon constat motivé.
- [ ] **P7** — document de réception (en-tête ADR-131 D3) ; suite complète `code/`.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

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
