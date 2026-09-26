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

Session : S381 — **en cours**. *« Continue »* après R29 (*« Je valide »*) : la pluie, **pièce 3 d'ADR-205 — le ciel de
pluie**. Agent : Claude Opus 5.5, application desktop ; fichiers, git, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Quand il pleut, le ciel est couvert (nimbostratus, épaisseur optique de plusieurs dizaines) : **aucun soleil
direct** (transmission `e^(−τ/μ)` nulle en pratique) — ni disque, ni éclat sur l'eau, ni caustiques, ni ombres —, et une
luminance du ciel **neutre** (lumière du jour D65, le blanc de sRGB) répartie selon le **ciel couvert normalisé de la
CIE** (Moon et Spencer 1942) : `L(h) = Lz·(1 + 2·sin h)/3`, trois fois plus clair au zénith qu'à l'horizon. L'éclairement
horizontal vaut alors `(7π/9)·Lz`. **L'œil s'adapte** : l'éclairement horizontal rendu reste celui du ciel clair (le
modèle de la scène, `gain·(0,6 + 0,4·sin h_soleil)`, soit `Lz = (9/7)·gain·0,939`) ; une surface inclinée reçoit du ciel
couvert `0,396 + 0,604·n_z` de l'horizontale (0,396 à la verticale, rapport de la CIE), et du sol la part `ρ_sol·(1 − n_z)/2`.
Un réglage `couvert` de 0 à 1 ; la pluie le met à 1 ; la météo le commandera.

**Critères, écrits avant.** (1) Ciel clair et temps sec : les 12 images **identiques au bit**. (2) Le ciel couvert rendu
suit `(1 + 2·sin h)/3` à ±1 % (élévations 0, 15, 30, 60, 90°) et reste neutre (canaux égaux à 1 %). (3) Aucun soleil :
au voisinage de sa direction, la radiance du ciel est celle du ciel couvert à 1 % ; ni éclat ni caustiques. (4)
L'éclairement horizontal conservé : une surface horizontale mate a la même radiance rendue sous le ciel clair et sous le
ciel couvert, à 1 %. (5) Photographies de ciel de pluie chiffrées (neutralité) ; jugement de l'utilisateur (R30).

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — références : ciels de pluie photographiés, neutralité et gradient mesurés.
- [ ] **P3** — `ciel.gdshaderinc` : `couvert`, la luminance de la CIE, le soleil éteint, `eclairage(n, soleil, direct)` ;
  `gain_eau` déplacé là (l'éclairement de la scène).
- [ ] **P4** — les nuanceurs : eau, bassin, parois, fond, caustiques, éclat ; critère 1.
- [ ] **P5** — les scènes : `couvert` par la pluie ou `COUVERT=` ; contrôles (critères 2 à 4).
- [ ] **P6** — images de R30 ; preuve `CIEL-PLUIE-S381` ; registres.
- [ ] **P7** — rituel.

### Notes de reprise
