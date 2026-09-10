# Dossier d'exécution du banc B2 — couche W et `λ_cut`

- **Statut S152** : partiellement exécuté — volet impact80m/60s et restauration locale30s.
  [BANC-B2-S152](BANC-B2-S152.md), ADR-105 : choix de résolution du candidat radial,
  pas de technologie ni de lambda_cut. Le protocole global ci-dessous reste ouvert.
- **Session** : S16
- **Complète** : [`PLAN-BENCHMARK`](PLAN-BENCHMARK.md) §B2, qui reste la vue d'ensemble des onze bancs
- **Dépend de** : ADR-001, ADR-005, ADR-009, ADR-021, SPEC-001, SPEC-003, SPEC-004 §5.1, SPEC-006

B2 est le banc le plus rentable du plan : il ferme **quatre points ouverts** (le décompte de
`AUDIT-POINTS-OUVERTS-S11` §7.1, plus que tout autre), il tranche `λ_cut` — « à décider en premier »
depuis S01 — et il est en tête du réordonnancement de `QUESTIONS-OUVERTES` §32. Trois établissements
indépendants du même ordre de priorité.

Ce dossier rassemble ce qu'il faut pour le lancer le jour où H1 et H3 existent, et surtout **ce
qu'on peut déjà savoir sans mesurer**.

---

## 1. Ce que B2 décide, et ce qui en dépend

| Décision | Ce qu'elle fixe en cascade |
|---|---|
| **Valeur de `λ_cut`** | la frontière W/δ · la **largeur d'éponge** `L_s = λ_cut/2` (ADR-005 §5) · le **coût minimal d'un domaine** · le périmètre de W |
| **Technologie de W** | tout le gameplay répliqué (ADR-009) · la faisabilité du déterminisme D1 · le coût réseau d'une arrivée en cours de partie |
| **Capacité maximale en paquets** | `paquets_W_max` du profil (ADR-012 §3) · le seuil d'élagage (ADR-021 §4) |

`λ_cut` est le paramètre le plus connecté du corpus. Deux conséquences s'y sont ajoutées depuis que
le banc a été écrit, et **les deux sont des critères de recevabilité**, pas des métriques :
voir §6.

---

## 2. Les candidats, et ce qu'on sait déjà sans mesurer

Trois candidats (ADR-007 §5.2) : **paquets d'ondes lagrangiens**, **champ de hauteur 2D intégré**
sur pyramide GPU, **Boussinesq faible dispersion**.

Le corpus a déjà tranché une partie de la comparaison, et il faut le savoir avant de monter le banc
— sans quoi la mesure redécouvrira à grands frais ce qui est écrit.

> **SPEC-004 §5.1.** `advance(t)` doit être une **fonction pure du journal d'événements et de `t`**.
> Les paquets lagrangiens le sont par construction : chaque paquet est entièrement déterminé par son
> événement source et le temps écoulé. Un champ 2D intégré ne l'est pas : son état à `t` dépend de
> tout l'historique d'intégration.

Ce n'est pas un départage, c'est un **coût caché à mesurer** : un champ 2D exige des points de
reprise à stocker, répliquer et transmettre à tout joueur qui rejoint — précisément le coût réseau
qu'ADR-009 avait supprimé, et que `PLAN-BENCHMARK` B2 porte depuis l'ajout S04.

Trois propriétés déjà acquises, que le protocole doit **vérifier et non redécouvrir** :

| Propriété | Ce qu'elle implique pour le candidat |
|---|---|
| Déterminisme **D1** de W répliqué (I-03) | hash identique entre plateformes ; un candidat qui ne l'atteint pas est éliminé, quel que soit son coût |
| Arrivée en cours de partie « en un aller-retour » (ADR-003 §3) | l'état transmis se réduit à `T_sim` + descripteurs + journal ; tout point de reprise s'y ajoute et se mesure |
| Élagage restreint (ADR-021 §4) | un paquet `W_rep` au-dessus du seuil de pertinence n'est **jamais** élagué : la capacité maximale est une contrainte, pas un réglage |

---

## 3. `λ_cut` est déjà encadré par le corpus, et l'encadrement est serré

Personne n'avait rapproché les deux contraintes. Elles se calculent sans aucune mesure.

### 3.1 Borne haute — l'éponge

ADR-005 §5 : `L_s = λ_cut/2` par face. Sur un domaine de largeur transverse `W`, l'intérieur utile
vaut `W − λ_cut`. Appliqué aux trois domaines de référence (SPEC-001 §2.3, §2.4) :

