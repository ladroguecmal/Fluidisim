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

Session : S155 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S154-1. B2 mesure des bilans à 60 s ; le noyau de pression refuse au-delà de 16 s.
Prendre la seconde branche annoncée par S154 — **isoler par mesure le blocage numérique** —
parce que la première (quantifier le domaine d'un sillage prolongé) est inaccessible tant que le
noyau refuse la durée à laquelle B2 mesure. Question : le 16 s d'ADR-071 est-il une limite
numérique ou un périmètre déclaré ? ADR-071 dit lui-même « à calibrer par réception » ; personne
ne l'a fait.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [x] **P2** — sonde : erreur du noyau modal contre l'oracle f64 `PressureMode` pour des âges de
      0 à 64 s, durée active inchangée. La constante d'horizon est relevée **localement et non
      committée** — c'est ce qui rend la mesure possible, et rien d'autre ne change.
- [x] **P3** — séparer ce que « 16 s » recouvre : l'**âge** auquel on échantillonne et la **durée
      active** du forçage n'empruntent pas le même chemin numérique. Mesurer la seconde seule.
- [x] **P4** — décider d'après les chiffres : nouvel ADR si les deux bornes se séparent, ou
      provenance mesurée écrite pour la borne conservée. Un ADR n'est jamais réécrit.
- [x] **P5** — appliquer la décision dans le code, avec un test **témoin** : désactiver le
      mécanisme doit faire échouer le test, sinon le test ne prouve rien.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 1643232 = master (S154, Codex). Worktree remis en avance rapide, rien d'unique.
296 tests/cinq ignorés, 105 ADR, 212 angles, 230 leçons, 18 invariants.

Ce que la lecture du noyau donne **avant** toute mesure, et qui oriente la sonde :
- après extinction, `sample` calcule la rotation libre par `phase(frequency, age - active, 1e6)`,
  arithmétique **entière** i128 réduite modulo un tour ; aucun flottant ne porte le temps ;
- la seule accumulation f32 dépendant du temps est `scale_integer(sinc/1e6, us)` dans la branche
  proche de zéro de J, et son argument est `active = min(age, duration)`, **borné par la durée** ;
- le commentaire de `scale_integer` dit « au plus 24 bits pour une durée <=16 millions de µs » :
  24 bits, c'est 2^24 = 16 777 216 µs. Le 16 s a donc l'air d'être un **nombre de bits**, pas une
  seconde physique.

Prédiction écrite avant la mesure, pour qu'elle puisse être démentie : l'erreur sera à peu près
**plate** en âge et croissante en **durée active**. Si elle croît aussi en âge, la prédiction est
fausse et c'est le résultat le plus intéressant de la session.

Piège : mesurer contre un oracle qui partagerait la quantification f32 de ω masquerait justement
ce qu'on cherche. L'oracle S95 convertit le **même** f32 en f64 — il isole l'erreur
d'implémentation, pas celle de l'entrée. Garder cette convention et le dire.

P2 — la prédiction est **démentie sur l'âge** et trompeuse sur la durée.
Écart contre l'oracle f64 (30 couples par ligne, 5 k x 6 rapports Doppler, 10 Pa, origine 0,7/-0,3) :
- âge croissant, durée active 4 s : 1,381e-7 m à 16 s, 2,539e-7 à 32 s, 5,931e-7 à 60 s. L'âge
  n'est donc **pas** gratuit, contrairement à ce que la lecture du code laissait croire.
- durée active croissante : 4,835e-7 m à 16 s, 7,463e-6 à 60 s — quinze fois plus. Mais
  l'amplitude elle-même passe de 7,50e-2 à 2,80e-1 m : la moitié de cette croissance est du signal.
- **l'erreur relative est la même dans les deux régimes** : 5,03e-7 (1 s), 7,37e-6 (16 s),
  3,18e-5 (60 s) pour l'âge ; 6,45e-6 (16 s), 2,66e-5 (60 s) pour la durée. Rapportée au temps
  écoulé, elle vaut **4,0e-7 à 5,3e-7 par seconde**, à peu près constante sur deux décades.
Loi apparente : dérive de phase linéaire en temps, ~5e-7 relatif par seconde, soit ~4 eps f32/s.
Hypothèse à trancher en P3 : omega est calculé en **f32** par `(gravity*magnitude).sqrt()`, erreur
relative ~6e-8 ; la phase omega*t hérite d'une dérive omega*t*6e-8. Pour le pire k=9, omega=9,4 rad/s,
cela donne 5,6e-7 par seconde — l'ordre de grandeur mesuré. Si c'est cela, la borne n'est ni
l'horizon ni les 24 bits de scale_integer, mais **le stockage de omega en f32**.
Artefact corrigé en cours de route : une ligne affichait 0,000000e0 parce que l'horizon demandé
dépassait ce que le constructeur acceptait et que toutes les combinaisons étaient sautées. Un
écart nul sans comparaison ressemble exactement à un résultat parfait. La sonde compte désormais
ses couples et annonce l'horizon accepté.
Constante d'horizon relevée à 64 s **localement, non committée** — sans elle rien n'est mesurable.

P3 — la dérive est attribuée, et l'erreur se décompose en deux termes qui n'ont rien à voir.
Premier dénominateur choisi (|eta| instantané) : instable, il explosait au voisinage des nœuds de
l'oscillation — 7,75e-5 pour k=(0,6 ; 0,8) qui n'était pas une perte de précision mais un
dénominateur proche de zéro. Repris avec l'amplitude invariante A = sqrt(|eta|^2 + |v|^2/omega^2),
conservée par la rotation libre. **Deuxième fois cette session qu'un artefact de sonde ressemble
à un résultat.**
Désaccord de pulsation, fait exact et non mesuré : domega/omega vaut -6,58e-9 (k=1),
-1,85e-8 (0,6 ; 0,8), -2,145e-8 (k=0,0234 et k=6), -5,73e-8 (k=9) — l'arrondi f32 de omega.
Écart mesuré contre l'oracle f64, rapporté à A, durée active 4 s :

| k | 16 s | 32 s | 64 s | prédit |domega|*64 | même omega, 64 s |
|---|---|---|---|---|---|
| (6, 0) | 1,554e-6 | 3,627e-6 | 8,794e-6 | 1,053e-5 | **1,505e-7** |
| (1, 0) | 8,285e-7 | 1,233e-6 | 2,132e-6 | 1,319e-6 | 1,912e-6 |
| (9, 0) | 2,739e-5 | 3,602e-5 | 2,283e-5 | 3,447e-5 | 4,402e-6 |
| (0,6 ; 0,8) | 1,028e-5 | 1,084e-5 | 1,250e-5 | 3,708e-6 | 5,377e-6 |
| (0,0234 ; 0) | 2,796e-7 | 7,657e-8 | 2,036e-7 | 6,582e-7 | 1,214e-7 |

Lecture : pour k=(6,0), l'écart croît d'un facteur 5,7 de 16 à 64 s, colle à la prédiction
|domega|*t à 20 % près, et **tombe d'un facteur 58 quand l'oracle porte le même omega** — la
cause est établie pour ce mode. Pour k=(9,0) et (0,6 ; 0,8), neutraliser omega ne suffit pas :
il reste 4,4e-6 et 5,4e-6, **constants en temps**. Il y a donc deux termes :
- un terme **indépendant du temps**, 1e-7 à 5e-6 selon le mode, venant de la phase spatiale et de
  l'amplitude en f32 (`from_distance` par axe, `magnitude` f32) — présent dès t=0 ;
- un terme **proportionnel au temps**, |domega|*t, venant du stockage de omega en f32.
Le second domine au-delà d'un croisement propre à chaque mode. Aucune rupture, aucun seuil,
rien qui distingue 16 s de 15 ou de 17 : **la fenêtre de 16 s n'a pas de justification numérique.**
Ce qu'elle a, c'est 2^24 microsecondes, c'est-à-dire un nombre de bits.

Fait décisif pour la décision : le budget est en **âge**, pas en durée active. ADR-104 découpe
déjà le mouvement en tronçons, chacun source distincte avec sa propre naissance ; un sillage de
60 s est fait de tronçons courts que l'on doit pouvoir **observer** 60 s plus tard. Ce qu'il
manque à B2 est l'horizon, pas la durée.
Deuxième constante à bouger, trouvée en cherchant : `bound_pressure::Context::new` borne aussi
`end - start` à 16 s. C'est la « fenêtre » de S151. Les deux disent la même chose au même endroit
du raisonnement, et l'une sans l'autre ne débloque rien.

P4 — ADR-106 : l'horizon d'observation n'est pas la durée de forçage. Horizon à 64 s dans
`modal_pressure` **et** `bound_pressure` ; durée active conservée à 16 s parce qu'ADR-104
découpe déjà le mouvement et que B2 a besoin d'observer, pas de forcer. Budget de précision
écrit à la place de la constante : <4e-5 relatif à 64 s, soit ~7e-5 en énergie contre le seuil
1e-4 E0 de B2 — sous le seuil, sans marge confortable, et il fallait le dire.
Non fait et nommé : corriger la dérive en portant omega au-delà du f32 (A213 à écrire en P5).

P5 — trois constantes, pas deux. `modal_pressure` (horizon 64 s, durée active 16 s explicitée),
`bound_pressure::Context::new` (fenêtre 64 s), et **`pressure_source::Source::new` qui en hérite
par `Context::from_recipe`** — celle-là, c'est un test existant qui l'a trouvée, en cessant de
refuser ce qu'il refusait. Note datée ajoutée à ADR-106 le jour même : je ne le réécris pas.
Témoin vérifié : horizon ramené à 16 s, le test S155 échoue à la construction (ligne 331), pas au
seuil ; horizon rétabli, il passe. Un test qui ne peut pas échouer ne prouve rien.
Le condensat S95 `8ea15f4a3334830b` est **inchangé** : seules des bornes de domaine ont bougé,
aucune arithmétique. C'est la contre-épreuve la moins chère de la session.
297 tests/cinq ignorés (204+93), debug et release, zéro échec.
