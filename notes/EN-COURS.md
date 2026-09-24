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

Session : S347 — **terminée**. **Revue R17, les deux cadences** ; chemin de la v1, porte C.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S345–S346 ([preuve](../docs/validation/COUT-DELTA3D-S341.md) §8–9) : la cadence de 30 Hz d'ADR-012 §7, seule à
passer sous 2 ms par image une fois étalée, change l'amplitude d'une onde forte de quelques pour cent — un
amortissement numérique par pas (A318). Aucune cadence n'est « la vraie ». Savoir si cela se **voit** appartient à
l'utilisateur (ADR-189 D3).

**Ce que la session doit rendre possible.** Le verdict : la scène de la porte B — le front de S302, la mer de R14
(`--meilleur --eau-physique=2`) — rendue avec δ à 60 Hz et à 30 Hz, aux mêmes instants, aux mêmes poses.

Critères, écrits avant le code :
1. `--pas-delta=<µs>` règle le pas de la scène δ 3D ; sans lui, la scène et ses captures de S302 et S339 au bit.
2. Captures à 2, 5 et 8 s, quatre poses, aux deux cadences, dans `viewer/captures/s347` ; aperçus PNG et
   différences ×6 entre cadences ; empreintes publiées.
3. **R17** au registre, questions écrites ; **arrêt pour le verdict**.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le pas de la scène réglable ; identité des captures sans lui ; critère 1.
- [x] **P3** — les captures aux deux cadences, aperçus, différences ; critère 2.
- [x] **P4** — R17 au registre, preuve (§10), file ; rituel ; arrêt ; critère 3.

### Notes de reprise
- **P2, critère 1 tenu.** `--pas-delta=<µs>` règle `Config::step_us` ; captures étiquetées `s347_<Hz>hz` dans
  `viewer/captures/s347`. Sans l'option, `INSTANTS=60,120 --houle --delta3d --captures` : **16 empreintes identiques
  au bit** à celles relevées en S339 — les leviers de S342–S343 n'ont pas bougé un pixel non plus.
- **P3, critère 2 tenu.** `INSTANTS=120,300,480 … --pas-delta=16667` et `INSTANTS=60,150,240 … --pas-delta=33333`,
  `--meilleur --eau-physique=2 --delta3d --captures` : 2, 5 et 8 s, quatre poses. Instants à 60 µs près entre
  cadences (120 × 16,667 contre 60 × 33,333 ms) : les images « B seul » diffèrent d'autant. **Pixels différents entre
  cadences** (avec δ) : référence 17–21 % dont 3,9–5,8 % de plus de 4 niveaux ; proche 36–41 % dont 12–15 % ; rasante
  11–14 % dont 1,3–1,8 % ; haute 9–14 % dont 1,3–2,8 %. À l'œil : les mêmes vagues ; la différence ×6 est un grain
  fin sur l'emprise de δ, pas la forme de l'onde. Empreintes proche avec 5 s : 60 Hz `0x3194cb20dc2cc6d6`, 30 Hz
  `0x887a04c139e18030`.