| Domaine | Emprise | `λ_cut` = 3 m | 4 m | 6 m | 10 m |
|---|---|---|---|---|---|
| **Impact** | 6 × 6 m | intérieur 50 % | **33 %** | **0 %** | — |
| Bateau | 24 × 12 m | 75 % | 67 % | 50 % | 17 % |
| Déferlement | 120 × 20 m | 85 % | 80 % | 70 % | 50 % |

En **surface au sol**, c'est le carré : à `λ_cut = 4 m`, un domaine d'impact est **89 % d'éponge**.
La borne est dictée par le **plus petit** domaine, et elle est serrée : `λ_cut ≤ 3 m` pour qu'un
domaine d'impact garde la moitié de son emprise. À 6 m il n'a plus d'intérieur du tout.

### 3.2 Borne basse — l'échantillonnage du champ de fond

Écart E08 (revue S08) : le fond est échantillonné une cellule sur quatre puis interpolé, et la
validité tient au rapport `λ_cut/dx`.

| `dx` | `λ_cut` = 3 m | 4 m | 6 m | 10 m |
|---|---|---|---|---|
| **0,25 m** (déferlement) | ratio 12 → **3 pts** après décimation | 16 → 4 pts | 24 → 6 pts | 40 → 10 pts |
| 0,10 m (bateau) | 30 → 7,5 pts | 40 → 10 pts | 60 → 15 pts | 100 → 25 pts |
| 0,05 m (impact) | 60 → 15 pts | 80 → 20 pts | 120 → 30 pts | 200 → 50 pts |

### 3.1 bis — La borne haute est ~~ROUVERTE~~ **REFERMÉE, et resserrée** *(B-S26, reporté en
S35 ; **rétracté en S39**)*

> **Note S39 — ce qui suit est rétracté.** La réouverture ci-dessous reposait sur une mesure faite
> en eau **non dispersive**, et sa propre réserve n° 1 disait de ne pas s'en servir avant de mesurer
> en dispersif. Ça a été fait : `R` vaut **22,7 %** à `L_s = λ/2` et il faut **`L_s ≥ 2λ`** pour un
> critère à 1 %. **La borne haute de `λ_cut` est refermée, et deux à quatre fois plus serrée
> qu'avant — l'éponge coûte plus cher, pas moins.** Voir
> [`ADR-046`](../adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md).
>
> *Le paragraphe est conservé tel quel : il dit ce qui a été cru pendant trois sessions, et
> pourquoi.*

> *~~Une **seconde voie** rouvre la même borne, indépendamment~~ — **faux, corrigé en S39** : la
> dissipation retire sa raison d'exister au **masque de décroissance** (ADR-037), pas à
> l'**absorbeur de bord**, d'où vient cette borne. `ADR-043` §5 pose la distinction que son §6
> n'a pas appliquée. Il n'y avait qu'une voie.*

Le §3.1 ci-dessus tient `L_s = λ_cut/2` pour une contrainte **dure**, et en tire `λ_cut ≤ 3 m`. Cette
largeur vient d'`ADR-005 §2`. **Elle a été mesurée en B-S26, et la mesure ne trouve pas sa trace** :
à `σ_max` proportionnel à `c/L_s`, le coefficient de réflexion ne bouge pas quand l'éponge passe
d'une longueur d'onde entière à un huitième. Ce qui la borne est `σ_max·dt < 1`, soit
**`L_s ≳ 5·dx`** — la maille et le pas de temps.

| | ADR-005 §2 | mesuré *(B-S26)* |
|---|---|---|
| domaine d'impact, `dx = 0,05 m` | `L_s = λ_cut/2 = 2 m` | `L_s ≈ 0,25 m` — **8× plus étroit** |
| intérieur utile d'un domaine de 6 m | 2 m sur 6 | **5,5 m sur 6** |

**Si cela tient hors de l'eau peu profonde, l'éponge cesse d'être ce qui plafonne `λ_cut`** — et le
§3.3 ci-dessous, « les deux bornes ne se croisent pas », perd l'une de ses deux bornes.

**Ne pas s'en servir tel quel.** Le solveur de B-S26 est non dispersif ; une éponge d'eau profonde doit
absorber une **bande** de célérités, et la règle `λ/2` protégeait peut-être exactement de cela.
**C'est le premier essai à ajouter au banc B2**, et il précède désormais tous les autres, puisqu'il
décide de la borne dont tout le reste dépend. Voir
[ADR-042](../adr/ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md) §6.

