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

Session : S301 — en cours.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, outils locaux, carte réelle.
Entrée : « Reprends le projet », 2026-09-19. Copie unique, master propre, jeton libre à l'amorce.

Capacité visée : le **pas couplé complet** résident sur la carte, sur **un seul device** —
prédiction (advection MAC, couplage à B, éponge), divergence, second membre couplé (S300),
projection bornée (S299) à départ chaud, correction, extrapolation, transport par débits
mouillés et bandes, relaxation de l'éponge, surface publiée (ADR-175 D7). Aujourd'hui les trois
étages de S299–S300 vivent chacun sur leur propre device et ne peuvent pas s'enchaîner.
Consommateur : critère 2 de la porte B (production contre référence), puis la scène (critère 3).

Critères avant code, posés ici :
- Un device, tampons réservés à la configuration (I-06). Par pas, le CPU écrit les phases
  (`O(composantes)`) et des uniformes `O(1)`, et enregistre un nombre de dispatchs **fixé par le
  profil** et publié ; aucune boucle CPU sur les mailles, aucune lecture dans le pas.
- Chaque étage reçu contre le cœur, écart publié champ par champ, aucune identité au bit
  exigée (D4). La référence ne bouge pas, sauf une fonction d'**essai** de prédiction sur le
  modèle de S299/S300, que la production n'appelle jamais.
- Réception du pas : cas S298 — impulsion de 18 cm sous B spectral résolu, 32×24×36, `dx`
  0,25 m, `dt` 5 ms, éponge 1 m à 2 s⁻¹. Écart de hauteur carte–cœur publié à chaque image ;
  **reçu si < 3 mm** (S201, ADR-175 §4.2) sur la durée déclarée, visée 6 s. Pente et phase
  publiées. Un dépassement se publie tel quel, cause cherchée ; aucun seuil relevé.
- **Affinage** : ADR-175 D3 tranche déjà — la production ne refait jamais un pas. Si la mesure
  montre qu'un affinage manque, il entre comme **quantité fixe du profil** (D2), pas comme reprise
  conditionnelle. Pas d'ADR tant que la mesure ne le demande pas.
- Hors lot : scène et revue (critère 3), coût au 99ᵉ centile (porte C), A286, multiplateforme.

### Plan

- [x] **P1** — amorce, lecture ciblée du lot, plan seul. *(Committé avec `[>]` par erreur ;
  coché au commit de P2.)*
- [x] **P2** — `Step3` : un device, tampons réservés, pipelines des trois sources WGSL (fond,
  gradient conjugué, pas) ; dépôt d'état et relecture de banc. Noyau `predict` (advection MAC,
  couplage `extra3`, éponge) et `predict_for_trials` dans le cœur ; reçu contre lui.
- [x] **P3a** — divergence, couplage S300 avec `eta_roundoff`, passage à la projection, départ
  chaud ; pression reçue contre le cœur sur un pas.
- [ ] **P3b** — correction aux faces et extrapolation verticale ; vitesses reçues.
- [ ] **P4** — transport par débits mouillés et bandes, relaxation d'éponge, compensation,
  surface publiée (D7) ; pas complet reçu champ par champ contre `step_perturbation_mobile`.
- [ ] **P5** — trajectoire du cas S298 : écart de hauteur par image, selon les cycles.
- [ ] **P6** — coût du pas complet, passe chronométrée, trois tailles ; dispatchs publiés.
- [ ] **P7** — diagnostics D3 sur la carte (divergence des lignes franches, dérive de masse,
  colonnes hors bornes), relus en différé avec leur âge.
- [ ] **P8** — preuve et rituel REPRISE §6.

### Notes de reprise

