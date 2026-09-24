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

Session : S343 — **en cours**. **Porte C, charge utile du fond** ; chemin de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S342 ([preuve](../docs/validation/COUT-DELTA3D-S341.md) §6) : le fond factorisé, au bit, ne retire que
0,11 ms au pas ; ce qui reste est l'**accumulation des 26 champs** de chaque face et leur **écriture**, 120 Mo par
pas. Le pas et le couplage n'en lisent que dix, selon l'axe de la face : `η`, `u` (3), `du/dt`, une ligne de
`grad u` (3), `p`, `grad p` sur cet axe.

**Maillons à deux, et le choix** (REPRISE §6). La porte C est la porte en cours ; son critère ne se franchit que
par la combinaison des leviers (ADR-131 D4), chacun mesuré. Chemin chiffré : charge utile (ici), puis projection
(multigrille ou fusion, 2,06 ms linéaires en cycles), puis cadence découplée, qui divise la contribution par image.
La porte A, comparée, demande l'ordonnanceur sur des domaines 3D et un banc B8 inexistant : plus loin d'un critère.
**Choix : C**, justification au journal si le troisième maillon tombe.

**Ce que la session doit rendre possible.** Un fond de δ à dix champs par face. Pour garder intacts les bancs de
S300, qui compilent le même fichier de noyaux avec 26 champs, la disposition compacte est une **constante de
compilation** (`override COMPACT`), vraie pour le pas seulement.

Critères, écrits avant le code :
1. **Empreinte de référence**, relevée avant tout changement : surface publiée et vitesses après 60 et 600 pas sur
   la scène de B, par un banc `--delta3d-empreinte`.
2. **Identité** : après le changement, les mêmes empreintes, au bit. Les bancs de S300 (`--delta3d-faces`,
   `--delta3d-couplage`) rendent leurs nombres publiés.
3. **Coût** : fond seul et pas entier, médiane et 99ᵉ centile ; secteur relevé ; preuve (§7), file.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le banc d'empreinte ; l'empreinte de référence ; critère 1.
- [ ] **P3** — la disposition compacte : noyaux, couplage, pas, tampon ; relectures de banc.
- [ ] **P4** — l'identité, les bancs de S300, le coût ; critères 2 et 3.
- [ ] **P5** — preuve, file ; rituel.

### Notes de reprise
- **P2, critère 1 tenu.** `--delta3d-empreinte`, commit `fc37f7b5` + banc, fond par tuiles, deux passages
  identiques : **60 pas** surface `0x5efa267462dfa0ad`, vitesses `0xc5c6a85d3d29f44b` ; **600 pas** surface
  `0x9325cf58781f8b74`, vitesses `0xea1bebe0ffabc19a` (h₀ 0,007277250 m). La carte est déterministe d'un passage à
  l'autre.