### 3.2 bis — Une troisième borne, mesurée en B-S24 : la dissipation *(reporté en S35)*

Les deux bornes ci-dessus sont géométriques : l'éponge occupe de la place, la décimation demande des
points. **Une troisième vient d'être mesurée, et elle est d'une autre nature** — elle ne dit pas
combien de mailles tiennent dans un domaine, elle dit **combien il en faut pour qu'une onde y
survive**.

Sur une seiche de 40 m de longueur d'onde, avec le solveur de B-S22-B-S24 ([ADR-040](../adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) §5), le
seuil de C03 — une demi-vie d'amplitude supérieure à 15 périodes — est franchi vers :

| Schéma | mailles par longueur d'onde | `dx` pour `λ = 40 m` |
|---|---|---|
| ordre un | **≈ 250** | 16 cm |
| ordre deux (MUSCL + RK2) | **≈ 55** | 75 cm |

**Ce chiffre borne `λ_cut` par le bas d'une façon que §3.2 ne voyait pas.** La décimation demandait
un nombre de points pour que l'*interpolation* du fond reste valide ; la dissipation en demande un
pour que l'*onde* ne s'éteigne pas. Les deux se lisent en points par longueur d'onde, mais l'une
protège une économie et **l'autre protège le résultat**.

Deux réserves, toutes deux dans le document qui produit le chiffre : le solveur est **1D**, et la
mesure est faite sur `δ`, pas sur `W`. Elle n'en fixe donc pas `λ_cut` — mais elle donne à B2 une
grandeur qu'aucune des deux bornes précédentes ne contenait, et **elle indique que l'ordre du schéma
pèse sur la maille utile plus lourd que tout le reste**.

### 3.3 Les deux bornes ne se croisent pas — et ce que cela veut dire

Le déferlement voudrait `λ_cut ≈ 10 m` pour que la décimation ×4 reste confortable ; l'impact exige
`λ_cut ≤ 3 m`. **Aucune valeur globale ne satisfait les deux**, à un facteur trois près.

Les deux contraintes ne sont pourtant pas de même nature :

- **l'éponge est une contrainte dure.** À `λ_cut ≥ 6 m`, un domaine d'impact n'a plus d'intérieur.
  Il n'y a pas de compromis, seulement un domaine inutile ;
- **la décimation est une contrainte de coût.** À quatre points par longueur d'onde on n'a pas un
  résultat faux, on a une économie qui s'effondre — ce qu'E08 disait déjà.

> **Résolution proposée au protocole : `λ_cut` reste global, c'est le taux de décimation qui
> s'adapte.** E08 avait écrit la contrainte sous la bonne forme, `dx ≤ λ_cut/N`, en laissant `N`
> libre. `N` est le paramètre, pas `λ_cut`.

| Domaine | `λ_cut/dx` à 4 m | Décimation admissible | Facteur d'économie |
|---|---|---|---|
| Impact, `dx` 0,05 | 80 | ×4 par axe | **64** |
| Bateau, `dx` 0,10 | 40 | ×4 par axe | **64** |
| Déferlement, `dx` 0,25 | 16 | ×2 par axe, au mieux | **8** |

**Conséquence directe sur le protocole.** B2 doit mesurer le coût du terme source dans une zone de
déferlement **à décimation réduite**, et non au facteur 64 nominal. Sans cela le coût de δ en régime
substitutif est sous-estimé d'un facteur voisin de huit — sur le domaine qui compte 384 000
cellules, le plus gros du corpus.

**Fenêtre restante** : `2,5 m ≤ λ_cut ≤ 3 m` — borne basse pour que le déferlement garde dix
cellules par longueur d'onde *avant* décimation, borne haute par l'éponge du domaine d'impact. La
valeur proposée depuis S01, **4 m, est au-dessus de cette fenêtre**. Ce n'est pas une réfutation :
c'est une hypothèse chiffrée que le banc doit trancher, et il doit donc **balayer 2 à 6 m**, pas
confirmer 4.

---

## 4. Les scénarios

Format de SPEC-003 §3 : fichier texte court, lisible en diff, assertions dans le scénario et non
dans le code, données référencées par empreinte de contenu, `t_sim_debut` explicite.

