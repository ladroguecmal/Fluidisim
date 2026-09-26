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

Session : S379 — **en cours**. *« Continue mais il faudra prévoir une session du plus dur et complexe de la création d'un
solveur […] 3D volumétrique ultra réaliste et performant en temps réel dynamiquement »*. Deux choses : **inscrire la
campagne du solveur** comme prochaine session de physique (conception d'abord) ; **continuer** par la session de rendu
prévue — **les rides de pluie factices** (ADR-202 D3, ADR-203 D7), sur le bassin et sur la mer.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Un effet de rendu, **sans simulation**, mais dont les nombres ont une provenance : les gouttes qui laissent un
anneau net (D ≥ 1,5 mm) arrivent au taux de **Marshall et Palmer** (1948 ; `N0` = 8 000 m⁻³·mm⁻¹, `Λ = 4,1·R^−0,21`) × vitesse
terminale d'**Atlas** (1973) — **448 m⁻²·s⁻¹ à 10 mm/h** ; l'anneau s'étend à la vitesse minimale des ondes
capillaires-gravité, **0,23 m/s** (λ ≈ 1,7 cm), et s'éteint en ≈ 0,6 s. Procédural (cellules, couches, âges), réglé par la
distance : quand l'empreinte d'un pixel dépasse l'anneau, sa pente devient de la **rugosité** (la variance de LEAN, ADR-161)
— la surface mate d'une eau sous la pluie.

**Critères, écrits avant.** (1) Sans pluie, les rendus d'avant **identiques au bit** (mer et bassin). (2) Le taux de
naissance des anneaux, compté sur des images de contrôle (cœurs d'anneaux de moins de 30 ms), **à ±10 %** de Marshall et
Palmer à 10 mm/h. (3) Au loin, aucun motif : les anneaux cèdent à la variance avant que l'empreinte du pixel n'atteigne
leur longueur d'onde (vérifié par le calcul de l'empreinte au seuil). (4) Photographies réelles cherchées et chiffrées ;
jugement de l'utilisateur (R28).

### Plan

- [>] **P1** — jeton, plan seul ; la campagne du solveur inscrite (file).
- [ ] **P2** — références : photographies de pluie sur l'eau (libres, lues sans téléchargement) ; ce qu'elles montrent.
- [ ] **P3** — les rides dans `bassin.gdshader` (le taux, l'anneau, le fondu en variance) ; critère 1 sur le bassin.
- [ ] **P4** — les rides sur la mer (`eau.gdshaderinc`, pente et covariance de la queue) ; critère 1 sur la mer.
- [ ] **P5** — contrôle du taux (critère 2), du fondu (critère 3) ; images de R28.
- [ ] **P6** — preuve `RIDES-PLUIE-S379`, liste (8.9 ou 8.4), file, feuille de route, index ; la campagne du solveur dans
  la feuille de route.
- [ ] **P7** — rituel ; session suivante : **la campagne du solveur volumique 3D** (conception).

### Notes de reprise
