# Journal des sessions — S300 à S399 (archive)

*Archivé en S480, 2026-10-04, depuis [`notes/JOURNAL.md`](../JOURNAL.md) ; texte inchangé (les liens relatifs recalés d'un niveau), 100 entrées.* Le journal vivant
garde les entrées depuis S470 ; une archive ne se modifie plus.

---

## S300 — 2026-09-19 — porte B : le fond passe sur la carte, et le couplage avec

**Entrée.** « Continue, et dis-moi quand pour le solveur 3d ». Copie unique, master propre, plan
committé avant tout code. Agent Claude Opus 5, application desktop, carte réelle.

**Capacité reçue.** La carte évalue **B elle-même**. L'hôte ne publie que les paramètres
analytiques — amplitude, nombre d'onde, direction, pulsation, et une phase temporelle repliée par
instant — soit `O(composantes)` par pas et jamais `O(mailles)`. La carte en tire les vingt-six
champs du fond, les trois familles de faces MAC, la surface totale, les fantômes de fond et le
**second membre couplé**. **Ce qui devient possible** : le pas de production de S299 cesse d'être
non couplé, et le fond cesse d'être un poste CPU. **Consommateurs** : cinq bancs de ce lot, puis
le pas complet d'ADR-175 D1. **Preuve** : [DELTA3D-FOND-GPU-S300](../../docs/validation/DELTA3D-FOND-GPU-S300.md).

**Mesures.** Champ complet, 891 sondes, onze hauteurs dont cinq au-dessus du plan moyen, trois
instants : écart relatif **1,0 à 2,6·10⁻⁶** sur les vingt-six champs, sans champ aberrant —
`eta` juste à **0,57 µm** pour une échelle de 64 cm, quand le repère de S201 vaut 3 mm. Faces MAC :
même ordre, pire écart sur `p_dyn` à 2,44·10⁻³ Pa pour 5 223 Pa. Second membre couplé, sur un
domaine à 1 510 mailles mouillées sur 2 310 et aucune colonne pleine : **6,0·10⁻⁸ à 1,1·10⁻⁶**.
Suite complète du cœur : **539 réussis**, 18 ignorés, aucun échec.

**Le coût, et ce qu'il tranche.** Faces et couplage sur la carte contre le seul échantillonnage
sur CPU : ×278 à 24×24×16, ×517 à 32³, **×541** à 48×48×24 — et le rapport est une borne basse,
la passe de la carte portant le couplage en plus. Autrement dit : échantillonner le fond sur CPU
à l'échelle 3D coûte **93 à 148 ms par pas** pour un budget total de 2 ms. Publier les paramètres
plutôt que les échantillons n'était pas une préférence d'architecture ; on a maintenant le
chiffre qui le dit, et il manquait depuis ADR-175.

**Ce que le banc a trouvé et que je n'aurais pas deviné.** Porter la même formule ne porte pas les
mêmes bits : `k·d` est identique au bit 576 fois sur 576, mais le `x − floor(x)` **compilé** en
diverge d'un ulp, 72 fois sur 576. Près de la borne d'I-08 il ne reste qu'une poignée de bits à
la fraction et cet ulp pèse 10⁻³ de tour — 3·10⁻³ sur le sinus. La fraction prise en **entier**
ramène les phases de 73 à 564 concordances au bit et l'écart de 4 145 152 unités à 128. Leçon
**L345** écrite là-dessus ; elle complète L332. Le reliquat vient du **cœur**, qui arrondit là où
l'entier est exact : la carte est plus juste que la référence sur ce point, et c'est dit tel quel.

**Une fixture rejetée, et pourquoi.** Le second membre couplé montrait 1,3·10⁻⁵ à `t = 0` et le
cœur refusait `Domain` au temps lointain : la hauteur totale franchissait la borne de
`check_edges3`. Fixture rentrée dans son domaine, tout redevient régulier. Le banc était sorti du
domaine de validité de ce qu'il mesurait ; le port n'était pas en cause. Le compteur
`mailles_franchement_divergentes` — qui seul verrait une **mouillure classée autrement** des deux
côtés — vaut zéro aux trois instants, et reste dans le banc.

**Partiel et suite.** Seuls B et le couplage sont sur la carte. Restent l'advection, les bandes,
l'éponge, la surface mobile et sa surface publiée (D7), les diagnostics D3 — puis la scène et la
revue. `eta_roundoff` vaut zéro tant que la surface n'avance pas : sa compensation se portera avec
l'avance. **A286 est sans effet sur ce chemin**, vérifié et non supposé : les fantômes couplés ne
consomment que `eta`, `p_dyn` et `grad_p_dyn`, que la carte calcule ; A286 reste ouverte sur le
prolongement de W lui-même. A98 entière, A270 due sur ces bancs. Aucun budget reçu, aucun retrait
d'ambition, aucun arbitrage nouveau.

**Rituel.** Maillons **0** : le critère 3 de la porte B exige une production, et son entrée réelle
existe désormais, reçue contre la référence. Sixième session consécutive sur la porte B, autorisée
par A211 puisqu'un critère avance à chaque fois. **L345** nouvelle ; pas d'angle mort nouveau.
File active relue, deux lignes remplacées dont A286 ; feuille de route et index actualisés, la
section J2 ramenée sous son plafond. I-04/I-06/I-08/I-13/I-15/I-17 inchangés. Navigation et
plafonds vérifiés à 0. Copie unique, jeton libre.

*Tenue du plan* : P3 et P4 ont été fusionnés en un commit, et P5 découpé en P5a et P5b — les deux
fois déclaré dans EN-COURS avant de committer. Un battement a été écrit 20:08 pour une horloge lue
20:07 ; corrigé au rituel, sans conséquence de concurrence, une seule copie ayant travaillé.

## S301 — 2026-09-19 — porte B : le pas de production est complet sur la carte

**Entrée.** « Reprends le projet ». Copie unique, master propre, jeton libre ; plan committé avant
tout code. Agent Claude Opus 5, application desktop, carte réelle.

**Capacité reçue.** Le **pas couplé entier** tourne sur la carte, sur un seul device : fond évalué
sur la carte, prédiction, second membre couplé, projection bornée à départ chaud, correction,
extrapolation, transport et bandes, éponge, **surface publiée** dans un tampon à part (D7) et
diagnostics D3 relus en différé. S299 et S300 vivaient chacun sur leur device et ne pouvaient pas
s'enchaîner ; `Step3` compile leurs WGSL tels quels à côté du nouveau. **Ce qui devient possible** :
rendre δ 3D dans une scène. **Consommateur** : la scène du critère 3. **Preuve** :
[DELTA3D-PAS-GPU-S301](../../docs/validation/DELTA3D-PAS-GPU-S301.md).

**Mesures.** Chaque étage reçu contre le cœur : prédiction à 9·10⁻⁶ de l'incrément, pression à
0,27 Pa sur 16 400 (64 cycles), vitesses à 2,6·10⁻⁵ de l'incrément, hauteur à un ulp et hauteur
vraie à 3–4·10⁻⁸ m. Pas entier : **0,84 ms à 64 cycles** sur le cas S298 (27 648 mailles),
64×64×32 sous 2 ms jusqu'à 32 cycles — médianes de banc, secteur relevé, pas la porte C.

**Deux défauts trouvés par les bancs, corrigés avant commit.** Faces `i = nx`, `j = ny` couplées à
tort (60 % de l'incrément, 1 % de la vitesse : rapporter à l'incrément était nécessaire). Et la
**somme compensée annulée par le compilateur** : `(η + inc) − η` rendait `inc` au bit sur 165
colonnes, inexact sur CPU pour 165. Remède en entiers sur les bits IEEE ; leçon **L346**.

**Ce que la trajectoire a dit.** Sur le cas S298 la carte suit le cœur à quelques micromètres
pendant une seconde, puis l'écart saute au centimètre **pour tous les nombres de cycles**. Sonde :
une maille sèche d'un côté, mouillée de l'autre, la surface à 10⁻⁶ m de son centre. Contre-épreuve
décisive : **deux cœurs partis à ±10⁻⁶ m divergent de la même façon** (4,5 cm en 6 s, premier
millimètre à 1,3 s). C'est la référence qui est discontinue — la face de la couche partiellement
mouillée bascule de projetée à extrapolée — et non la production. **A297** ouverte, sévérité 2.
Le critère 2 se lit sur l'horizon de prévisibilité mesuré ; au-delà, la carte est dans
l'enveloppe de la référence contre elle-même. Une fausse piste, notée : une sonde décalée d'un pas
avait accusé B, que `--delta3d-fond-s298` a disculpé (9·10⁻⁸ m).

**Partiel et suite.** Ni scène ni revue ; les cas de cuve de §4.1 ne sont pas rejoués sur la
production (`Step3` exige un fond). À 16 cycles, 84 % des pas sont « dégradés » au sens d'ADR-144
alors que la hauteur suit : constat remonté à la porte C, seuil non relevé. A286 et A98 inchangées.
Aucun arbitrage de l'utilisateur requis : l'affinage du cœur ne se porte pas, ADR-175 D3 l'avait
déjà tranché — ce n'était pas un ADR à écrire.

**Rituel.** Maillons **0** : le pas de production, entrée du critère 3, est reçu. Huitième session
sur la porte B, un critère avançant à chaque fois (A211). File active relue, quatre lignes
remplacées dont A297 ajoutée ; feuille de route J2 et §3 bis, index. I-01/I-06/I-08/I-13/I-17
relus, inchangés. Plafonds et navigation à 0. Recommandation du bilan S293 portée : porte B d'abord.

*Tenue du plan* : P1 committé avec `[>]` (coché au commit suivant) ; P5, P6 et P7 fusionnés,
déclaré ; `REPRISE.md` ré-encodé par `Set-Content` de PowerShell 5.1 dans le commit de P2, restauré
par un commit séparé.

## S302 — 2026-09-20 — porte B : la scène existe, et elle est soumise au jugement

**Entrée.** « Continue avec la scène pour que je la juge ». Copie unique, master propre, jeton
libre ; plan committé avant tout code. Agent Claude Opus 5, application desktop, carte réelle.

**Capacité reçue.** Le domaine δ 3D **vit dans l'afficheur** : il naît sur le device du rendu, avance
d'un pas par image, et le nuanceur lie **sa seule surface publiée** (D7, I-13) qu'il ajoute à la
somme des couches. **Ce qui devient possible** : le critère 3 de la porte B — une onde qui traverse
une mer étalée, jugée par l'utilisateur. **Consommateur** : la revue R11, demandée.
**Preuve** : [SCENE-DELTA3D-S302](../../docs/validation/SCENE-DELTA3D-S302.md).

**La scène.** Mer `--houle` (64 composantes, `Hs` ≈ 2,5 m, la plus courte à 3,5 m, donc portée par
la maille de 25 cm). Domaine de 30 × 28 m sur 7 m, éponge de 3 m, 32 cycles, un pas de 16,667 ms
par image. Onde : un **front linéaire injecté** — 65 cm, 16 m, crête longue de 12 m, hauteur **et**
vitesses de la théorie linéaire, donc il se propage au lieu de se scinder. Mesuré avant tout
rendu : **aucune colonne hors bornes** en 13 s, traversée à **2,2 m/s** (théorie 2,5), amplitude
0,65 → 0,46 → 0,16 m à l'éponge de sortie, 4,62 ms par pas, **197 Hz** dans la fenêtre.

**Deux allers-retours qui ont coûté et qui servent.** À 24 × 32 m avec une onde de 25 cm, les
images avec et sans δ se ressemblaient : l'onde se noyait dans une mer de 2,5 m. En passant à
32 × 32 m, le device a **refusé** : le tampon des faces du fond (26 flottants par face) franchit
les 128 Mio d'une liaison de stockage — ce qui plafonne le domaine à 1,15 million de faces et
désigne une optimisation précise. Retenu : 30 × 28 m et un front de 65 cm.

**Les à-coups d'A297, chiffrés avant la revue.** Sur la scène, la dérivée seconde temporelle par
colonne montre des à-coups **locaux** — au pire, la colonne vaut 7,4 fois la moyenne de ses huit
voisines — et un grain à l'échelle de la maille de 2,0 à 2,2 mm d'écart-type, un cinquième de la
signature de l'onde. **Le balayage de 32 à 512 cycles ne le change pas** : ce n'est pas une pression
sous-convergée, c'est le schéma. Le dire dans la demande de revue plutôt que le corriger en silence.

**Témoin.** Sans domaine δ 3D, les sept images de la revue R9 gardent leurs empreintes **au bit**.
Le rendu existant n'a pas bougé.

**Partiel et suite.** Aucun verdict n'est déduit des images ; R11 pose quatre questions explicites —
R10 avait échoué faute de dire ce qu'on attendait. Restent : le critère 2 sur les cas de cuve
(`Step3` exige un fond), la porte C (4,62 ms contre 2), la charge utile par face, A289 et la dérive
de volume relevée. A297 mesurée, non corrigée.

**Rituel.** Maillons **0** : la scène est l'entrée du critère 3, et elle tourne. Neuvième session
sur la porte B, un critère avançant à chaque fois (A211). File active relue, quatre lignes
remplacées ; feuille de route J2 et §3 bis, index, REVUE-VISUELLE §16. I-01/I-04/I-06/I-13/I-17
relus, inchangés. Plafonds et navigation à 0. Copie unique, jeton libre.

*Tenue du plan* : un battement a été écrit sans lire l'horloge (corrigé au commit suivant, L237) ;
une fenêtre interactive s'est ouverte par mégarde après un échec de compilation — l'ancien binaire
ignorait le drapeau neuf.

## S303 — 2026-09-20 — verdict R11 : la mer n'a aucune asymétrie, et c'est mesuré

**Entrée.** Verdict R11 de l'utilisateur : δ sans artefact visible, raccord du domaine invisible,
mais « la mer ne fait pas réaliste […] la topologie est à revoir […] les micro vaguelettes ou pics
doivent être convexes plutôt que concaves », avec consigne de rechercher par moi-même. Copie
unique, master propre, jeton libre ; plan committé avant tout travail.

**Capacité reçue.** On sait maintenant **de quoi la forme de notre mer s'écarte, et de combien**.
Recherche sourcée sur l'anatomie d'une mer profonde, puis mesure sur la réalisation même du rendu,
puis décision. **Ce qui devient possible** : construire les asymétries au lieu de retoucher à vue.
**Consommateur** : ADR-176, puis la revue R12. **Preuve** :
[ANATOMIE-SURFACE-S303](../../docs/validation/ANATOMIE-SURFACE-S303.md).

**Ce que la recherche donne.** L'asymétrie verticale d'une mer profonde vient des harmoniques
liées du second ordre — crêtes pointues, creux plats ; en bande étroite `Sk = 3k̄σ`
(Longuet-Higgins 1963, Tayfun 1980), soit **0,156** pour notre mer. Les pentes se mesurent au
miroitement depuis Cox & Munk (1954), révisées par 150 millions d'observations IASI. Et surtout :
les **rides ne sont pas uniformes** — elles se raccourcissent et se redressent sur les crêtes des
vagues longues, s'aplatissent dans les creux (JFM 2024 : pente modulée de 20 % à `ε_L` = 0,1,
doublée à 0,4), avec un retard qui place leur maximum **en avant** de la crête.

**Ce que la mesure dit.** Trois choses, dont une qui m'incombe.

1. **Faute de protocole** : la scène soumise à R11 tournait sous `--houle` **seule**, sans la queue
   d'équilibre ni les vagues pointues construites en S260–S261. L'utilisateur a jugé la plus pauvre
   des trois mers du dépôt — celle que S260 avait déjà mesurée « pentes quasi gaussiennes ». Toute
   demande de revue déclarera désormais les options actives (ADR-176 D5).
2. **Le fond du retour tient quand même** : même la meilleure variante a `Sk` = 0,003 contre 0,156
   et `c₀₃` = 0,001 contre −0,222. Crêtes et creux également arrondis : « concave » est le mot juste.
3. **Le second ordre par composante ne peut pas y répondre** (0,0022 mesuré) : il est quartique en
   amplitude. Ce sont les **termes croisés** entre composantes qui portent l'asymétrie.

