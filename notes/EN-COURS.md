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

Session : S360 — **en cours**. **Rendu 4 : la surface fine** — la demande de l'utilisateur prime sur l'alternance.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — **verdict R20** : (1) *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la qualité du rendue
final »* ; une capture de l'eau : *« ce rendue du point de vue topologie est pas réaliste »* ; (2) *« La couleur me
paraît parfaite sincèrement »* ; (3) *« Tente les caustique, mais pour l'ecume […] l'ecume n'apparaît presque jamais
sur le vaguelettes uniquement sur des grandes vagues avec déferlement mais très rare voir quasi impossible »* ; (4) pas
de référence, *« tu peux faire tes recherches »*. **Mesuré avant le plan** : la queue qui dessine les petites vagues
compte **60 ondes planes pour 5,5 octaves** (λ 7 cm à 3,4 m) réparties sur 360°, pentes isotropes (rapport 1,04 ;
Cox et Munk : 1,37 à 7,8 m/s) — des taches sans direction ni crête. Ordre : la topologie d'abord (les caustiques
projettent la forme de la surface) ; caustiques et ciel à la session suivante.

Critères, écrits avant le code :
1. **R20 consigné** tel quel ; **recherches sourcées** : Beaufort 4, taille des moutons, anisotropie de Cox et Munk,
   étalement d'Elfouhaily et al. (1997), océan par FFT de Tessendorf (2001).
2. **L'écume au déferlement** : tirée de la seule bande (les vagues dominantes), à une empreinte fixe ; couverture de
   Monahan tenue à ± 25 % (mesurée au nadir, mode contrôle), **plus aucune tache sous 0,5 m** de diamètre équivalent.
3. **Le spectre fin, dans le cœur** : densité continue de la queue d'équilibre de B (même niveau) avec l'étalement
   d'Elfouhaily ; sa variance sur la plage de la queue égale celle de la queue discrète à 1 % ; son rapport de pentes
   au vent / au travers à ± 10 % de Cox et Munk ; amplitudes de départ `h0` sur deux grilles 256² (32 m et 4 m), graine
   fixe, exportées par l'afficheur.
4. **La FFT dans Godot** (calcul de `RenderingDevice`, sans téléchargement) : pentes, gradient du déplacement,
   hauteur ; temps replié en double sur une période de répétition, dispersion quantifiée (I-08) ; **contrôle** — le
   champ de la FFT contre la somme directe des mêmes composantes en quatre points, écart ≤ 10⁻⁴ relatif ; pente
   quadratique moyenne à 1 % de `Σ|h0|²k²`.
5. **Le nuanceur** : la queue de 60 composantes remplacée par les deux cascades, pondérées par l'empreinte ; ce qu'elles
   ne résolvent pas va à la covariance filtrée (ADR-161). Contrôles de S359 inchangés ; rapport mer/ciel sous l'horizon
   à ± 15 % de l'afficheur.
6. **Images R21** : les poses de R20 et la zone de la capture de l'utilisateur.
7. Preuve, ADR, liste 8.9 et 8.4, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — R20 consigné ; recherches ; critère 1.
- [ ] **P3** — l'écume au déferlement, mesurée avant et après ; critère 2.
- [ ] **P4** — le spectre fin dans le cœur, ses essais ; l'export des `h0` ; critère 3.
- [ ] **P5** — la FFT dans Godot et son contrôle ; critère 4.
- [ ] **P6** — le nuanceur sur les cascades ; critère 5.
- [ ] **P7** — les images de R21 ; critère 6.
- [ ] **P8** — preuve, ADR, file, liste, index ; critère 7.
- [ ] **P9** — rituel.

### Notes de reprise
