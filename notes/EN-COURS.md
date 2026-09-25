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

Session : S371 — **en cours**. **Rendu 10 : la caméra à demi immergée** (ADR-019 §6, liste 8.6), session de rendu de
l'alternance d'ADR-191 D3, suite déclarée par S369 et S370. Aujourd'hui (S365) le mode immergé bascule **d'un bloc** sur
la hauteur de la bande sous l'œil : quand la ligne d'eau traverse l'objectif, tout le cadre est faux d'un côté.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local
(`~/Downloads/Godot_v4.4.1-stable_win64.exe/Godot_v4.4.1-stable_win64_console.exe`).

**Thèse.** Le milieu se décide **par pixel**, au point où le rayon du pixel traverse le plan proche — l'objectif : sous la
surface de B à son aplomb (déplacement horizontal inversé par point fixe), le pixel est vu de l'eau, sinon de l'air. Une
seule fonction, dans `optique_eau.gdshaderinc`, lue par l'eau, le fond et le ciel ; analytique, donc sans le scintillement
d'une ligne émergée du maillage (ADR-019 §6). Hors de la bande des vagues, le résultat est celui du drapeau global.
L'audio (deux mixages) est hors session : à la fin (ADR-197 D5).

**Critères, écrits avant.** (1) La ligne d'eau rendue contre l'intersection analytique de la surface et du plan proche,
recalculée en double dans `mer.gd`, sur au moins 16 colonnes et trois états de mer : **≤ 1 pixel**. (2) Scintillement :
sur 60 images consécutives à 1/60 s, caméra fixe dans la houle, **aucun pixel ne change de milieu à plus de 2 pixels de
la ligne analytique**. (3) Non-régression : les poses au-dessus (proche, rasante) et sous l'eau (sous_eau, zénith)
**identiques au bit** au rendu d'avant. (4) La ligne sur l'objectif (ménisque) décrite d'après des photographies réelles
cherchées par nous, avant d'être rendue ; jugée par l'utilisateur (R26).

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — les images témoins d'avant (quatre poses, commit de départ) ; références réelles de la ligne d'eau
  (photographies « dessus-dessous » libres, lues sans téléchargement) : ce qu'elles montrent, consigné ici.
- [ ] **P3** — la classification par pixel dans `optique_eau.gdshaderinc` (point du plan proche, hauteur de la bande à
  son aplomb, borne pour sortir tôt) ; la bande passée au fond et au ciel.
- [ ] **P4** — l'eau, le fond et le ciel lisent le milieu du pixel (profondeur d'origine par pixel, trajet dans l'eau
  depuis l'objectif) ; la brume de Godot sur les pixels vus de l'eau.
- [ ] **P5** — `--controle-ligne-eau` : critère 1, puis critère 2 ; non-régression (critère 3).
- [ ] **P6** — la ligne sur l'objectif (ménisque), d'après P2 ; les images de R26.
- [ ] **P7** — preuve `DEMI-IMMERGEE-S371`, ADR-019 note datée, liste 8.6, file, feuille de route, index ; revue R26.
- [ ] **P8** — rituel.

### Notes de reprise