**Décision : [ADR-176](../../docs/adr/ADR-176-asymetries-de-la-surface-rendue.md).** Second ordre en
bande étroite (Tayfun) **par système**, qui contient tous les termes croisés pour une somme de
plus ; et modulation de la queue **retardée** de −0,20 tour, rides maximales en avant de la crête.
Mesuré ensemble : `c₀₃` −0,155 (70 % de l'observé), `c₂₁` −0,057 contre −0,058 **sans avoir été
visé**, `c₄₀` 0,34 dans les barres, `Sk` 0,066 — vingt fois mieux, deux fois moins que la bande
étroite. Le retard est le seul paramètre libre, calé sur `c₀₃` et dit comme tel ; son signe, lui,
est imposé par le mécanisme. Tout cela vit dans le **rendu**, pas dans B : les requêtes de jeu ne
bougent pas, l'écart s'ajoute à celui d'A288 (6 cm contre 0,365 m pour CWM).

**Partiel et suite.** Rien n'est construit : ADR-176 porte sa réception écrite avant le code.
Restent nommés : le noyau exact du second ordre pour deux systèmes (la vérité est entre 0,063 et
0,150), les capillaires parasites, l'asymétrie horizontale, la `mss` 14 % haute. Et la lisibilité
de la scène : l'utilisateur ne pouvait pas dire si l'onde était circulaire ou linéaire — elle est
linéaire, et une onde d'impact serait plus parlante.

**Rituel.** Maillons **0** : la capacité est un écart mesuré et une décision qui s'ensuit, et elle
débloque un lot de construction nommé. File active relue, quatre lignes remplacées ; feuille de
route J2 et §3 bis, index, REVUE-VISUELLE (verdict R11). I-01/I-04/I-13/I-14/I-15 relus, inchangés.
Plafonds et navigation à 0. Copie unique, jeton libre.

*Tenue du plan* : un battement écrit sans lire l'horloge, corrigé dans la minute (L237, deuxième
fois en deux sessions — le geste doit précéder l'écriture, pas la suivre).

## S304 — 2026-09-20 — les asymétries construites, et la mer soumise de nouveau

**Entrée.** « Continue », après le verdict R11 et ADR-176. Copie unique, master propre, jeton
libre ; plan committé avant tout code.

**Capacité reçue.** La surface rendue a **ses deux asymétries** : verticale (crêtes pointues,
creux plats) et de pente (face avant plus rugueuse). **Ce qui devient possible** : soumettre à
l'utilisateur une mer qui a la forme statistique d'une mer observée, et non plus une somme de
sinusoïdes. **Consommateur** : la revue R12, demandée. **Preuve** :
[ASYMETRIES-S304](../../docs/validation/ASYMETRIES-S304.md).

**Construit.** Dans le nuanceur : `tayfun()` — second ordre en bande étroite par système,
`η₂ = ½k̄(η² − η̂²)` — et `lagged_eps()` — déformation déphasée de −0,20 tour. Les accumulations par
système entrent dans la boucle qui somme déjà la bande ; branches explicites, FXC refusant
l'indexation dynamique d'un vecteur en écriture (L345). Un douzième `vec4` d'uniforme porte
`(split, k̄₁, k̄₂, retard)`. La référence CPU porte les mêmes termes. Le fragment n'a pas changé :
le sommet lui transmet la déformation retardée.

**Réception, cinq critères écrits avant le code, cinq tenus.** `Sk` 0,003 → **0,0656** (exigé
≥ 0,06), `c₀₃` 0,001 → **−0,155** (exigé entre −0,18 et −0,13), `c₂₁` **−0,057** contre −0,058
observé — **sans avoir été visé**, le seul paramètre calé étant le retard, sur `c₀₃`. GPU contre
CPU : 1,59·10⁻⁶ m et 2,69·10⁻⁴, aucun repli. Scènes antérieures **identiques au bit** au binaire de
S301, `--sans-asym` compris. Écart au jeu 0,3651 → 0,3999 m (+3,5 cm sur 36,5 dont CWM porte
l'essentiel ; A288 aggravée de moins de 10 %). Coût **+0,25 %** : 1,0138 → 1,0163 ms.

**Contrôle qui vaut d'être dit** : les `k̄` par système calculés par l'instrument (0,1742 et
0,0317) sont ceux que l'hôte publie au nuanceur. Les deux implémentations portent le même modèle,
et c'est ce qui autorise à lire les statistiques de l'instrument comme celles du rendu.

**Partiel.** `Sk` atteint 0,066 pour 0,156 observés : c'est la borne prudente d'ADR-176 D1, le
noyau exact du second ordre pour deux systèmes restant à écrire. `c₀₃` atteint 70 % de l'observé.
Capillaires parasites, écume, micro-déferlement, asymétrie horizontale et `mss` 14 % haute :
inchangés. Aucune couche δ dans les images de R12 — la question posée est celle de la mer.

**Rituel.** Maillons **0** : une capacité construite, reçue sur cinq critères, et son consommateur
est la revue demandée. File active relue, deux lignes remplacées ; feuille de route J1-bis, index
(liste et tableau des ADR), REVUE-VISUELLE §17. I-01/I-04/I-13/I-15 relus, inchangés. Plafonds et
navigation à 0. Copie unique, jeton libre.

*Tenue du plan* : P2 à P5 fusionnés en un commit — la construction et sa réception forment une
seule thèse, et les quatre étapes ont tenu dans l'heure. Le battement a encore été écrit avant la
lecture de l'horloge (L237) : geste corrigé dans la minute, règle à appliquer dans l'ordre.

## S305 — 2026-09-20 — la production dans une cuve, et le seul critère de porte qui n'avait rien

**Entrée.** « Reprends le projet ». Jeton libre, copie unique, `master` propre ; plan committé
avant tout code. Le verdict R12 n'étant pas là, le lot a été pris dans la **porte en cours**
(REPRISE §6.7) : le critère **2** d'[ADR-175](../../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
§4, seul des quatre dont **rien** n'était mesuré sur les cas de cuve de §4.1. Les images de R12
ont été renvoyées à l'utilisateur en parallèle ; le lot ne les attendait pas.

**Capacité reçue.** Le pas de production GPU **suit la référence 3D dans une cuve** — fond nul,
murs, mode oblique (1, 1) — à **3·10⁻⁷ m** pour les **3 mm** exigés, aux trois raffinements et
sur la seconde entière déclarée, avec une erreur de phase qui **décroît** 1,223° → 0,248° →
0,0652°. **Ce qui devient possible** : le critère 2 de la porte B repose désormais sur **deux**
familles de cas au lieu d'une, et il ne manque plus à cette porte que le **critère 3** — le
verdict de l'utilisateur sur la mer. **Consommateur** : la porte B elle-même, et la géométrie
« bassin sans B » dont la porte D aura besoin. **Preuve** :
[CUVE-GPU-S305](../../docs/validation/CUVE-GPU-S305.md).

**Construit — rien dans le solveur.** Quatre bancs dans `viewer/src/delta3d_step.rs`
(`--delta3d-cuve`, `-chainon`, `-trajectoire`, `-longue`). Aucun nuanceur touché : le **mode sans
fond** passe par la **donnée** — un fond à une composante d'amplitude nulle — et non par une
branche ni une seconde source (L137). `Step3::on_device` refuse zéro composante, jamais une
composante nulle ; le noyau du fond étant linéaire en amplitude, il rend exactement zéro.
**Vérifié** : 754 624 valeurs sur 29 024 faces, **aucune non nulle**.

**Le résultat qui porte les autres.** À fond nul, `step_perturbation_mobile` — le schéma que la
carte reproduit — est **identique au bit** à `step_surface_mobile`, la référence reçue en
S295/S296 : écart 0 sur 1 000 pas, aux deux résolutions. Sans cette identité, un écart
carte/référence se serait partagé entre la carte et le schéma couplé ; avec elle, il est
attribuable à la **carte seule**. C'était l'objet du quatrième critère écrit avant la mesure.

**Deux choses dites parce qu'elles sont mesurées, non parce qu'elles arrangent.**
1. **L'écart n'est pas borné.** Sur 5 s (2,33 périodes) il croît de 2,5 à 7,8·10⁻⁷ m, ≈ 1,2·10⁻⁷ m
   par seconde, à peu près linéairement — les 3 mm seraient franchis vers **sept heures** de temps
   simulé. La phrase juste est « il croît lentement », pas « il est borné ». Nouvel angle mort
   **A298**, sévérité 1 ; suspect nommé (la somme compensée de la carte, L346), **non démontré**.
2. **L'amplitude du cas avait été mal choisie en amont.** La référence de S296 travaillait à
   `A` = 1 mm, sous une tolérance de 3 mm : un banc qu'une production entièrement fausse aurait
   passé. Le cas a été repris à 5 cm — quinze fois la tolérance, `a·k` = 0,044, régime linéaire
   préservé. Leçon **L347**.

**Pourquoi la durée est entière ici.** La surface reste dans 4 ± 5 cm et **aucun centre de maille**
ne s'y trouve aux trois raffinements : la bascule de mouillure d'A297 n'est jamais déclenchée.
C'est une propriété du **cas**, pas une correction du schéma — **A297 reste entière**, et le
constat est porté dans sa ligne de file. Autre observation : la projection est **déjà convergée à
64 cycles** sur ce cas, là où le cas S298 en demandait 128 — une cuve à fond plat est mieux
conditionnée qu'une mer résolue.

**Partiel et non-fait.** Seul le cas **3** de §4.1 est éprouvé côté production ; les cas 1 et 2
(HOS à `ny` = 1, invariance en `y`) restent reçus pour la seule référence. Régime linéaire
uniquement. Dérive de la moyenne de la carte 400 fois celle du cœur, non poursuivie. Aucun coût
mesuré — il relève de la porte C et de la scène, pas d'une cuve. Rien de visuel : le critère 3
reste suspendu au verdict R12. Les bancs allouent par pas (I-06 se lit sur la boucle d'image,
qu'ils ne touchent pas).

**Rituel.** Maillons **0** : capacité reçue, consommateur nommé, preuve écrite, et un critère
« reçu si » de la porte en cours qui avance. Suite de tests du cœur rejouée, 0 échec. File active
relue en entier ; cinq lignes remplacées et une ajoutée (A298), feuille de route §3 bis et §2 (B),
index, angles morts, leçons. I-04, I-06, I-13, I-15 et I-17 relus : inchangés, aucun nuanceur ni
chemin d'image touché. Plafonds et navigation à **0**. Copie unique, jeton libre.

*Tenue du plan* : découpage déclaré en cours de route (P5 devenu la durée longue, preuve en P6,
rituel en P7) — l'objection « borné ou séculaire ? » n'était pas prévue au plan et méritait sa
mesure. L237 tenu cette fois : l'horloge lue dans un appel séparé **avant** chaque écriture de
battement, cinq fois sur cinq. Une affirmation fausse écrite puis corrigée avant commit :
« secteur aux deux bornes » dans la preuve, alors que rien n'avait été relevé ; l'alimentation
l'est maintenant, et le document dit pourquoi A270 ne s'applique pas à ce lot.

## S306 — 2026-09-20 — un guide reçu, et la cause des stries enfin attribuée

**Entrée.** Un fichier, sans consigne : `guide_topologie_ocean_haute_mer_plage.md`, v1.0, 773
lignes, recopié tel quel dans [`docs/sources/`](../../docs/sources/guide_topologie_ocean_haute_mer_plage.md).
Il analyse « les deux images reçues » — une photo de haute mer et un rendu : c'est donc,
vraisemblablement, une réponse indirecte à R12 obtenue ailleurs. **Le verdict R12 lui-même n'a
toujours pas été donné** ; la question est reposée en R13, le travail n'en a pas dépendu.

**Capacité reçue.** Le dépôt sait désormais **d'où viennent les stries de sa mer**, par la mesure :
la queue spectrale porte **80 à 85 %** de l'énergie haute fréquence de l'image, et le contraste
global ne le voyait pas. **Ce qui devient possible** : régler la bande de rendu plutôt que la
statistique de la surface, et soumettre à l'utilisateur un arbitrage chiffré au lieu d'une
impression. **Consommateur** : la revue **R13**, demandée, et l'ADR qui suivra son verdict.
**Preuve** : [STRIES-S306](../../docs/validation/STRIES-S306.md).

**Construit — aucune modification du chemin de rendu.** Trois sorties de diagnostic dans
`ocean_fragment` (hauteur, normales géométriques sans queue, jacobien), portées par
`p.reflection.w` **qui valait un zéro littéral**. `--coupure=<f>`, treizième `vec4` d'uniforme,
qui déplace les deux bornes du filtre spectral ; `f` = 1 est ADR-148 **au bit**.
`outils/spectre_image.py`, qui mesure l'énergie haute fréquence d'une capture — parce qu'un
écart-type de luma ne distingue pas une masse d'eau d'un tapis de stries.

**Les quatre résultats, dans l'ordre d'importance.**
1. **La queue porte les stries** : `hf_rms` ×5,1 en pose `proche`, ×6,7 en `rasante` quand on
   l'allume. Pendant ce temps `luma_et` **baisse** (51,41 → 51,29). Aucune mesure antérieure du
   dépôt ne pouvait l'attraper : toutes portaient sur la surface ou sur des empreintes.
2. **Mais l'essentiel de cette énergie est légitime.** La queue porte 60 % de la variance de pente
   et tout l'excès sur Cox–Munk (`mss` 0,0497 contre 0,0437) ; il lui faudrait −10,7 % en
   amplitude pour tomber juste — ce qui ne diviserait `hf_rms` que par 1,1. Le problème n'est pas
   *combien*, c'est **dans quelle bande on la rend**.
3. **La bande est un bouton, et il rapporte.** `spectral_weight` garde tout son poids jusqu'à
   `λ = 4·empreinte` et ne tombe qu'au Nyquist du pixel : la borne **basse** de la fourchette que
   le guide recommande. Élargir divise l'énergie haute fréquence par 2,2 (`f` = 2) ou 3,6 (`f` = 3)
   **sans changer le contraste**, et coûte **6,2 % de GPU en moins** (1,0212 → 0,9574 ms).
4. **`replis = 0`** sur les treize modèles de l'instrument : le jacobien ne se retourne jamais.
   L'indicateur que le guide met au premier rang est publié, et il écarte une hypothèse.

**Ce que le guide a changé, et ce qu'il n'a pas changé.** Ses treize sections sont classées dans
[LECTURE-GUIDE-OCEAN-S306](../../docs/registres/LECTURE-GUIDE-OCEAN-S306.md) : **aucune divergence
réelle** avec une décision actée. Il décrit au contraire ce que le dépôt fait déjà, parfois mot
pour mot — sa « couche résiduelle définie » est ADR-175 D7, son projected grid est notre maillage
depuis S211, et son §5.4 (coutures, T-junctions) est sans objet chez nous. Ses §6 et §7 (côte,
plage, déferlement) sont quasi absents du dépôt, et c'est daté (A234, porte F) ; sa matière
sourcée y est consignée. **Ce qu'il apporte vraiment est un ordre de diagnostic** : normales fines
et environnement lumineux avant la topologie. S303–S304 avaient fait l'inverse, et le test qui
départage n'avait jamais été fait.

**Arbitrage rendu à l'utilisateur, pas pris ici.** Élargir la coupure retire aussi du micro-détail
**réel** ; le compensateur correct est de transférer ces pentes vers le reflet (ADR-161, guide
§5.3) et il n'est pas mesuré. Choisir `f` est une décision visuelle → **R13**, cinq questions,
options déclarées. R12 reste demandée : elle porte sur la forme des crêtes, R13 sur la fréquence
spatiale. **ADR-176 n'est ni confirmée ni contredite** par ce lot.

**Partiel et non-fait.** L'**environnement lumineux** n'est toujours pas testé — nouvel angle mort
**A299** : le dépôt a un ciel relevé sur la photo (S261) et ADR-162, mais aucune comparaison
contrôlée, ni vérification du Fresnel. L'anisotropie mesurée (4 à 9) n'est pas interprétée : une
caméra rasante étire les structures, la mesure est confondue avec la perspective. Aucun ADR n'est
pris. Rien de la côte n'est construit.

**Correction sourcée portée au dépôt.** SPEC-001 §3 donnait `H/h ≈ 0,78` (McCowan) comme critère
de déferlement ; note factuelle datée : c'est un repère du **cas de la vague solitaire sur fond
horizontal**, pas une loi universelle ni un déclencheur suffisant (*Coastal Engineering Manual*).
L'attribution reste juste, l'emploi comme critère unique ne l'est pas. Aucun ADR réécrit.

**Rituel.** Maillons **0** : capacité mesurée, consommateur nommé (R13), preuve écrite, et elle
fait avancer le critère 3 de la porte B — la seule chose qui lui manque est un jugement, et ce
lot lui donne de quoi juger. Suites de tests rejouées : cœur 0 échec, afficheur 36 passés
0 échec. File active relue en entier ; quatre lignes remplacées, deux ajoutées (stries, A299).
Feuille de route J1-bis, index (deux entrées), angles morts, leçons, SPEC-001, REVUE-VISUELLE
§18. I-01/I-04/I-13/I-14/I-15 relus, inchangés. Plafonds et navigation à **0**. Copie unique,
jeton libre.

*Tenue du plan* : découpage déclaré en cours de route (P5 devenu le balayage de la coupure, les
conséquences en P6, le rituel en P7). **Deux erreurs commises et corrigées dans la session**, qui
font la leçon **L348** : une constante de couleur supposée en linéaire dans une cible sRGB — le
filtre n'excluait rien, et un compteur trop rond le disait sans que je le lise ; et un témoin
d'identité au bit rejoué avec des drapeaux que la preuve S304 n'employait pas — il a crié à la
régression alors qu'il comparait deux choses différentes. Les deux fois, c'est le côté **supposé**
de la comparaison qui a cassé, en silence.

## S307 — 2026-09-20 — le verdict, et trois revues qui ne montraient pas notre rendu

**Entrée.** Le verdict, enfin : « **le rendu actuel est toujours mauvais** », avec une consigne de
méthode — aller au-delà du guide reçu, lire ses références et les références de ses références,
multiplier les étapes. Consigné dans [REVUE-VISUELLE](../../docs/validation/REVUE-VISUELLE.md).

**Capacité reçue.** Le dépôt sait pourquoi son rendu était jugé mauvais, et ce n'est pas ce que
deux lots de correction avaient supposé. **Ce qui devient possible** : soumettre une mer que le
dépôt sait réellement produire, et corriger des valeurs fausses au lieu de raffiner des valeurs
justes. **Consommateur** : la revue **R14**, qui annule R12 et R13. **Preuve** :
[RENDU-ECART-S307](../../docs/validation/RENDU-ECART-S307.md).

**Le fait principal, et il est désagréable.** **Trois revues consécutives ont été soumises avec
des fonctionnalités que l'utilisateur avait lui-même acceptées, éteintes.** R11 tournait sans les
vagues pointues (trouvé par S303). R12 et R13 tournaient sans `--ciel-clair` — le ciel construit
en S261 **d'après sa propre photo de référence** — et sans `--reflets-filtres`, qu'il avait
accepté en **R7**. Mesuré : le ciel oublié remplaçait une brume à 6 km par une brume à **500 m**
sur une scène qui porte à 1 500 m ; les reflets oubliés divisent l'énergie haute fréquence de
l'image par **2,5**.

**Comment cela s'est vu** : j'ai **regardé l'image**, ce qu'aucune session n'avait fait, puis lu
le nuanceur. Toutes les mesures du dépôt portaient sur la surface ou sur des empreintes d'octets
— sur l'entrée et sur l'identité, jamais sur ce qui est montré (angle mort **A301**).

**Le second fait, sourcé.** La couleur du corps d'eau, `vec3(0.012, 0.105, 0.13)`, **n'a aucune
provenance** — ce qu'I-14 interdit — et elle est **9 fois trop verte**. Dérivation depuis
[Pope & Fry 1997](https://omlc.org/spectra/water/data/pope97.txt) (absorption, jeu téléchargé) et
Morel 1974 (diffusion moléculaire, `b_b = b/2`) : `R(0⁻) ≈ 0,33·b_b/(a+b_b)` donne B/G = **10,9**
contre 1,24 mesuré. [ADR-177](../../docs/adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md)
la dérive désormais du calcul. Angle mort **A300** : ce n'était pas un cas isolé — ciel, brume,
Fresnel et miroitement n'ont pas davantage de source, parce qu'I-14 n'avait jamais été portée sur
le rendu, tenu pour cosmétique alors que c'est lui que l'utilisateur juge.

**Recherche, trois niveaux.** *Niveau 1* : les douze références du guide portent sur la forme et
la physique côtière ; **aucune** ne traite couleur, absorption, diffusion, écume, ciel ni
exposition — exactement ce que nous traitons par des constantes. *Niveau 2* : Bruneton et al.
2010, lu par son implémentation de référence — le remède au scintillement est de convertir les
pentes non résolues en **rugosité de BRDF**, pas de les couper ; nous l'avions déjà (ADR-161),
éteint. Sea of Thieves (SIGGRAPH 2018) : masque de crête tiré de la compression horizontale pour
la diffusion sous-surface et l'écume — **ce masque, nous l'avons déjà**, c'est le jacobien de CWM,
calculé à chaque pixel et jamais employé qu'à un repli. *Niveau 3* : ECKV/Elfouhaily 1997, qui
remplacerait notre queue `f⁻⁴` **et** le calage Cox–Munk ; écume de Monahan (0,42 % à notre vent,
réflectance 0,22).

**Construit.** `--meilleur`, qui active tout ce qui est accepté, la liste vivant dans le code à
côté de ce qui la consomme, et une ligne de capture qui publie **toutes** les options (leçon
**L349**). `--eau-physique[=gain]`, couleur dérivée, gain déclaré libre. Les scènes de référence
restent **identiques au bit** à chaque étape.

**Une erreur de ma part, corrigée par l'image.** J'avais d'abord remis `R(0⁻)` à la luminance de
l'ancienne constante : le bleu valait alors 0,40 de réflectance, quatre fois le maximum physique,
et le rendu virait à l'outremer. La renormalisation compensait sans le dire l'irradiance de ciel
absente. Aucun chiffre ne l'avait signalé ; l'image, en une seconde. C'est ce qui a produit D2
d'ADR-177 — nommer le facteur d'échelle pour ce qu'il est.

**Partiel et non-fait.** Écume, diffusion aux crêtes et spectre ECKV : **décidés en ordre, aucun
construit**. Ciel physique et exposition non mesurés (A299). Le PDF de Bruneton et le talk Sea of
Thieves n'ont été lus qu'en résumé ou par leur implémentation. Et la limite la plus dure du
dispositif est nommée dans R14 : **le dépôt n'a aucune photographie de référence** dont on
connaisse vent, exposition et focale.

**Rituel.** Maillons **0** : capacité mesurée, consommateur nommé (R14), preuve écrite, et elle
fait avancer le critère 3 de la porte B — dont les trois verdicts précédents portaient sur des
images qui ne montraient pas notre rendu. File active relue en entier ; deux lignes ajoutées,
quatre remplacées. Feuille de route J1-bis, index (preuve + ADR-177), angles morts (**A300**,
**A301**), leçons (**L349**), REVUE-VISUELLE (verdict, règle de protocole, R14). I-01/I-04/I-13/
**I-14**/I-15 relus : I-14 était violée par le rendu, elle ne l'est plus pour la couleur.
Plafonds et navigation à **0**. Copie unique, jeton libre.

*Tenue du plan* : plan à neuf étapes déclaré d'avance, découpage annoncé une fois (le remède de
protocole passé avant la synthèse parce qu'il devenait urgent). L237 tenu : horloge lue avant
chaque battement. Une affirmation fausse écrite puis corrigée dans la session — un « artefact
brun-olive » que la mesure a réfuté (zéro pixel sur 921 600 n'a `R > B`).

## S308 — 2026-09-20 — la photographie devenue cible, le lot du rendu clos par la mesure, et la stratégie en trois systèmes

**Entrées.** Le verdict **R14** avec une **photographie de référence** : la géométrie suffit, le
travail prioritaire est optique. Puis, **en cours de session**, une nouvelle stratégie de
développement de l'utilisateur, qui redirige le projet vers la physique.

**Session interrompue puis reprise.** Une coupure a arrêté S308 après P4, avec P5 mesuré mais non
écrit et son code non committé. Reprise à chaud selon EN-COURS : le diff était cohérent et sa
thèse déclarée, j'ai **complété** l'étape plutôt que de l'annuler — code rebâti, images refaites,
tous les chiffres revérifiés au centième avant commit. La photographie remesurée redonne ses
valeurs exactes : l'instrument est reproductible.

**Construit.** `outils/cible_image.py` (P2) : la photographie devient une cible **chiffrée** —
horizon, profil du ciel, histogramme de la mer, couleur creux/crêtes, contraste local, fraction
claire —, chaque grandeur déclarée **comparable ou non**, avec la raison. `--ciel-mesure[=degrés]`
(P4) : extinction exponentielle par canal en `sin(élévation)`, les deux constantes étant la
photographie mesurée ; signature de Rayleigh, bleu à peine atténué, rouge effondré.
`--tonalite=e,g,w` (P5) : pied en puissance, épaule de Reinhard, **sur la luminance seule** — la
version par canal crevait la teinte, et la mesure l'a dit en un passage. `--miroitement=<f>` (P6),
1 au bit. `outils/courbe_tonalite.py` (P7) : recherche à deux étages, 6 300 candidats, critère
déclaré avant la mesure.

**Ce que la mesure a tranché, et c'est l'apport principal.** La photographie a **renversé l'ordre
des travaux** : notre ciel était plat (sommet à 0,809 de l'horizon contre 0,356) et pâle (`B/R`
1,82 contre 5,14), et comme la mer est un miroir, c'est lui qui écrasait crêtes, dynamique et
contraste. Le ciel calé rapproche la teinte partout — crêtes `B/R` 1,73 → 3,03, creux 14,1 → 29,5
pour 30,0 mesurés — mais pas la dynamique. Puis deux hypothèses successives ont été **réfutées par
l'expérience, pas par le raisonnement** : la coupure spectrale (elle *ajoute* des pixels clairs et
détruit le contraste local) et le miroitement du soleil (l'éteindre complètement ne retire pas un
pixel clair). Enfin P7 : **les huit meilleurs réglages de courbe donnent tous le même contraste
local, 0,311–0,318 pour 0,455 mesurés**, et c'est pour tous la cible la plus dure. Une courbe est
point à point, le contraste local est spatial : **il n'y a pas de réglage à trouver, ce qui manque
est dans la mer**. Le lot optique est arrivé à son point d'arrêt, et il y est arrivé **mesuré**.

**La stratégie en trois systèmes** ([ADR-178](../../docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md),
[confrontation](../../docs/registres/TROIS-SYSTEMES-S308.md)). Trois écarts entre la stratégie et le
dépôt, tous mesurés. **Le dépôt a déjà** la séparation référence CPU / production GPU qu'elle
réclame (ADR-175). **δ 3D tourne** — cuve fermée reçue à 3·10⁻⁷ m pour 3 mm, scène de 30 × 28 m
rendue en direct — **mais sa surface est une fonction hauteur** : l'essai phare de la stratégie,
la boule qui tombe avec cavité et projections, est hors de cette représentation **par
construction** (ADR-175 D5), et la seconde représentation n'a jamais été choisie. **Le couplage
est à sens unique et sans aucun compteur** : δ ne ressort pas vers W, et masse, quantité de
mouvement et énergie n'ont jamais été mesurées à l'interface — alors que R11 a jugé ce raccord
« invisible ». Six interfaces nommées, l'ordre en sept lots, les compteurs d'abord.

**Preuves et limites.** Les chiffres de P5 à P7 portent sur **une pose** (« proche ») d'**une**
scène ; la photographie n'est pas une cible physique — vent, focale, exposition et réponse capteur
restent inconnus, seules les grandeurs normalisées par la scène sont comparables. L'inventaire de
P8 lit le code et les preuves citées ; il ne re-mesure aucun coût et n'exécute aucun banc neuf.

**Non-fait.** Diffusion aux crêtes, écume et ECKV : toujours aucun construit, mais désormais **avec
leur cible chiffrée**. La structure fine de la mer que réclame le contraste local : non ouverte —
c'est un lot de forme, et ils étaient suspendus. R15 non envoyée : la stratégie a rendu la campagne
sans objet.

**Rituel.** Maillons **0**. La justification n'est pas une capacité de code : c'est une **décision
qui lève un blocage en nommant le lot désormais exécutable** (critère S227/S294). Cinq sessions de
suite avaient porté sur le rendu ; ADR-178 nomme le lot 1 — les compteurs de conservation — sur
des scènes qui existent déjà. La recommandation du dernier bilan (BILAN-GLOBAL-S293 §5) est
**portée** : son lot 4, la scène-témoin de la v1, devient les lots 3 et 4 d'ADR-178 D7.

*Tenue du plan* : dix étapes, dont deux découpages déclarés (P6 séparé de sa construction ; P8/P9
redéfinis à la réception de la nouvelle consigne). L237 tenu. Deux erreurs écrites puis corrigées
dans la session : le miroitement présenté comme coupable en P5 — P6 l'a réfuté — et un `min`
d'écrêtage ajouté dans `courbe_tonalite.py` en croyant expliquer un écart qui venait d'ailleurs ;
le `min` est juste, l'explication était fausse, et le fichier le dit.

## S309 — 2026-09-20 — la moitié de la demande que S308 n'avait pas faite

**Entrée.** La stratégie en trois systèmes, **renvoyée telle quelle** par l'utilisateur après
S308. Elle demandait deux confrontations : à l'**état réel du dépôt** — faite, `TROIS-SYSTEMES-S308`
— et à la **liste de contrôle du projet terminé** — pas faite. S308 s'était contentée d'écrire, en
limites, que son décompte datait de S276 et sous-estimait δ 3D. **Noter un manque n'est pas le
combler**, et la session avait rendu compte comme si la demande était close.

**Changé.** Quinze points de la liste retouchés, chacun citant la preuve qui le modifie. L'essentiel
tient en trois lignes : **4.1 n'est plus « manque la 3D »** (MAC x-y-z reçu contre HOS à 0,148 %,
production GPU, cuve à 3·10⁻⁷ m pour 3 mm) ; **4.19 n'est plus « manquent le GPU et la 3D »**
(0,84 ms à 64 cycles sur 27 648 mailles, 4,62 ms sur la scène de 376 320) ; **8.7 porte δ 3D rendu
en direct** à 197 Hz. Trois points sont revus **en moins bien**, parce que S308 les avait trouvés
surestimés : 4.8 est une absence de **chemin** et non de réglage, 4.18 n'a jamais eu de bilan à
l'interface, 4.7 repose sur un jugement visuel jamais chiffré.

**Décompte recalculé point par point, pas corrigé à vue : 3 validés, 51 partiels, 66 absents.** Le
total de S276 était faux de deux unités — la ligne « Socle » comptait encore 1.4 en absent alors
que S278 l'avait rendu partiel, ce que l'audit S293 avait signalé sans refaire la table. Un seul
point change de catégorie aujourd'hui (9.1). **Aucun ne devient validé** : entre S276 et S308 le
dépôt a écrit un solveur 3D, l'a porté sur GPU et l'a rendu en direct **sans amener un seul point
de cette liste à son périmètre final**.

**Les 120 points rangés par système** (`TROIS-SYSTEMES-S308` §8) : A 20, B 29, C 7, **hors des
trois 64**. Trois lectures. La stratégie couvre **56 points sur 120** — le reste (V, rendu, budget,
multijoueur, grande échelle, outillage, validation) vient après, par construction : *finir A, B et
C ne fait pas le moteur fini, cela en fait la moitié dont tout le reste dépend*. B est l'endroit où
le moteur reste à écrire — 23 absents sur 29 —, mais la **surface non graphe en commande cinq à
elle seule**. Et une phrase de S308 est **bornée** : « A est une base avancée » vaut pour la mer de
vent et de houle en eau profonde uniforme, pas pour les douze points absents de A — lacs,
rivières, canaux, bathymétrie, hauts-fonds, courants 3D, tsunamis, explosions, déferlement de W,
écume. **Or l'objectif nomme les plages et les rivières.** Le verdict « A suffit » tenait pour le
couplage ; il ne tenait pas pour le périmètre.

**Une erreur de S308 corrigée, et rendue impossible à répéter.** Ses battements après 13:12 étaient
**extrapolés au lieu d'être lus** — le dernier valait 1 h 30 de plus que l'heure réelle. L237
l'interdit depuis longtemps ; rien ne le vérifiait. Ce n'est pas cosmétique : AGENTS.md fait d'un
jeton `occupé` de moins de deux heures un **refus de reprise**, donc un horodatage avancé bloque
silencieusement la session suivante. Remède **exécutable** (L349) : `etat_projet.py --check` refuse
un battement dans le futur, et ses quatre essais prennent pour contre-exemple l'erreur réelle de
S308 plutôt qu'un cas inventé. Angle mort **A303**.

**Preuves et limites.** Aucun banc exécuté, aucun coût mesuré : les états cités sont ceux de leurs
preuves, à leur date. Le rangement A/B/C est **dérivé de l'énoncé** de chaque point ; trois points
sont à cheval et ont reçu leur système principal, dit dans le texte (4.18 en C, 6.7 en B, 7.1 en
A). Le décompte par système a été calculé, pas estimé.

**Non-fait.** Le lot 1 d'ADR-178 — les compteurs de conservation — n'est pas ouvert. Rien du code
de simulation n'a été touché.

**Rituel.** Maillons **1**, et c'est délibéré. Cette session a produit une **carte** et un
**contrôle**, pas une capacité du système : aucun critère « reçu si » d'une porte n'a avancé, et le
seul point de la liste qui change de catégorie le doit à S278, pas à S309. Compter zéro ici serait
exactement la dérive que BILAN-GLOBAL-S293 §3.4 reproche au dépôt. **La session suivante prend le
lot 1 et en sort une capacité mesurée**, ou le troisième maillon devra être justifié.

*Tenue du plan* : cinq étapes, un découpage déclaré (P4, le contrôle exécutable, séparé du rituel).
L237 tenu cette fois — horloge lue avant chaque battement, y compris une correction en cours
d'étape quand j'ai recommencé à extrapoler de onze minutes.

## S310 — 2026-09-20 — le premier compteur : ce qui entre, ce qui sort, et ce que l'éponge efface

**Entrée.** Le lot 1 d'[ADR-178](../../docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7,
ouvert par l'angle mort **A302** : le couplage δ ↔ B/W tournait depuis S250 et **rien n'avait
jamais compté** ce qui y entrait ni ce qui en sortait.

**Construit.** `delta3d_balance.rs` — `Balance3` (volume, bande, perturbation, éponge, résidu),
`perturbation_volume`, `perturbation_energy`, `perturbation_momentum` —, le bilan tenu par le pas
**couplé** et par le pas **non couplé**, et la publication des mesures dans deux bancs existants.

**Pourquoi c'est exact, et pas seulement précis.** Le transport ne produit que des flux de colonne,
dont les termes intérieurs **télescopent** : la variation de volume vaut la somme des faces de
bord, en arithmétique exacte. **L'écart mesuré est donc le plancher du schéma**, pas une erreur
d'instrument — et c'est ce qui permet de le publier au lieu de l'absorber dans une tolérance.

**Ce que la mesure a trouvé, et que la relecture n'avait pas vu.** Compter l'`increment` de
l'éponge sur-compte de `eta_roundoff` à chaque colonne et à chaque pas : **0,86 %** du volume
retiré, **quatre ordres de grandeur au-dessus du plancher**. La somme compensée de S233 est faite
pour que ce soit `η − eta_roundoff` qui décroisse, pas `η`. Le transport, lui, était juste d'emblée
— son incrément retranche déjà le reste que la hauteur compensée rajoute. **Aucune relecture ne
l'aurait donné** : c'est l'essai de l'éponge, et lui seul, qui l'a dit (**L353**).

**Trois résultats.** *(1)* **La cuve de S305 ne perd rien** : murs à **zéro exact** à chaque pas,
dérive de volume **7,4·10⁻¹² m** de hauteur moyenne sur 5 s. Conséquence pour **A298**, dont la
dérive carte/référence vaut ≈ 1,2·10⁻⁷ m/s sur la même cuve : **ce n'est pas une fuite de volume du
schéma**, c'est propre au chemin de la carte. *(2)* **La dissipation numérique du schéma est
mesurée** — 100,55 J → 100,08 J en 5 s, soit **0,0935 % par seconde**, un cinquième de pour cent
par période : elle borne la durée de vie utile d'un domaine, et le dépôt n'avait pas ce nombre.
*(3)* **A302 devient un nombre** : sur une scène couplée à fond spectral réel, l'éponge échange
**10,2 % du contenu perturbatif du domaine par seconde**, et rien n'en revient dans W. *L'éponge ne
laisse pas sortir la perturbation : elle l'efface.* Le raccord que R11 avait jugé « invisible »
l'est parce qu'elle est **douce**, pas parce qu'elle **conserve** — et distinguer les deux
demandait exactement ce compteur.

**Ce qui ne se ferme pas, dit comme tel.** Énergie et quantité de mouvement sont publiées comme
**états**, pas comme bilans : leur fermeture demande le travail de la pression aux faces de bord et
le flux advectif, que le pas ne calcule nulle part sous une forme récupérable. Les fabriquer après
coup donnerait un nombre qui ressemble à un bilan sans en être un — l'erreur exacte qu'A302
reproche au raccord « invisible ». Ce qu'il faudrait est nommé dans la preuve, non improvisé.

**Preuve et limites.** [BILAN-MASSE-S310](../../docs/validation/BILAN-MASSE-S310.md). Tout est mesuré
sur la **référence CPU** : le compteur n'existe pas sur la carte, et la scène du §3 est une scène
*voisine* de celle de S302, pas la même. Une seule machine (A98). Aucun banc déclaré reçu.

**Trois tolérances proposées**, chacune adossée à une mesure, à trancher par l'utilisateur : T1
l'instrument (résidu ≤ 10⁻⁶, tenu ×10 à ×2 000), T2 domaine fermé (≤ 10⁻⁶ sur 10 s, tenu ×10⁴), et
**T3, le seul critère de fond** — ce que δ absorbe reparaît dans W à 5 % près, réflexion au bord
sous 1 % en énergie. **T3 n'est pas tenu : 100 % de ce qui est absorbé est effacé.** C'est le
critère du lot 2, et il existe maintenant.

**Non-fait.** Le compteur sur la carte. Les bilans d'énergie et de quantité de mouvement. Le retour
δ → W lui-même, qui est le lot 2.

**Rituel.** Maillons **0**. Capacité : le dépôt sait ce qui entre et sort d'un domaine δ, et à quel
plancher. Consommateur nommé et immédiat : le **lot 2**, qui ne peut pas se juger sans compteur —
on ne saura pas si un retour conserve tant qu'on ne sait pas ce que l'absorption retire. Preuve :
BILAN-MASSE-S310. Point **4.18** de la liste du projet fini avancé. 544 essais, 0 échec.

*Tenue du plan* : neuf étapes, aucun découpage supplémentaire. L237 tenu — l'horloge lue par un
appel du shell qui écrit lui-même le battement, après que S309 eut construit le contrôle qui le
vérifie.

## S311 — 2026-09-20 — la frontière est une paroi, et ce qui en sort, W ne peut pas le porter

**Entrées.** La décision de l'utilisateur sur les tolérances T1/T2/T3, et le **lancement du lot 2**
— le retour δ → W. Avec une précision qui a commandé la session : définir la grandeur à restituer,
ne pas confondre le volume net signé et la quantité absolue, ne pas fabriquer une vague pour
compenser l'activité de l'éponge, et **identifier** ce que W ne peut pas représenter.

**Acté.** [ADR-179](../../docs/adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md), huit
décisions. La centrale, **D3**, sépare trois grandeurs que S310 mesurait ensemble : le **flux
sortant** — celui qu'on restitue —, l'**activité absolue** de l'éponge, et son **net signé**.
Fabriquer une vague pour compenser l'activité absolue créerait de l'énergie, puisqu'une partie de
cette activité amortit une onde qui **entrait**. Et une honnêteté inscrite : **T2 n'est pas tenue**
— S310 l'a mesurée sur 5 s pour 10 s demandées.

**Le fait de la session.** Le plan posait la question avant d'y répondre : le flux sortant
existe-t-il déjà, jeté par la garde du transport ? **Non.** La vitesse normale aux faces
extérieures vaut **0 exactement**, quand le champ atteint 0,605 m/s à l'intérieur. **Un domaine δ
est une boîte fermée** ; l'éponge n'est pas une frontière absorbante mais une région
d'amortissement *à l'intérieur*. « δ ne ressort pas vers W » est plus fort que ce que S310 disait :
ce n'est pas un chemin manquant, **c'est une paroi**.

**Construit.** `Volume3::control_flux_x` — la perturbation sortante se lit sur une **surface de
contrôle intérieure**, la ligne intérieure de la bande d'éponge, que le transport calcule déjà. Et
`examples/sortie_canal.rs`, le **cas contrôlé** d'ADR-179 D6 : onde longue purement progressive,
fond nul, grandeur de référence non nulle (le volume de l'onde), erreur normalisée par elle,
réflexion en énergie sur une jauge dont les deux fenêtres se déduisent de la géométrie.

**Reçu.** Cas de réception, `λ/h₀` = 12, éponge de `8σ`, **deux mailles** : erreur de restitution
**0,151 %** (25 cm) et **0,149 %** (10 cm) pour 5 % admis ; **réflexion en énergie 1,485·10⁻⁶ et
1,489·10⁻⁶** pour 1 %, mesurée **directement en 3D**. Les deux mailles s'accordent à 0,3 % pour un
rapport de 2,5 : ce n'est pas un artefact de grille. Le banc sort avec le code 0.
[Preuve](../../docs/validation/SORTIE-DELTA-S311.md).

**Deux erreurs de montage, et c'est la méthode qui les a trouvées.** `λ/h₀ = 4` n'est pas une onde
longue — la traîne dispersive traversait la ligne dans les deux sens, retour de 36 % **insensible
au taux de l'éponge entre 2 et 20**, ce qui a **disculpé** l'éponge. Et l'éponge est **symétrique** :
la bosse démarrait dans celle de gauche, qui en effaçait 14,7 % avant le premier pas, avec pour
signature une erreur **constante quel que soit `λ/h₀`**. Deux balayages, deux coupables (**L354**).

**Le point dur, et il faut une décision.** W n'a que deux primitives de production — l'impact
radial et le sillage — et **l'impact porte de l'énergie, pas du volume** : le dépôt imprimait déjà
le chiffre sans jamais l'affirmer, **−3,6·10⁻⁷ m³** de volume net pour 0,01 J, zéro à la
quadrature près. Or la grandeur que le cas contrôlé identifie **est** un volume net sortant. Le cas
prouve donc d'un coup que la perturbation sortante est identifiée **et** qu'elle ne peut pas être
remise à W en l'état. Trois issues, aucune tranchée : la perdre en la chiffrant, étendre W, ou
choisir une surface de contrôle où elle est nulle. Recommandation portée à l'utilisateur : la
première pour le premier transfert.

**Limites.** Référence CPU, une machine (A98). Cas **unidirectionnel et invariant en `y`** : il ne
dit rien d'un front oblique ni d'une frontière courbe. La séparation entrant/sortant est triviale
**parce que rien n'entre** ; sur une scène quelconque elle demande une décomposition en
caractéristiques, non écrite. La maille de 12,5 cm sur `λ/h₀ = 20` a été arrêtée pour dépassement
de temps.

**Non-fait.** Le transfert lui-même — points 2 et 3 d'ADR-179 D8. Aucune revendication d'énergie
ni de quantité de mouvement (D7).

**Rituel.** Maillons **0**. Capacité : la perturbation sortante d'un domaine δ est **identifiée,
mesurée et reçue** contre les deux seuils de T3, sur un banc qui se rejoue. Consommateur nommé : le
transfert, qui n'avait jusqu'ici ni grandeur de référence, ni normalisation, ni moyen de séparer le
transmis du perdu. Preuve : SORTIE-DELTA-S311. Point 1 d'ADR-179 D8 tenu.

*Tenue du plan* : neuf étapes, une ajoutée en cours (P8b, le banc de réception et la comparaison de
mailles). L237 tenu. Deux longues exécutions arrêtées faute de temps, dites comme telles.

## S312 — 2026-09-20 — le transfert existe, et il dit enfin ce qui manque à W

**Entrée.** La décision de l'utilisateur du 2026-09-20, « Retour δ → W et conservation du
volume », en réponse à la question laissée ouverte par S311. Option 1 **limitée aux tests**, tout
volume non restitué **comptabilisé et publié séparément**, jamais présenté comme une restitution.
Et une instruction qui commande l'ordre de la session : *« Il ne faut pas créer une nouvelle
primitive dans W avant d'avoir vérifié si cette responsabilité relève déjà d'une autre couche. »*

**Acté.** [ADR-180](../../docs/adr/ADR-180-retour-delta-w-et-conservation-du-volume.md), huit
décisions. D2 fixe les trois catégories publiées séparément ; D3 avertit que la catégorie non
représentable **ne se réduit pas au volume net** — forme, direction, spectre, phase, régime — et
interdit de la lire dans un encodage ; D4 impose l'ordre de l'étude du receveur.

**Le fait de la session.** La question de S311 était mal posée, et la mesure l'a redressée. Ce
n'est pas *l'impact* qui ne porte pas de volume, c'est **W**. Le volume net d'un champ **est**
l'amplitude de son mode `k = 0` ; aucune des trois productions de W ne l'a — impact 0,785 rad/m
au plus bas, champ périodique 0,393, sillage 0,187 — et `ModalPressure::new([0,0])` le **refuse**.
Une « primitive de W portant un volume » serait un déplacement du plan de repos, c'est-à-dire **B
sous un autre nom**. Le champ périodique, seul à s'intégrer **sans troncature**, donne net/absolu
**1,3·10⁻⁹**. L'impact radial et le sillage donnaient des nets non nuls : balayés en taille, ils
**changent de signe** et tombent — c'était la troncature, et S311 ne pouvait pas le voir avec un
seul rayon.

**Et une découverte qui a changé le plan.** Les champs de W **refusent** le cas contrôlé de S311 —
`Regime` et `Medium`. Seuils balayés : `h ≥ λ` pour l'impact radial, `h ≥ 2λ` pour le périodique ;
et `ModalPressure` n'a pas de paramètre de profondeur du tout. **Toute la couche W est en eau
profonde**, et forcer le transfert donnerait **+44,3 %** de célérité à l'onde de S311. Repère utile :
la scène δ 3D réelle de S302 n'est qu'à **0,41 %** — c'est le canal qui était peu profond.

**Construit.** `delta3d_transfer.rs`, `Ledger3` — un registre qui se défend par sa forme :
`pending()` est une **différence**, rien ne l'écrit donc rien ne l'efface ; il est signé et
négatif il dit qu'on a **créé de l'eau** ; aucune méthode ne porte le mot « restituer », et la
conformité qu'ADR-180 D1 interdit d'affirmer est **calculée**. Et `examples/transfert_paquet.rs`,
le second cas contrôlé : un **paquet d'ondes en eau profonde**, dont le volume net vaut
`exp(−k²σ²/2)` fois l'échelle du volume absolu — ce qui répond au point 2 de l'utilisateur mieux
qu'un découpage, puisque les deux grandeurs s'y font varier indépendamment.

**Reçu.** λ = 2 m, `h₀` = 2,5 m, maille 12,5 cm, 5 432 pas. Net/absolu au passage de la ligne
**9,49·10⁻⁴**. Période à **0,46 %** de la théorie — les deux côtés du raccord portent la même
dispersion. **T3 tenue sur le transfert effectivement réalisé** : amplitude **1,24·10⁻⁵** pour
5 %, et **réflexion 2,84·10⁻⁷** pour 1 %, mesurée à part. `band_in` et `perturbation_in` valent
**0 exactement** sur tous les pas — pas de double comptage.
[Preuve](../../docs/validation/TRANSFERT-DELTA-W-S312.md).

**Les trois pertes, chiffrées — c'est la réponse à « ce qui manque véritablement à W ».**
**La moitié** de l'énergie transférée repart à contresens (0,5000 mesuré, l'anisotropie est
refusée à toute valeur non nulle) ; le **spectre** de l'impact est une bande de **deux octaves**
quand le paquet en demandait 11 %, d'où 27 % d'écart de longueur d'onde et 12 % de vitesse ; et
la **phase** est impossible — le champ naît au repos, `∂η/∂t` = 0 partout. Le volume net,
lui, reste **entièrement** en attente : 4,93·10⁻⁵ m³ registrés, 0 transféré, 0 créé.

**Une trouvaille non cherchée : T1 n'est pas mesurable sur un cas de moyenne nulle.** Son
dénominateur — `max(|delta|, |band_in|, |sponge_out|)` — rétrécit avec `dt` et avec l'activité
(1,15·10⁻⁷ m³ au maximum du banc) quand le résidu reste au plancher `f32` (2,0·10⁻¹¹ m³) : le
rapport vaut 1,95 sans plancher et **encore 1,8·10⁻⁴ sur les 318 pas les plus actifs**. Normaliser
par `Balance3::volume` ne sauve rien — somme **signée**, nulle par construction pour un paquet.
Rapporté à une grandeur **absolue** : **8,3·10⁻¹⁰**. Le schéma n'est pas en cause ; la
normalisation l'est. **Précision proposée à l'utilisateur**, et aucun banc de la session ne
revendique T1.

**Trois biais d'instrument, trouvés parce que la réponse était connue d'avance** (L357) :
coordonnées relatives données à un `sample` qui les veut absolues ; longueur d'onde lue sur la
queue de bruit `f32` ; grille de partage à compte **impair**, qui donnait 0,579 au lieu de 0,5 —
le plus dangereux des trois, parce qu'il est plausible.

**Limites.** Référence CPU, une machine, **une seule maille** — la convergence n'est pas faite
pour ce second cas, contrairement à S311. Cas unidirectionnel et invariant en `y` ; séparation
entrant/sortant triviale parce que rien n'entre. Transfert **ponctuel et unique** : ni cadence, ni
recouvrement. Le champ de W n'est **pas** rebouclé dans δ — A302 reste ouverte pour l'autre sens.
**Le prototype n'est pas conforme à la conservation globale**, et ADR-180 D1 interdit de l'écrire
autrement.

**Non-fait.** Le receveur du volume net lui-même : l'étude le désigne (V, ou un niveau moyen dans
B), la décision appartient à l'utilisateur. Aucune revendication d'énergie ni de quantité de
mouvement comme bilan (ADR-179 D7).

**Rituel.** Maillons **0**. Capacité : **un transfert δ → W existe, tourne, passe T3 sur la
grandeur qu'il transporte, et publie séparément le transféré, l'attente et le résidu**.
Consommateur nommé : la décision de l'utilisateur sur le receveur du volume net et sur la
primitive orientée, qui n'avaient jusqu'ici ni chiffres ni alternative mesurée. Preuve :
TRANSFERT-DELTA-W-S312. Points 1 à 6 d'ADR-180 §1 tenus sur ce cas. Trois leçons : L355, L356, L357.

*Tenue du plan* : dix étapes, **fusion déclarée P6+P7+P8+P9** — le cas, le transfert, sa
vérification et sa preuve sont un seul objet. Battement du commit P4 écrit à 16:55 pour 16:48 à
l'horloge (L237), corrigé au commit suivant et dit au plan. Cinq exécutions du banc, dont trois
jetées pour corriger un instrument.

## S313 — 2026-09-20 — le plancher a une loi, et l'instrument a une portée

**Entrée.** La décision de l'utilisateur du 2026-09-20 (S312), « Conservation, transfert δ → W et
suite du développement ». Elle tranche le receveur du volume net — **V** pour un contenant, **B**
pour une masse ouverte, avec un **niveau moyen régional adossé à une région identifiable**, et
surtout **pas** une modification arbitraire du niveau global —, révise T1 sans fixer de chiffre,
déclare le transfert **non validé**, et ordonne le lot 2 en A → E. Cette session prend **A**.

**Acté.** [ADR-181](../../docs/adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md),
douze décisions. D2 est celle qui coûte : un niveau moyen de B s'adosse à une **région ou un volume
de contrôle identifiable**, jamais à un océan — la solution la plus simple à écrire est celle que
la décision interdit. D6 ajoute ce que personne n'avait demandé : **l'instrument se prouve sur un
défaut qu'on lui donne à trouver**.

**Le fait de la session.** S312 avait attribué son résidu au « plancher `f32` » **sans le
démontrer**. Trois hypothèses ont été écrites **avant** de mesurer, et deux sont **réfutées** : le
résidu est **strictement linéaire en amplitude** sur quatre décades (exit la représentation de la
hauteur) et **linéaire en `dt`** (exit l'accumulation `f64`). Reste l'arrondi de l'**incrément**,
et le balayage de résolution en précise le mécanisme : à aire constante, `N` quadruple et le résidu
est divisé par ≈ 2 — une accumulation en **`√N`**, une colonne à la fois. D'où la borne, écrite
puis vérifiée :

```text
plancher = u₃₂ · activité / √N
```

**Jamais dépassée d'un facteur 3** sur tout le balayage. Et les bornes naïves en `ulp(h₀)` sont
fausses de **cinq ordres** : la somme compensée de S233 retire la hauteur du problème, et c'est la
première fois que sa valeur est chiffrée. [Preuve](../../docs/validation/PLANCHER-BILAN-S313.md).

**L'échelle pertinente n'a pas été choisie, elle a été trouvée.** Le rapport `résidu / volume
**absolu** de perturbation` vaut 6,2 ; 5,8 ; 5,2 ; 5,4·10⁻¹¹ sur quatre décades — constant. Le
volume **signé** est nul par construction sur un paquet, et l'incrément du pas rétrécit avec `dt` :
ce sont les deux dénominateurs qu'A304 accusait.

**Construit.** `delta3d_closure.rs`, `Closure3` : les quatre grandeurs de l'utilisateur, chacune
avec son unité, et **aucun seuil dans le module** — il publie, l'appelant compare. Un pas sans
activité n'y fabrique pas de rapport ; c'est la faute d'A304, et elle ne peut plus se produire.

**L'erreur volontaire, et elle apprend deux choses.** Une **fuite de bilan** — un écart d'un seul
signe ajouté au résidu — est détectée sans ambiguïté à **10⁻¹³ m³ par pas**, soit 0,11 fois le
plancher et 3·10⁻¹² du volume absolu ; la forme du cumulé sature à `√pas`, comme prédit. Mais une
**fuite d'état** — du volume réellement retiré entre deux pas — **n'est pas vue du tout** : le
résidu ne quitte jamais son plancher pendant que le domaine perd **52 %** de son volume. Ce n'est
pas un défaut, c'est **la portée** de T1 : elle ferme *un pas*. **T1 et T2 ne sont donc pas
redondantes**, et aucun document du dépôt ne le disait.

*En passant* : de 10⁻⁹ à 10⁻⁷ de fuite d'état, les sorties sont **identiques au bit** — l'offset
est sous `ulp(2,0)`, et la fuite **n'a pas lieu**.

**T2 est rendue.** Sur les 10 s demandées, à deux mailles : **2,99·10⁻⁹** et **5,77·10⁻¹⁰** pour
10⁻⁶ admis — 340 et 1 700 fois de marge. De 1 s à 20 s la dérive croît de 5,1 fois pour `√20` = 4,5
attendus : **marche aléatoire**, pas fuite. Et elle diminue quand la maille se raffine, conforme à
la loi en `A/√N` sur un banc qui n'avait pas servi à l'écrire. La dette inscrite par S311 est levée
**par la mesure**.

**Proposé, non acté** (ADR-181 D5 en fait une décision de l'utilisateur) : **C1** rapport au
plancher ≤ 10, **C2** forme du cumulé ≤ 5 sur ≥ 200 pas, **C3** T2 inchangée — chacun encadré par
le témoin et par la plus petite fuite détectée, et **les trois requis ensemble**.

**Et le critère rend aussitôt son premier service** : le **cas ouvert échoue C2**, avec une forme
du cumulé de **13,67** pour `√200` = 14,14 — son résidu est **d'un seul signe**. Minuscule en
valeur (5·10⁻⁸ de la dérive physique) mais **systématique**, et six sessions de bilans ne l'avaient
pas vu. **A305**, suspect nommé et non démontré : la bande ou l'éponge.

**Limites.** Référence CPU, une machine, une cuve fermée et un cas ouvert à fond uniforme — ni
scène, ni carte. La constante 3 de la borne est **mesurée**, pas prouvée. L'erreur volontaire est
d'**un seul signe et d'une seule forme** : une fuite alternée n'a pas été essayée, et la forme du
cumulé ne la verrait pas. Les trois seuils ne sont **pas actés**, et aucun banc ne les applique.

**Non-fait.** Les ordres B à E : la primitive orientée, la vérification conjointe, la comptabilité
du volume net, le couplage complet. Aucun n'était de cette session.

**Rituel.** Maillons **0**. Capacité : **le dépôt sait dire si un écart de conservation est
numérique ou physique, connaît la portée de son instrument, et l'a éprouvé sur un défaut qu'il
s'est donné**. Consommateur nommé : les ordres B, C, D et E, qui s'appuient tous sur ce verdict —
et qui, sans lui, auraient reçu des bancs sur un critère dont A304 disait qu'il ne mesurait rien.
Preuve : PLANCHER-BILAN-S313. Deux leçons : L358, L359. Un angle mort : A305 ; A304 reçoit sa note.

*Tenue du plan* : dix étapes, **fusion déclarée P3+P5+P6** — la loi ne se lit pas sur un balayage.
Battement écrit en avance de deux et trois minutes à P1 et P2 (L237), corrigé dès constat ;
c'est la seconde session de suite, un garde-fou vaut mieux qu'une note (porté en file).

## S314 — 2026-09-20 — W sait enfin viser, et un instrument se dégradait quand le solveur s'améliorait

**Entrée.** La décision de l'utilisateur du 2026-09-20 (S313) : C1, C2 et C3 actés **sous
conditions**, ordre B **autorisé**, A305 maintenu ouvert et parallèle.

**Acté.** [ADR-182](../../docs/adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md), douze
décisions. D1 acte C1 à 10 **mais impose de distinguer partout une loi observée d'une borne
démontrée** — appliqué le jour même au module et à la preuve de S313, par une **note datée** et non
une réécriture. D2 acte C2 et ferme les deux échappatoires du cas ouvert. D8 redit que deux
critères tenus ne font pas une réception.

**La question posée avant d'écrire une ligne.** L'impact de W est isotrope **parce que c'est un
champ d'Hankel** : `J₀(kr)` ne dépend que du rayon, et aucun réglage ne lui donne une direction.
La famille qui porte direction, spectre et phase existait déjà — **dans B**, `background::Component`,
une onde plane. Mais une onde plane **ne meurt pas**, et W n'a que des champs qui s'éteignent. Un
paquet meurt, lui : il s'étale. **S'il s'étale vite, il devient de la mer et n'a pas sa place.**

**Mesuré avant de construire** (`examples/etalement_paquet.rs`, aucune dépendance au dépôt) :
l'enveloppe du paquet de S312 ne s'élargit que de **1,25 % en 10 s**, et la loi
`σ(t) = σ₀√(1+(ω''t/σ₀²)²)` tombe à **0,03 %**. `τ = σ₀²/|ω''|` vaut **64 s**. **Un transfert dure
dix secondes ; l'enveloppe en tient soixante.** Deux contrôles indépendants tombent juste au
passage : le centre avance exactement à `cg`, et le produit `crête × σ` reste à 1.

**Conséquence de conception.** L'horizon et le rayon d'un train **se calculent** au lieu d'être
choisis — deux refus, `Horizon` et `Radius`, qu'aucune autre production de W ne porte.

**Construit.** `src/wave_train.rs`, `WaveTrain<N>` : une somme d'ondes planes bornée en **bande**
et en **secteur**. Chaque mode étant une solution exacte de la houle linéaire en eau profonde,
**direction, spectre et phase ne sont pas approchés, ils sont portés**. Douze essais unitaires, un
par propriété : amplitude à 10⁻⁷, volume net nul à 10⁻⁵, invariance transverse **au bit**, quart de
tour de phase **exact**, vitesse de groupe à 1 %, **borne de pente atteinte** (0,9 à 1,0 — donc
aucune constante de conversion à calibrer, contrairement aux 1,795 et 1,702 des champs d'impact),
et une direction oblique portée **composante par composante** à 2 %. Cinq refus, dont
`Resolution`, qui rend un service immédiat : **ouvrir le secteur coûte des modes de bande**, et un
secteur de cinq directions est refusé à `N` = 64.

**Le transfert**, en trois gestes : lire `η(t)` sur la ligne de contrôle de S311 ; l'identifier par
**cinq nombres** issus du même signal — arrivée, largeur, amplitude, pulsation, **phase** ;
émettre un train qui les porte. Rien n'est ajusté après coup, et surtout pas l'amplitude.

**Reçu — les trois limitations de S312 sont corrigées** ([preuve](../../docs/validation/TRANSFERT-ORIENTE-S314.md)) :

| | S312, impact | **S314, train** |
|---|---:|---:|
| direction, part vers l'avant | 0,500 | **1,00000** |
| spectre, écart de bande | deux octaves pour 11 % | **3,5 %** *(à 6,25 cm)* |
| phase | impossible | régression **0,9938**, forme **5,7 %** |
| vitesse de groupe | 12,4 % | **0,035 %** |
| réflexion *(à part)* | 2,84·10⁻⁷ | **2,84·10⁻⁷** |
| volume net | 100 % en attente | **100 % en attente** |

**Et une trouvaille qui a demandé deux passages.** Au premier, **le maillage le plus fin était le
pire** : `ω` lue passait de −4,1 % à −0,45 % puis **+4,7 %**, changement de signe, erreur de forme
à 60 %. Deux points suggéraient une convergence nette ; le troisième la détruisait. **La cause
n'était pas le schéma mais l'estimateur** : les passages par zéro comptent *toutes* les traversées,
et **une maille fine amortit moins les courtes** — l'instrument se dégradait *exactement quand le
domaine s'améliorait*. Avec le maximum du périodogramme, les six grandeurs convergent de façon
monotone. Les deux estimateurs s'accordent à 0,3 % aux mailles grossières et divergent de **4,7 %**
à la plus fine.

**Limites et non-fait, et ils sont importants.** **L'essai 3 n'est réalisé qu'à moitié** : la
primitive porte l'oblique, mesuré en deux composantes, mais **la propagation oblique à la
frontière** — un front qui sort d'un domaine large en `y` — reste due. L'essai 4 est mesuré **sur
la ligne d'émission**, pas à distance. Le champ de W n'est **pas rebouclé** dans δ (A302). Et
**le transfert n'est pas déclaré validé** : quatre essais sur cinq, deux réserves de montage sur
l'essai 1, et ADR-182 D8 l'interdit.

**Rituel.** Maillons **0**. Capacité : **W porte un train orienté, à bande étroite et à phase
prescrite, et le transfert δ → W le lui donne depuis le signal mesuré**. Consommateur nommé :
l'ordre C, la vérification conjointe des six propriétés, qui n'avait aucune primitive capable de
les porter toutes. Preuve : TRANSFERT-ORIENTE-S314. Deux leçons : L360, L361. Un angle mort : A306.

*Tenue du plan* : onze étapes, **deux fusions déclarées** — P5+P6+P7 (l'extracteur n'existe que
pour alimenter le train) et P9+P10. P8 rendu **à moitié**, et marqué `[~]` plutôt que coché. Quatre
biais d'instrument trouvés et corrigés dans la session, tous parce que la réponse attendue était
connue d'avance (L357) : coordonnées relatives données à `sample`, normalisation de projection à
`1/√2`, fenêtres de direction recouvrant le paquet, et l'estimateur de fréquence.

## S315 — 2026-09-20 — l'oblique ne fabrique aucune direction, et la phase à distance n'est pas mesurable ainsi

**Entrée.** La décision de l'utilisateur du 2026-09-20 (S314) : essai oblique autorisé, puis ordre
C ; le transfert **reste partiel**. Deux vérifications dues — l'oblique **à la frontière**, et la
phase **à dix longueurs d'onde** contre un oracle indépendant.

**Acté.** [ADR-183](../../docs/adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md), neuf
décisions. D1 déplace l'essai oblique de la primitive vers le **raccord**. D2 exige que l'oracle
soit **une autre physique** — et exclut explicitement le train, qui propage exactement par
construction. D3 interdit de rattraper une phase par une amplitude **ou par un décalage**. D5 fait
de l'**attribution** le cœur de l'ordre C.

**L'oracle.** δ sert d'oracle à lui-même : **deux lignes de contrôle** séparées de dix longueurs
d'onde, le train émis depuis la première, δ vivant jusqu'à la seconde. Le train prédit, δ constate.

**La prédiction, écrite avant que le banc existe** : à fréquence égale et nombre d'onde différent,
la phase se sépare linéairement avec la distance. Depuis les seules mesures de S314 : **−0,853 tour
à 25 cm, −0,165 à 12,5 cm, −0,0072 à 6,25 cm** — deux ordres de grandeur entre les extrêmes, donc
un vrai test.

**Ce que l'oracle établit.** Le périodogramme **spatial** retrouve `k_δ` = 3,1395 pour 3,1416 posé
— **0,07 %** — sans rien supposer de la condition initiale. Le terme d'espace tombe **exactement**
sur la prédiction : −0,1655 mesuré contre −0,165 annoncé. Et deux propriétés de δ que personne
n'avait chiffrées sur ce trajet : **6,3 % de dissipation** sur dix longueurs d'onde, et une vitesse
de groupe **7,5 % sous** celle du train.

**Ce qu'il n'établit pas, et pourquoi.** Il reste un résidu de phase que le modèle n'explique pas.
Le **balayage de séparation** le qualifie — 0,058 à 1 λ, 0,064 à 3 λ, **0,356 à 10 λ** : il
**croît**, donc ce n'est pas un décalage d'instrument. La raison est structurelle : **une phase
n'est connue que modulo un tour**, et dès que les vitesses de groupe diffèrent le train et δ
n'arrivent plus ensemble — 1,66 s, soit **1,46 tour** de porteuse. Plusieurs enroulements sont
compatibles, et rien dans ce banc ne tranche. **La phase à dix longueurs d'onde est indéterminée :
ni validée, ni invalidée.** Le remède est nommé — dérouler la phase le long du trajet — et n'est
pas construit (**A307**). Ce que je n'ai **pas** fait et qui aurait « marché » : décaler le train
de 1,66 s. D3 l'interdit, et cela n'aurait rien mesuré.

**L'essai oblique, et son résultat le plus net.** Rien dans l'extraction ne connaît l'angle : la
direction se mesure par périodogramme **à deux dimensions** sur `η(y, t)`, le signe de `k_y` venant
du couplage espace-temps. Trois angles :

| `θ` posé | `θ` lu | écart | **miroir transverse** |
|---:|---:|---:|---:|
| 0° | **0,00°** | **0,000°** | *(sans objet)* |
| 20° | 22,57° | 2,57° | **1,28·10⁻⁶** |
| 40° | 45,10° | 5,11° | **2,09·10⁻⁷** |

**Le raccord ne crée aucune composante transverse artificielle** — le miroir `−k_y` vaut 10⁻⁶ à
10⁻⁷ de la composante utile. Troisième point de la décision, tenu net. L'écart d'angle
**s'attribue** : `ω` lue à −3,3 % (dispersion de δ à huit mailles par longueur d'onde, celle que
S314 a vue converger) et `k_y` à +4–5 % (largeur spectrale d'un paquet court), qui s'additionnent
puisque `θ = asin(k_y/k)`. Prévu, **non mesuré** : que l'écart converge avec la maille.

**Deux fois le contrat a attrapé quelque chose.** Il a **refusé la distance** demandée — l'enveloppe
s'élargit de 11 % sur les cinquante secondes nécessaires, pour 10 % admis. J'ai **déclaré**
l'élargissement (`TrainSpec::spread_limit`) au lieu de relever la constante, ce qui l'aurait fait
en silence pour tous les appelants. Puis il a **refusé le train oblique** (`Envelope`) : l'enveloppe
valait une longueur d'onde *posée* pour une longueur d'onde *lue* plus grande. La chaîne est fermée
par un essai unitaire sur les `(k_x, k_y)` **mesurés** — direction portée à **0,05°** près.

**Deux fautes de méthode.** Comparer deux phases référencées à **deux instants différents** (1,45
tour d'origine, lu comme 0,38 tour « inexpliqué »). Et surtout : deux exécutions périmées
**verrouillaient l'exécutable**, `cargo build` échouait, son erreur était **avalée par un filtre**,
et l'ancien binaire tournait. Une mesure de phase calculée par la version d'avant a failli être
publiée (**L362**).

**Limites.** Oblique à **une maille** et trois angles ; la convergence de l'écart d'angle est
attribuée, non mesurée. Le montage oblique n'a **qu'un milieu** : la conservation de la composante
tangentielle à une **discontinuité** n'est pas éprouvée, et aucune réflexion n'y est mesurée.
L'oracle suppose une enveloppe gaussienne et une porteuse unique. Le volume net reste **intégralement
en attente**. Le couplage reste à **un sens** (A302).

**Rituel.** Maillons **0**. Capacité : **le raccord sait lire une direction qu'on ne lui a pas
soufflée, et il n'en fabrique aucune** ; et le dépôt sait désormais **pourquoi** une phase ne se
compare pas à distance par les extrémités. Consommateur nommé : l'ordre C, dont §7 de la preuve est
l'entrée — chaque propriété avec son écart et son attributaire. Preuve : ORACLE-ET-OBLIQUE-S315.
Deux leçons : L362, L363. Un angle mort : A307.

*Tenue du plan* : dix étapes, **deux fusions déclarées** (P5..P9 ; le balayage de séparation ajouté
en cours de P4 comme discriminant). Battement reporté de l'horloge à chaque commit.

## S316 — 2026-09-21 — l'ordre C : la primitive est exacte, le raccord ne coûte qu'un degré, et le reste appartient à δ

**Entrée.** La décision de l'utilisateur du 2026-09-20 (ADR-183 D5) : les six propriétés d'un même
transfert, ensemble, chacune attribuée à la primitive, au raccord ou à δ. Contrôle de REPRISE §6.7 :
le lot 2 portait S310–S315 ; c'est la décision de l'utilisateur, qui nommait l'ordre C, qui a
autorisé une septième session sur le même fil.

**L'instrument d'abord (A307).** La phase se projette à **fréquence fixe**, sur une fenêtre commune,
et se **déroule** le long des colonnes : plus de pulsation ajustée, plus de terme de temps. Éprouvé
sur un défaut connu — deux trains exacts dans deux gravités — : −0,85299 tour lu pour −0,85299
attendu, là où l'extrémité seule lisait +0,147. Deux impasses sur la route : le retard de groupe
pris entre les lignes ratait de 11 % (une gaussienne coupée à la naissance a son propre retard) ;
et le déroulement doit être ancré **en `ω`** avant de l'être en `x`, faute de quoi un tour d'écart
entre deux fréquences se lit comme 4,8 s de retard.

**Ce que la mesure attribue** ([preuve](../../docs/validation/ORDRE-C-S316.md), trois mailles, deux
amplitudes, un pas de temps moitié) :

- **primitive** : rien — 10⁻⁴ contre l'évolution exacte de sa propre demande, sans croissance ;
- **raccord** : ≈ 1° de phase, 0,03 s de retard, ≤ 0,3 % d'amplitude, 2 à 4 % de spectre — une
  porteuse sous enveloppe gaussienne ne décrit pas le chirp d'un paquet déjà dispersé ; et une
  direction lue **+1 à +2°** trop ouverte, qui ne converge pas (A309) ;
- **δ** : la phase à dix longueurs d'onde, **déroulée**, −0,914 ; −0,237 ; −0,045 tour, dont la
  part linéaire converge à l'ordre deux ; la vitesse de groupe, −25 ; −7,2 ; −1,65 % ; l'éponge,
  réflexion ≤ 1,5·10⁻⁶ **séparée par sens** ;
- **ce que W ne porte pas** — une colonne que la règle ne prévoyait pas : la non-linéarité. δ porte
  6, 17 puis 24 % de la correction de Stokes, et un transfert d'énergie vers la fréquence centrale,
  les deux en `a²` ;
- **volume net** : d'ordre deux (×4,0 quand `a` double), ≈ 1,5·10⁻⁴ m³ par mètre de crête —
  intégralement en attente, pour l'ordre D.

**Corrections du passé.** Le raccord de S314/S315 tirait de l'onde **posée** sa conversion
d'enveloppe, et faisait naître le train une demi-maille avant la jauge (A308, close dans le banc
de l'ordre C). Les « 6,3 % de dissipation » de S315 étaient une différence d'étalement — à fréquence
fixe, rien ne se perd ; ses prédictions de phase ignoraient le jacobien `1/c_g` entre spectres
temporel et spatial (L365). La réflexion de S314 à 25 cm était lue avec une fenêtre calée sur la
vitesse posée d'un δ 25 % plus lent (L364). Notes datées ajoutées aux deux preuves.

**Prédictions écrites avant, et leur sort.** S315 (−0,853 ; −0,165 ; −0,0072) : manquées de 0,04 à
0,07 tour, par le jacobien. Retard de groupe à 12,5 cm : 1,66 s contre 0,93 s — **la loi en
`(k·dx)²` est réfutée** (1,75 s). Stokes « indépendante de la maille » (P4a′) : **fausse**, elle
converge avec δ. 6,25 cm révisée depuis deux mailles : tenue sous 2 cm, manquée de 0,005 sous 1 cm.
Oblique : 20° tenue, 40° manquée de 0,15°. Pas de temps « suspect » (P2) : **levé**, 7·10⁻⁴ tour.

**A306 close.** Les emplois de l'estimateur de période dans le harnais relus par un second
estimateur, sur le même signal : écarts 0,03 à 0,24 % pour 1 % de marge, aucune réception affectée
— et c'est le périodogramme qui se trompe sur ces signaux de mode (fuite de fenêtre, −0,236 % à huit
périodes). Trois essais permanents `a306_*`. Suite : **571 réussis, 18 ignorés, 0 échec**.

**L'utilisateur, en cours de session.** Il a rappelé que δ doit permettre **plusieurs couches d'eau
sur une même verticale** — des billes ou autres. L'exigence est dans les sources et dans ADR-001 ;
ADR-175 D5 avait démarré la 3D en colonnes et renvoyé le reste au lot 5, après la v1. Il a confirmé
l'ordre, puis suivi la recommandation : **ADR-184**, le lot 5 avance en parallèle du lot 2, par
sessions alternées, en commençant par une comparaison chiffrée.

**Limites.** Un paquet, une ligne, une émission ; trois mailles, donc des limites extrapolées sur
trois points ; couplage à un sens (A302) ; A305 ouverte. Coût : 6,25 cm demande 1 h 52 par calcul
(3 h 07 au pas moitié), le pas de δ 3D sur CPU ne se parallélisant pas.

**Méthode.** Trois fois le battement a été écrit **avant** de lire l'horloge (08:00, 08:09, 10:12
au lieu de 07:49, 08:00, 10:09), chaque fois corrigé au commit suivant ou avant le sien : L237 dit
de lire l'horloge **dans un appel séparé**, et je l'appelais en parallèle de l'écriture.

**Rituel.** Maillons **0**. Capacité : **le transfert δ → W est qualifié propriété par propriété**,
chaque écart attribué, la phase à distance mesurable ; point 4.8 de la liste passé d'absent à
partiel. Consommateurs : la décision de l'utilisateur sur le qualificatif du transfert et l'ordre D ;
la comparaison du lot 5. Deux leçons : L364, L365. Angles morts : A306 et A307 closes, A308 (close),
A309 (ouverte). Recommandation du dernier bilan (S293) : sans objet pour ce lot, l'ordre venant
d'ADR-178 et ADR-184.

## S317 — 2026-09-21 — l'ordre D : le volume net a un receveur, et ce receveur n'est pas le monde

**Entrée.** Décision de l'utilisateur, le soir même : *« On passe à l'ordre D »*. Rien n'a été dit
du qualificatif du transfert : il reste « partiel » (ADR-183 D6, lecture prudente).

**La question posée avant tout code.** ADR-181 D1 nommait V ou le niveau régional de B comme
receveurs. Mais δ tourne sur le client, sans autorité ; B et V sont répliqués ; I-11 interdit tout
chemin du client vers le monde répliqué, et I-15 le tranche sans arbitrage. **ADR-185** : en eau
ouverte, le receveur est le niveau de B **tel que ce client le représente** — le statut de
`W_local` — porté par une région déclarée, adossée à la ligne de contrôle ; en contenant, δ
s'asservit à V (porte E). Construire d'abord aurait bâti un chemin qu'un invariant interdit (L367).

**La grandeur** ([preuve](../../docs/validation/RESTITUTION-S317.md) §2). Le flux à la ligne ferme le
bilan de l'intérieur au plancher d'arrondi : 1,6·10⁻¹⁰ et 2,5·10⁻¹⁰ m³ pour 0,13 et 0,053 m³ de flux
cumulé (prédit ≤ 10⁻¹⁰ : borne trop serrée d'un facteur 3 à 5). Deux faits inattendus : **l'eau entre
aussi par l'arrière** — ce qui sort devant revient derrière à 2 % près à 12,5 cm, le paquet déplace
de l'eau (L366) — et **les éponges en ajoutent**, en ramenant un niveau abaissé au repos.

**Le receveur.** `RegionalLevel` : une région adossée à un segment de ligne, profondeur déclarée,
niveau = volume / aire, frontière à zéro par choix déclaré (ADR-185 D8) ; un `Receipt` qui ne se
construit que par `receive`, ne se copie pas et se consomme au registre ; `Ledger3` gagne
`restituted`, et **`global_conservation_claimable` devient faux dès qu'une région locale a reçu** —
la représentation se ferme, pas le monde. Sept essais, verts au premier passage.

**La restitution sur le banc.** Une région par ligne : attente des deux registres **exactement
nulle**, aucune eau créée, bilan δ + régions à 1,6·10⁻¹⁰ et 2,5·10⁻¹⁰ m³ ; niveaux de −15 à +5 µm —
invisibles, mais comptés. Suite complète : **578 réussis, 18 ignorés, 0 échec** (cœur 457, dont sept neufs).

**Ce que l'ordre D ne fait pas encore.** Le rayonnement de l'anomalie hors de sa région (onde
longue à `√(g·h)`) ; le cas couplé où la bande B/W traverse la ligne (ordre E) ; le contenant, δ
asservi à V (porte E) ; l'affichage du niveau dans la composition ; la production GPU.

**Limites.** Deux lignes de contrôle, pas un contour fermé à quatre côtés ; pas de fond B/W ; un
paquet en eau profonde, deux mailles.

**Rituel.** Maillons **0**. Capacité : **le volume net qui quitte δ a un receveur**, et le bilan de
la représentation se ferme par construction ; points 1.1, 4.8 et 4.18 avancés. Consommateur :
l'ordre E. Deux leçons : L366, L367. Aucun angle mort nouveau — le rayonnement est une limite
déclarée, portée par la file. Suivant, selon ADR-184 : le lot 5.

## S318 — 2026-09-21 — trois façons de porter plusieurs couches d'eau, comparées au même niveau

**Entrée.** *« Continue »*, après S317 ; l'alternance d'ADR-184 désignait le lot 5 : la comparaison
chiffrée des représentations où plusieurs couches d'eau tiennent sur une verticale, avant que
l'utilisateur choisisse.

**Le banc** ([preuve](../../docs/validation/COMPARAISON-LOT5-S318.md)). APIC, ensemble de niveaux et SPH
faiblement compressible, en deux dimensions, **au même niveau** : même grille MAC et même pression pour
les deux qui en ont une, même fluide fantôme, mêmes mesures, mêmes degrés de liberté. Repos,
ballottement contre la dispersion linéaire, rupture de barrage `a × 2a` contre ses invariants — les
données de Martin et Moyce étant derrière un péage, aucun chiffre de mémoire (I-14).

**Ce qu'il montre.** APIC garde la masse **exactement**, ne crée jamais d'énergie, tient la période à
**0,15 %** à 2,5 cm et coûte le moins (10 s par seconde simulée sur le barrage fin). L'ensemble de
niveaux porte l'onde à une fraction de maille (période à 0,19 %) mais **perd ou gagne jusqu'à 7,6 %
de volume** et **crée 8 % d'énergie** dès que l'écoulement devient violent. SPH conserve la masse,
respire de 0,4 % en volume, et coûte **40 fois** APIC par seconde simulée — le pas acoustique.
Les trois fronts s'accordent à 1,6 % : un accord, pas une validation.

**Six fautes attrapées en route**, dont une qui valait leçon : APIC avec `p = 0` au centre des
cellules d'air éteignait le ballottement en trois secondes, à 23 % de période ; avec une surface
**reconstruite** des particules et le fluide fantôme de l'ensemble de niveaux, 0,15 % (L368). Et une
onde plus petite que l'espacement des particules n'existe pas : le protocole est passé de 1 à 2 cm.

**Ce que je n'ai pas su faire.** SPH garde une erreur de période de −6 % et 27 % d'amortissement par
période **qui ne convergent pas** ; quatre diagnostics — diffusion δ, viscosité, parois glissantes,
maille — ne l'isolent pas. Il n'est pas attribué à la famille (A310).

**Proposition à l'utilisateur** : **APIC**, qui partage la grille et la pression du δ construit, porte
les gouttes, et ce que les sources décrivent pour le déferlement. **Le choix est le sien.**

**Limites.** Deux dimensions, deux mailles, trois cas simples ; ni objet, ni cavité — B10 vient à la
prochaine session du fil, sur le candidat choisi ; carte graphique argumentée, pas mesurée.

**Rituel.** Maillons **1** : la comparaison **prépare** une décision de l'utilisateur ; aucun point de
la liste n'avance tant qu'elle n'est pas prise, et le compteur le dit. Une leçon : L368. Un angle
mort : A310. Suivant : l'ordre E du lot 2, selon l'alternance — ou B10 sur le candidat choisi, si
l'utilisateur tranche d'abord.

## S319 — 2026-09-21 — l'ordre E devait commencer par un témoin nul ; le témoin n'est pas resté nul

**Entrée.** *« Continue »*, puis, en cours d'amorce, **« Ok pour APIC »** : ADR-186 acte la seconde
représentation. L'alternance d'ADR-184 gardait S319 pour le lot 2 — l'ordre E, le couplage complet
— et donne B10 sur APIC à S320 ; l'utilisateur en a été prévenu.

**Le premier échelon** ([preuve](../../docs/validation/MER-S319.md)). Une houle B d'une composante autour
du domaine, δ nul au départ : **une mer que δ ne perturbe pas ne doit rien faire restituer**. Elle fait
restituer **292 fois** le volume d'un paquet de 2 cm à 25 cm, 179 fois à 12,5 cm — le critère était
10 %. δ monte à 2,8 cm le long du sens de propagation, et, calé en phase sur B, fait passer à la ligne un
transport croisé que la restitution de S317 prend pour de l'eau sortie de δ.

**Le remède essayé, et pourquoi il échoue.** Un δ témoin, même mer sans paquet, et la différence des flux :
18 fois le paquet encore, parce que **le témoin lui-même monte à 17 cm en 42 s**. Prolongé à deux minutes,
δ sous la seule houle **croît jusqu'à trois fois l'amplitude de la mer** — taux ≈ 0,04 s⁻¹ sous 2,5 cm,
≈ 0,10 sous 5 cm, un peu plus lent à maille moitié : ce n'est pas l'advection centrée, qui irait quatre
fois plus vite. **A289 se matérialise** — B linéaire, δ porteur de l'onde totale non linéaire —, et plus
vite que sa formule ne le disait. **Aucune session n'avait fait vivre δ plus de quelques secondes sous une
houle** (L369).

**Ce que je n'ai pas fait.** E3, le contour fermé, reporté : il n'apprendrait rien qui débloque. La cause
n'est pas démontrée, seulement restreinte.

**Deux fautes de méthode.** Un script d'édition a échoué sur un motif présent dans plusieurs fonctions, et
j'ai compilé puis lancé **l'ancien binaire** sans le voir — L362 encore, par un autre chemin : la sortie
avait l'air juste. Et j'ai d'abord prédit qu'E2 retrouverait le paquet à 30 % près : manqué.

**Arbitrage proposé à l'utilisateur.** Les trois voies d'A289 — rappel lent de δ vers zéro, durée de vie
bornée des domaines, dispersion d'amplitude dans B —, la dernière changeant la mer de tous les clients.

**Rituel.** Maillons **2** : un défaut trouvé n'est pas une capacité reçue. À deux maillons, la suite se
prend dans un lot qui fait avancer une capacité : **B10 sur APIC** (S320), que l'alternance désignait déjà.
Une leçon : L369. A289 relevée ; ADR-186.

## S320 — 2026-09-21/22 — lot 5 : une cavité se pince au même instant à toute échelle, la gerbe suit la maille

**Entrée.** *« Continue »*, après S319 ; l'alternance d'ADR-184 donnait B10 sur APIC.
Coupée à 08:30 pendant le calcul de P5b ; **reprise à chaud à 20:11** (Claude Opus 5.5) : arbre propre, calcul relancé, rituel
terminé sans attendre son résultat, que S321 versera.

**Le corps dans APIC** ([preuve](../../docs/validation/B10-APIC-S320.md) §2). Un cylindre **cinématique** —
aucun corps rigide n'existe (ADR-184 D3) — : cellules solides, faces à sa vitesse, particules repoussées.
Au repos il ne crée pas d'écoulement ; enfoncé lentement, il ne soulevait l'eau que de **39 %** de son
volume : **la masse d'APIC est exacte, son volume géométrique ne l'est pas** (L370), les particules se
tassent contre la paroi. Séparation des paires à 0,4 maille, fixée avant la mesure : 92 % en masse,
**99 % en géométrie** ; sensible (110 % à 0,45).

**B10** (§3 à §5). `Fr` = 1 : fermeture peu profonde, pas de cavité. `Fr` = 2 et 4 : **pincement** à
2,2 et 2,5 `√(D/g)`. Similitude de Froude **à 4·10⁻⁴** entre `D` = 0,4 et 0,8 m, après quatre fautes du
banc que la similitude a trouvées ; la sensibilité (`Fr` × (1 + 10⁻⁶)) dit l'incertitude vraie — 8 % sur
le pincement à `Fr` = 4 (L371). Convergence `D/dx` 8 → 16 : temps de pincement à 5 %, cavité maximale à
9 % ; **la couronne et le jet changent de 40 à 60 %** — sans tension de surface, c'est la maille qui les
arrête (A312). §5 bis, `D/dx` = 32 à `Fr` = 2, est en calcul : relancé à 20:11, versé par S321 à son retour.

**Lecture de simufluid**, à la demande de l'utilisateur ([lecture](../../docs/registres/LECTURE-SIMUFLUID-S320.md)) :
leur résidu dérive sous houle à un taux porté par le **nombre de pas** — S319 n'a jamais varié le pas à
maille fixe, essai à faire avant l'arbitrage d'A289 ; la level set conservative garde son volume mais
échoue sur coque mobile ; la **loi de conservation géométrique** est l'hypothèse de cause du tassement
(A313).

**Limites.** Deux dimensions, corps imposé, air non modélisé (A311), un seul corps, banc hors du cœur ;
aucune référence expérimentale (géométrie plane, I-14).

**Rituel.** Maillons **0** : ce qui devient possible — **une cavité d'air derrière un corps, que la
fonction hauteur ne peut pas porter** ; le chemin qui le consomme — le raccord particules ↔ colonnes,
puis la porte D et C20 ; la preuve — B10-APIC-S320 ; point **4.12** passé d'absent à partiel. Deux
leçons : L370, L371. Trois angles morts : A311, A312, A313. Suivant : **S321, demande de
l'utilisateur** — analyse complète et réorganisation de la méthode. Le fil du lot 5 reprendra au
raccord, précédé du compteur de volume géométrique (A313) ; A289 attend l'essai du pas de temps,
puis la voie de l'utilisateur.

## S321 — 2026-09-22 — analyse complète, et une méthode qui retire au lieu d'ajouter

**Entrée.** Demande de l'utilisateur : analyser code, documents et méthode, réorganiser ce qui peut
l'être. S320, coupée à 08:30, reprise à chaud et close d'abord.
**Constats** ([bilan](../../docs/registres/BILAN-GLOBAL-S321.md)). Code sain : 578 + 36 essais, 0 échec.
Le dispositif s'usait par accumulation — `EN-COURS` 93 → 1 686 lignes, lecture à froid 155 Ko,
une leçon par session et les erreurs récurrentes revenues avec leur leçon écrite, 28 % des commits
récents en plans et rituels. Les réceptions portées par des bancs ne sont protégées par rien ;
APIC n'existe que dans un banc ; décomptes de la liste divergents ; version minimale fausse.
**Refonte** ([ADR-187](../../docs/adr/ADR-187-methode-refondue-s321.md)). Rituel en deux parties ;
lecture à froid 68 Ko ; `EN-COURS` limité à la session en cours ; dix-sept protections actives
dans METHODE, LECONS en archive ; six contrôles de plus dans `etat_projet.py --check`, chacun
contre son contre-exemple réel ; « Reproduire » en tête des preuves ; carte par système en tête
de l'index. Hygiène : `.pyc` retirés, README réécrits, `rust-version` 1.83, zéro avertissement —
dont un vrai défaut, `wake_plafond` imprimait des refus vides. Suites inchangées.
**S320 P5b** : calcul encore en cours à la clôture — reporté, point daté de la file avec sa commande.
Piège d'outillage : l'outil d'édition convertit les échappements Unicode, et le contrôle d'encodage
s'est d'abord attrapé lui-même ; son motif est désormais construit par `chr(92)`.
**Limites.** Aucune physique rejouée hors des suites ; la refonte se juge en S330 (file).
**Rituel.** Maillons **1** : une méthode, pas une capacité. Ni leçon ni angle mort neufs ; quatre
points de file. Suivant, proposé : **S322, lot 2 — l'essai du pas de temps d'A289**, avant
l'arbitrage ; les deux décisions proposées sont au bilan §7.

## S322 — 2026-09-22 — A289 : la croissance de δ sous houle ne suit pas le pas de temps

**Entrée.** *« Continue »* ; suite proposée par S321 et par la lecture de `simufluid` : l'essai du
pas de temps, à faire avant l'arbitrage d'A289.
**Mesure** ([preuve](../../docs/validation/MER-S319.md) §8). E1 de S319 — houle 5 cm, λ = 4 m, δ nul au
départ, maille 25 cm — à 20, 10, 5 et 2,5 ms, soit 80 à 640 pas par période. Taux de `δ_max` entre
10 et 30 s : 0,1015, 0,1012, 0,1009, 0,1007 s⁻¹ — **0,8 %** d'écart, pour moins de 10 % prédits si
le modèle domine. À 10 ms, S319 se reproduit aux chiffres publiés.
**Ce que cela dit.** Pas un défaut d'intégration du pas couplé : chez `simufluid`, le taux changeait
de signe avec le nombre de pas ; ici, il ne bouge pas. L'advection croisée explicite, dont le taux
suivrait le pas, est exclue, comme la maille l'avait fait en S319. Le mécanisme reste à nommer ; la
voie d'A289 reste à l'utilisateur, et un remède numérique du pas ne servirait à rien.
**Méthode.** Première preuve au format de S321 : section datée dans la preuve du fil, « Reproduire »
en tête, sortie brute de la non-régression gardée. P1 committé avec `[>]` une deuxième fois :
écrire P1 déjà coché dans le plan.
**Non fait.** S320 P5b, encore en calcul (file).
**Rituel.** Maillons **2** : un diagnostic qui restreint une cause n'est pas une capacité. La suite
doit en faire avancer une : **S323, lot 5 — le raccord particules ↔ colonnes**, précédé du compteur
de volume géométrique (A313) ; l'alternance d'ADR-184 le désigne aussi. Trois décisions attendent
l'utilisateur (file).

## S323 — 2026-09-22 — lot 5 : un raccord à masse exacte, et la surface qui dépend de l'arrangement

**Entrée.** *« Continue »* ; alternance d'ADR-184, et deux maillons : la session devait viser une
capacité — le raccord particules ↔ colonnes, qu'A313 faisait précéder d'un compteur de volume.
**Le compteur** ([preuve](../../docs/validation/B10-APIC-S320.md) §10). Aire sous la surface
reconstruite, par carrés marchants sur l'interface que voit le fluide fantôme : exacte sur un plan,
d'ordre deux sur un disque. Il mesure enfin le tassement : −1,24 % au corps lent sans séparation,
**−12 %** en B10, ±0,3 % avec. Au repos, le volume géométrique est sous la masse de 0,146 maille par
longueur de surface, et un écoulement s'y relaxe : ce n'est pas une dérive (A313 résolu).
**Le raccord, statique.** Une colonne à un seul segment posé sur le fond passe aux colonnes ; les
autres — corps, cavité — restent aux particules. **La masse par colonne n'est pas une hauteur** : après
une seconde, elle varie du simple au double quand la surface reste lisse. Voie mixte, déclarée avant
sa mesure : forme géométrique, niveau de masse — masse exacte, puis point fixe ; mais la surface saute
de 0,07 à 0,17 maille, jusqu'à 0,35 : le volume d'une masse dépend de l'arrangement (A314). Critère
de 0,2 maille manqué ; la masse tient.
**Non fait.** Le raccord dynamique ; S320 P5b, encore en calcul (file).
**Rituel.** Maillons **3**, justifiés : le raccord ne se construit pas sans son instrument ni sans savoir
quelle grandeur conserver ; les deux sont établis, et la borne trouvée. Aucun point de liste ne bouge.
Suivant : **la décision de l'utilisateur** sur le lot 3 à la place du lot 2 bloqué ; à défaut, le
raccord dynamique. Un angle mort : A314.

## S324 — 2026-09-22/23 — lot 3 : le fond coupé entre dans la référence 3D

**Entrée.** *« Je suis ta recommandation »* : le lot 3 prend la place du lot 2 bloqué dans
l'alternance (ADR-188). Maillons à 3 : il fallait une capacité.
**La géométrie** ([preuve](../../docs/validation/FACES-COUPEES-3D-S324.md)). La découpe de S232 portée en
x-y-z : coins par moyennes emboîtées, empreinte en quatre triangles, intégrales exactes ; les formules
2D servent telles quelles quand le fond ne dépend pas de `y`. Identique au bit à la 2D ; plan exact.
**L'opérateur.** Pondéré comme la 2D, sur un chemin séparé : le fond plat reste S295 au bit. À `ny` = 1
sur les trois fonds de S232, 200 pas linéaires identiques au bit à la 2D ; lac au repos exact ; les
pas mobile et couplé refusent la découpe. Suite : 589 réussis, afficheur 36.
**L'ordre.** Débit ouvert après un pas : témoin sans `y` à 1,4·10⁻⁶ de S232 ; bosse 3D d'ordre
**1,956**. Mais à 128, **16 029 itérations et 708 s** pour un pas, contre 347 et 4 s : les petites
cellules, sans préconditionneur (A315).
**Méthode.** Les longs blocs passés au shell ne s'analysent plus : écrire par fichier. Première
preuve contrôlée par « Reproduire ».
**Non fait.** Mode mobile, obstacles, Jacobi ; S320 P5b, toujours en calcul (file).
**Rituel.** Maillons **0** : ce qui devient possible — un domaine δ 3D sur un fond non plat ; le chemin
qui le consomme — le lot 3, puis les corps du lot 4 ; la preuve — FACES-COUPEES-3D-S324 ; 4.15 avance
(la 3D qu'il déclarait manquante). Un angle mort : A315. Suivant, selon ADR-188 : **S325, lot 5 — le
raccord dynamique** ; le lot 3 reprend ensuite par Jacobi.

## S325 — 2026-09-23 — lot 5 : le raccord dynamique tient la masse, pas encore la frontière

**Entrée.** *« Continue »* ; alternance d'ADR-188 : le lot 5, le raccord dynamique.
**Le montage** ([preuve](../../docs/validation/B10-APIC-S320.md) §11). Dans le banc APIC, la moitié droite
en colonnes transportées par les flux de la grille, particules réensemencées depuis elles ; à la
frontière, une particule libre qui entre verse sa masse à la colonne, le flux sortant devient
particules. Une faute attrapée avant la mesure : une particule de colonne glissée à gauche aurait été
comptée deux fois.
**Ce qui tient.** La masse, à l'arrondi, et l'échange dans les deux sens (0,155 m² entrés, 0,154 sortis).
**Ce qui ne tient pas.** Au repos, 1,4 cm/s de vitesse parasite ; en ballottement, la surface saute
de 1,8 à 2,8 mailles à la frontière, le mode perd jusqu'à 16 % par période. L'hypothèse du lissage
grille → réseau → grille est **contredite** : l'amortissement ne suit pas la taille de la zone
(16 %, 0,9 %, 10 %). Cause non attribuée (A316).
**Non fait.** L'attribution, un suspect à la fois ; S320 P5b, toujours en calcul (file).
**Rituel.** Maillons **1** : un échange à masse exacte, mais pas de frontière reçue ; la liste ne bouge
pas. Un angle mort : A316. Suivant, selon ADR-188 : **S326, lot 3 — Jacobi sur le chemin coupé**
(A315) ; le lot 5 reprendra par A316.

## S326 — 2026-09-23 — lot 3 : les petites cellules préconditionnées ; S320 P5b tranché

**Entrée.** *« continue »* ; alternance d'ADR-188 : le lot 3, contre les 16 029 itérations de S324 (A315).
**Le remède** ([preuve](../../docs/validation/FACES-COUPEES-3D-S324.md) §6). Le Jacobi du mode mobile 3D,
porté au chemin coupé du mode linéaire, seulement quand `ny > 1` : la 3D reste la 2D au bit à `ny` = 1.
Bosse à 128 : **16 029 → 425 itérations, 708 → 5,7 s** ; débits à 2,4·10⁻⁶ près ; ordre 1,947 ; suite :
590 réussis. Le témoin sans `y` coûte un peu plus (347 → 422) : le remède sert les coins.
**P5b, sur deux questions de l'utilisateur** — *« Il n'y a pas de problème avec P5b ? »*, puis *« Ce
calcul est essentiel ou non ? »* : oui, et non. Annoncé pour 1 à 2 h, reporté depuis S321, il tournait
depuis 13 h sans rien écrire. Une trace a montré la vitesse qui s'emballe (9 → 290 m/s) à la fermeture de
la poche sans pression, et le pas à 3·10⁻⁵ s (A311). Arrêté ; la part utile, lue avant le pincement, a
coûté 32 min ([preuve](../../docs/validation/B10-APIC-S320.md) §5 bis) : **le temps de pincement ne
converge pas** à trois mailles (2,20 → 2,30 → 2,40), contrairement à ce que 4.12 affirmait sur deux.
**Méthode.** Deux protections changées : un calcul long écrit sa progression et se diagnostique au
double de sa durée (L372) ; qui change un défaut corrige les « Reproduire » qui le citent.
**Rituel.** Maillons **2** : une optimisation pas encore consommée, et une affirmation retirée ; 4.15 et
4.12 restent partiels. À deux maillons, la suite vise une capacité : **S327, lot 5 — recevoir le raccord
dynamique** (4.12), A316 un suspect à la fois puis corrigé ; troisième session du raccord, vérifiée —
sans lui, APIC reste un banc isolé. A315 résolu ; notes datées sur A311 et A312.

## S327 — 2026-09-23 — lot 5 : le raccord dynamique attribué, pas encore reçu

**Entrée.** *« Continue »* ; alternance d'ADR-188, maillons à 2 : recevoir le raccord (A316).
**La série** ([preuve](../../docs/validation/B10-APIC-S320.md) §12). À la frontière, la première colonne se
vidait par le flux de la grille avant qu'aucune particule ne franchisse, puis les recevait en rafale.
**Deux causes, chacune éprouvée seule.** L'échange asymétrique fait le saut (1,81 → 0,71 maille) ; la
surface des colonnes arrondie au quart de maille fait la dissipation (16 → 6,3 %). Une règle en sort :
tout retard entre la colonne qui reçoit l'eau et les particules qui la perdent agit comme une
résistance. Le meilleur montage — ensemencement continu, frontière en paroi pour les particules — donne
0,24 / 0,59 maille et 4,2 / 0,71 % d'amortissement à 5 / 2,5 cm ; repos 0,65 cm/s ; APIC seul au bit.
**Contredites** : hauteur mouillée centrée, demi-quantum, solde signé, hystérésis, insertion au réseau
(b), mémoire de vitesse sur la grille (d).
**Non reçu** : amortissement à 5 cm (4,2 % pour −0,4 % ± 1), période aux zéros à 5 cm (+1,9 point),
écart à 2,5 cm (0,59, un bruit sans biais). Les colonnes dissipent d'elles-mêmes à 5 cm, cause inconnue.
**Rituel.** Maillons **3**, justifiés : la session visait une capacité et l'a manquée de peu à 2,5 cm ;
deux causes sont attribuées et corrigées, sept hypothèses écartées, et le reste est localisé. Aucun point
de liste ne bouge. Suivant, selon ADR-188 : **S328, lot 3 — le mode mobile sur fond coupé** (4.15), une
capacité à portée ; le lot 5 reprendra par la dissipation à 5 cm, après avoir vérifié qu'elle bloque
encore l'usage — ce sera la quatrième session du raccord.

## S328 — 2026-09-23 — lot 3 : le mode mobile porte le fond coupé

**Entrée.** *« continue »* ; alternance d'ADR-188, maillons à 3 : une capacité était due.
**Le portage** ([preuve](../../docs/validation/FACES-COUPEES-3D-S324.md) §7). Le pas mobile 2D portait déjà
la découpe (S237) ; sa ligne, portée à six faces et pondérée par les ouvertures dans l'ordre de ses
opérations, donne à la 3D un pas à surface mobile sur fond coupé. Garde : deux mailles au-dessus du plus
haut coin du fond. Fond plat au bit de S296 ; le pas couplé refuse toujours la découpe.
**Mesuré.** À `ny` = 1, trois fonds de S232 : 200 pas mobiles **identiques au bit** à la 2D. Lac au repos
exact sur la bosse. Fond sans `y` à `ny` = 4 : tranches identiques au bit à `ny` = 1 ; `v` nul à
l'arrondi du Jacobi, comme sur fond plat. Premier pas sur la bosse : ordre **1,954**, à 3·10⁻⁶ du mode
linéaire. Suite : 594 réussis.
**Méthode.** Un essai exigeait `v` nul au bit, ce que le critère ne demandait pas ; le fond plat de S296
montre le même arrondi : borne relative, et la raison écrite dans l'essai.
**Non fait.** Le pas couplé à B/W sur fond coupé ; les obstacles qui ne sont pas un fond.
**Rituel.** Maillons **0** : ce qui devient possible — une surface libre mobile au-dessus d'un fond non
plat, en 3D ; le chemin qui le consomme — les corps flottants du lot 4 (porte D), puis les obstacles ;
la preuve — FACES-COUPEES-3D-S324 §7 ; 4.15 avance, le mode mobile n'y manque plus. Suivant, selon
ADR-188 : **S329, lot 5** — la dissipation propre aux colonnes à 5 cm, après avoir vérifié qu'elle bloque
encore l'usage (quatrième session du raccord).

## S329 — 2026-09-23 — la v1 d'abord ; lot 3 : un solide quelconque, Archimède exact

**Entrée.** *« continue, jusqu'à la v1 »*. **ADR-189** : les lots 3 et 4 jusqu'à la porte D ; l'alternance
avec le lot 5 suspendue jusque-là — interprétation écrite comme telle, réversible sur un mot.
**Le chemin, lu dans le dépôt.** Lot 3 : obstacles fixes puis mobiles (essais 2 et 3) ; lot 4 : corps
rigide jugé sur C10 ; porte D : un bateau qui flotte sur B + W et que δ voit comme une paroi (I-04).
**La géométrie** ([preuve](../../docs/validation/FACES-COUPEES-3D-S324.md) §8). Un solide par sa distance
signée aux nœuds, coupé exactement pour le champ linéaire — faces en 4 triangles, mailles en 24
tétraèdres, formules closes stables. Volume déplacé d'une sphère d'**ordre 2,0** ; poussée hydrostatique
intégrée sur la paroi discrète **= ρg·V du polyèdre à 10⁻⁹** — Archimède au niveau discret.
**Les pas.** Lac au repos au bit autour de la sphère, modes linéaire et mobile ; débit du premier pas
autour d'elle d'**ordre 1,966**. Un défaut corrigé : l'extrapolation mobile remplissait les faces fermées
d'un solide immergé — invisible sous un fond, donc en 2D.
**Non fait.** Le solide qui bouge ; celui qui perce la surface ; le pas couplé.
**Rituel.** Maillons **0** : ce qui devient possible — un obstacle quelconque immergé dans la référence
3D, la géométrie d'une coque ; le chemin qui le consomme — la frontière mobile, puis le corps du lot 4 ;
la preuve — §8 ; 4.15 avance. Suivant : **S330, la frontière mobile** — l'essai 3.

## S330 — 2026-09-23 — lot 3 : la frontière mobile, la masse ajoutée d'une sphère

**Entrée.** *« continue, jusqu'à la v1 »* (ADR-189) ; suite déclarée par S329 : l'essai 3 de la piscine.
**Le solide bouge** ([preuve](../../docs/validation/FACES-COUPEES-3D-S324.md) §9). L'hôte donne, avant chaque
pas, la distance signée à la nouvelle position et la vitesse ; la découpe est refaite en place depuis le
fond seul, sans allocation ; la part des faces couverte par le solide avance à sa vitesse dans la
divergence ; les faces qui s'ouvrent naissent à sa vitesse ; l'eau déplacée monte dans sa colonne par la
somme compensée. Refus vérifiés avant toute écriture.
**Mesuré.** Solide reposé immobile : 50 pas au bit. Sphère à 1 m/s : volume suivi à 3,6·10⁻¹¹ m³, faces
saines. Départ impulsif : **masse ajoutée `C_m` = 0,508** à 12 mailles par rayon, 1,6 % de la théorie,
convergent — l'excès dans le sens du confinement. Cœur : 482 réussis.
**Méthode.** Une fausse alerte : l'essai calculait sa sphère avec un pas en f64, le volume en f32 converti ;
les deux ne se coupaient pas pareil. Un essai de bit compare deux choses construites de la même façon.
**Non fait.** Rotation ; solide qui perce la surface ; mode mobile et pas couplé avec un solide qui bouge.
**Rituel.** Maillons **0** : ce qui devient possible — un corps que le jeu déplace et que l'eau de δ
entraîne avec sa masse ajoutée ; le chemin qui le consomme — le corps du lot 4, puis le bateau de la
porte D ; la preuve — §9 ; 4.15 avance. Suivant : **S331, le corps rigide sur B + W**, jugé sur C10.

## S331 — 2026-09-23 — lot 4 : le corps rigide du jeu tient C10

**Entrée.** *« continue, jusqu'à la v1 »* (ADR-189) ; suite déclarée par S330 : le lot 4.
**Le corps** ([preuve](../../docs/validation/CORPS-RIGIDE-S331.md)). `rigid_body.rs` : six degrés de liberté,
quaternion, inertie principale, masse ajoutée diagonale ; poussée par un proxy de points volumiques
(ADR-008 §2), exacte pour une ligne d'eau plane ; pas symplectique en `f64`. L'eau n'arrive que par une
requête — B + W, jamais δ (I-04).
**C10.** Tirant **0,243958** m pour 0,243902 ; période **0,990724** s pour 0,990726 ; avec la masse ajoutée
du disque équivalent, rapport **1,40775** — C10 attendait cette masse ajoutée depuis S21. Roulis d'un
pavé plat à 0,04 % de sa période métacentrique. Deux trajectoires identiques au bit.
**Un fait écrit avant la mesure.** Le cube de C10 à 500 kg/m³ en mer a `GM` = −4,3 cm : instable en
roulis, comme un cube réel de cette densité. L'essai de pilonnement reste bref.
**Non fait.** La houle derrière la requête d'eau ; le corps dans δ ; la masse ajoutée en rotation.
**Constat pour la suite.** Ni l'afficheur ni le harnais n'appellent les pas CPU de δ : la scène rendue
passe par la production GPU. La porte D se recevra sur la référence CPU (ADR-178 D4, D5).
**Rituel.** Maillons **0** : ce qui devient possible — un objet qui flotte sous l'autorité de B + W ; le
chemin qui le consomme — le corps dans δ, puis le bateau de la porte D ; la preuve — CORPS-RIGIDE-S331 ;
6.1 avance. Suivant : **S332, le corps dans δ**.

## S332 — 2026-09-23 — lot 4 : le corps dans δ, et δ ne le pilote jamais

**Entrée.** *« continue, jusqu'à la v1 »* (ADR-189) ; suite déclarée par S331.
**Le couplage** ([preuve](../../docs/validation/CORPS-RIGIDE-S331.md) §4). Paroi de δ en mouvement de corps
rigide ; coque qui perce le couvercle linéaire — l'opérateur pondérait déjà la condition de surface par
l'ouverture ; le corps du jeu donne sa coque à δ à chaque pas, et la force de δ n'anime qu'un ressort
visuel borné à 8 cm.
**Mesuré.** Trajectoire de jeu **identique au bit** avec ou sans δ (I-04) ; décalage visuel 6,7 cm au plus.
Lac au repos exact autour de la coque ; volume tenu à 2,7·10⁻¹⁰ m³. Masse ajoutée du cube de C10 selon δ :
**41,7 kg**, 0,67 de sa masse — le disque de C10 la surestime de 47 %, le rapport des périodes deviendrait
1,291, encore dans sa tolérance.
**Un critère manqué, publié.** Sphère qui tourne sur elle-même : 1,08 % de `Ω·R` à 6 mailles par rayon,
0,73 % à 12, ordre 0,56 — la vitesse de paroi est prise au centre des faces. Remède nommé.
**Non fait.** La houle, la paroi relative à l'eau qui la porte, la rotation du décalage visuel.
**Signalé.** La porte D cite C13 et C14 — bulle et moutons —, qui ne portent pas sur les solides.
**Rituel.** Maillons **0** : ce qui devient possible — un corps de jeu qui remue l'eau de δ sans que δ ait
d'autorité ; le chemin — la porte D ; la preuve — §4 ; 6.1 avance. Suivant : **S333, la porte D**, puis
le verdict visuel de l'utilisateur.

## S333 — 2026-09-24 — porte D : une coque sur la houle de B, et δ qui porte sa perturbation

**Entrée.** Reprise à chaud, *« Reprends le projets »* : S333 coupée à 23:47 au début de P2, diff cohérent
(`BackgroundWater`) → **complété**. Plan amendé avant le code : P2 bis ajouté, P3 et P4 précisés.
**Fait** ([preuve](../../docs/validation/PORTE-D-S333.md)). B derrière la requête du corps ; la poussée suit le
gradient de la pression du proxy — sur une houle, le corps était soulevé sans être entraîné (ADR-008 §2) ;
la coque dans δ **relative à l'eau qui la porte**, pose f32, paroi par différence finie ; banc `porte_d`, images.
**Mesuré.** Pilonnement forcé 1,04889·a pour 1,04892 prédit ; cavalement 0,99059·ξ ; au repos relatif, δ au
repos au bit ; scène : trajectoire de jeu au bit sur 800 pas, anneaux de 9,4 cm pour un lâcher de 10 cm.
**Manqué, publié.** Volume de δ à 3,8·10⁻⁹ m³ pour 10⁻⁹ : plancher f32 du transport — témoin sans coque,
3,6·10⁻¹⁰ en 200 pas. La tolérance n'est pas réécrite.
**Trouvé.** **A317** : une paroi qui ne laisse qu'une lamelle de 8 % dans sa maille rend le rayonnement de δ
dissymétrique à 8 s ; scène posée à 30/30. Le jeu, lui, ne roule pas : impasse du roulis paramétrique écartée.
**Non fait.** W derrière la requête ; amortissement par rayonnement ; rotation du décalage visuel ; la coque
dans la production GPU.
**Rituel.** Maillons **0** : ce qui devient possible — un bateau qui flotte sur la houle de B et rayonne dans
δ sans autorité sur le jeu ; le chemin — le verdict de la porte D, donc la v1 ; la preuve — PORTE-D-S333 ;
6.1 avance, 6.2 passe à partiel. Suivant : **le verdict visuel de l'utilisateur** (ADR-189 D3), puis A317.

## S334 — 2026-09-24 — A317 : la paroi dans sa maille, un défaut de structure du couvercle

**Entrée.** *« Continue, pour la référence je n'ai pas trouvé »* — R15 sans référence ni verdict ; suite : A317.
**Fait** ([preuve](../../docs/validation/PORTE-D-S333.md) §6). Banc `a317_lamelle` — pilonnement imposé, 3D et
tranche ; **couvercle partiel** : surface `(η − z₀)/a`, dépôt de la coque exclu, eau poussée par une paroi
qui glisse rendue aux voisines ; commutable.
**Mesuré.** 3D à 25 cm : 35 % d'écart selon le placement. Tranche : le couvercle de S332 **ne converge pas**
(38,5 → 44,2 → 30,5 %), le partiel si (33,4 → 10,9 → 1,8 %), moyenne extrapolée 19,98 mm, S332 11 % dessous.
Porte D : dissymétrie des flancs 3,42 → 1,22.
**Une erreur de parcours, publiée.** P3 a jugé le diagnostic faux sur une seule maille ; la convergence l'a
rétabli. Et diviser par `a` exige d'exclure l'eau que la coque vient de déposer.
**Manqué.** Critère 9 : 5,47 m/s dans la contre-épreuve de S333 — le résidu de rotation de S332, amplifié en
lamelle — : couvercle partiel **éteint par défaut**, S3xx au bit. Critère 5 : flancs à 1,22, non ± 5 %.
**Non fait.** Vitesse de paroi au centroïde de la part couverte ; résolution près des coques (~25 mailles
pour ± 2 %) ; 3,125 cm : gradient conjugué non convergé.
**Rituel.** Maillons **1** : A317 attribué, remède construit mais éteint — aucune capacité reçue par défaut.
Suivant : **S335, la vitesse de paroi au centroïde** — troisième session sur ce fil, justifiée : elle seule
allume le couvercle partiel, sans lequel une coque qui bouge dans δ rayonne selon sa maille. Verdict attendu.

## S335 — 2026-09-24 — A317 corrigé : le couvercle partiel par défaut, et une impasse

**Entrée.** *« Continue »* ; suite déclarée par S334 — troisième session sur ce fil, justifiée au journal S334.
**Impasse, publiée** ([preuve](../../docs/validation/PORTE-D-S333.md) §7). La vitesse de paroi au centroïde exact
de la part couverte — le remède nommé en S332 — ne retire pas les pointes du couvercle partiel (5,46 m/s) ;
elle divise par deux le résidu de la sphère qui tourne sur les faces ouvertes, mais le double sur les faces
presque fermées, où le polyèdre discret pompe. Écartée ; le diagnostic de S334 était faux.
**La cause, mesurée.** Les colonnes ouvertes à moins de 10 % : un plancher d'ouverture de 10 % ramène la coque
tenue sur la houle à 0,55 m/s. La tranche converge encore : 42,8 → 10,6 → **3,1 %**, moyenne 20,51 mm.
**Allumé par défaut.** 496 réussis ; trois valeurs S3xx changent, publiées. Porte D : 30/30 au bit de S334,
8/52 à 1,22 entre flancs (S332 : 3,42).
**Non fait.** La résolution près des coques — ± 43–49 % à 25 cm, ~25 mailles pour ± 3 %.
**Méthode.** Un battement écrit avant de lire l'horloge (P6, deux minutes d'avance) : L237 rappelée.
**Rituel.** Maillons **0** : ce qui devient possible — une coque dans δ qui rayonne sans dépendre de sa
position sous la maille, à la résolution près ; le chemin — la porte D, un bateau qui bouge dans δ ; la
preuve — §6–7, correction d'intégrité reproduite puis éprouvée. Suivant : **S336, l'amortissement par
rayonnement de la coque**, mesuré par δ hors ligne ; verdict visuel toujours attendu (R15).

## S336 — 2026-09-24 — porte D : la coque qui cesse de pilonner

**Entrée.** *« Continue »* ; suite déclarée par S335. Verdict visuel toujours attendu (R15).
**Fait** ([preuve](../../docs/validation/RAYONNEMENT-COQUE-S336.md)). Banc `rayonnement_coque` : pilonnement imposé,
force de δ décomposée en masse ajoutée et amortissement de rayonnement. Corps du jeu : `radiation_damping`,
linéaire en la vitesse relative à l'eau ; masse ajoutée sur l'accélération **relative** — celle de S331
agissait sur l'absolue, fausse sur la houle (+6 % à 6 s, +33 % à 3 s).
**Mesuré.** A ≈ 0,9–1,05 fois la masse, B 4 800–7 100 N·s/m ; archétype A = 3 200 kg, B = 6 400 N·s/m, ζ = 0,158,
± 10 %. Lâcher en eau calme : période et décrément à 0,05 %. Scène : la coque s'arrête en 3–4 s ; elle dissipe
**353 J**, δ en reçoit **345 — rapport 0,977**.
**Manqué, publié.** Le résidu de l'ajustement, 5–9 % pour 5 % : sauts de découpe quand le fond franchit une
face.
**Non fait.** Les autres degrés de liberté ; la dépendance en fréquence ; W derrière la requête.
**Rituel.** Maillons **0** : ce qui devient possible — un bateau de jeu dont le mouvement et l'eau qu'il
remue sont cohérents en énergie ; le chemin — la porte D ; la preuve — S336 ; 6.1 avance. Suivant : **le
verdict visuel** sur les images de S336 (ADR-189 D3) ; sans verdict, W derrière la requête du corps.

## S337 — 2026-09-24 — la coupure au bord de δ, d'après le verdict R15

**Entrée — verdict R15** sur les images de S336 : *« 1. Oui »* (le bateau qui se pose est juste) ; *« 2. On voit
la coupure encore »* (le bord de la grille de δ) ; *« 3. Pas forcément »* (pas d'autre défaut).
**Fait** ([preuve](../../docs/validation/PORTE-D-S333.md) §8). Le pli était le bord d'un domaine local, non de la
physique de l'eau : le banc linéaire avait des murs et ajoutait δ jusqu'au bord, là où la production amortit
par une éponge et fond en cosinus (R11 : sans raccord visible). **Éponge du mode linéaire** — celle du pas
couplé, volume retiré compté — et **fondu de composition** de la production dans le rendu de la porte D.
**Mesuré.** Tranche : énergie restante 0,4 % de celle des murs, volume au plancher. Scène de 16 s : plus de pli ;
agitation du centre après 12 s 4,0 mm contre 8,2 avec des murs ; l'éponge retire 0,49 m³, l'eau que la coque
déplace en se posant ; I-04 au bit.
**Non fait.** Le retour δ → W (A289), qui rendrait ces anneaux à la mer au lieu de les éteindre ; le tangage
de la coque, non amorti, qui remue encore l'eau.
**Rituel.** Maillons **0** : ce qui devient possible — une scène de la porte D dont le bord de δ ne se voit
pas, sur la décision de l'utilisateur (R15) ; le chemin — la réception visuelle de la porte D ; la preuve — §8.
Suivant : **le verdict sur les images de S337** ; s'il reçoit la porte D, la consigner — puis W derrière la
requête du corps.

## S338 — 2026-09-24 — la porte D reçue sur la référence CPU

**Entrée — verdict final** sur les images de S337 : *« Plus de coupure »*. Avec R15 (*« 1. Oui »*, *« 3. Pas
forcément »*), le critère visuel de la porte D est tenu.
**Fait.** La porte D est consignée **reçue sur la référence CPU** — son « reçu si » tient par S333–S337 et R15
([preuve](../../docs/validation/PORTE-D-S333.md) §9) — dans la revue visuelle, la feuille de route, la liste, la file
et REPRISE. **La v1 n'est pas atteinte** : ADR-174 D4 la définit par les portes A, B, C et D ; le tableau
d'ADR-189 §1 ne nommait que la D (note datée) et REPRISE écrivait « v1 = porte D franchie » (corrigé). Liste :
**6.4** passe à partiel (3 / 55 / 62) — les parois mobiles de S330–S332 n'y étaient pas reportées — ; 4.15, 6.1,
6.3, 6.5 actualisés. Le tableau des portes citait C13 et C14, bulles et écume, pour D : C10, C11, C23.
**Limites.** Réception sur B : W, nul dans la scène, n'est pas derrière la requête ; aucune coque dans la
production GPU ; B6 sur un archétype ; le verdict juge la perception, sans référence réelle.
**Erreur de parcours.** La note d'ADR-189 disait que la porte B n'attend que le verdict ; les cas 1 et 2
d'ADR-175 §4 sur la production manquent aussi (CUVE-GPU-S305 §7) — correctif daté, feuille de route corrigée.
**Arbitrage réel.** Le terme de la suspension du lot 5 (ADR-189 D2) : la porte D, ou la v1 entière.
**Rituel.** Maillons **0** : ce qui devient possible — la porte D franchie, la v1 passe aux portes B, C et A ;
le chemin — la porte B ; la preuve — PORTE-D-S333 §9. Suivant : **porte B**, la revue R16 — une onde de δ née
d'un impact sur la mer de référence provisoire (R14) — pour le verdict ; puis les cas 1 et 2 sur la production.

## S339 — 2026-09-24 — porte B : une onde née d'un point, sur la mer de R14 (revue R16)

**Entrée.** Porte en cours B (§3 bis), après la porte D (S338). Critère 3 : une onde qui traverse une mer étalée
et s'y déforme, jugée convaincante ; R11 ne savait pas dire si le front était « circulaire ou bien linéaire » ;
R14 a fixé la mer de référence, que δ n'avait jamais traversée.
**Fait** ([preuve](../../docs/validation/SCENE-DELTA3D-S302.md) §8). Deux défauts de chemin : les captures de δ 3D
partaient avant la lecture de `--eau-physique` et des autres options de rendu — déplacées, scène de S302 au bit — ;
la scène ne savait injecter qu'un front. **L'impact** (cratère de Cauchy–Poisson, 65 cm sur 5 m, pente sous 0,26) :
stable, anneaux à la vitesse de groupe, mais de 10 à 17 cm — **invisibles** dans une mer de `Hs` 2,5 m. **L'anneau
préparé** (41 cm pour 10 m, `a·k` 0,26 comme le front) : stable, crête au trajet prévu ; **il se lit** et se déforme.
**Limites.** Anneau préparé, non né d'un impact ; volume net 9,6 m³ ; fine ligne claire sur sa crête, non
attribuée ; bande claire en bas de la pose rasante, préexistante ; domaine de 30 m, plafonné par le tampon des faces.
**Non fait.** Les cas 1 et 2 d'ADR-175 §4 sur la production (critère 2) ; la porte C.
**Rituel.** Maillons **1** : R16 prête, aucun critère « reçu si » n'a encore bougé. Suivant : **le verdict R16** ;
s'il reçoit le critère 3, les cas 1 et 2 sur la production.

## S340 — 2026-09-24 — la porte B reçue

**Entrée — verdict R16** : *« Tout parrait bon visuellement »* — critère 3 de la porte B. Restaient les cas 1 et
2 sur la production (critère 2 d'ADR-175 §4).
**Fait** ([preuve](../../docs/validation/CUVE-GPU-S305.md) §9). `Background::from_components` : un fond de B par ses
composantes. **Cas 2** : houle de B selon `x` ; la carte reste invariante en `y` à 1 ulp et suit la référence à
1,0·10⁻⁶ m ; à 10 cm de houle, la surface franchit le centre de maille et A297 décroche la carte à 1,25 s — le
cas retenu reste sous le seuil, comme S305. **Cas 1** : l'onde stationnaire de S297 faite de deux composantes de
B ; ce fond s'écarte de l'analytique de 4,4 % dans l'eau, de 33 % au-dessus du plan moyen ; la référence sur ce
fond tient les tolérances de S253 contre HOS (0,98 / 1,07 % à 128) ; la carte la suit sous 10⁻⁴ m aux trois
maillages. **Porte B reçue** (critères 1 à 3 ; le coût à la porte C).
**Limites.** Cas 1 sur le fond de B, non sur l'analytique ; pente à 8,9·10⁻³ au maillage le plus fin, non
attribuée ; A297 et A289 ouvertes ; coût 4,6 ms pour 2 ms.
**Rituel.** Maillons **0** : ce qui devient possible — la porte C, qui se reçoit sur la scène de B ; le chemin —
le coût de δ sur cette scène ; la preuve — §9. Suivant : **porte C**, puis A. En attente : le terme de D2.

## S341 — 2026-09-24 — porte C : où vont les 4,5 ms

**Entrée.** *« Continue »*, après la porte B reçue. Porte en cours : C — δ ≤ 2 ms GPU au 99ᵉ centile par image sur
la scène de B. On savait 4,62 ms en médiane de banc ; ni le 99ᵉ centile, ni la part de chaque étage.
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md)). Horodatage des trois passes du pas, sans effet sur lui
(surface identique au bit sur 60 pas). Sur secteur, témoin de S302 rejoué à 4,630 ms : **pas de 4,45 ms, 99ᵉ
centile 4,5 à 4,65** ; l'évaluation du fond de B **seule 1,53 ms**, le reste de la première passe 0,46, la projection
2,06 — **0,087 ms plus 0,062 par cycle** —, correction et transport 0,39. Le 99ᵉ centile suit la médiane à 1–4 % :
la porte C est une affaire de moyenne. Techniques présentes et absentes publiées (ADR-131 D3).
**Décidé sur la mesure.** Premier levier : l'évaluation du fond — factorisation de la phase par colonne et de
l'atténuation par couche, et charge utile réduite (26 champs écrits, une dizaine lus), toutes deux recevables au
bit. Puis la projection (multigrille, fusion), puis la cadence découplée (I-05).
**Limites.** Pas soumis seuls ; ni rendu concurrent ni recouvrement ; une seule scène.
**Rituel.** Maillons **1** : une mesure, aucun critère franchi. Suivant : **le fond de δ factorisé et allégé**,
reçu au bit. En attente : le terme de D2 (lot 5).

## S342 — 2026-09-24 — porte C : le fond de δ factorisé

**Entrée.** S341 : l'évaluation du fond de B, 1,53 ms des 4,45 du pas, premier levier désigné.
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §6). `sample_faces_tiled` : tuiles de 16 colonnes × 16
couches, sinus et cosinus par colonne, atténuation par couche, en mémoire de groupe, même accumulation. **Au
bit** : 29 871 296 valeurs du fond identiques à trois instants, 60 pas de production identiques. Par défaut
jusqu'à 64 composantes.
**Mesuré** (secteur, même session) : fond 1,527 → **1,237 ms**, pas 4,456 → **4,348 ms**, q99 4,501 → 4,405.
**Ce que cela apprend.** Le calcul transcendant n'était pas l'essentiel du fond : restent l'accumulation des 26
champs de chaque face et leur écriture, 120 Mo par pas, dont le pas ne lit qu'une dizaine selon l'axe.
**Limites.** −0,11 ms au pas ; loin des 2 ms, comme attendu d'un levier seul (ADR-131 D4).
**Rituel.** Maillons **2** : aucun critère de porte franchi depuis S340 ; la suite reste sur la porte C, qui ne se
franchit que par la combinaison — justification : c'est la porte en cours, et chaque levier est mesuré. Suivant :
**n'écrire que les champs lus**, reçu au bit.

## S343 — 2026-09-24 — porte C : dix champs par face

**Entrée.** S342 : le fond factorisé ne retirait que 0,11 ms ; restaient l'accumulation et l'écriture des 26
champs de chaque face, dont le pas n'en lit que dix.
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §7). `override COMPACT` : le fond du pas écrit dix champs
par face selon son axe ; les bancs de S300, sans la constante, rendent leurs nombres publiés. **Au bit** :
empreintes de la surface et des vitesses après 60 et 600 pas, relevées avant, identiques après.
**Trouvé** : depuis S342, créer le pas prenait **247 s** — la mise à zéro de la mémoire de groupe, déroulée par le
compilateur Dx12 ; désactivée pour le fond du pas, qui l'écrit avant de la lire : **4,8 s**.
**Mesuré** (secteur) : **pas 3,679 ms, q99 3,727** — contre 4,452 / 4,505 en S341 ; la projection, inchangée,
pèse 56 %.
**Limites.** Il manque 1,73 ms au 99ᵉ centile ; aucun critère de porte franchi.
**Rituel.** Maillons **3** — justification : la porte C est la porte en cours et ne se franchit que par la
combinaison (ADR-131 D4) ; chaque session a livré une part mesurée et reçue au bit (−17 % en trois) ; la suite
débloque encore la v1, qui l'exige. **Erreur de parcours**, vue à la clôture : §6.4 interdisait déjà cette
troisième session consécutive sur le même point ; la justification des maillons ne la lève pas. Suivant : **la
porte A** ; la projection de la porte C reprend après.

## S344 — 2026-09-24 — porte A : deux domaines δ 3D se disputent un budget

**Entrée.** *« Continue »* ; §6.4 écartait une quatrième session sur la porte C. Porte A, premier critère :
plusieurs candidats réels se disputent un budget, sur des domaines 3D.
**Fait** ([preuve](../../docs/validation/ARBITRAGE-3D-S344.md)). Deux pas de production — la scène de la porte B, à 60 m
l'un de l'autre —, une caméra qui longe la côte, l'ordonnanceur de S278 sous un budget de banc de 5 ms. Parts d'écran
mesurées ; seuils calibrés dessus, 0,10 / 0,05. Un premier trajet, tête tournée seulement, écarté : B y reste plus
petit que A — la surface décide.
**Mesuré.** Budget tenu (3,720 ms au pire) ; le plus visible servi hors de la bande d'hystérésis (0,700 s par
bascule) ; extinction 1,000 s après le seuil ; 6 images affamées par bascule ; coûts médians 3,69 et 3,68 ms.
**Trouvé.** Sans l'oubli des coûts, le premier pas — carte froide, 22,6 ms — exclut le domaine pour toujours :
l'exclusion absorbante de S279 §4, reproduite en 3D ; l'oubli de S286, recopié, la lève.
**Limites.** Un domaine vit ou meurt : ni déplacement, ni redimensionnement, ni dégradation de rang 1. L'oubli vit
dans chaque hôte ; sa place est dans le cœur.
**Rituel.** Maillons **0** : ce qui devient possible — l'ordonnanceur arbitre des domaines 3D réels ; le chemin — les
deux autres critères de la porte A ; la preuve — ARBITRAGE-3D-S344. Suivant : **porte C, la projection**.

## S345 — 2026-09-24 — porte C : la cadence de 30 Hz, éprouvée

**Entrée.** Le pas coûte 3,68 ms ; ADR-012 §7 fixe δ à 30 Hz, le rendu interpolant : étalé sur deux images, le
pas contribuerait ≈ 1,85 ms par image. Avant de l'étaler, la physique à 33,3 ms. La fusion des réductions de la
projection, au bit, a été écartée à la lecture : chaque groupe relirait les 5 880 partiels.
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §8). **Cuve de S305** à 1, 16,7 et 33,3 ms : période à
0,039 %, amplitude à 0,013 % — **tenu**. **Scène de B**, 30 contre 60 Hz, 12 s : aucune colonne hors bornes ;
l'onde isolée **jusqu'à +6,3 %** d'amplitude — **non tenu** (5 %). Témoin (60 Hz, 64 cycles au lieu de 32) : 1,3 %
d'amplitude, mais le maximum y saute aussi de 8 m — la partie « position » de mon critère était mal posée. 30 Hz à
64 cycles : 8,4 % — pas la projection ; l'éponge est exacte en temps.
**Limites.** Candidat non démontré : la dissipation de l'advection par la mer, par pas. Aucune des deux cadences
n'est « la vraie ». Angle mort **A318** (sévérité 2).
**Erreur de parcours.** Un commit intermédiaire a laissé passer une ligne de file à 100 mots : le contrôle,
enchaîné derrière un `tail`, ne bloquait pas ; ligne corrigée, contrôle désormais lu avant chaque commit.
**Rituel.** Maillons **1** : aucun critère franchi. Suivant : **attribuer A318** — le front sur une mer au repos,
aux deux cadences ; puis étaler le pas sur deux images et mesurer le 99ᵉ centile par image.

## S346 — 2026-09-24 — porte C : A318 attribué en partie

**Entrée.** S345 : à 30 Hz, l'onde de la scène de B garde jusqu'à 6,3 % d'amplitude de plus qu'à 60 Hz (A318).
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §9). Le front sur **une mer au repos** : l'écart à 30 Hz
tombe à 1–4 % — la mer en rajoute, sans en être la seule cause. **Une cadence de 40 Hz** : ≤ 0,73 % tant que l'onde
est groupée, au niveau du témoin des cycles ; au-delà, écarts sans ordre (horizon d'A297). Tant que l'onde est
groupée, l'écart **suit le pas**, toujours du même signe : un **amortissement numérique par pas**, dont le candidat
est l'advection non linéaire du pas couplé.
**Limites.** Candidat non localisé dans le code ; aucune cadence n'est « la vraie ». À 40 Hz le pas contribuerait
≈ 2,45 ms par image : seul 30 Hz passe sous les 2 ms.
**Rituel.** Maillons **2** : aucun critère franchi depuis S344. Comparée à la porte A (déplacement,
redimensionnement), la revue R17 est courte et son verdict décide de la voie de la porte C. Suivant : **revue R17**
— la scène de B à 30 et 60 Hz, côte à côte —, arrêt pour le verdict.

## S347 — 2026-09-24 — porte C : revue R17, δ à 30 Hz contre 60 Hz

**Entrée.** A318 attribué en partie (S346) : à 30 Hz, une onde forte garde quelques pour cent d'amplitude de plus ;
savoir si cela se voit appartient à l'utilisateur (ADR-189 D3).
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §10). `--pas-delta` règle le pas de la scène ; sans lui,
captures de S302 identiques au bit — et les leviers de S342–S343 n'avaient bougé aucun pixel. La scène de la porte B
sur la mer de R14, à 2, 5 et 8 s, quatre poses, aux deux cadences : les mêmes vagues à l'œil ; 9 à 41 % des pixels
diffèrent, 1,3 à 15 % de plus de 4 niveaux — un grain fin sur l'emprise de δ.
**Limites.** Instants à 60 µs près entre cadences ; images fixes : l'interpolation du rendu, que 30 Hz demandera,
n'est pas montrée.
**Rituel.** Maillons **3** — justification : R17 est une question à l'utilisateur, qui décide de la voie de la
porte C ; aucune session de plus sur ce point sans son verdict. Suivant : **le verdict R17** ; s'il ne voit rien,
étaler le pas sur deux images ; sinon, la porte A pendant que l'écart se corrige.

