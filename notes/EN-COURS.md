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

Session : S300 — terminée : fond B et second membre couplé reçus sur la carte.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, outils locaux, carte réelle.
Entrée : « Continue, et dis-moi quand pour le solveur 3d », 2026-09-19.

Capacité visée : la carte **évalue elle-même** le fond différentiel de B. Le CPU ne publie que
les paramètres analytiques par composante — phase temporelle repliée comprise (I-08) — soit un
travail en `O(composantes)`, jamais en `O(mailles)` (ADR-175 D1). C'est le blocage nommé par
S299 : le second membre couplé en dépend, et sans lui le pas de production reste non couplé.
Consommateur : le second membre couplé, puis le pas complet d'ADR-175 D1.

Critères avant code, posés ici :
- Aucune boucle CPU sur les mailles par pas ; seul le tableau des composantes est téléversé.
- La phase **temporelle** se calcule sur CPU — `from_time` passe par 128 bits, absent de WGSL —
  et se publie repliée. La phase **spatiale** et le `sin_cos` Q32 vivent sur la carte.
- Écart à `differential_local_extended` publié **champ par champ**, sous et au-dessus du plan
  moyen. Aucune identité au bit exigée (ADR-175 D4) ; l'écart attendu est de l'ordre de l'ulp,
  et tout écart plus grand se publie tel quel au lieu d'être arrondi dans un résumé.
- Bornes d'I-08 respectées (`|d| < 4096 m`) ; refus contrôlés des deux côtés.
- Portée : **B seul**. W local au-dessus du plan moyen reste A286, et ce lot ne le prétend pas.
- La référence CPU ne bouge pas : elle est l'instrument.

### Plan

- [>] **P1** — amorce, lecture ciblée du lot, plan seul.
- [ ] **P2** — publication des composantes de B sur la carte et WGSL de base : phase spatiale,
  `sin_cos` Q32, tampons réservés à la configuration.
- [x] **P3** — `differential_local_extended` en WGSL **sous** le plan moyen, reçu contre le
  cœur champ par champ ; refus et réserve testés.
- [x] **P4** — **au-dessus** du plan moyen, règle d'ADR-154, reçu contre le cœur.
- [x] **P5a** — **découpage déclaré** : les trois familles de faces MAC échantillonnées sur
  la carte, reçues contre `BackgroundGrid3` du cœur.
- [x] **P5b** — fantômes couplés et second membre assemblés sur la carte depuis ces faces.
- [x] **P6** — coût du fond sur la carte, comparé au chemin CPU de S276/S298.
- [x] **P7** — preuve et rituel REPRISE §6 : file, feuille de route, index, journal, jeton libre.

### Notes de reprise

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
