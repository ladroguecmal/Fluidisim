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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S744 — **terminée**. En autonomie ; session longue. REPOS-PENTE-S743 : la projection de densité met le lac en mouvement contre un
fond. **La question** : une densité rapportée à la valeur nominale de chaque maille, qui compte le fond, rend-elle à la projection le repos
et la remontée, sans perdre l'onde solitaire ?

**La cause, lue dans le code** (`apic3d_densite.rs`) : la densité d'une maille est `Σ w/8`, les poids trilinéaires des particules. Au bord
du domaine, les poids sont rabattus : rien ne se perd, et le canal de S740 tenait. Contre un fond posé dans le domaine, une partie des
poids tombe dans les mailles solides. Les mailles d'eau qui touchent le fond paraissent moins denses qu'elles ne le sont, et la projection
comble un déficit qui n'existe pas.

**Le remède** (`set_density_bed_aware`, **éteint par défaut : au bit**) : chaque maille a sa densité nominale, `Σ w/8` d'un réseau régulier
(2 × 2 × 2 par maille) posé partout hors du fond, sous l'escalier ou le fond lisse. Elle est calculée une fois, au premier pas, dans un
tableau réservé à la configuration (I-06). La densité vaut alors `Σ w / nominale`. Au repos, elle vaut 1, fond compris.

**Les essais, et leurs critères écrits avant.**
1. **Le repos** (S743), l'escalier, 1:30 et 1:12, la projection consciente du fond : la vitesse sous 1 cm/s, l'écart par les particules sous
   3 mm (S743 sans elle : 6,3 cm/s, 4,7 mm). Le fond lisse rapporté (son défaut propre, S743).
2. **Le canal** (S740 E5, 5 cm, fond = plancher) : **identique au bit** à la projection d'avant (aucun poids perdu).
3. **La remontée de S645** (B4 de S742) avec la projection consciente : à **10 %** de la loi (S742 : −12 %).

**Contrôles du plan** (ADR-236, ADR-276, ADR-286, ADR-287)

- **témoin** :
  - le repos exact ;
  - la projection d'avant, au bit sur le canal ;
  - la loi de Synolakis ;
  - S645 sans projection (+0,5 %).
- **instrument** : celui de S743 (deux lectures) et de S645 (la particule la plus haute).
- **calcul** : le repos, quatre cas, 3 min ; le canal, 2 min ; B4, 6 min.
- **ADR** :
  - ADR-276 D2 (une cause : la normalisation) ;
  - ADR-287 D1 (le banc) ;
  - ADR-236 (le réseau nominal est celui de la pose, `dx/2`).
- **pièges** :
  - le fond doit être posé avant le premier pas (la nominale est calculée au premier) ;
  - une maille presque toute sous le fond a une nominale minuscule : sous 0,05, la densité n'y est pas corrigée.

### Plan

- [x] **P1** — jeton ; plan ; les lignes du lot S741–S743.
- [x] **P2** — la densité nominale ; (1), (2).
- [x] **P3** — (3), la remontée.
- [x] **P4** — preuve ; fermeture.

### Notes de reprise
- **(1) tenu** : l'escalier avec la projection consciente, 6,8 mm/s et 0,02–0,07 mm (comme sans projection). Le lisse : 0,26–0,80 m/s,
  7–11 mm (il ne diverge plus).
- **(2) manqué**, la prémisse fausse : la crête finale 112,24 contre 114,02 mm ; aux murs, la nominale dépasse 1 (les poids rabattus).
- **(3) manqué** : la remontée 0,2042 m (−11,0 %), contre −12 % avant. Le fond n'était pas la cause du freinage.