## S348 — 2026-09-24 — porte C reçue sur le banc

**Entrée — verdict R17** : *« Continue je valide »* — la cadence de 30 Hz d'ADR-012 §7 validée à l'œil (A318 accepté).
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §11). Le pas de 33,3 ms coupé en **deux parts**, une par
image de 60 Hz — fond, prédiction, couplage et 7 cycles ; puis les 25 autres, le résidu et la fin du pas — : mêmes
dispatchs, même ordre ; **60 pas identiques au bit** au pas d'un seul tenant, dont l'empreinte n'a pas bougé.
**Mesuré** (secteur, 1 000 pas) : **1,848 et 1,918 ms au 99ᵉ centile**, 1,934 au pire ; le balayage de `k` donne 7
pour l'équilibre. **Porte C reçue sur le banc** (ADR-175 §4.4).
**Limites.** Parts soumises seules, sans rendu concurrent ; l'interpolation du rendu d'ADR-012 §7 manque — δ change
à 30 Hz dans une image à 60 ; marge de 4 % ; une scène, un domaine.
**Écart de méthode.** P3 et P4 dans un même commit : le plan les séparait.
**Rituel.** Maillons **0** : ce qui devient possible — δ tient son budget par image sur la scène de la porte B ; le
chemin — la v1, dont il ne reste que la porte A ; la preuve — §11. Suivant : **porte A**, un domaine qui se déplace
et se redimensionne ; l'interpolation du rendu, suite de la porte C.