```toml
[scenario]
id          = "B2-01-sillage-profond"
duree_s     = 60.0
t_sim_debut = 1735689600000000
graine      = 20260905

[region]
hs = 1.2 ; tp = 6.0 ; theta = 0.0 ; u10 = 8.0
bathymetrie = "sha256:…"              # fond plat, 200 m

[[acteur]]
type = "coque" ; archetype = "vedette_12m"
trajectoire = [[0,0,0,0.0], [60,600,0,0.0]]     # 10 m/s

[w]
candidat = "paquets_lagrangiens"      # · "champ2d" · "boussinesq"
lambda_cut = 4.0                      # balayé : 2, 3, 4, 5, 6

[assertions]
angle_sillage       = { ref = "19.47", tolerance_deg = 2.0 }
lambda_transverse   = { ref = "2*pi*v^2/g", tolerance_rel = 0.05 }
derive_energie_60s  = { max = 0.02 }
hash_interplateforme= { egal = true }
```

**Cinq scénarios**, les trois de `PLAN-BENCHMARK` B2 plus deux imposés par ce qui a été écrit depuis :

| # | Scénario | Ce qu'il mesure | Référence |
|---|---|---|---|
| B2-01 | Sillage en eau profonde, 5 / 10 / 15 m/s | angle de Kelvin, `λ = 2πv²/g` | **analytique** — SPEC-001 §5 |
| B2-02 | Sillage en eau peu profonde, 5 m de fond | `arcsin(1/Fr_h)`, disparition des transverses | **analytique** — ADR-011 §4 |
| B2-03 | Anneau d'impact | dispersion, conservation d'énergie sur 60 s | analytique — SPEC-001 §1 |
| B2-04 | Houle réfractée sur bathymétrie de plage | réfraction, levée, `H/h = 0,78` | analytique — SPEC-001 §3 |
| **B2-05** | **Arrivée en cours de partie à `t` = 30 s** | volume d'état à transmettre, temps de reconstitution | ADR-003 §3 — *ajouté S16* |

B2-05 est le scénario que le corpus réclamait sans l'avoir écrit : l'ajout S04 de `PLAN-BENCHMARK`
demande de mesurer « la taille d'un point de reprise, sa fréquence, et le volume à transmettre à une
arrivée en cours de partie ». Aucun des quatre autres ne l'exerce.

---

## 5. Le protocole iso-qualité, appliqué à B2

SPEC-003 §5.2 impose de régler chaque candidat jusqu'à une **qualité cible commune**, puis seulement
de comparer les coûts. Une campagne à réglage fixe désigne le mauvais candidat, et le résultat est
d'autant plus dur à défaire qu'il est étayé par des chiffres.

**Métrique d'erreur unique retenue pour B2** : l'**erreur de célérité relative**, mesurée sur le cas
canonique C02 (dispersion monochromatique), intégrée sur la bande `[λ_cut, 4·λ_cut]`. Motif : c'est
la grandeur que C02 produit déjà, elle a une référence analytique fermée, et c'est elle qui gouverne
le défaut visible d'un candidat de W — un train de vagues qui avance trop vite ou trop lentement se
désynchronise du fond en quelques dizaines de secondes.

**Trois niveaux d'erreur cible** : 5 %, 2 %, 1 %. Pour chacun, chaque candidat est réglé — nombre de
paquets, résolution de pyramide, ordre de Boussinesq — jusqu'à l'atteindre à 5 % près, **puis** on
compare coût CPU, coût GPU, mémoire, et volume réseau de B2-05.

**Plancher de bruit d'abord** (SPEC-003 §5.3, L18) : avant toute comparaison, chaque candidat est
exécuté cinq fois à réglage identique pour établir l'écart-type de son coût. Un écart inférieur à
trois fois cet écart-type n'est pas un écart.

---

## 6. Les deux critères de recevabilité — ils précèdent les métriques

Une valeur de `λ_cut` peut être excellente en coût et **irrecevable**. Les deux critères se
vérifient avant de regarder un chiffre de performance.

**Critère 1 — fermeture de l'autorité** *(ADR-021 §3.2, ajout S05 au protocole)*. Aucun phénomène de
conséquence gameplay ne doit pouvoir naître exclusivement dans δ. Une valeur de `λ_cut` qui laisse
tomber un tel phénomène dans δ est irrecevable, quelles que soient ses qualités de coût.

