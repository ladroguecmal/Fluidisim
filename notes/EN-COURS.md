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

Session : S373 — **terminée**. Demande : *« Continue et ensuite commence à permettre de visualiser le système de piscine
avec déversoir et pompe »* — d'abord la suite du jeton (le rendu), puis **S374** : la piscine de V dans Godot.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Choix.** Des deux rendus proposés, **notre perspective aérienne** : bornée, mesurable, et elle lève la limite de S371
(la brume éteinte sur toute l'image dès que la caméra est à demi immergée, 17 niveaux au pire côté air). L'échelle
radiométrique du soleil reste dans la file : sa cible photographique est floue (le soleil hors cadre dans la pose de
Hanifaru, la courbe AgX qui comprime la fenêtre) — à reprendre avec une mesure qui la fonde.

**Thèse.** La brume de Godot (exponentielle, densité 0,00012, couleur prise au cube de radiance du ciel dans la direction
de visée ; source 4.4-stable, `fog_process`) est réécrite dans nos nuanceurs par la sortie `FOG` : même quantité
`1 − exp(−ρ·d)`, couleur `ciel_b` de la même direction — le ciel dont le cube est tiré —, **multipliée par la part d'air
du pixel**. Sous l'eau et dans l'eau d'une image à demi immergée, aucune brume ; dans l'air, la même partout.

**Critères, écrits avant.** (1) Poses au-dessus (proche, rasante, référence, haute, plongeante ; proche de la scène
côtière) contre les rendus d'avant : **99,9ᵉ centile ≤ 2 niveaux, pire ≤ 8**. (2) Pose `demi` : côté air, l'écart à la brume
de Godot tombe de 17 niveaux (S371 §5) à **≤ 8** ; côté eau, **identique au bit** au rendu de S371. (3) Sous l'eau :
identique au bit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — images témoins d'avant ; `brume_air` dans `optique_eau.gdshaderinc`, `FOG` dans l'eau et le fond.
- [x] **P3** — mesures des critères 1 à 3.
- [x] **P4** — preuve : section §10 de `DEMI-IMMERGEE-S371` (un fil, une preuve), « Reproduire » corrigé ; file, liste 8.6.
- [x] **P5** — rituel ; puis S374.

### Notes de reprise

**P2 — fait.** `brume_air` (`ciel.gdshaderinc`) : `1 − exp(−ρ·d)`, couleur `ciel_b(direction, 4)`, × part d'air ; `FOG`
dans l'eau et le fond (nul en contrôle) ; `brume_densite` posée par `mer.gd` (`brume()`, 0 dans les contrôles et avec
`BRUME=0`). **Premier essai — la brume réécrite partout — critère 1 manqué** : proche p99,9 6 / max 10, rasante 5 / 8,
référence 6 / 11, **haute 9 / 10 (72 % des pixels)**, plongeante 2 / 2, côtière 6 / 10. Cause relue dans `fog_process` :
`mip_level = mix(1/MAX, 1, 1 − (|z| − near)/(far − near))` — loin du plan lointain (20 km), Godot lit son cube de radiance
au **niveau le plus flou** (rugosité 1), une moyenne diffuse du ciel ; la nôtre prend l'horizon dans la direction. Le
seuil ne se relève pas : **deux variantes** (`eau.gdshader` / `eau_demi.gdshader`, `sol` de même, corps commun dans
`*.gdshaderinc`, `#define BRUME_PAR_PIXEL`), échangées par `mer.gd` quand le mode demi change ; la brume du moteur
partout où elle peut servir.

**P3 — fait.** Avec les variantes : **critère 1** — proche, rasante, référence, haute, plongeante, côtière **identiques au
bit** (la brume du moteur, inchangée). **Critère 2** — pose `demi`, côté air : **7 niveaux** au plus de la brume de Godot
(p99,9 = 3 ; S371 : 15, sur ce côté à 3 px de la ligne) ; côté eau : **identique au bit** à S371. **Critère 3** — sous
l'eau (`sous_eau`, zénith) : identiques au bit. `--controle-ligne-eau` inchangé (0,078 px, 0 mal classé).