## S349 — 2026-09-24 — porte A : un domaine qui se déplace

**Entrée.** Porte C reçue (S348) ; la v1 ne demande plus que la porte A. Deuxième critère, première moitié : un
domaine qui se déplace au lieu d'être allumé ou éteint.
**Fait** ([preuve](../../docs/validation/ARBITRAGE-3D-S344.md) §5). La position du domaine n'entre dans le pas que par le
fond de B. `Step3::shift(di, dj)` décale l'état de mailles entières — vitesses, surface et reste, pression de
départ, surface publiée — et avance l'origine du fond ; ce qui entre naît au repos. **Au bit** : sept tableaux
identiques à l'ancien translaté après (+3, −2) ; le pas entier garde ses empreintes.
**Mesuré.** Sur la côte de S344, **un seul domaine suit la caméra** : 480 décalages d'une maille sur 120 m, part
d'écran constante (0,1629), une naissance au départ et **aucune extinction** — deux domaines fixes en demandaient
deux de chaque ; aucune colonne hors bornes. Décalage : 1,46 ms en médiane, sept soumissions séparées.
**Limites.** Le décalage n'est pas groupé en une soumission ; l'hôte, non l'ordonnanceur, décide du déplacement ;
ce qui sort n'est pas rendu à W (A289).
**Rituel.** Maillons **0** : ce qui devient possible — un domaine δ 3D qui accompagne ce qu'on regarde ; le chemin
— la porte A, dont restent le redimensionnement et le rang 1 ; la preuve — §5. Suivant : **se redimensionner**,
puis la dégradation de rang 1.