**Architecture retenue (P2).** `Step3` crée **un** device et y compile les trois sources telles
quelles — `delta3d_background.wgsl` (S300), `delta3d_cg.wgsl` (S299), `delta3d_step.wgsl`
(nouveau) — chacune avec sa propre disposition de liaisons, sur des tampons communs. Rien de
S299/S300 n'est réécrit (L137). Rangements : `vel = [u|v|w courants | u|v|w prédits]`, l'indice
d'une face étant aussi celui de son échantillon de fond ; `cells_in = [eta | divergence |
eta_roundoff]` — **eta vit là**, c'est l'entrée de `couple_columns` ; `cells_out` = celui de S300 ;
`work = [flux_x | bande_x | flux_y | bande_y | surface publiée]`. Les passages vers la projection
(surface totale → `heights`, second membre → tranche B, préconditionneur → tranche M) seront des
copies de tampon **dans l'encodeur**, sans retour CPU.

**P2 reçu** (`--delta3d-prediction`, 15×11×14, 7 459 faces, fond S300 à 64 composantes, vitesses
d'ordre 0,2 m/s, éponge (1 ; 0,75 ; 2 s⁻¹), dt 5 ms, trois instants) : pire écart **6,0·10⁻⁸ m/s**,
soit **≤ 9,0·10⁻⁶ de l'incrément** du pas, sur les trois familles ; 70 à 85 % des faces au bit ;
zéro face fautive. Refus : durée nulle, éponge trop large, `dt²g/dx > 1`, longueur.

**Défaut trouvé par le banc, corrigé avant commit** : le noyau couplait les faces `i = nx` (u) et
`j = ny` (v), que le cœur saute — borne comparée à `n + 1` au lieu de `n`. Écart de **60 % de
l'incrément**, sur ces seules faces. Le compteur `faces_fautives` (> 5 % de l'incrément) reste
dans le banc : c'est lui qui voit une règle de bord portée autrement, l'arrondi ne le peut pas.
Rapporter l'écart à l'incrément et non à la vitesse était nécessaire : rapporté à la vitesse, le
même défaut ne pesait que 1 %.

**Incident d'outillage, corrigé par un commit séparé** : le battement de P2 a été écrit par
`Get-Content -Raw | Set-Content -Encoding utf8` de Windows PowerShell 5.1, qui **lit en ANSI** :
`REPRISE.md` est parti ré-encodé (mojibake) dans `fff03d5`. Restauré depuis `7ebeeb5`, battement
réécrit à l'outil d'édition. **Ne jamais réécrire un fichier du dépôt par `Get-Content` /
`Set-Content`** ; `[IO.File]::ReadAllText/WriteAllText` (UTF-8 par défaut) ou l'outil d'édition.

**P3a reçu** (`--delta3d-pression`, même fixture, 1 510 mailles mouillées sur 2 310, départ
`p = 0` des deux côtés). Le cœur converge en 65 et 68 itérations, sans affinage. La carte :

| cycles | dispatchs | écart p (t=0) | relatif | écart p (t=1,23 s) | relatif |
|---|---|---|---|---|---|
| 8 | 51 | 2 757 Pa | 0,17 | 2 992 Pa | 0,18 |
| 32 | 171 | 218 Pa | 1,3·10⁻² | 196 Pa | 1,2·10⁻² |
| 64 | 331 | **0,27 Pa** | 1,6·10⁻⁵ | **0,20 Pa** | 1,2·10⁻⁵ |
| 128 | 651 | 0,38 Pa | 2,3·10⁻⁵ | 0,077 Pa | 4,7·10⁻⁶ |

Échelle 16 400 Pa. Plateau dès 128 cycles : plancher f32. En usage : 0,27 Pa ≈ **0,03 mm d'eau**.
Aucune maille sèche non nulle. Résidu vrai de la carte 3,6·10⁻⁷ à 64 cycles, celui du cœur
2,3·10⁻⁷. À 8 cycles l'écart vaut 27 cm d'eau **depuis p = 0** : en trajectoire le départ chaud
part de la pression du pas précédent, et c'est P5 qui dira combien de cycles il faut alors.
`couple_rhs` lit désormais `eta_roundoff` (`cells_in` après la divergence) ; banc S300 rejoué,
chiffres identiques (1,06·10⁻⁶ ; 6,0·10⁻⁸ ; 3,9·10⁻⁷). `init_warm` ajouté à `delta3d_cg.wgsl`.

---

### Notes de reprise de S300 (conservées pour le lot)

`PhaseQ32::from_time` multiplie `freq_q32` (u64) par les microsecondes en **u128** avant de
diviser par 10⁶ : intransposable en WGSL, qui n'a ni u64 ni u128. Mais cette phase ne dépend
**que** de la composante et de l'instant, pas du point : elle se calcule une fois par composante
et par pas, côté CPU, et se publie. C'est littéralement « le CPU publie les paramètres
analytiques de B/W, phases repliées » d'ADR-175 D1.

`from_distance` est du `f32` pur : `frac(k·d)` puis `× 2³²` en `u32` — portable tel quel.
`sin_cos` est un quadrant (`>> 30`), un repliement sur `π/4` et deux polynômes de Horner
(sin ordre 9, cos ordre 10) — portable tel quel, erreur ≈ 2·10⁻⁹ annoncée par le cœur.

`viewer/src/water.wgsl` évalue déjà B pour le **rendu**, mais par le chemin CWM/bandes
(`band_cwm`, `tail_cwm`) : ce n'est pas `differential_local_extended` et ça ne se réutilise pas.
`viewer/src/delta.rs` échantillonne le fond de la bande 2D **sur CPU** puis téléverse : c'est
exactement ce que ce lot remplace.

**P2 : trouvaille à ne pas reperdre.** Le produit `k·d` est identique au bit sur la carte
(576/576), mais le `x − floor(x)` **compilé** en diverge d'un ulp (72/576 seulement au bit).
Près de la borne d'I-08, `k·d` vaut des milliers de tours : il ne reste qu'une poignée de bits à
la fraction, et un ulp y pèse **10⁻³ de tour** — sin/cos à 3,2·10⁻³ et 5,6·10⁻³ d'écart.
**La fraction se prend en entier** : `x = mantisse·2^(e−23)` donc `x·2³² = mantisse·2^(e+9)`
modulo 2³², signe par complément. Exact, et indépendant du compilateur.
Après : phases **564/576 au bit**, pire écart **128 unités** (3·10⁻⁸ tour), sin 1,94·10⁻⁷,
cos 2,09·10⁻⁷, exp 3,02·10⁻⁸ contre `exp(−x)` en f64.
Le reliquat vient du **cœur** : son `turns − floor(turns)` arrondit quand la somme n'est pas
représentable (ex. turns = −0,344, frac 0,656 perd un bit). La carte est donc plus exacte que
la référence sur ce point précis. À dire tel quel, sans le présenter comme une identité.

**Fusion déclarée P3+P4** : un seul banc (`--delta3d-champ`) couvre les deux branches, et les
séparer aurait voulu dire couper un banc en deux pour la forme. Un commit, deux cases.

P3/P4 reçus : 891 sondes (9×9 en x-y, onze hauteurs dont **cinq au-dessus** du plan moyen),
trois instants dont 9 876 s. Champ par champ, écart relatif **1,0 à 2,6·10⁻⁶**, sans champ
aberrant ; `grad_eta.z` exactement nul des deux côtés. Par point et par branche, métrique plus
sévère : au-dessus 1,2 à 4,2·10⁻⁵, au-dessous 1,5 à 1,9·10⁻⁵ — les grands rapports tombent là où
l'atténuation a tout éteint et où l'échelle du point est minuscule.
En grandeurs physiques : eta à 0,57 µm près pour une échelle de 0,64 m ; p_dyn à 1,2·10⁻² Pa
pour 12 209 Pa. Le repère de 3 mm de S201 est cinq mille fois plus large.
Hors domaine : le cœur refuse, la carte marque en NaN — un noyau ne peut pas refuser.

**FXC refuse l'indexation dynamique d'un vecteur en écriture** (`grad_u0[j] = …` dans une
boucle) : « array reference cannot be used as an l-value ». Boucles déroulées à la main. À
retenir pour tout noyau à écrire : dérouler dès qu'une composante de vecteur est une cible.

**A286 levée pour ce lot** : les fantômes couplés de `delta3d_coupling.rs` ne consomment que
`eta`, `p_dyn` et `grad_p_dyn[axis]` des échantillons de face — rien qui soit propre à W.
La carte les calcule déjà tous. Quand W entrera dans le fond publié, il passera par le même
chemin sans le modifier. A286 reste ouverte sur le **prolongement de W**, pas sur ce lot.

P5a reçu (`--delta3d-faces`, 17×11×13, origine non alignée, 7 844 faces, trois instants) :
pire écart absolu sur `p_dyn`, 1,46 à 2,44·10⁻³ Pa pour des valeurs de 129 à 5 223 Pa — soit
4,7·10⁻⁷ à 3,3·10⁻⁶ en relatif, du même ordre que le champ ponctuel de P3/P4.
Les trois familles se comportent pareil ; refus de capacité et de domaine vide obtenus.
`sample_faces` retrouve l'axe par soustractions successives : un seul dispatch couvre les
trois familles, et leur ordre u, v, w est celui du cœur.

**P5b, ce qui reste à porter** (`delta3d_coupling.rs` l. 60-125) : `surface_total = eta + eta_fond`,
`ghost_bg_up[c] = rho·g·s.eta − (s.p_dyn + dz·s.grad_p_dyn.z)` à la face au-dessus de la
dernière maille mouillée, et `ghost_bg_{x,y}[f] = −(s.p_dyn + signe·(θ−0,5)·dx·s.grad_p_dyn[axe])`
sur les faces où la mouillure change. La constance verticale de `eta` que le cœur vérifie est
automatique ici : la carte évalue `eta` sans dépendance en z.

P5b reçu (`--delta3d-couplage`, 15×11×14, 1 510 mailles mouillées sur 2 310, aucune colonne
pleine — donc les fantômes latéraux sont réellement exercés) : second membre à 6,0·10⁻⁸,
1,1·10⁻⁶ et 3,9·10⁻⁷ en relatif sur trois instants ; préconditionneur du même ordre.
Fantôme du haut de 211 à 389 Pa : le fond entre vraiment dans le second membre.

**Première fixture rejetée, et pourquoi.** À `rest = (nz−4)·dx` avec une perturbation de
±1,4·dx, la hauteur totale franchissait la borne de `check_edges3` (`2·dx ≤ h ≤ (nz−1)·dx`) :
le cœur refusait `Domain` au temps lointain, et à t=0 l'écart montait à 1,3·10⁻⁵. Fixture
rentrée dans le domaine (`rest = (nz−5)·dx`, ±0,9·dx) : tout redevient régulier.

**Instrument à garder** : `mailles_franchement_divergentes` compte les mailles dont le second
membre s'écarte de plus de 10⁻³ de l'échelle. Un arrondi ne peut pas produire ça ; une
**mouillure classée autrement** des deux côtés, si. Ici **zéro** aux trois instants, mais le
risque existe dès qu'une surface totale passe à un ulp d'un centre de maille : garder ce
compteur dans tout banc qui compare des géométries, il est le seul à voir la bascule.

P6 mesuré (`--delta3d-cout-fond`, 64 composantes, 20 passages, premier écarté) :
24×24×16 — carte 0,099520 ms contre CPU 27,6712 ms, **×278** ;
32³ — 0,179552 contre 92,8762 ms, **×517** ;
48×48×24 — 0,273856 contre 148,044 ms, **×541**.
Et la passe de la carte porte **faces + couplage**, là où le CPU ne porte que les faces :
le rapport est donc une borne basse.

Ce que ça tranche : échantillonner le fond sur CPU à l'échelle 3D coûte **93 à 148 ms par pas**.
Ce n'est pas lent, c'est impossible — le budget entier de δ est de 2 ms (ADR-174 D3). Le choix
d'ADR-175 D1 de publier les paramètres plutôt que les échantillons n'était pas une préférence
d'architecture : c'était la seule voie, et on a maintenant le chiffre qui le dit.
Avec la projection de S299 (0,31 ms à 32³, 32 cycles), les deux postes font ≈ 0,5 ms — mais
le pas reste incomplet, donc aucun budget n'est reçu.
Temps muraux, tâches concurrentes, alimentation non relevée : A270 due sur ce banc.
