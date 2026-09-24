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

Session : S350 — **en cours**. **Porte A, un domaine qui se redimensionne** ; la dernière porte de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S349 : un domaine δ 3D se déplace, au bit, et suit la caméra sans s'éteindre. Reste du deuxième critère :
**se redimensionner** ; puis la dégradation de rang 1 (session suivante), qui en a besoin.

**Un défaut trouvé en préparant.** Les faces normales du bord — `u` en `i = 0` et `i = nx`, `v` en `j = 0` et
`j = ny` — ne sont jamais écrites par le pas : ce sont des murs pour δ, nuls depuis l'état initial. Le décalage de
S349 y recopie des valeurs intérieures, qui restent ensuite constantes : un débit parasite à travers le bord.
L'identité de S349 l'a déclaré conforme parce qu'elle comparait à l'ancien translaté, murs compris.

**Ce qui permet le redimensionnement.** Les noyaux lisent les dimensions dans des uniformes. Les tampons se
réservent à la **capacité** — la forme de création — ; une **forme courante** (`nx`, `ny` au plus la capacité, `nz`
fixe) commande comptes, dispatchs, copies et relectures. Redimensionner, c'est réécrire l'état dans la nouvelle
disposition — recouvrement au bit, murs nuls, repos ailleurs —, puis les uniformes et l'origine.

Critères, écrits avant le code :
1. **Les murs** : après un décalage, les faces normales du bord sont nulles, le reste au bit comme en S349 ; l'effet
   sur le volume de δ, mesuré au banc de suivi avant et après.
2. **La forme courante** : quand elle vaut la capacité, les empreintes du pas de S343 (60 et 600 pas) sont inchangées.
3. **Le redimensionnement** (`Step3::resize`, une seule soumission) : (a) l'état réécrit est identique au bit à
   l'ancien dans le recouvrement, murs nuls, repos ailleurs ; (b) **le pas d'un domaine redimensionné est identique
   au bit à celui d'un domaine créé à cette forme** avec le même état — un domaine, pas une vue ; (c) aucune
   allocation.
4. **Le coût suit l'emprise** : pas médian et q99 à 100, 75, 50 et 25 % de la surface, alimentation relevée (A270) ;
   le décalage, devenu un redimensionnement à forme égale, remesuré.
5. Preuve, file, feuille de route.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les murs après un décalage ; critère 1.
- [x] **P3** — la forme courante dans `Step3` ; critère 2.
- [x] **P4** — `Step3::resize` en une soumission ; critère 3.
- [ ] **P5** — le coût selon l'emprise ; critère 4.
- [ ] **P6** — preuve, file, feuille de route ; critère 5.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** Avant correction, au suivi : murs figés jusqu'à **0,169 m/s**, débit net **−2,80 m³/s** après
  l'aller, +2,76 après le retour ; après : **0 exactement**. Dérive du volume de δ quand le domaine ne bouge pas :
  −2,42 → −2,23 m³/s après l'aller, +3,60 → +3,28 après le retour, +0,39 → +0,64 avant tout décalage — **la même** :
  c'est le fond qui la porte (le volume de δ n'est pas conservé sous B), et `fluxes` n'emploie pas les faces de bord.
  Mais `divergence` les lit : essai déterministe `--delta3d-murs` (deux domaines, décalage (+3, 0), mur gauche de S349,
  −0,385 m³/s, 120 pas) : surface écartée de **7,2 cm** à moins de 3 mailles du mur, 1,8 cm à 3–30 mailles, 3,0 cm
  au-delà ; vitesse jusqu'à 1,25 m/s ; témoin `CONTROLE=1` identique au bit. Identité du décalage : 6 272 murs `u`,
  6 720 `v`, 0 différence. Correction datée ajoutée à la preuve §5.
- **P3, critère 2 tenu.** `Step3` : `capacity` (forme de création, réservations) et `shape` (forme courante) ; comptes
  de faces, mailles, colonnes, débits, diagnostics et tuiles devenus des méthodes de la forme courante. Empreintes
  S343 **inchangées** : 60 pas 0x5efa267462dfa0ad / 0xc5c6a85d3d29f44b, 600 pas 0x9325cf58781f8b74 /
  0xea1bebe0ffabc19a ; décalage (+3, −2) toujours 0 différence.
- **Reprise à chaud, 21:06.** Conversation coupée pendant P4 (battement 20:40, aucune autre session en ligne) ;
  l'utilisateur demande la reprise. Diff de P4 lu, cohérent avec la thèse : **complété**, pas annulé.
- **P4, critère 3 tenu** (`--delta3d-redimensionnement`, scène de la porte B après 30 pas). Rétréci 120×112 → 90×84
  depuis (13, 17), puis élargi → 110×96 depuis (−6, −9) : (a) **0 différence** sur les sept tableaux, murs nuls
  (4 704 `u` / 5 040 `v`, puis 5 376 / 6 160), entrant au repos (78 960 `u`, 3 000 colonnes) ; (b) domaine créé à la
  forme et à l'origine, même état : **0 différence** sur 881 832 puis 1 230 728 valeurs après 60 pas, volumes égaux
  (29,74 et 31,65 m³), surface jusqu'à 0,86 m ; (c) allocateur avant = après, **89 784 320 octets, 27 allocations**,
  les deux fois. Rejoués : empreintes S343 inchangées, décalage (+3, −2) 0 différence.