## S350 — 2026-09-24 — porte A : un domaine qui se redimensionne ; la liste du projet fini actualisée

**Entrée.** S349 : un domaine se déplace ; reste du critère, se redimensionner. Coupée pendant P4 (après 20:52) ;
**reprise à chaud** à 21:06 sur *« Reprends le projets, et mais à jour le document de la to do list »* : aucune
autre session en ligne, diff de P4 cohérent → **complété** ; la liste du projet fini (ainsi nommée en S271) ajoutée au plan.
**Fait** ([preuve](../../docs/validation/ARBITRAGE-3D-S344.md) §6). Les murs : le décalage de S349 recopiait les faces
normales du bord, que le pas n'écrit jamais — 7,2 cm d'écart en 2 s près du mur ; laissées nulles, correction datée
au §5. Une forme courante sous la capacité réservée ; `Step3::resize` en une soumission, `shift` à forme égale.
**Mesuré.** Rétréci 120×112 → 90×84 puis élargi → 110×96 : état au bit, murs nuls, entrant au repos ; un domaine
**créé** à la même forme, même état, **identique au bit après 60 pas** ; allocateur inchangé. Coût **0,09 + 3,57 ms
× surface** (3,70 / 2,74 / 1,83 / 1,02 ms à 100 / 75 / 50 / 25 %, secteur) ; décalage 1,46 → 0,38 ms. Empreintes S343 inchangées.
**Liste** ([LISTE-PROJET-FINI](../../docs/LISTE-PROJET-FINI.md)) : chaque session de S309 à S349 relue contre les 120 points ;
dix-neuf retouchés, trois de plus pour S350 ; **4.13 passe à partiel** — le proche-coque de la porte D ; **3 / 56 / 61**,
aucun point validé : trois portes de la v1 reçues depuis S309 sans amener un point à son périmètre final.
**Limites.** L'hôte décide de la forme, pas l'ordonnanceur ; capacité fixée à la création, `nz` fixe ; aucun verdict
visuel sur une croissance ; ce qui sort n'est pas rendu à W (A289). Afficheur : 36 essais réussis, 0 échec.
**Écart de méthode.** L'identité de S349 comparait à l'ancien translaté, murs compris : elle ne pouvait pas voir des murs figés.
**Rituel.** Maillons **0** : ce qui devient possible — rétrécir un domaine libère son coût au prorata de sa surface ;
le chemin — la dégradation de rang 1, dernier critère de la porte A, donc de la v1 ; la preuve — §6. Suivant : **le
rang 1**. En attente : le terme de D2 (lot 5). File, feuille de route, index et liste à jour.

## S351 — 2026-09-24 — porte A : le rang 1 ; les quatre portes reçues, la v1 atteinte au sens d'ADR-174 D4