**Critère 2 — validité du signal de navigation** *(SPEC-006 §5.6, ajout S15 au protocole)*. Le signal
de traversabilité ignore δ, en s'appuyant sur le même argument de fermeture. Un `λ_cut` relevé
invaliderait donc **aussi** ce signal — et le défaut se manifesterait par des PNJ qui traversent un
gué chez un joueur et se noient chez un autre.

**Comment les vérifier, concrètement.** Pour la valeur candidate, énumérer les phénomènes de
conséquence gameplay du corpus et vérifier que la longueur d'onde caractéristique de chacun dépasse
`λ_cut` :

| Phénomène | Longueur caractéristique | Source |
|---|---|---|
| Vague qui pousse un joueur | `H/h = 0,78`, déferlement côtier — dizaines de mètres | SPEC-001 §3 |
| Sillage détectable | `λ = 2πv²/g` — 16 m à 5 m/s, 64 m à 10 m/s | SPEC-001 §5 |
| Bore, front de crue | métrique, non ondulatoire — appartient à V | ADR-010 |
| Emportement par un courant | courant, non ondulatoire | SPEC-002 §5 |
| Gerbe d'impact, claque de coque | **brève et locale, < 1 m** | ADR-023 §2 |

Le plus court des phénomènes gameplay ondulatoires est le sillage à faible vitesse : **16 m à
5 m/s**. La marge est donc large jusqu'à `λ_cut ≈ 6 m`, ce qui n'est pas la contrainte qui mord —
celle qui mord est l'éponge du domaine d'impact (§3.1). **À vérifier tout de même à chaque valeur
balayée** : c'est un critère de recevabilité, pas une estimation.

---

## 7. Procédure de décision

Comment on passe des mesures à une valeur, sans discussion :

```
1. Éliminer les candidats qui n'atteignent pas D1 sur le hash inter-plateformes (I-03).
   Aucun coût ne rachète cela.

2. Pour chaque λ_cut ∈ {2, 3, 4, 5, 6} m :
     a. vérifier les deux critères de recevabilité (§6). Si l'un échoue → valeur écartée.
     b. vérifier que le domaine d'impact garde ≥ 50 % d'emprise utile (§3.1).
     c. déterminer le taux de décimation admissible par domaine (§3.3).

3. Pour chaque couple (candidat, λ_cut) recevable, régler jusqu'aux trois niveaux d'erreur
   cible (§5), puis relever : cpu_p99, gpu_p99, mémoire, volume réseau B2-05.

4. Retenir le couple de moindre coût au niveau d'erreur « nominal » (2 %), à condition que
   son coût au niveau « fin » (1 %) reste dans le budget d'ADR-012 §3.

5. Publier la valeur ET le taux de décimation par classe de domaine : les deux sortent
   ensemble du banc, et l'un sans l'autre ne veut rien dire.
```

Le point 5 est ce que ce dossier ajoute au protocole d'origine. B2 ne produit pas un nombre mais un
**couple** : `λ_cut` et le tableau des décimations admissibles.

---

## 8. Ce qui doit être vrai avant de lancer
> **Actualisation S147 — 2026-09-10.** S63-1 est close : W dispersive est construite et
> contrôlée (ADR-060, S127–S129). Le tableau S63 ci-dessous est historique. La réception
> de W ne donne pas la coupure W/δ : ADR-054 §2 distingue ces deux questions. B2 garde
> notamment la comparaison des candidats, la bathymétrie, le sillage, les budgets cibles et
> le déterminisme croisé. Voir [CLOTURE-S63-1-S147](CLOTURE-S63-1-S147.md).

> **État vérifié en S63, 2026-09-08.** *Le tableau ci-dessous datait de S14 et annonçait cinq
> blocages. **Quatre étaient levés depuis une quarantaine de sessions**, et rien ne l'avait
> signalé : un lecteur y voyait un banc hors d'atteinte alors qu'il ne manque qu'une pièce.*
> C'est le troisième mécanisme de défaillance des prescriptions recensé en S63 — non pas fausse
> à l'écriture, mais **périmée en silence** (**A185**). La colonne « état » porte désormais la
> session qui l'a constaté ; **un état sans date se lit au présent, et il ne l'est plus**.