**Entrée.** *« Continue, après la V1 ton objectif seras de completer entièrement la to do liste »* : **ADR-190** —
après la v1, la liste du projet fini entière, points validés au périmètre final ; le lot 5 reprend après la v1 (D2
d'ADR-189 tranchée). Porte en cours : A, dont restait la dégradation de rang 1 d'ADR-012 §4.
**Fait** ([preuve](../../docs/validation/ARBITRAGE-3D-S344.md) §7). Dans `scheduler.rs` : `Shrink` déclaré, focal servi
entier, non-focaux à une échelle commune, descente immédiate, remontée rampée après une seconde ; sans déclaration,
S278 au bit (empreinte `6aebff024c734fc9`). Dix essais `_s351`, suite du cœur 631 réussis. Banc `--delta3d-rang1` :
deux domaines à 36 m, une pause où les deux sont voulus — à 60 m, aucune pose ne les rendait voulus ensemble.
**Mesuré** (carte préchauffée) : **aucune image affamée contre 612** au témoin ; 615 images où les deux sont servis ;
focal jamais rétréci ; q99 mesuré **4,983 ms pour 5** avec le coût annoncé au maximum des huit derniers pas — à la
médiane, 20 images au-dessus, +0,045 ms au pire.
**Manqué, publié.** Carte froide : un focal seul dont le premier pas coûte 22 ms reste affamé 17 images (témoin : 16),
démarrage de S344 §3 ; le critère « aucune image affamée » n'est tenu que carte chaude.
**Le prix** : une descente coupe jusqu'à **11,3 cm** de δ, sans verdict — **A319**, la gratuité jamais mesurée.
**Porte A reçue sur le banc ; la v1 atteinte au sens de D4** — portes reçues chacune sur son banc ou sa référence, pas
encore réunies en une scène vivante (feuille de route, « La v1 »). Liste : **4.2 et 9.9 à partiel**, 3 / 58 / 59.
**Rituel.** Maillons **0** : ce qui devient possible — deux domaines δ servis ensemble sous un budget qui n'en tient
qu'un ; le chemin — la liste, dont 4.2 et 9.9 ; la preuve — §7. Suivant (ADR-190 D3) : **ranger les points restants
par dépendance** dans la feuille de route.

## S352 — 2026-09-24 — après la v1 : la liste rangée par dépendance

**Entrée.** Suite de S351 dans la même demande : la v1 atteinte, [ADR-190](../../docs/adr/ADR-190-apres-la-v1-la-liste-entiere.md)
D3 demande de ranger les points restants par dépendance avant de les prendre un à un.
**Fait.** [DEPENDANCES-LISTE](../../docs/registres/DEPENDANCES-LISTE.md) : pour chacun des 117 points non validés, son
système, ce qu'une session peut en faire maintenant, ce qu'il attend, ce qu'il débloque et son front. Les données
sont écrites à la main dans `outils/dependances_liste.py` ; fronts et « débloque » se calculent, et
`etat_projet.py --check` refuse un registre qui ne suit plus la liste — éprouvé sur trois défauts fabriqués. Un cycle
levé en route (4.2 ↔ 4.9). Feuille de route **§3 ter** : l'ordre.
**Ce que la carte dit.** Front 0 : **34** points ; fronts 1 à 5 : 42 ; **41** attendent un fait de l'utilisateur, dont
22 directement — **le réseau (10.1) en commande 14**, puis la météo, le verdict du rang 1, la voie d'A289. En aval :
**4.16** commande 22 points, **2.7** 20. Ordre proposé : la v1 en scène vivante (6.4, 4.2, 4.19), le lot 5 une
session sur deux, la bathymétrie.
**Limites.** Les dépendances sont un jugement, point par point, sur l'énoncé et l'état : une session qui en trouve
une fausse la corrige dans les données. Les systèmes refaits par énoncé comptent B 25 et H 65 là où S309 comptait
29 et 64 — S308 §8 ne publiait que les comptes.
**Rituel.** Maillons **1** : une carte, pas une capacité — aucun point ne change d'état, comme le plan l'annonçait.
Suivant : **la v1 en scène vivante**, en commençant par l'interpolation du rendu à 30 Hz (4.19, 8.7) ; puis le lot 5
(4.16 par A316), une session sur deux.

## S353 — 2026-09-24 — la v1 en scène vivante, 1 : δ à 30 Hz dans la fenêtre, interpolé au rendu

**Entrée.** *« Continue »*. Feuille de route §3 ter, front 0, point 1 : réunir les portes en une scène vivante. S348
avait reçu la porte C au banc et laissé l'interpolation d'ADR-012 §7 : δ changeait à 30 Hz dans une image à 60.
**Fait** ([preuve](../../docs/validation/COUT-DELTA3D-S341.md) §12). Le pas garde la surface qu'il remplace, copiée sur la
carte avant la passe qui publie ; le rendu lie les deux et mélange selon β, une branche rendant la lecture d'avant à
β = 0 ; la fenêtre fait une part du pas par image, β = ½ après la part 1. Touche `I`, témoin `--delta3d-sans-pas`.
**Mesuré.** Captures de l'anneau : 4 empreintes sur 4 inchangées ; image à β = ½ identique au bit à celle de la
moyenne calculée sur CPU (témoin : 550 530 octets différents) ; saccade — sans interpolation une image sur deux
immobile (119 sur 239), avec aucune, chaque image variant comme à 60 Hz (0,72 à 1,24 fois la médiane) ; **en direct
avec le rendu**, même scène : part de δ par image **3,31 ms à 60 Hz, 1,49 à 30 Hz interpolé**.
**Défaut de parcours, trouvé en P6 et corrigé.** À 30 Hz, P4 laissait la fenêtre avancer B d'un pas entier par image :
B et δ divergeaient. Pas d'image ramené à 16,667 ms, horloge de δ décalée d'une demi-image, captures en pas entiers.
**Limites.** Mesure en direct par différence d'intervalles, 960 × 540, sans 99ᵉ centile ; une scène, un domaine ; la
fluidité se juge à l'œil — **R18 préparée**, en direct (REVUE-VISUELLE §23).
**Rituel.** Maillons **0** : ce qui devient possible — δ à 30 Hz dans la fenêtre sans saccade, à 1,49 ms par image ;
le chemin — la v1 en scène vivante, la coque dans la production puis l'ordonnanceur dans l'afficheur ; la preuve —
§12 ; 4.19 et 8.7 avancent. Suivant, par l'alternance d'ADR-184 : **le lot 5** — 4.16, par A316. Verdict R18 attendu.

## S354 — 2026-09-25 — lot 5, A316 : l'instrument relu sur 30 s

**Entrée.** *« Reprends le projet »* ; alternance d'ADR-184 après S353. S327 concluait que les colonnes dissipent
d'elles-mêmes (1,3 %) et que le raccord amortit 4,2 % par période à 5 cm contre −0,4 % pour APIC seul.
**Fait** ([preuve](../../docs/validation/B10-APIC-S320.md) §13). La jauge du ballottement était **aveugle** dans les
colonnes — particules réensemencées, quantifiées par demi-maille — : relue sur `h`. Et **dix secondes ne lisent pas
le point** : sur un signal connu, la mesure se trompe de 0,3 à 1,2 point, de 0,15 au plus sur 30 s. Sur 30 s, **les
colonnes seules ne dissipent pas plus qu'APIC seul** — prédiction écrite avant, tenue.
**Ce que 30 s montrent.** La frontière ne tient pas la **densité** des particules : tassées à 5 par cellule (paroi,
solde), dilatées à 3,5 (eulérien) ; la masse migre vers elles, +12 mm en 30 s à 5 cm. La paroi, soupçonnée, n'est
pas la cause : absorbées, elles se tassent autant. Période +9,96 % contre +7,31 % à 5 cm, saut 1,0 maille à 2,5 cm :
**non reçu**. A316 change de nature, note datée.
**Écart de méthode, de S327.** Deux protections actives manquées : l'instrument non réépreuvé quand ce qu'il mesurait
passait dans les colonnes, la durée d'usage non tenue (L360, L369). Aucune protection nouvelle : elles existaient.
**Rituel.** Maillons **1** : une attribution corrigée, aucune capacité — aucun critère de porte, aucun point de la
liste ne change d'état. Suivant, par l'alternance : **la v1 en scène vivante** — la coque dans la production de δ
(6.4). Au lot 5 ensuite : la densité tenue à la frontière. Verdict R18 toujours attendu.

## S355 — 2026-09-25 — la question du réalisme : ADR-191 ; la coque sur la carte reportée

**Entrée.** *« Continue »* : la v1 en scène vivante, la coque dans la production de δ (6.4). Constat avant tout code :
la coque a été reçue dans le mode **linéaire** (couvercle, S330–S337) ; la production est le pas mobile couplé, qui
n'a de solide nulle part. Chemin retenu au plan : porter sur la carte le mode linéaire de la porte D.
**Question de l'utilisateur**, avant la première ligne : *« Est ce que tu penses que l'architecture […] doit changé ?
[…] pas assez réaliste »*, référence FluidNinja LIVE-2 — une simulation 2D attachée au joueur qui pilote, dans
Unreal, un rendu riche : écume et détail entraînés par ses vitesses, particules, scène dessinée.
**Réponse de la session.** Notre architecture a la même forme — δ près du joueur, B au loin — et va plus loin ;
l'écart est dans le rendu, un instrument arrêté par ADR-178 D2 : ni écume, ni réfraction, ni absorption, ni lumière
sous la surface, ni embruns, ni décor.
**Décision de l'utilisateur** ([ADR-191](../../docs/adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md)) :
l'architecture ne change pas ; le jeu aura un **moteur maison, à construire** ; le rendu final s'y fait, et **nous
en écrivons la part de l'eau en module**, depuis l'afficheur ; **une session de rendu, une de physique**.
**Non fait, reporté tel quel** à la prochaine session de physique : `Linear3`, son banc contre `Volume3`, son coût,
l'ADR de la coque (plan au commit `a2c81dea`) — aucune ligne n'avait été écrite.
**Rituel.** Maillons **2** : une décision, pas une capacité ; la suite en est une. Suivant (ADR-191 D3) : **rendu 1,
l'écume des crêtes de B** (8.4) — le gradient du déplacement CWM est déjà calculé par pixel ; puis la physique, la
coque sur la carte. Verdict R18 toujours attendu.

## S356 — 2026-09-25 — rendu 1 : les crêtes de B ; le rendu de l'eau dans Godot (ADR-192)

**Entrée.** ADR-191 : une session de rendu ; l'ordre de S307 que le verdict R14 nommait — lumière des crêtes, écume.
**Fait** ([preuve](../../docs/validation/RENDU-CRETES-S356.md)). La couverture de Monahan au vent de la mer, 0,421 % à
`U₁₀` = 7,79 m/s, tirée du jacobien CWM normalisé, dont la queue basse est plus lourde que la gaussienne (−2,31 contre
−2,64). **Hypothèse contredite** : un seuil normalisé unique ne tient pas quand le rendu filtre (jusqu'à 2 × W) ; un
seuil par empreinte la tient à ±7 % sur tirage indépendant. Module séparable `water_cretes.wgsl` : écume (Koepke 0,22,
écume fraîche 0,55), lumière des crêtes (transmission de Pope & Fry sur `Hs`, cyan) ; éteint, images au bit ; +0,026 ms.
**Ce que les images ont dit.** À 0,22 l'écume est grise et disparaît en rasant ; à 0,55, des taches blanches sur la crête
de la houle ; la première teinte des crêtes faisait du bleu électrique.
**Décision de l'utilisateur**, en cours de session : *« Peut être godot ou unreal serait envisageable car rendu toujours
pas convaincant »* → **Godot 4**, présent sur le poste (Unreal absent) — [ADR-192](../../docs/adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) :
le rendu final de l'eau dans Godot, premier pas un prototype de la mer de B jugé sur images ; l'afficheur reste le banc.
R19 dans l'afficheur abandonnée.
**Limites.** Écume sans durée, sans texture, sans source hors de B ; crêtes non calibrées ; rien de jugé : 8.4 absent.
**Rituel.** Maillons **3**, justifiés : deux décisions de l'utilisateur ont réorienté le rendu en deux sessions ; ce qui
est construit — la loi de l'écume, la teinte des crêtes — ne dépend d'aucun moteur et se porte tel quel ; la suite rend
une image à juger. Suivant : **le prototype Godot**, premier pas choisi par l'utilisateur (ADR-192 D2) ; l'alternance
reprend ensuite avec la coque sur la carte (6.4). R18 toujours attendu.

## S357 — 2026-09-25 — rendu 2 : la mer de B dans Godot 4.4.1

**Entrée.** ADR-192 D2 : le premier pas choisi par l'utilisateur — la mer de B rendue dans Godot, jugée sur images avant
tout portage. Godot 4.4.1 était sur le poste ; rien n'a été téléchargé.
**Fait** ([preuve](../../docs/validation/PROTOTYPE-GODOT-S357.md)). `--export-godot` : la mer de `--meilleur` à 12 s,
bande, queue, modulation, asymétries, seuils de l'écume, points de contrôle du cœur. `godot/` : grille polaire autour
de la caméra, phases repliées en double, nuanceur porté — CWM et Tayfun, queue filtrée, écume et crêtes de S356 ; à
Godot, l'éclairage, les reflets, AgX, la brume ; la pente non résolue en rugosité. **Hauteur retrouvée à 1,05·10⁻⁷ m**
contre le cœur.
**Ce que les images ont dit.** Le ciel physique par défaut de Godot : un crépuscule gris — remplacé par le ciel clair de
R14. Les reflets à l'écran assombrissaient la mer rasante — éteints. L'écume tombe où l'afficheur la mettait.
**R19 préparée** (REVUE-VISUELLE §24) : les deux rendus côte à côte, proche et rasant ; images envoyées.
**Limites.** Ni W, ni δ, ni corps, ni intégration native ; paramètres de Godot non calibrés ; coût non mesuré.
**Rituel.** Maillons **4**, justifiés : l'utilisateur a déplacé le rendu vers Godot ; ce prototype est le premier pas
qu'il a choisi, et son verdict décide de la suite. Suivant : **R19 d'abord** ; la physique en attendant, par
l'alternance — la coque sur la carte (plan de S355). R18 toujours attendu.

## S358 — 2026-09-25 — la coque dans la production de δ, 1 : le pas linéaire sur la carte

**Entrée.** *« Reprends le projet »*, sans verdict R19 : la physique (ADR-191 D3), le plan de S355 reporté — liste 6.4.
**Fait** ([preuve](../../docs/validation/LINEAIRE-GPU-S358.md)). `Linear3` : le pas linéaire de la porte D sur la carte,
géométrie en donnée découpée par le cœur, Jacobi à cycles fixes et départ chaud. Contre `Volume3`, grille de la porte D :
5,7·10⁻⁵ m à 8 cycles, **1,3·10⁻⁵ m à 16 autour d'une sphère fixe** (témoin sans découpe : 4,7 mm), l'ulp de 2 m à 64 ;
16 cycles, **0,32 ms** au 99ᵉ centile. [ADR-193](../../docs/adr/ADR-193-le-domaine-d-une-coque-est-lineaire-sur-la-carte.md) :
le domaine d'une coque est linéaire, sur la carte ; le pas mobile couplé garde les autres.
**Défaut trouvé et corrigé.** La surface publiée perdait son reste compensé : `(η − z₀) − reste` réassocié par le
compilateur (L345). Instrument : l'avance rejouée au bit depuis les entrées de la carte ; somme vraie tenue à 10⁻⁹ m,
somme publiée +3,4·10⁻⁵ m dès le pas 1. **Toute la production résidente l'avait** (publiée, fantôme du haut, couvercle
couplé) : sur la cuve de S305, écart à la référence 3·10⁻⁷ → 2,4·10⁻⁸ m, dérive de la moyenne au niveau du cœur, pente
séculaire divisée par 13 — S305 §7.3 attribuée.
**Limites.** Ni coque qui bouge, ni couvercle partiel, ni scène, ni diagnostics différés ; 6.4 et 6.5 restent partiels ;
la pente résiduelle de la cuve (≈ 8·10⁻⁹ m/s) n'est pas attribuée.
**Rituel.** Maillons **0** : correction d'intégrité reproduite (S305 au chiffre près) puis testée. Devient possible : une
production de δ dont la surface publiée est la hauteur vraie, et une coque dans δ sur la carte ; consommée par le rendu
de δ (8.7) et la coque qui bouge (6.4) ; preuve §2 et §4. Suivant : **R19 d'abord** ; la physique en attendant, par
l'alternance d'ADR-184 — le lot 5 (A316, la densité à la frontière) —, puis la coque qui bouge. R18, R19 attendus.

## S359 — 2026-09-25 — rendu 3 : l'eau a une épaisseur

**Entrée.** Verdict R19 : *« la mer n'est pas du tout crédible, mais c'est pas grave on continue, car je pense qu'il
manque plein de chose avec la trnasparence en fonction de la prfondeur etc... »*. Relu : un aplat opaque, et un horizon
qui s'assombrit. Session de rendu (ADR-191 D3).
**Fait** ([preuve](../../docs/validation/EPAISSEUR-EAU-S359.md)). **Mesuré d'abord** (`outils/horizon_mer.py`) : sous
l'horizon, la mer de Godot renvoyait 0,11 du ciel, l'afficheur 0,73 ; la rugosité confiée à Godot en explique une part
(témoin). **La lumière de l'afficheur portée** ([ADR-194](../../docs/adr/ADR-194-la-lumiere-de-l-eau-calculee-par-notre-nuanceur.md)) :
un seul ciel pour la scène et les reflets, Fresnel, pente non résolue intégrée — **0,71 et 0,68**, à 5 % de
l'afficheur. **La colonne d'eau** : profondeur lue au tampon (3 à 5 cm de la bathymétrie), réfraction de Snell,
Maritorena sur le trajet oblique avec l'eau pure d'ADR-177 — transmission à 0,005 du modèle. Une scène côtière
(`--cote`) : sable de 6 à 40 m, puis le large. **R20** préparée, images envoyées.
**Limites.** Ni caustiques ni particules ; les vagues ne sentent pas le fond (2.7) ; `μ̄_d` et l'albédo du sable à
calibrer ; une couture au centre du ciel de Godot, non attribuée ; coût non mesuré.
**Rituel.** Maillons **0** : 8.5 passe d'*absent* à *partiel* — devient possible, voir le fond à travers l'eau selon sa
profondeur, et une mer qui renvoie le ciel ; consommé par la revue R20 et la suite du rendu dans Godot ; preuve §1–3.
Suivant : **R20 d'abord** ; la physique en attendant, par l'alternance — le lot 5 (A316), puis la coque qui bouge
(ADR-193). R18, R20 attendus.

## S360 — 2026-09-25 — rendu 4 : la surface fine par FFT, l'écume au déferlement

**Entrée.** Verdict R20 : la couleur *« parfaite »* ; *« ce rendue du point de vue topologie est pas réaliste »* ; le
ciel limite les reflets ; *« Tente les caustique »* ; l'écume seulement au déferlement des grandes vagues. Recherches :
Beaufort 4, taille des moutons (Callaghan, Bondur–Sharkov), Elfouhaily 1997, Tessendorf 2001.
**Fait** ([preuve](../../docs/validation/SURFACE-FINE-S360.md), [ADR-195](../../docs/adr/ADR-195-la-queue-de-b-rendue-par-fft.md)).
**Mesuré** : la queue qui dessinait la surface fine, 60 ondes pour 5,5 octaves sur 360°, pentes plus fortes en
travers (0,88 ; Cox–Munk 1,37 au vent). Le cœur donne sa densité continue (4·10⁻⁷ de la queue discrète) et l'étalement
d'Elfouhaily ; l'afficheur exporte **10 612 composantes** ; Godot les évolue et les ramène par FFT sur la carte — à
3·10⁻⁵ d'une somme directe —, et l'eau les lit à l'empreinte, variance non résolue par LEAN vers les reflets filtrés.
**L'écume** tirée des vagues dominantes : au nadir, 4 652 taches de 4 cm deviennent des moutons de 1,3 à 3,8 m, à la
couverture de Monahan. Contrôles de S359 et horizon inchangés. **R21** préparée, images envoyées.
**Critères manqués, dits.** Anisotropie 1,23 pour 1,37 (−10,4 % pour ± 10 %) ; 8 taches d'écume sur 37 sous 0,5 m au
loin. **Limites.** L'afficheur garde sa queue (divergence dite) ; cascades répétées, coût non mesuré ; écume lisse.
**Rituel.** Maillons **1** : aucune case de la liste ne change (8.9 partiel, 8.4 absent jusqu'au verdict). Suivant :
**R21 d'abord** ; les caustiques, demandées en R20, puis le ciel ; ensuite la physique par l'alternance (lot 5, la coque
qui bouge). R18, R21 attendus.

## S361 — 2026-09-25 — rendu 5 : les caustiques sur le fond

**Entrée.** R20 : *« Tente les caustique »*. Surface de S360.
**Fait** ([preuve](../../docs/validation/CAUSTIQUES-S361.md)). La hessienne de η dans la FFT (3·10⁻⁵ de la somme directe).
**Première méthode, à rebours** — du fond au point de surface par point fixe, `1/|det J|` : exacte à 2,4 % devant la
focale sur une onde, mais **énergie 2,0 et 5,6** sur la scène : elle ne suit qu'un antécédent, et le fond de la scène
est au-delà de la focale de la cascade de 32 m (≈ 13 m). **Seconde méthode, directe** (Wyman 2006) — la surface
projetée sur le fond triangle par triangle dans une vue orthographique hors écran, rapports d'aire additionnés : exacte
à **4,6 %** contre la solution indépendante dans les deux directions (témoin retourné : 49 %), plis au-delà de la focale
additionnés, **énergie de la scène 1,008 et 1,002** ; le disque solaire borne les plis. Réseau d'un à deux mètres, sous
l'eau un miroitement bleuté. **R22** préparée, images envoyées.
**Limites.** Carte de 64 m devant la caméra ; cascade fine exclue (focale ≈ 0,8 m) ; ni objets immergés, ni rayons dans
l'eau ; coût non mesuré.
**Rituel.** Maillons **2** : 8.5 reste partiel. À deux maillons, la suite prend une capacité : **R21 et R22 d'abord** ;
puis la physique — la coque qui bouge sur la carte (6.4, ADR-193 §3) ou le lot 5 (A316) — ; le ciel à la session de
rendu suivante. R18, R21, R22 attendus.

## S362 — 2026-09-25 — physique : la houle qui sent le fond, la référence (2.7)

**Entrée.** *« Continue »*, sans verdict R21/R22. À deux maillons, un lot qui fait avancer la liste : la suite proposée
(la coque qui bouge, le lot 5) laissait chaque point dans sa case ; **2.7**, absente, vingt points en aval, et la scène
côtière de S359 qui la montre manquer.
**Fait** ([preuve](../../docs/validation/BATHYMETRIE-S362.md)). `bathymetrie.rs`, une référence f64 : dispersion en
profondeur finie, vitesse de groupe, levée, réfraction de Snell sur isobathes droites, phase intégrée, profondeur de
déferlement (McCowan). Tenue contre les résultats publiés : **Fenton–McKee 1,63 %** (borne publiée 1,7 %), **levée
minimale 0,91299 à kh = 1,1995** (0,913 des manuels), flux d'énergie à 5·10⁻¹⁶, phase à 3·10⁻⁸ ; une houle de 1 m et
10 s déferle par **1,78 m** de fond. L'instrument de la phase corrigé deux fois (coin du profil, troncature), jamais le
seuil. Cœur : 516 réussis, 14 ignorés.
**Ce qui ne se tranche pas.** Où la bathymétrie entre : ADR-004 §2.1 garde les composantes de B identiques partout et
place levée et réfraction dans W ; faire tourner les composantes avec le fond le contredirait — un ADR mesuré, à venir.
**Limites.** Isobathes droites ; ni diffraction, ni réflexion, ni non-linéarité (A234) ; aucun consommateur encore.
**Rituel.** Maillons **0** : 2.7 passe à partiel ; devient possible, recevoir tout candidat qui fera sentir le fond à la
houle ; consommé par l'ADR d'entrée et la scène côtière ; preuve §2. Suivant : **R21 et R22 d'abord** ; par l'alternance,
le rendu — **le ciel** (R20) — ; puis la physique : l'entrée de la bathymétrie, ou la coque qui bouge. R18, R21, R22.

## S363 — 2026-09-25 — rendu 6 : le ciel, et la courbe calée sur la photographie

**Entrée.** Alternance d'ADR-191 D3 ; R20 : le ciel *« n'aide pas aux reflets »* ; sans verdict R21/R22. Coupée après P4
(13:36) ; **reprise à chaud à 18:34** sur *« Reprends le projet »* : arbre propre, une copie, rien à trancher.
**Fait** ([preuve](../../docs/validation/CIEL-S363.md)). **La mesure d'abord** : contre la photographie de R14, la surface
fine de S360 relève le contraste local (0,27 → 0,36 ; 0,54 en linéaire pour 0,455) — le manque « spatial » de S308 est
comblé dans Godot ; AgX écrase la dynamique et la fraction claire. **La couture** du ciel et les **nuages en blocs** :
un hachage `fract(sin)` qui perdait sa précision ; un hachage entier — saut au centre 10,7 → 0,03 fois ses voisins.
**Le ciel** de la photographie (`ciel_mesure` de S308), ciel et reflets. **La courbe** : Godot rend en HDR ;
`tonalite_godot.py` rejoue ses courbes recopiées de `tonemap.glsl` 4.4.1, **jamais plus d'un octet** sur huit réglages ;
1 772 essais : ACES tient la luminance (0,143 sans écrêtage) mais triple le bleu des creux, qu'AgX tient à 0,3 % ;
luminance et teinte ensemble, **`TONALITE=photo`** (ACES, saturation 0,5) : **0,166 contre 5,8** pour AgX, en option.
**Critères ajoutés en cours, dits** : l'écrêtage (le premier gagnant brûlait 18 % de la mer), la teinte des creux.
**Limites.** Une pose, une photographie ; crêtes grises (B/G 1,2 pour 2,89) — les reflets, pas la courbe ; la
saturation pâlit aussi le ciel ; coût non mesuré. **R23** préparée, images envoyées.
**Rituel.** Maillons **1** : aucun point de la liste ne change d'état (8.10 reste partiel, son constat actualisé).
Suivant : **R21 à R23 d'abord** ; par l'alternance, la physique — l'entrée de la bathymétrie (un ADR mesuré), ou la
coque qui bouge. R18, R21, R22, R23 attendus.

## S364 — 2026-09-25 — physique : la bathymétrie entre dans B (2.7)

**Entrée.** *« Continue »*, sans verdict R21 à R23 ; alternance d'ADR-191 D3 après S363. Suite de S362 : l'entrée, que
S362 renvoyait à un ADR mesuré. W naît d'événements et le relais B → W mélangerait deux réalisations (ADR-004 §3).
**Fait** ([ADR-196](../../docs/adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md),
[preuve](../../docs/validation/BATHYMETRIE-S362.md) §5). `bathymetrie_cote.rs` : chaque composante de B reçoit des tables
cuites depuis la référence — correction de phase **entière**, `K_s·K_r`, `k_y`, `coth kh` —, interpolées en O(1).
**Mesuré** : 0,23 mm de la référence au pas de 2 m (en `Δ²`, prédit 0,4) ; sur huit composantes, η à 0,13 mm, pente et
vitesse à 0,02 % ; **au large, B au bit** (120 évaluations) ; même hash sur deux passes ; requête ×1,2 à ×1,6 de B près
des côtes, indépendante du pas ; 258 Ko par km de profil. **Trouvé** : à λ₀/2, le « fond qui cesse de se sentir » des
manuels et de SPEC-005 §8, la levée vaut encore 0,990 — 5 mm de marche ; à λ₀, 4·10⁻⁵, prédit : les tables commencent
à λ₀ (D3, note datée dans SPEC-005). ADR-004 §2.1 et §5 révisés, note datée. Cœur : 518 réussis, 14 ignorés.
**Limites.** Isobathes droites ; ni 2D (128 Mo/km² en tables régulières : autre paramétrage à mesurer), ni marée, ni
déferlement dissipé, ni diffraction ; déterminisme entre plateformes par construction, non mesuré (A98).
**Rituel.** Maillons **2** : la décision lève le blocage de S362 et nomme le lot exécutable — les chemins de B jusqu'à la
scène côtière de Godot —, mais aucun chemin ne consomme encore la capacité (L258) et 2.7 reste partiel. Suivant :
**R21 à R23 d'abord** ; sinon le rendu, et à deux maillons un lot qui fait avancer une case — 8.6, la vue sous-marine ;
la côte de Godot vue par B consommerait ADR-196 sans changer de case. R18, R21, R22, R23 attendus.

## S365 — 2026-09-25 — rendu 7 : sous la surface (8.6)

**Entrée.** *« Continue »*, sans verdict R21 à R23 ; alternance d'ADR-191 D3 après S364. À deux maillons, un lot qui fait
avancer une case : **8.6**, absente (ADR-019, B11), sur 8.5 partielle.
**Fait** ([preuve](../../docs/validation/SOUS-MARIN-S365.md)). `optique_eau.gdshaderinc`, une source pour la surface, le fond
et le fond du ciel. **La surface vue d'en dessous** : Fresnel eau → air, ciel réfracté × n², réflexion totale au-delà de
l'angle critique ; mer plate, zénith, 16 directions : bord de la **fenêtre de Snell à 48,254°** pour 48,268° (pire
0,048°). **Le milieu** : `exp(−c·d)`, `c = a + 2·b_b` (Pope & Fry, Morel), et la lumière de l'eau intégrée exactement le
long d'une ligne en pente ; transmission relue à **0,004** près de 5 à 19 m ; le fond et ses caustiques vus de l'eau.
**Mode immergé** d'un bloc (`immersion()`), non-régression au-dessus de l'eau au bit (proche, rasante).
**Impasses et défauts, dits.** `FRONT_FACING` : la grille présente sa face avant par en dessous. La plus forte chute de
luminance se trompe sur les nuages tassés au bord de la fenêtre : mesure sur le coefficient de Fresnel. Fresnel à 0/0
sous incidence rasante (pixels noirs) ; une ligne sous l'horizon (profondeur moyenne du trajet) — corrigés.
**Limites.** Caméra à demi immergée non traitée (ADR-019 §6) ; fond absent du miroir ; ni bulles, ni rayons, ni
turbidité ; `f(ω)` à calibrer ; coût non mesuré. **R24** préparée, images envoyées, une référence sous l'eau demandée.
**Rituel.** Maillons **0** : 8.6 passe à partiel (3 / 61 / 56) ; devient possible, descendre la caméra sous l'eau ;
consommé par le rendu de Godot (poses sous l'eau, scène côtière) ; preuve §1–2. Suivant : **R21 à R24 d'abord** ; par
l'alternance, la physique — les chemins de B lisent la côte (ADR-196), la coque qui bouge, ou le lot 5 (A316).

## S366 — 2026-09-25 — rendu 8 : les références sous l'eau, la lumière de l'eau calée sur une mesure

**Entrée.** Verdict R24 : *« Pour les références trouve les sinon rien a redire cela me paraît good, continue »* — la
demande prime sur l'alternance. S365 avait laissé `f(ω)` à calibrer et aucune référence.
**Fait** ([preuve](../../docs/validation/SOUS-MARIN-S365.md) §6). **Références**, lues dans le navigateur, rien téléchargé :
la radiance mesurée par Tyler (1960, lac Pend Oreille, via Mobley), numérisée par ses pixels ; trois photographies libres
de Wikimedia Commons (Hanifaru, Maldives ; un récif ; une piscine, le fond dans le miroir). **Mesuré** : à l'horizontale,
2,46 fois la radiance montante à l'opposé du soleil, **8,71 côté soleil** ; le modèle de S365, sans azimut, s'en écarte
(résidu logarithmique 0,362). Critère dépassé : **lobe avant** de Henyey-Greenstein autour du soleil réfracté, g = 0,855,
résidu **0,120** ; réalisé, rapport face / dos 2,18–2,23 pour 2,215 prédits. Photographie, indicatif : teinte de l'eau
4,07 contre 4,10 ; **fenêtre trois fois trop terne** — le ciel ne porte pas l'éclairement du soleil que reçoit l'eau.
**Défaut trouvé** : la brume de Godot assombrissait l'horizon vu d'en dessous — éteinte sous l'eau ; au-dessus, au bit.
**Limites.** Une mesure, d'une eau de lac, un plan d'azimut : lobe emprunté par notre eau pure, dit ; photographies en
rapports seulement. **R25** préparée, images envoyées (sans réponse, la lueur reste).
**Rituel.** Maillons **1** : 8.6 reste partiel. Suivant : **R21 à R23, R25** ; par l'alternance, la physique — les
chemins de B lisent la côte (ADR-196), la coque qui bouge, ou le lot 5. Au rendu : la caméra à demi immergée, l'échelle
radiométrique du ciel et du soleil.

## S367 — 2026-09-25 — physique : le champ d'écume de B (7.1)

**Entrée.** Verdict : *« Je valide les rendue sauf ecume »* — lu sur R21, R22, R23, R25 : forme fine, caustiques, ciel
(AgX reste le défaut), lueur face au soleil validés ; **l'écume refusée** ; R18, en direct, toujours attendu. Alternance :
la physique ; 7.1, absente, est la physique de l'écume refusée — celle-ci n'avait pas de mémoire.
**Fait** ([preuve](../../docs/validation/ECUME-S367.md)). `ecume.rs` : le champ d'ADR-014 — deux canaux (actif 3 s, résiduel
30 s), advection semi-lagrangienne orbitale, décroissance exacte, déferlement par l'accélération des crêtes ; tenu à
7,7·10⁻⁷, centre advecté à 10⁻⁵ m, déterministe. **Trouvé** : au seuil physique (0,45 g), la mer de B ne déferle jamais —
sa bande est autosimilaire, `σ_a/g` = 0,0836 à tout vent, et aucun seuil fixe ne suit Monahan. Place du déferlement par
la physique, quantité par Monahan : seuil `κ·σ_a` calé, **vérifié sur une autre graine à 1,02 / 1,06 / 0,94** (384 m ;
le 0,51 d'un champ de 128 m était du bruit). **Traînées** : le résiduel advecté s'allonge le long du vent, Ly/Lx 5,3
contre 1,8 ; ni bord d'entrée (domaine périodique), ni diffusion (croît quand le texel baisse). Cœur : 522 réussis.
**Limites.** Référence CPU ; sources de B seules ; transfert 1 : 1 trop généreux (résiduel moyen 0,40) ; mer pleinement
développée seulement ; Ly en butée du domaine.
**Rituel.** Maillons **0** : 7.1 passe à partiel (3 / 62 / 55) ; devient possible, une écume qui dure et s'étire ;
consommé par le rendu de Godot (8.4), au prochain rendu ; preuve §1–3. Suivant : **le champ porté sur la carte de Godot**
et l'écume rendue qui en naît (8.4), par l'alternance ; R18 attendu.

## S368 — 2026-09-26 — rendu 9 : l'écume qui dure, puis suspendue

**Entrée.** *« Continue »* ; alternance après S367. L'écume de S360, refusée en R21, n'avait pas de mémoire ; S367 avait
posé la référence du champ d'ADR-014.
**Fait** ([preuve](../../docs/validation/ECUME-GODOT-S368.md)). `ecume.comp`, `ecume.gd` : le champ sur la carte, 1 024² texels
de 0,25 m, le pas de S367 à l'identique, 60 s de passé rejouées à chaque recentrage. Advection à **0,47 mm** ; décroissance
à **8,6·10⁻⁶** après un **défaut trouvé** — le transfert `e^(−λr·dt) − e^(−λa·dt)` s'annulait en f32 au pas de 1/60 s
(2,5·10⁻⁵) : coefficients en double. Couverture : κ de S367 → 0,62 % ; **κ = 3,09**, prédit par sa pente → **0,405 %**
pour 0,421 %. Rendu : moutons au bord irrégulier qui pâlissent en se trouant, dentelle résiduelle derrière eux — deux
motifs rejetés sur image (labyrinthe du bruit de valeurs, résille de Worley).
**Décision de l'utilisateur, pendant P5** : *« Oublie l'ecume sauf si tu trouve des photos qui informe de la forme et
couleur et position dans la topologie »*. La série Beaufort de la NOAA (force 4, notre mer) : 400 × 386, ne renseigne pas.
L'écume **s'arrête**, éteinte par défaut (`ECUME=champ` la rallume) ; code et mesures conservés ; pas de revue.
**Rituel.** Maillons **1** : 8.4 reste absent (rien de rendu n'est reçu). Suivant : par l'alternance, **la physique** — les
chemins de B lisent la côte (ADR-196), la coque qui bouge, ou le lot 5 ; au rendu, la caméra à demi immergée et
l'échelle radiométrique du ciel. R18 attendu.

## S369 — 2026-09-26 — physique : les réponses du 2026-09-26, puis la voie d'A289

**Entrée.** Réponses de l'utilisateur aux questions de S368 : Godot moteur du jeu entier, tout flotte ou coule, pas de
terrain à hydrologie, météo et son à la fin, *« pas de réseau »* ; A289 déléguée — *« le plus favorable au réalisme ainsi
que les performances, simple »* ; seconde cible : pas encore ; R18 *« Rendu convaincant »* ; photos d'écume : plus tard.
**Décisions** ([ADR-197](../../docs/adr/ADR-197-reponses-du-2026-09-26.md)) : 8.1 au front 0 ; 6.7 sans attente ; **5.11 hors du
périmètre** ; « pas de réseau » lu « aucun format », la portée (multijoueur ?) demandée ; R18 reçu.
**Fait** ([preuve](../../docs/validation/MER-S369.md), [ADR-198](../../docs/adr/ADR-198-la-voie-d-a289.md)). La croissance de
S319 est **forcée** par trois termes du pas couplé qui ne dépendent que de B (résidu de quantité de mouvement, l'essentiel ;
bande jusqu'à sa surface ; erreur de pression à sa surface). Retirés — δ relatif à B —, **δ nul reste nul au bit** sous
la houle (E1 40 s, test), en un tiers du temps. Voie retenue sur les trois critères, contre les trois d'A289.
**Mais** (critère 3 manqué) : un germe de 1 mm tient 40 s puis croît dès 50 s, 0,060 s⁻¹ ; le paquet d'E2 aussi (0,052,
1 620 fois le volume) — instabilité convective, ≈ `a²`, indépendante du pas, plus lente à maille fine ; **bisection** :
portée par `u'·∇U` seul. Sous 2,5 cm, stable, mais le critère de volume est mal posé (transport croisé, 21 fois).
**Limites.** Une composante, tranche de deux rangées ; production GPU dans l'ancien mode ; forme de Bernoulli non essayée.
**Rituel.** Maillons **0** : 4.21 passe à partiel (3 / 63 / 54) — devient possible une mer de fond dans δ identique à B,
au bit ; consommé par l'ordre E et la production ; preuve §1. A289 résolue à sa cause, **A320** ouverte (sévérité 3).
Suivant : par l'alternance, **le rendu** (caméra à demi immergée) ; à la physique suivante, A320 par la forme de Bernoulli.

## S370 — 2026-09-26 — consignation : « pas de réseau = pas encore »

**Entrée.** Réponse de l'utilisateur à la question de fin de S369 (ADR-197 D1 : aucun format réseau, ou pas de
multijoueur ?) : *« Pas de réseau = pas encore »*.
**Fait.** Note datée sur [ADR-197](../../docs/adr/ADR-197-reponses-du-2026-09-26.md) D1 : **le multijoueur reste dans
l'ambition** (ADR-127), rien n'est retiré ; comme la seconde cible et le serveur (D7), 10.1 et ses quatorze points en aval
attendent un fait nouveau de l'utilisateur, et aucun travail de réseau ne commence. Dépendances, file, feuille de route,
index ; aucun code.
**Rituel.** Maillons **1** : aucun point de la liste ne change d'état. Suivant : inchangé — par l'alternance, **le rendu**
(caméra à demi immergée) ; à la physique suivante, A320 par la forme de Bernoulli.

## S371 — 2026-09-26 — rendu 10 : la caméra à demi immergée

**Entrée.** *« Reprends le projet »* ; par l'alternance, le rendu : la caméra à demi immergée (ADR-019 §6), que S365 faisait
basculer d'un bloc — le ciel rendu comme de l'eau dès que la ligne traverse l'image.
**Fait** ([preuve](../../docs/validation/DEMI-IMMERGEE-S371.md)). Le milieu **par pixel, à l'objectif** — le point où le rayon
traverse le plan proche, sous ou au-dessus de la surface de B (point lagrangien par Newton, Tayfun compris) —, pour
l'eau, le fond et le ciel (`surface_b.gdshaderinc`). Contrôle contre l'intersection exacte en double : **0,078 px** au
pire (quatre cas), **0 pixel mal classé** sur 60 images, caméra fixe et flottante. **Trouvé** : la borne de |η| (6,43 m)
allumait le mode à 4 m — test rigoureux serré à 0,43 m ; Newton par pixel coûtait **4,7 ms GPU** — la surface à l'ordre 2
au centre du plan proche, hessienne eulérienne analytique, ordre 3 borné à 0,37 px : **≤ 0,03 ms** ; le bord de la
fenêtre en escalier vu de 4 cm venait de la FFT lue en bilinéaire — bicubique au-delà d'un grossissement de 32 (au seuil
4, les poses validées changeaient). Brume éteinte en mode demi : 17 niveaux au-dessus, 48 évités au-dessous. Deux
photographies libres chiffrées ; ménisque calé sur l'une. Témoins identiques au bit ; profondeur prise à l'objectif sur
la surface exacte (≤ 1 niveau sous l'eau). **R26** préparée, images envoyées.
**Limites.** Un état de mer ; champ large (2,4 px à 100°) ; brume non réglable par pixel ; ménisque d'une photographie.
**Rituel.** Maillons **2** : 8.6 reste partiel. Suivant : par l'alternance, la physique — à deux maillons, un lot qui
change l'état d'un point : A320 par la forme de Bernoulli (4.8), la coque qui bouge (6.4) ou les chemins de B et la côte
(2.7). R26 attendu.

## S372 — 2026-09-26 — physique : vannes et pompes dans V (5.4)

**Entrée.** Verdict **R26** : *« je valide continue »* — la caméra à demi immergée et son ménisque reçus. Par
l'alternance, la physique ; à deux maillons, un lot qui change l'état d'un point : **5.4**, absent, au front 0, dans V.
**Fait** ([preuve](../../docs/validation/VANNES-POMPES-S372.md), [ADR-199](../../docs/adr/ADR-199-vannes-et-pompes-dans-v.md)).
Une **commande** entière par arête, état répliqué ; la **vanne** (section ou largeur commandée) ; la **pompe** en réseau
ouvert — courbe parabolique contre la hauteur statique, clapet, prise à sec, similitude. À commande pleine, trajectoires
de quatre montages **identiques au bit** (empreinte relevée avant la modification). C12 à demi-ouverture : −0,10 % de
l'analytique ; fermée, 0 ml ; rouverte, +600 pas exactement. Pompe : prise dénoyée à +0,025 % de l'intégrale
analytique, barrage à 1 ml près, mi-vitesse 2 499 ml/s pour 2 500. Réseau d'avarie (brèche, vanne, deux pompes, commandes
changeantes) : masse exacte, reproduit au bit. **WVST v2** : les commandes changées sauvegardées, suite restaurée au bit,
témoin d'omission discriminant ; v1 refusée. Cœur : 536 réussis, 0 avertissement.
**Limites.** Réseau ouvert (le fermé reste 5.8) ; `C_d` fixe ; ni pertes, ni énergie ; aucun consommateur encore.
**Rituel.** Maillons **0** : 5.4 passe à partiel (3 / 64 / 53) — devient possible une avarie commandée (fermer une vanne
de coursive, lancer une pompe de cale), sauvegardée ; consommée par le pas V du serveur et de l'hôte, et par 5.8 et 5.9 ;
preuve §2–4. Suivant : par l'alternance, **le rendu** — l'échelle radiométrique du ciel et du soleil, notre perspective
aérienne.

## S373 — 2026-09-26 — rendu 11 : la brume réglée par pixel

**Entrée.** *« Continue et ensuite commence à permettre de visualiser le système de piscine avec déversoir et pompe »* —
la suite du jeton, le rendu, puis S374. Des deux rendus proposés, la perspective aérienne : bornée, et elle lève la limite
de S371 (brume éteinte sur toute l'image à demi immergée).
**Fait** ([preuve](../../docs/validation/DEMI-IMMERGEE-S371.md) §10). `brume_air` réécrit la brume de Godot — quantité
exponentielle, couleur du ciel dans la direction — × la part d'air du pixel, par `FOG`. **Critère 1 manqué** partout
(poses au-dessus : p99,9 jusqu'à 9, pire 11) : Godot lit son cube de radiance au niveau le plus flou, une moyenne
diffuse. Seuil gardé ; **deux variantes** de l'eau et du fond (`#define`, corps en `.gdshaderinc`), la nôtre seulement à
demi immergée : six poses au-dessus et deux sous l'eau **identiques au bit** ; à demi immergée, côté air à 7 niveaux de
Godot (15 avant), côté eau au bit ; contrôle de la ligne inchangé.
**Limites.** Changement de modèle de brume à l'entrée du mode (≤ 7 niveaux), compilation au premier passage.
**Rituel.** Maillons **1** : aucun point ne change d'état. Suivant : **S374, la piscine de V visualisée** (demande de
l'utilisateur) ; au rendu, l'échelle radiométrique, avec une mesure qui la fonde.

## S374 — 2026-09-26 — la piscine de V dans Godot, puis la dynamique en 3D volumétrique

**Entrée.** *« commence à permettre de visualiser le système de piscine avec déversoir et pompe »* ; pendant la session,
*« La dynamique de fluide doit se faire en 3D volumétrique »*.
**Fait** ([preuve](../../docs/validation/PISCINE-V-S374.md)). `piscine_v.rs` : une piscine à débordement dans V — bassin
8 × 4 m, déversoir de 4 m, bac tampon, pompe de 12 l/s ; volume exact sur 3 300 pas ; régime établi à −0,21 % (déversoir)
et +0,001 % (pompe) du point de fonctionnement analytique. Godot la rejoue (`piscine.tscn`), surfaces à 3,6·10⁻⁸ m de
celles que V publie ; `bassin.gdshader` et `paroi.gdshader` reprennent l'optique et l'éclairement de la mer.
**Décision de l'utilisateur** ([ADR-200](../../docs/adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md)) : le
mouvement de l'eau des contenants se calcule par δ en 3D, V garde la masse (ADR-025), les arêtes de V deviennent sources
et puits ; la lame et le jet attendent APIC en 3D. L'habillage balistique préparé (P4) est écarté, non versé.
**Limites.** Aucune dynamique encore ; rejeu d'un scénario, pas de commande en direct.
**Rituel.** Maillons **2** : aucun point ne change d'état (5.4 gagne un consommateur, 5.10 une décision). Suivant :
**S375, δ 3D dans le bassin** (ADR-200 D4) — domaine sur l'intérieur, masse asservie à V, source du jet, puits du seuil,
surface rendue dans Godot ; à deux maillons, viser 5.10 absent → partiel.

## S375 — 2026-09-26 — le bassin de la piscine en δ 3D (5.10)

**Entrée.** ADR-200 D4 (1), décision de l'utilisateur : la dynamique des contenants en 3D volumétrique. Interrompue par la
limite d'usage à P3 (commit `948f55de`), reprise à chaud à 09:53 (*« Réessayer »*), sans perte.
**Fait** ([preuve](../../docs/validation/PISCINE-DELTA-S375.md)). `add_column_volume` (volume exact au bit), `shift_rest` (le
repos suit V), `last_refused_report`. `piscine_delta.rs` : le bassin en δ 3D à 20 cm, jet (volume et quantité de mouvement
verticale dans un panache), puits du seuil, forçage vers V ; 330 s, 13 200 pas, 27 ms/pas, 0 refus ; niveau de V à
0,1 µm (critère 2). **Deux défauts de la référence mobile** : une élévation uniforme refusée (ADR-201, plancher de
vitesse) ; un pas calme refusé de justesse (repos resté 8,5 mm sous V : `shift_rest`). Premier jet sans dissipation : sortie
du domaine à 7,4 s — panache amorti, à calibrer. **Critère 3 manqué** (front à +41 % au seuil de 1 mm ; −3 % à 0,3 mm,
relu). Godot : maillage de hauteur, hauteurs au bit (critère 4).
**Limites.** **La dynamique ne se voit pas** à l'échelle réelle (mm à 20 cm ; témoin ×100 : la chaîne de rendu est bonne) ;
panache sans mesure ; bac tampon plan ; lame et jet pour APIC.
**Rituel.** Maillons **0** : 5.10 passe à partiel (3 / 65 / 52) — devient possible un contenant de V dont δ fait le
mouvement et V garde la masse ; consommé par le rendu de Godot et par la porte E ; preuve §1–5. **R27** envoyée. Suivant :
**δ sur GPU dans Godot** (nuanceur de calcul, 5 à 10 cm, temps réel), si R27 le confirme.

## S376 — 2026-09-26 — consignation : le niveau de détail des contenants

**Entrée.** Réponse à R27 : δ sur GPU dans Godot, *« je ne pense pas qu'il faut le faire maintenant »* ; puis la règle :
les contenants suivent le principe de la haute mer — V par défaut, effets factices au loin, δ 3D près d'un joueur ou d'un
perturbateur, en zones selon la taille ; prévision et niveaux de détail ; météo précalculée en amont qui donne les litres
à V, des éléments bloquants qui l'empêchent ; impacts de pluie factices ; débordement vu simulé. *« Si tu as des zones
d'ombre cites les moi. »*
**Fait.** [ADR-202](../../docs/adr/ADR-202-niveau-de-detail-des-contenants.md) consigne ces règles et **corrige ADR-200 D1**
(S374 l'avait écrit trop large : tout contenant *vu* → δ). Presque tout était déjà écrit (sources §2.1–2.2, ADR-010 §5–6,
ADR-012, ADR-013, ADR-025, ADR-197 D5). **Sept zones d'ombre** posées, une proposition chacune (§3) : qui calcule la
météo et comment V reçoit la même pluie partout ; les éléments bloquants qui changent en jeu ; la lame vue de loin ; le
seuil contenant entier / zone ; nuages et éclairs ; chaleur, évaporation, gel ; jusqu'où les effets factices. R27 reçu.
**Rituel.** Maillons **1** : aucun point ne change d'état. Suivant : les réponses aux zones d'ombre ; sinon, par
l'alternance, la physique — A320 (4.8), la coque qui bouge (6.4), la côte (2.7).

## S377 — 2026-09-26 — consignation : les réponses aux zones d'ombre d'ADR-202

**Entrée.** Les réponses de l'utilisateur aux sept questions d'ADR-202 §3.
**Fait.** [ADR-203](../../docs/adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) : la météo *« aussi poussée que l'eau »*,
réaliste et performante, son autorité déléguée au projet à sa conception (V recevra les mêmes litres partout) ; le joueur
**pose et retire une bâche entière ou une demi-bâche à tout moment** — exposition au ciel dynamique et fractionnaire ; de
loin, la lame et ses obstacles sont factices ; les contenants se divisent en domaines comme la mer, sans seuil unique ;
**après l'eau** : la météo, la topologie d'un territoire, le feu, la neige (5.11 non rouvert) ; la chaleur agit par V
(évaporation, gel) ; tout ce qui est lointain et sans conséquence est factice. Liste 5.5, 2.8, 7.6 ; aucun code.
**Rituel.** Maillons **2** : aucun point ne change d'état. Suivant : par l'alternance, **la physique**, et à deux maillons
un point qui change d'état — A320 (4.8), la coque qui bouge (6.4), la côte (2.7), ou l'exposition dynamique de 5.5.

## S378 — 2026-09-26 — physique : la pluie dans V, bâches comprises (5.5)

**Entrée.** *« Continue »* ; par l'alternance, la physique ; à deux maillons, un point qui change d'état : 5.5, décidée la
veille (ADR-203 D2 : bâche entière ou demi posée et retirée en temps réel). La météo n'est pas construite (à la fin).
**Fait** ([preuve](../../docs/validation/PLUIE-V-S378.md), [ADR-204](../../docs/adr/ADR-204-la-pluie-arete-de-v.md)). Une arête
de pluie du ciel vers un contenant : surface d'ouverture (et non surface libre, ADR-010 §5) × exposition — **la commande
de l'arête**, bâche 0, demi-bâche 500, sauvegardée par WVST v2 — × intensité, entrée du pas (`step_meteo`, `Meteo`) ;
`step` reste sans pluie, **au bit**. Une heure à 10 mm/h sur 32 m² : 319 999 ml pour 320 000 ; demi-bâche, bâche, bâches
posées en cours de pluie, à 1 ml. La piscine à débordement sous 20 mm/h : le déversoir débite exactement la pluie, charge
0,8560 mm pour 0,8569 (−0,11 %). Bilan exact, refus atomiques. Cœur : 543 réussis, 0 avertissement. **Erreur du critère
écrit** (240 000 ml pour la bâche entière posée à 30 min : c'est la demi-bâche) — dite, les deux cas éprouvés.
**Limites.** Ni absorption par le sol ni pluie hors contenant ; une intensité par réseau ; l'exposition calculée depuis
les objets posés n'est pas faite ; aucun consommateur.
**Rituel.** Maillons **0** : 5.5 passe à partiel (3 / 66 / 51) — devient possible une cuve qui se remplit sous la pluie
selon sa couverture, bâches posées ou retirées en jeu, sauvegardée ; consommée par l'hôte et, à la fin, par la météo ;
preuve §2. Suivant : par l'alternance, **le rendu** — l'échelle radiométrique, ou les rides de pluie factices (ADR-202 D3).

## S379 — 2026-09-26 — rendu : les rides de la pluie, factices (8.9) ; la campagne du solveur inscrite

**Entrée.** *« Continue mais il faudra prévoir une session du plus dur et complexe […] un solveur […] 3D volumétrique ultra
réaliste et performant en temps réel »* : la campagne inscrite en tête des sessions de physique (conception d'abord) ; par
l'alternance, le rendu — les rides de pluie factices (ADR-202 D3, ADR-203 D7). **Fait** ([preuve](../../docs/validation/RIDES-PLUIE-S379.md)).
Six photographies libres lues sans téléchargement. `pluie.gd` (Marshall et Palmer × Atlas : 447 anneaux/m²/s à 10 mm/h),
`pluie.gdshaderinc` (deux trains capillaires-gravité, 1,71 cm à 0,231 m/s et 4,4 cm à 0,178 m/s ; couches de mailles au
taux exact ; niveau de détail le long du rayon de chaque anneau, crêtes → bande anisotrope → rugosité), bassin et mer (en
coordonnées de Lagrange). Sans pluie, 12 images au bit ; taux compté à −4,2 / −0,6 / −1,7 % (2 / 10 / 50 mm/h) ; crêtes
éteintes à λ/2. **Deux impasses** dites : l'empreinte isotrope effaçait tout au-delà de 1 à 2 m ; la mer ne bougeait pas
(crêtes sous le pixel) — d'où le second train et la bande anisotrope. **Erreur** : un battement extrapolé (11:55 au lieu
de 11:22), vu par `--check`, corrigé au commit suivant (L237). **Coût** (720p, effet isolé) : mer +1,1 / +6,2 ms à 10 /
50 mm/h ; l'utilisateur : *« les LOD vont complètement bouleverser les performances »* — mesuré, texture à moments inscrite.
**Limites.** Pentes réglées sur les photos ; ni gerbes ni pluie dans l'air ; seul le ciel se reflète ; une intensité par
scène. **Rituel.** Maillons **1** : 8.9 reste partiel, R28 posée (non reçue). Suivant : **la campagne du solveur
volumique 3D temps réel, conception** (demande de l'utilisateur).
**Verdict R28** (après le rituel) : *« Parfait »* — les rides reçues ; consigné dans REVUE-VISUELLE §33, la liste, la file.

## S380 — 2026-09-26 — rendu : la pluie complète (ADR-205), d'abord dans l'air

**Entrée.** *« Pas de solveur continue la pluie ajoute les manquants »* (après R28, *« Parfait »*) : la campagne du solveur
attend. **Fait.** [ADR-205](../../docs/adr/ADR-205-la-pluie-complete.md) : treize pièces (air, surfaces, V, coût), leur ordre,
leurs sources, la frontière avec la météo. Pièces 1 et 2 ([preuve](../../docs/validation/PLUIE-AIR-S380.md)) : les gouttes
d'au moins 1 mm près de l'œil, au nombre de Marshall et Palmer, placées par hachage de l'indice (GPUParticles3D), vitesse
d'Atlas, traînées de Garg et Nayar ; l'extinction `β = (π/2)·2·N0/Λ³` dans la brume. Quatre photographies (averse sur des
piscines : voile gris, traînées faibles devant les fonds sombres). Sans pluie, 12 images au bit ; gouttes comptées par
classe à 3 % près (2, 10, 50 mm/h) ; extinction égale à la loi et à l'intégration numérique (visibilité 2,5 km à 10 mm/h).
**Erreurs dites** : le critère « ±5 % » oubliait le bruit de Poisson (30 gouttes attendues dans une classe à 2 mm/h) —
jugé à 2 σ sous 2,5 % de bruit ; le contrôle comptait les margelles comme gouttes blanches. **Coût** des gouttes : +0,1 à
+1,2 ms. **Limites.** Ciel ensoleillé (le voile en prend le bleu), pas de rideau à moyenne distance, pas de vent.
**Rituel.** Maillons **2** : 8.4 reste *absent* (R29 posée, non reçue), 8.8 reste partiel. À deux maillons, la suite doit
faire avancer un point : **R29 reçue fait passer 8.4 à partiel**. Suivant : la pluie, pièce 3 — le ciel de pluie (CIE
couvert) —, avec le verdict de R29.
**Verdict R29** (après le rituel) : *« Je valide »* — **8.4 passe à partiel** (3 / 67 / 50), maillons **0** : devient possible une scène sous la pluie où les gouttes tombent au nombre de la loi et voilent le lointain ; consommée par la piscine et la mer ; preuve §2.

## S381 — 2026-09-26 — rendu : le ciel de pluie (ADR-205, pièce 3)

**Entrée.** *« Continue »* après R29 : la pièce 3 d'ADR-205. Coupée après P5 (12:14) ; **reprise à chaud** à 12:19 sur
*« Reprends le projet »* — jeton `occupé` depuis 12:08, mais aucun processus d'agent en vie hors le nouveau, aucune
session de l'application en cours, arbre propre : rien à compléter ni à annuler (P6, marqué, pas commencé).
**Fait** ([preuve](../../docs/validation/CIEL-PLUIE-S381.md)). Deux ciels de pluie photographiés, mesurés : neutres (bleu
+1,5 à +7 %), rapport haut / horizon 1,72 contre 1,68 pour la CIE (champ supposé). `ciel.gdshaderinc` : `couvert`, le
ciel couvert normalisé de la CIE, neutre, `Lz` tenu pour que l'éclairement horizontal reste celui du ciel clair (l'œil
s'adapte, hypothèse dite) ; `eclairage()` partagé (orientation, sol renvoyé) ; `soleil_direct()` éteint disque, éclat,
caustiques, crêtes. La pluie couvre, `COUVERT=` force. Ciel clair : 12 images au bit ; couvert à 0,15 % de la CIE,
neutre ; vers le soleil −0,07 % ; sol mat, rapport 1,00000 ; coût +0 à +0,03 ms. **Erreur dite** : la lumière des
crêtes (S356), solaire, oubliée par P3–P4 — taches claires sur la mer couverte, vues sur les images de R30 ; éteinte
(P6a). **Limites.** Pas d'occultation du ciel : le bloc de la piscine se confond avec le sol (0,196 contre 0,188, par
les albédos) ; ciel sans texture ; lobe solaire sous l'eau ; orientation interpolée (+4,3 % au pire). **Rituel.**
Maillons **1** : aucun point de la liste ne change (le ciel sert 8.10 ; R30 posée, non reçue). **Arbitrage réel** (R30) :
l'œil adapté ou une scène plus sombre ; l'occultation du ciel avant la suite. Suivant : la pluie, pièce 4 — les
gerbes —, ou l'occultation du ciel si R30 la demande.
**Verdict R30** (après le rituel) : *« Je valide, ajoute l'occultation du ciel puis continue, et ensuite le plus important le
solveur 3D »* — le ciel de pluie reçu (8.10 reste partiel, maillons **1**) ; suivant : l'occultation du ciel (S382).

## S382 — 2026-09-26 — rendu : l'occultation du ciel et les ombres portées (ADR-206)

**Entrée.** Verdict R30 : *« Je valide, ajoute l'occultation du ciel puis continue, et ensuite le plus important le solveur
3D »*. **Fait** ([preuve](../../docs/validation/OCCULTATION-CIEL-S382.md), [ADR-206](../../docs/adr/ADR-206-la-visibilite-du-ciel-par-des-occultants-analytiques.md)).
Chaque surface de la piscine reçoit le ciel qu'elle voit — occultants analytiques (16 boîtes), 32 azimuts × 32 bandes
d'égal angle solide, quatre directions aux cellules de bord, total exact (forme close du ciel couvert incliné, 4·10⁻⁸,
qui remplace l'interpolation de S381) ; cuite une fois aux sommets (passe de points, `VERTEX_ID`), faces graduées aux
arêtes. Par ciel clair, le même calcul vers le soleil : les ombres portées, à chaque pixel. Critères : sans occultant, 12
images au bit ; la part vue à **0,0088** d'une intégration indépendante (`outils/occultation_ciel.py`, 15 points) ; bord
d'ombre à 2,5 mm (½ pixel de 5 mm), 539 819 pixels au soleil inchangés. **Échecs mesurés, dits** : 0,0147 d'abord (bouts
de boîtes en azimut) ; coût **+8,6 ms** par pixel, puis +2,3 par sommet, puis **+0,1 à +0,3 ms** cuit ; l'interpolation
manquait sous le débord (0,09) avant les faces graduées. **Erreurs** : deux battements écrits avant la lecture de l'horloge
(12:48 pour 12:46, 13:22 pour 13:17 — L237), corrigés au commit suivant. **Limites** : pas d'interréflexion, pénombre
étroite (10 pour 33 mm), fond immergé sans Snell, référence photographique qualitative (vignetage). **Rituel.** Maillons
**2** : 8.10 reste partiel, R31 posée ; la suite suit la demande de l'utilisateur. **Arbitrage réel** : jusqu'où la pluie
avant le solveur (« puis continue, et ensuite le plus important le solveur 3D »). Suivant : verdict R31, la pluie pièce 4
(les gerbes), puis la campagne du solveur volumique 3D.
**Verdict R31** (après le rituel) : *« Je valide R31, continue la pluie »* — occultation et ombres reçues (8.10 reste partiel) ;
suivant : la pluie, pièce 4, les gerbes (S383).

## S383 — 2026-09-26 — rendu : les gerbes de la pluie (ADR-205, pièce 4)

**Entrée.** R31 : *« Je valide R31, continue la pluie »*. **Fait** ([preuve](../../docs/validation/GERBES-S383.md)). Sources :
Murphy et al. (2015) repris par Wang et al. (2023) — goutte de 4,1 mm à 7,2 m/s, **relevée sur leur figure** (dôme 19,4 mm à
12 ms, jet 24,7 mm à 18 ms) ; exposants de Watson et al. (2024) pour l'échelle `s(D)`. Le tirage des impacts sorti des rides
(fonctions partagées : sous la pluie, 8 images au bit) ; particules aux mêmes impacts (piscine ; mer, posées sur la surface
déplacée — `bande_b.gdshaderinc` sorti de `surface_b`) ; silhouette relevée, moyennée sur la pose de 1/60 s ; au loin, la
part d'aire `M/tan ε`. Critères : sans pluie 12 au bit ; 961 gerbes au centre de leur anneau (0,76 mm) ; nombre à −2,8 %
(Poisson 3,2 %) ; sommet à 0,04 mm des relevés ; coût +0,01 à +0,33 ms. **Défaut de S380 trouvé et corrigé** : dans les
captures de la mer, la pluie ne suivait pas la pose — gouttes de la pose « référence » ≈ 11 m trop loin dans R29 et R30.
**Impasses dites** : `return` interdit dans les particules ; particules recouvertes par l'eau (ordre des transparents) ;
carré de contrôle mêlé par l'anticrénelage. **Limites** : silhouette simplifiée (ni doigts ni gouttelettes), une seule
mesure, éclaboussures au sol différées à la pièce 5 (film mince, Cossali et al. 1997). **Rituel.** Maillons **3** — 8.4
reste partiel, R32 posée ; **justifié** : la demande explicite de l'utilisateur (« continue la pluie ») prime sur la suite
automatique, et aucune pièce de pluie ne change seule l'état d'un point (8.4 demande aussi écume, spray, bulles). Suivant :
verdict R32, puis la pièce 5 (surfaces mouillées, éclaboussures au sol), ensuite la campagne du solveur volumique 3D.
**Clôture** (14:42, session cloud, reprise à chaud) : l'utilisateur a mis le dépôt en ligne (GitHub, `ladroguecmal/Fluidisim`,
branche `main`) pendant le rituel ; EN-COURS et le journal étaient écrits, le jeton non — **complété** : jeton libéré.

## S384 — 2026-09-26 — physique : la campagne du solveur volumique 3D, sa conception

**Entrée.** Session cloud (sans carte ni Godot), après la mise en ligne du dépôt par l'utilisateur ; S383 close (jeton
libéré). **Décision de l'utilisateur** : *« Solveur 3D ici »*. **Fait** : la conception demandée par la feuille de route
§3 ter ([CAMPAGNE-SOLVEUR-3D-S384](../../docs/registres/CAMPAGNE-SOLVEUR-3D-S384.md)) et
[ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md). Inventaire tiré des preuves ; état de l'art identifié
par recherche — colonnes hautes (Irving 2006, Chentanez et Müller 2011), hybride fonction hauteur + 3D + particules
(Chentanez, Müller et Kim 2014), multigrille (McAdams 2010, faces coupées Weber 2015), APIC, FLIP en bande, MPM, PBF,
DFSPH, Boltzmann, particules diffuses. **Constat** : le domaine de la porte B consomme seul les 2 ms de δ (1,92 ms,
S348) ; ≈ 0,4 M mailles au coût actuel (*estimé*) ; 5 cm près du joueur demandent des **colonnes hautes** (÷ 3,1 à la
porte B, *estimé*) et un coût par maille ÷ 2 (*à calibrer*). **Découpage** : C1 à C11, critères « reçu si » écrits, cinq
sans carte. **Limites** : aucun article lu en entier (hôtes bloqués) — chiffres de résumés seulement, lectures rangées
avant C2, C5, C7, C8 ; gains estimés, non mesurés ; suite Rust inchangée (664 réussis, 18 ignorés). **Erreur dite** : le
battement du commit de P8 porte 15:01 pour une horloge à 14:59 (deux appels lancés ensemble, L237) ; corrigé au rituel.
**Rituel.** Maillons **4** : aucune capacité reçue ; **justifié** — la conception est le préalable déclaré de la campagne
(§3 ter) et la demande explicite de l'utilisateur ; C1 est du code à critère mesurable. **Question** : δ dans Godot en C11
ou plus tôt (conception §6). Suivant : **C1**, la multigrille 3D de la référence ; au poste, verdict R32 et la pluie, pièce 5.

## S385 — 2026-09-26 — physique : C1, la multigrille 3D de la référence

**Entrée.** Verdict **R32** : *« n'est pas valide […] les gerbes n'apparaissent pas sur tous les impacts, elles peuvent tenir
dans le vide, et sont vraiment moches vues de près. Mais on laisse valide pour l'instant »* — reçu pour l'instant, trois
défauts à la file (peaufinage, non expliqués) ; *« Oui »* à C1. **Fait** ([preuve](../../docs/validation/MULTIGRILLE-3D-S385.md)) :
le cycle en V de S245 porté en 3D (`delta3d_multigrid.rs` ; Jacobi ω = 6/7 dérivé pour sept points, restriction moyenne,
injection, niveaux rediscrétisés depuis la surface), préconditionneur du pas mobile derrière `enable_multigrid`, **désactivé
par défaut**. **Critères 1 à 6 tenus** : cycle symétrique défini positif (essai **vu échouer** sur un cycle asymétrique) ;
mêmes surfaces à 10⁻⁶ m ; **9 / 10 / 11 itérations** de 12 288 à 786 432 mailles contre 102 / 191 / 365 pour Jacobi, bosse
de même ; à 128, premier pas ÷ 4,3 à 4,5, pas chaud ÷ 2,2 à 2,5 ; suite 667 réussis, zéro avertissement. **Limites** :
durées d'un conteneur ; divergence sur toutes les lignes 1,41·10⁻⁵ à 128 (lignes franches 5,9·10⁻⁶), dite, non expliquée ;
une itération coûte ≈ 8,7 Jacobi (coefficients fantômes recalculés) ; division stricte (porte B : deux niveaux) ; ni carte,
ni pas couplé mesuré. **Rituel.** Maillons **5** : aucun critère de porte ni état de point n'a changé (4.19 reste partiel) ;
**justifié** — la campagne est la demande explicite de l'utilisateur, et C1 tient le critère qu'ADR-207 lui a écrit. Suivant :
**C2**, les colonnes hautes (sans carte) ; au poste, la pluie, pièce 5 ; question ouverte : δ dans Godot, quand.

## S386 — 2026-09-26 — physique : C2, les colonnes hautes mesurées avant d'être construites

**Entrée.** *« Je suis d'accord avec toi pour le branchement à la fin »* — δ dans Godot en C11 (ADR-207, note) ; *« Réalise
la suite »* : C2. **Fait** ([preuve](../../docs/validation/COLONNES-HAUTES-S386.md)). La dispersion d'une colonne **se
calcule** (`scheme_frequency`, S295) : une colonne haute est une restriction de Galerkin du schéma fin, calculée avant
d'être écrite (`outils/colonnes_hautes.py`, six essais ; S295 redonné). **Critère d'usage** écrit avant : l'erreur de
fréquence ajoutée ≤ max(celle du schéma fin, 10⁻⁴), λ de 1 à 14 m, porte B. **Mesuré** : une colonne haute unique
linéaire ne le tient qu'à 17 couches cubiques sur 28 (÷1,47) — **l'estimation ÷3,1 de S384 était fausse** (seize fois le
permis à 14 m) ; grille étirée ÷2,0 ; **pression linéaire par morceaux sur nœuds étirés ÷2,55** (bassin 3 m : ÷2,14).
**ADR-208** remplace ADR-207 D2 : la colonne graduée ; en mode mobile, les couches cubiques doivent contenir la course de la
surface (D4). **Construit** au pas linéaire (`delta3d_graded.rs`, `enable_graded`) : `Pᵀ·A·P`, divergence restreinte ; six
essais (symétrie vue échouer sur une transposée faussée ; repos au bit ; volume 10⁻⁹ m³ ; l'onde oblique suit sa fréquence
calculée à 0,0048 %, Rust = Python) ; suite 673 réussis, zéro avertissement. **Limites** : aucun coût gagné (vitesses et
opérateur fins) ; ni pas mobile, ni fond coupé, ni carte. **Rituel.** 4.3 passe à **partiel** (3 / 68 / 49). Maillons
**0** : devient possible une pression à 11 inconnues sur 28 ; chemin qui le consomme : C2b puis C3 ; preuve ci-dessus.
Suivant : **C2b** — pas mobile, stockage compact, multigrille graduée.

## S387 — 2026-09-26 — physique : C2b, la colonne graduée au pas mobile, et la course de la surface

**Entrée.** *« Continue »*. **Correction d'abord** : le cas que S386 appelait « porte B » est une colonne de 7 m d'eau sous
surface fixe ; la scène de la porte B a **3,5 m d'eau sous 3,5 m d'air** (notes correctives d'ADR-207 et ADR-208). **Fait**
([preuve](../../docs/validation/COLONNES-HAUTES-S386.md) §5). **La course de la surface**, mesurée sur B (mer `--houle`
reconstruite, 840 points, 10 min) : **4,79 m** — 26 couches cubiques sur 28 avec le paquet : **la colonne graduée ne paie
pas en haute mer** (÷1,27 au plus en suivant la moyenne de B) ; en eau calme, ÷2,14 à 10 cm, ÷2,50 à 5 cm — elle sert les
contenants. **Construite au pas mobile** (garde de la course, départ chaud par injection, diagonale condensée) ; essais :
tous les nœuds = pas fin à 10⁻⁶ m, volume 10⁻⁹ m³, refus rendu au bit ; **ballottement** : écart au pas fin 1,62·10⁻⁴ m pour
1,92·10⁻⁴ prédits (rapport 0,84, critère 0,5–2 tenu). Suite 675 réussis, zéro avertissement. **Trois défauts corrigés** —
l'opérateur linéaire appliqué au pas mobile (nouveau) ; et **deux de S386** : le « vrai résidu » jamais recalculé (taille lue
sur un tampon emprunté), le résidu écrasé par la divergence ; chiffres de S386 inchangés. **Vu en passant** : le creux de B
plus le paquet touche presque le fond de la scène de la porte B (0,11 m), à la file. **Rituel.** Maillons **1** : aucun point
ni critère de porte ne change d'état. Suivant : **C3** au poste — la multigrille sur la carte, levier probable de la haute mer
(la projection pèse 2,06 ms des 3,68) ; **C4** dans le cloud — APIC en 3D.

## S388 — 2026-09-26 — physique : C4a, APIC en 3D dans le cœur

**Entrée.** *« Continue »* : C4, découpée — C4a ici (le solveur, le repos, le ballottement), C4b ensuite (B10 en 3D). **Fait**
([preuve](../../docs/validation/APIC3D-S388.md)) : `apic3d.rs`, le candidat 2D de S318–S320 porté en 3D **dans le cœur**, avec ses
leçons — surface reconstruite et fluide fantôme, extrapolation qui garde les faces alimentées, séparation, huit particules par
maille, `f32`, mémoire réservée et comptée. **Tenus** : un champ affine traverse les transferts (vu échouer sans le terme
affine) ; repos à 5,6 mm/s, masse exacte ; suite 681 réussis, zéro avertissement. **Manqués, mesurés** : la surface lue à 1 %
de maille — le rayon de S318 lit une face à −15 %, aucun rayon ne tient partout, le minimax retenu lit ±6,1 % (deux rangées de
particules par maille) ; le ballottement (1, 0) à **+2,05 % puis +1,04 %** (5 et 2,5 cm ; 2D : +5,9 et −0,15 %), manqué de
0,04 point à 1 % ; l'oblique (1, 1) à +2,69 % pour 2 % ; l'énergie « créée » oscille de +0,15 à −0,56 fois l'onde et décroît —
le critère de 1 % était plus fin que sa mesure. **Attribution** : le rayon de S318 fait +7,87 % au lieu de +2,05 % — **la lecture
de la surface commande la période** ; la séparation n'y est pour rien. **Faute d'instrument** corrigée : le modèle f64 de la
reconstruction ne mettait pas `dx` sans voisine. **Rituel.** Maillons **2** : aucun point ni critère de porte ne change (4.16
reste absent, construit non reçu). Suivant : **C4b** — la surface d'abord (plus de particules par maille, ou un ensemble de
niveaux, ADR-186 D4), puis B10 en 3D ; au poste, C3 et la pluie, pièce 5.

## S389 — 2026-09-26 — physique : C4b, première part — la surface d'APIC

**Entrée.** *« continue »* : la surface d'abord, que S388 désignait comme commandant la période. **Fait** ([preuve](../../docs/validation/APIC3D-S388.md)
§5) : la lecture de la surface jugée sur **huit** positions continues d'une maille, non deux — le noyau d'une maille se trompe
jusqu'à 9,93 %, non 6,12 %. Calculé en `f64` avant d'être codé : **le rayon du noyau** commande l'erreur, pas le nombre de
particules. **Retenu** : noyau de deux mailles, rayon minimax (lecture **2,46 %** ; modèle et 3D à 10⁻³ % près). **Trouvé en
chemin** : le noyau large faisait courir le repos à 15 cm/s — près d'une paroi, il ne voit des particules que d'un côté ; les
parois reflètent désormais les particules : repos **6,2 µm/s** ; les 5,6 mm/s de S388 venaient des parois. **Tenus** (critères
de S388, inchangés) : ballottement (1, 0) **+0,39 %** et oblique (1, 1) **+1,01 %** à 2,5 cm ; amortissement par période
positif partout (+0,10 à +0,78 %) — l'énergie ne croît pas. **Attribution** : les images aux parois corrigent (1, 0), le noyau
large l'oblique et divise l'amortissement par 18. Suite 680 réussis, zéro avertissement. **Limites** : lecture à 2,46 %, au-dessus
du 1 % de S388 (reste manqué) ; un noyau large lisse les nappes minces — non mesuré, et c'est ce que B10 éprouvera ; coût de
reconstruction ×1,5. **Rituel.** Maillons **3** : 4.16 reste absent (construit, non reçu). **Justification** : demande explicite
de l'utilisateur dans la campagne qu'il a choisie (ADR-207) ; la v1 est déjà reçue (§3 bis), la liste entière est la cible et
4.16 en est un point absent ; la suite, B10 en 3D, est la première épreuve d'une surface non graphe — elle fait passer 4.16 à
partiel ou dit pourquoi non. Suivant : dans le cloud, **C4b** — B10 en 3D (une sphère qui entre dans l'eau) ; au poste, C3 et
la pluie, pièce 5.

## S390 — 2026-09-26 — physique : C3, première part — la multigrille sur la carte

**Entrée.** *« Reprends le projet »*, puis, au poste, **C3** choisi par l'utilisateur (entre la pluie, pièce 5, et les gerbes).
**Fait** ([preuve](../../docs/validation/MULTIGRILLE-3D-S385.md) §5) : le cycle en V de S385 porté tel quel dans la production
(`viewer/src/delta3d_mg.*`), préconditionneur du gradient conjugué résident, **éteint par défaut** (empreintes au bit),
option vivante `--multigrille`. **Instrument** : carte contre réplique `f64` à 4,4·10⁻⁷, symétrie 1,8·10⁻⁷, vue échouer à
0,233 ; deux essais. **Porte B à 30 Hz** : le résidu de Jacobi-32 en **6 cycles** ; un cycle 0,147 ms ; projection **1,08 ms
contre 2,08**, pas q99 2,75 contre 3,77. **Cuves** : au plus 5,6·10⁻⁵ m (cas 1, sans niveau grossier ; 3,9·10⁻⁷ à 64
cycles). **Trouvé** : la scène amplifie tout écart (deux références convergées à 6 mm dès 1 s) ; **sur une minute, à 30 Hz,
tout explose en 24 à 40 s, références convergées comprises ; à 60 Hz, la minute tient** — **A321**, sévérité 3 : la porte C
n'avait jamais été éprouvée sur sa durée d'usage. **Limites** : 10 cm et A298 (C3b) ; défaut non changé ; le plus grossier
freine (0,43 par cycle ; 32 lissages : 0,27, plus cher). **Rituel.** Maillons **4** : 4.19 reste partiel. **Justification** :
choix explicite de l'utilisateur ; A321 est un défaut bloquant pour tout usage vivant de 30 Hz. Suivant : au poste, **la pluie,
pièce 5** (alternance d'ADR-191), puis **A321** avant C3b ; dans le cloud, **C4b** (B10 en 3D : 4.16 à partiel).

## S391 — 2026-09-26 — physique : A321 corrigée — l'advection de δ au second ordre en temps

**Entrée.** *« Corrige A321 d'abord »*. **Fait** ([preuve](../../docs/validation/A321-S391.md), [ADR-209](../../docs/adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md)) :
banc `--delta3d-a321`, commutateurs compilés dans des pipelines de banc à part (dans ceux de la production, ils en changeaient
les empreintes). **Cause** : la prédiction avance les vitesses de δ par Euler explicite et différences centrées (FTCS),
instable pour tout pas — explosion en ~`1/dt` (72 s à 60 Hz, 33 s à 25 ms, 23 à 40 s à 30 Hz), mode à l'échelle de la maille
(0,4 % → 31 %) ; aucun terme éteint seul ne sauve la scène. **Remède** : le terme que l'Euler omet, `+(dt²/2)·V_a·V_b·∂_a∂_b u`
(Lax-Wendroff) — actif par défaut dans la production, option du cœur (681 réussis, au bit). Essai de von Neumann : FTCS ×7,55,
corrigé ×0,134, comme prédit. **Tenu** : deux minutes à 30, 25 et 16,7 ms, **cinq minutes à 30 Hz** ; cuves au plus
2,5·10⁻⁵ m ; dispersion inchangée à 10⁻⁵ point ; porte C 1,910 / 1,927 ms. **Limites** : défaut du cœur non migré (ADR-209
D3) ; autres scènes non éprouvées sur la durée. **Rituel.** Maillons **0** : une correction d'intégrité reproduite puis
testée — **devient possible** δ vivant à 30 Hz au-delà de vingt secondes, la cadence de la porte C sur sa durée d'usage ;
**chemin** : la scène vivante (`--pas-delta=33333`), puis C3b et les scènes de C10 ; **preuve** : A321-S391. Suivant : au
poste, **la pluie, pièce 5** (alternance d'ADR-191), puis C3b ; dans le cloud, **C4b**.

## S392 — 2026-09-26 — rendu : la pluie, pièce 5a — les surfaces mouillées

**Entrée.** *« Continue avec la pluie, pièce 5 »* ; découpage déclaré : 5a les surfaces mouillées, 5b les éclaboussures au
sol. **Fait** ([preuve](../../docs/validation/SURFACES-MOUILLEES-S392.md)) : `godot/mouille.gdshaderinc` — la forme d'Ångström
avec la réflexion interne de Lekner et Dorf, `a·(1 − r̄ᵢ)/(1 − a·r̄ᵢ)·(1 − R(θ))`, `r̄ᵢ` = 0,47459 par deux intégrations
(`outils/sol_mouille.py`) ; reflet du film (ciel couvert, occultants) ; mouillé si la verticale est libre, sec sous les
débords, pieds de murs par rejaillissements (hypothèse), nez de margelle par ruissellement. **Tenus** : sans pluie 7 / 7 au
bit ; formule sur l'image à 0,03 %, reflet à 0,3 % ; bord sec à 0,00 mm. **Manqué de peu** : coût +0,34 ms en vue
d'ensemble (seuil 0,3). **Vu en chemin** : un rayon vertical dans le test de dalles divise par zéro — le dessous des débords
passait pour mouillé. **Photographie** (premières gouttes sur l'asphalte, Newport) : taches 0,08 du sec en linéaire contre
0,53 prédits — écart non attribué (courbe de l'appareil, reflets, second effet de Lekner et Dorf). **Limites** : film
uniforme, régime établi, sans vent ; R33 dira s'il faut assombrir. **Rituel.** Maillons **1** : 8.10 reste partiel.
Suivant : **R33** ; au poste, C3b (alternance : physique) ; dans le cloud, C4b ; la pièce 5b après R33.

## S393 — 2026-09-26 — physique : verdict R33 ; C4b, B10 en 3D — une sphère entre dans l'eau

**Entrée.** *« Reprends le projet, pour R33 je valide actuellement mais pour plus tard des sessions de peaufinage »*. Le travail
du poste (S390–S392, branche `poste`, poussée à ma demande) rejoint la branche en avance rapide. **R33** reçu pour l'instant,
peaufinage à venir sans défaut nommé ; la pièce 5b est libre ; liste 8.4 corrigée (« R32 posée », périmé). **Fait**
([preuve](../../docs/validation/B10-APIC3D-S393.md)) : un corps cinématique dans `apic3d.rs` — sphère, mailles solides, faces à sa
vitesse, paroi mobile, particules repoussées ; sans corps, le ballottement de S389 au chiffre près. **Trouvé** : au repos, 9,4
cm/s contre la sphère — le biais de paroi de S389 ; le corps reflète désormais les particules : 9,6 mm/s (critère 1 cm/s,
de justesse ; vu échouer sans mailles solides). **B10** sur un quart de domaine (un pas d'écart au domaine entier) : `Fr` = 2,
pincement **2,084 √(R/g)** à 16 mailles par diamètre, **dans la plage publiée** (1,72 à 2,29), **convergé à 0,63 %** entre 12
et 16 ; `Fr` = 4 à −2,3 % ; suite 684 réussis, zéro avertissement. **Limites** : plage lue dans des résumés (articles bloqués
par le réseau) ; couronne de maille (A312), bulle sans air (A311) ; parois à 2 D, effet d'un pas. **Rituel.** Maillons **0** :
**4.16 passe à partiel** — **devient possible** une surface non graphe calculée dans le cœur en 3D, reçue contre une mesure ;
**chemin** : C5 (le raccord aux colonnes), C6 (la bascule), C7 (la carte) ; **preuve** : B10-APIC3D-S393. Suivant : dans le
cloud, **C5** ; au poste, **C3b** (alternance : physique), puis la pluie, pièce 5b.

## S394 — 2026-09-26 — physique : C5a, A316 en 2D d'abord — l'échange comprime

**Entrée.** *« Continue »* : C5, le raccord particules ↔ colonnes. Chentanez, Müller et Kim (2014), à lire d'abord, sont
bloqués (quatre adresses) ; seul leur résumé est connu — un champ de densité commun. A316 se règle d'abord en 2D, où 30 s se
rejouent en 16 s. **Fait** ([preuve](../../docs/validation/B10-APIC-S320.md) §14) : la référence de S354 rejouée au chiffre près,
puis trois candidats, critères écrits avant, chacun attribué avant le suivant. **(A)**, la bande de S354 réensemencée depuis sa
hauteur géométrique : les particules se vident — elle cède à chaque pas le biais de reconstruction et l'arrondi des rangées.
**(A')**, la bande gardant ses particules : elle les piège — un réensemencement par pas détruit le transport sous la maille.
**(B)**, la densité corrigée en position (un champ dont la divergence vaut `n/4 − 1`, deux colonnes) : densité tenue (3,9 à
4,0 contre 4,96), masse tenue à 5 cm (±0,0013 m² contre +0,0113), repos 0,65 cm/s — **mais l'onde croît** de 4 % par
période, et à 2,5 cm la masse part dans l'autre sens. **Attribution** : la correction ajoute 102 J/m en 30 s, pour une onde de
deux ; S354 mesurait −87 J/m sous l'échange — **l'échange comprime**, c'est la racine d'A316. **Limites** : article non lu ;
(B) à facteur 1 et bande de 2, non réglés. **Rituel.** Maillons **1** : aucun point ne change (4.12, 4.16 partiels). Suivant :
dans le cloud, **C5a, deuxième part** — mesurer où l'échange comprime, puis un échange qui voit la densité ; au poste, C3b
puis la pluie, pièce 5b.

## S395 — 2026-09-26 — physique : C5a, deuxième part — une circulation permanente à la frontière

**Entrée.** *« Continue »* : où l'échange comprime (S394), puis un échange qui ne comprime pas. **Fait**
([preuve](../../docs/validation/B10-APIC-S320.md) §15) : un bilan par profondeur de la dernière colonne libre. **Prédiction
contredite** : l'excès n'est pas où l'on insère — on insère en haut, on retire en bas, l'excès est au milieu (5,0 à 5,5 par
maille). **(C)**, l'échange au sommet, déclaré avant : **pire** — le fond s'entasse (7,8 par maille), la correction de S394
devrait y ajouter 457 J/m ; un échange doit retirer où les particules arrivent. **La source, mesurée** : sur la face de la
frontière, une vitesse moyenne de **+21 mm/s en bas, −54 mm/s en haut**, tenue sur 30 s (APIC seul : moins de 2) — l'eau
entre dans les colonnes par le bas et en ressort par le haut ; même chose sans paroi, avec la mémoire de vitesse, frontière
déplacée : elle naît des colonnes. **Hypothèse non tranchée** : les colonnes, réensemencées à chaque pas, n'advectent pas la
quantité de mouvement ; l'épreuve par l'amplitude est dégénérée à 1 cm (onde sous l'espacement des particules). Sans
variable, tout au bit. **Rituel.** Maillons **2** : aucun point ne change. **Deuxième session de suite sur le raccord** : la
troisième ne se prend pas sans critère de porte franchi (S294) ; comparé à la file, le lot qui fait avancer une capacité dans
le cloud est **C8 en référence** — blocs épars, domaine qui suit la perturbation, fusion et séparation (4.9, absent). Suivant :
C8 (référence) dans le cloud, puis le raccord (trancher l'advection des colonnes) ; au poste, C3b puis la pluie, pièce 5b.

## S396 — 2026-09-26 — physique : C8a, la fusion et la séparation de domaines, en référence

**Entrée.** *« continue »* ; S395 désignait C8 (S294 : pas de troisième session de suite sur le raccord). **Fait**
([preuve](../../docs/validation/FUSION-S396.md)) : ADR-006 §3–4 écrit pour la première fois. `domain_blocks.rs` — un domaine est un
ensemble de blocs (colonnes de 8 × 8, toute la profondeur), fusion = union, séparation = partition, **une seule relation** pour
les deux (dilatations de 4 m qui se touchent), séparation après une seconde continue ; capacité réservée, sans allocation.
`Volume3::transplant` — l'état recopié au bit sur le réseau commun, les murs du receveur nuls, refus hors réseau. **Tenus** :
ensembles (quatre essais, vu échouer) ; aller-retour `C → (A, B) → C'` au bit hors de la coupure, et la coupure rendue le pas
suit au bit (vu échouer sans la pression de départ) ; **fusion au critère (0,94 s) à 0,26 % de l'amplitude** du domaine unique
sur 5 s — forcée après que les ondes ont frappé les murs, 3,4 puis 8,5 % ; suite 691 réussis, zéro avertissement. Publié : une
séparation ne saute pas, puis ses murs réfléchissent (2,6 % à 5 s). **Trouvé** : un premier témoin de séparation était
symétrique autour de la coupure — un plan de symétrie, où un mur ne change rien ; remplacé. **Limites** : des boîtes, pas de
blocs épars stockés ; des murs ; pas de croissance. **Rituel.** Maillons **0** : **4.9 passe à partiel** — **devient possible**
de fusionner et de séparer des domaines de δ sans rupture, par leurs ensembles de blocs ; **chemin** : la croissance d'un
domaine qui suit la perturbation (C8b), puis la carte et l'ordonnanceur (rang 4) ; **preuve** : FUSION-S396. Suivant : dans le
cloud, le raccord (trancher l'advection des colonnes) ou C8b ; au poste, C3b puis la pluie, pièce 5b.

## S397 — 2026-09-26 — physique : C5a, troisième part — la circulation était un défaut du banc

**Entrée.** *« Continue, l'objectif est de peaufiner et terminer le solveur »* (décision consignée) : le raccord (C5), verrou de
C6, C7 et C10, d'abord. **Fait** ([preuve](../../docs/validation/B10-APIC-S320.md) §16) : H1 de S395 éprouvée directement — les
colonnes du banc, réensemencées à la vitesse de la grille en des points fixes, n'advectaient pas la quantité de mouvement ;
avec une advection semi-lagrangienne au réensemencement, la vitesse moyenne sur la face de la frontière passe de +21 / −54 à
**4 mm/s** (critère 2 tenu ; sans variable, au bit). **Décisif pour la 3D** : le pas mobile de `Volume3` advecte (ADR-209) —
la circulation, la compression de S354 et la pompe de S394 étaient un défaut du **modèle de colonnes du banc**. **Critère 3
non tenu** : à 5 cm (solde) la masse tient mais densité 3,7 et période +3 points ; à 2,5 cm la période tient mais la masse
migre encore de +0,006 m² en 30 s, comme en S354. (D), la hauteur mouillée amont à la frontière, déclaré avant : **réfuté**.
**Limites** : la seconde cause non attribuée ; un banc 2D. **Rituel.** Maillons **1** : aucun point ne change (A316 scindé).
Suivant : dans le cloud, **C5b** — le raccord en 3D, APIC 3D et `Volume3`, avec les instruments de S394–S397 ; au poste, C3b
puis la pluie, pièce 5b.

## S398 — 2026-09-27 — physique : C5b, première part — la zone des colonnes dans APIC 3D

**Entrée.** *« Continue »* (objectif : terminer le solveur) ; le raccord en 3D. **Fait** ([preuve](../../docs/validation/RACCORD-3D-S398.md)) :
le raccord demande une seule projection pour colonnes et particules ; `Apic3` reçoit une **zone de colonnes** (`apic3d_columns.rs`),
masque réservé à la configuration — surface `η` par colonne (`φ = z − η`, sans reconstruction ni son biais), vitesse eulérienne
advectée au pied de la caractéristique (la leçon de S397), `η` par débits mouillés comme δ ; quatre crochets inertes sans masque.
**Tenus** : sans masque, le ballottement de S389 au chiffre près, suite 694 réussis, zéro avertissement ; toutes colonnes, repos
2,4·10⁻⁵ m/s, volume à 10⁻¹¹ ; ballottements **à ≤ 0,03 point de δ** sur les trois cas qu'il calcule, amortissement ≥ 0
(+0,3 à +1,4 % par période : l'advection semi-lagrangienne dissipe plus que celle de δ). **Observé sur δ** : sa projection
mobile refuse la cuve mince (1, 0) à 2,5 cm, Jacobi comme multigrille — non étudié. **Limites** : ni bande ni échange encore ;
advection du premier ordre. **Rituel.** Maillons **2** : aucun point ne change d'état. La demande de l'utilisateur prime (terminer
le solveur) ; la suite fait avancer une capacité — le raccord reçu. Suivant : dans le cloud, **C5b, deuxième part** — une bande
de particules dans la zone de colonnes, l'échange à la frontière (flux de la face, retrait où les particules arrivent), les
critères de S394 sur 30 s en 3D ; au poste, C3b puis la pluie, pièce 5b.

## S399 — 2026-09-27 — physique : C5b, deuxième part — la bande et l'échange, presque reçus à la maille fine

**Entrée.** *« Continue »* (objectif : terminer le solveur). **Fait** ([preuve](../../docs/validation/RACCORD-3D-S398.md) §5) : dans
APIC 3D, une bande de particules à côté de la zone des colonnes, une seule projection. La reconstruction **voit les colonnes**
(particules virtuelles en rangées étirées sur `[0, η]`, l'idée du champ de densité) : surface lue contre la zone à 2,19 % de maille
comme au milieu de la bande (14,7 % sans elles, vu échouer). **L'échange** par le flux de la face et des soldes `f64` —
absorption, retrait où les particules arrivent, pose contre la face. **Tenus** : sans zone et toutes colonnes au bit ; volume total
à 10⁻¹⁰ sur 30 s ; suite 697 réussis, zéro avertissement ; ballottement de 30 s, frontière au nœud, **à 2,5 cm** : masse de la bande
à ±0,6 mm d'APIC seul (2D : +6 mm), saut 0,15 maille, période à 0,01 point, amortissement à 0,09 point. **Manqués** : la densité à
2,5 cm (7,3 par maille pour 7,6) ; à 5 cm, +3,1 mm par un courant de surface (−7 mm/s) ; le repos, 1,007 cm/s. **Attribués, pas
éprouvés** : la bande lit sa surface avec le biais de la reconstruction, les colonnes exactement (marche de 1,1 mm, seiche, courant) ;
la séparation pousse à travers la frontière des particules absorbées. **Rituel.** Maillons **3**. **Justification** : la demande de
l'utilisateur — terminer le solveur ; le raccord est le verrou de C6, C7 et C10, et il est à un critère d'être reçu à la maille fine,
ce qui ferait avancer 4.12 et fermerait A316 ; sa suite débloque toujours l'usage visé. Suivant : **S400**, la zone qui lit sa
surface comme la bande (`η + e(η)`) et la séparation tenue du côté de la bande, critères inchangés ; au poste, C3b puis la pluie 5b.