| Préalable | Où | État | Constaté |
|---|---|---|---|
| **H1** — lecteur de scénario, hôte harnais, mode `check`, hashes | SPEC-003 §10 | **écrit** | S20 |
| **H3** — cas canoniques analytiques, mode `physics` | SPEC-003 §10 | **écrit** | S20 |
| **C01** passé | `CAS-CANONIQUES` | **passe**, sur deux véhicules | S22, S36 |
| **C02** passé | `CAS-CANONIQUES` | **inexécutable en l'état** — voir ci-dessous | S63 |
| ADR-020 acté | ADR-020 | **ACTÉE** | S19 |
| `paquets_W_max` mesuré, non supposé | ADR-012 §3 | à mesurer en B2 même | inchangé |

**Il ne manque donc qu'une chose, et ce n'est pas du travail d'exécution.** C02 n'est pas « non
exécuté » : il est **inexécutable avec les véhicules existants**, et la nuance change ce qu'une
session doit faire en le lisant. `CAS-CANONIQUES` §C02 dit qu'il « **produit `λ_cut`** » — la plus
petite longueur d'onde que le solveur δ transporte sans erreur de célérité rédhibitoire. Or les
deux δ d'essai sont **Saint-Venant, non dispersifs** : leur erreur de célérité en fonction de `λ`
n'existe pas au sens où C02 la mesure (ADR-030 §5, S22). Et le milieu à **dispersion exacte**
écrit en S39 ne la fournit pas davantage : son en-tête pose qu'il *n'est pas un solveur δ* et que
sa dispersion est exacte par construction — son erreur est nulle, donc la mesure serait vide.

**Ce qui manque est une couche dispersive du projet — `W`, ou un `δ` d'une autre famille.** C'est
une dépendance de conception, pas un retard de codage, et elle n'était pas dans le graphe d'origine.

C02 donnerait alors une **borne basse physique** mesurée, à croiser avec les deux bornes calculées
en §3.

---

## 9. Les pièges de mesure propres à B2

Les six pièges généraux de SPEC-003 §6 s'appliquent. Quatre sont propres à ce banc :

1. **Mesurer W sans δ.** B2 porte sur la couche W. Un scénario qui active un domaine δ mêle les deux
   coûts, et le candidat qui perd est celui dont le voisin est cher. Les cinq scénarios s'exécutent
   **sans aucun domaine δ**, sauf B2-04 où le déferlement l'impose — et là, le coût est ventilé.
2. **Le coût réseau n'est pas un coût de frame.** Le volume de B2-05 se compte en octets par arrivée,
   pas en millisecondes. Le mêler aux autres métriques ferait disparaître un facteur mille.
3. **Un champ 2D intégré n'a pas de coût constant.** Son état dépend de l'historique : le mesurer
   sur 60 s de scénario ne dit rien de son coût après une heure de jeu. Mesurer la **dérive** de coût
   et de mémoire sur toute la durée, pas leur moyenne.
4. **La qualité d'un candidat de W se juge sur la célérité, pas sur l'aspect.** Un champ trop
   dispersif est joli et faux ; le défaut ne se voit qu'après des dizaines de secondes, quand le
   train de vagues s'est désynchronisé du fond. C'est pour cela que la métrique d'iso-qualité est
   l'erreur de célérité (§5) et que les scénarios durent 60 s et non 10.

---

## 10. Ce qui reste ouvert

1. **La fenêtre 2,5–3 m contre la valeur proposée de 4 m** (§3.3). Le balayage 2 à 6 m tranche ;
   si la fenêtre se confirme, l'éponge du domaine d'impact devient le facteur limitant de toute
   l'architecture, et c'est une conclusion à porter à ADR-005 par une note.
2. **Le « ≈27 % » d'ADR-005 §5** n'est pas reproductible : selon les faces qui portent l'éponge, la
   même géométrie donne 20 %, 36 % ou 49 %. Le document ne dit pas lesquelles, et cette fraction est
   le coût d'entrée de tout domaine. À trancher avant B2, parce qu'elle fonde la borne haute de §3.1.
3. **Un domaine d'impact peut-il être plus grand ?** Toute la borne haute vient de son emprise de
   6 × 6 m (SPEC-001 §2.4). L'agrandir déplacerait la contrainte au prix du `dx⁻⁴` — 6 → 8 m coûte
   un facteur 2,4 en cellules. Arbitrage de coût, pas de conception, mais il appartient au banc.
4. **Le taux de décimation par classe de domaine** sort de B2 (§7, point 5) mais sa validité se
   vérifie en B4, dont c'est un paramètre direct (SPEC-004 §10.3). Les deux bancs partagent ce
   paramètre et doivent partager sa valeur.
