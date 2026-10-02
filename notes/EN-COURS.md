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

Session : S436 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **A324**
([preuve](../docs/validation/MER-S369.md) §7), avant C7d-3b.

**Ce que la session trouve en entrant.** Banc `transfert_oriente mer`, 12,5 cm, masque 7, germe de 1 mm : sous 6 cm de houle, δ à
1,1 mm après 1 s ; sous 6,5 cm, 8,6 mm, 73 % de l'énergie sous `4·dx`. La demi-maille est le seuil : δ porte sa surface **totale**
(`surface_total = eta + η_B`, `prepare_background3`) ; la mouillure de ses mailles suit donc la houle, et une maille s'ouvre ou se
ferme quand la surface de B franchit son centre — sous B seul, les termes relatifs l'annulent au bit ; sous δ non nul, quelque chose
l'amplifie.

**Ce que la session fait.** (1) Localiser : à chaque pas, où δ saute, et si c'est sur une colonne dont la mouillure vient de changer ;
la bisection par les bits d'essai (8 à 64) et par les étapes du pas (prédiction, projection, extrapolation, transport). (2) Corriger
la cause trouvée, sous un réglage éteint par défaut tant qu'il n'est pas reçu, puis par défaut s'il l'est.

**Critères, écrits avant.** **Reçu si** : (1) à 12,5 cm sous 6,5 et 7,5 cm de houle, germe de 1 mm : δ max ≤ 1,5 mm à 1 s et part
sous `4·dx` ≤ 1 % à 2 s ; (2) le témoin (δ nul) reste nul au bit ; (3) à 25 cm sous 7,5 cm (aucun franchissement), le pas **au bit**
— le taux d'A320 inchangé (0,1151) ; (4) l'essai `zero_delta_stays_zero…` et la suite du cœur, zéro avertissement ; (5) un essai
du cœur qui garde le cas. Si la cause n'est pas trouvée, elle est écrite telle quelle (ce qui est écarté, ce qui reste).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — localisation (instrument, bisection).
- [ ] **P3** — correction, mesures, critères.
- [ ] **P4** — essai ; suite ; preuve ; A324 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — banc `a324_franchissement` (20 m à 12,5 cm, pas à pas, témoin à côté). **Correction de S435** : le banc `mer` ne fait pas
  avancer son témoin en mode germe (seulement avec le paquet) — son zéro ne prouvait rien. **Ici, le témoin ne reste pas nul** sous
  6,5 cm (1 mm en 50 ms, 1 cm en 0,3 s, des jets de 0,2 m/s) ; sous 6 cm, nul au bit. A324 est donc **une rupture du point fixe** du
  mode relatif. Bits d'essai 8, 16, 32 : rien ne change ; 64 (la pression croisée) : refus au premier pas. **La cause** : le fantôme
  latéral (`prepare_background3`, entre une colonne mouillée et une sèche au même étage — il n'existe que si la surface franchit un
  centre entre elles) retranche l'erreur de B **interpolée entre les colonnes**, alors que ce qu'il corrige est la pression de B **au
  point de surface** : sous B seul, un reste d'ordre `dx²` divisé par `θ`. Essai 1 (l'erreur prise au point de B) : oubliait le terme
  `ρg(z − repos)` de `ghost_side3` — pire. Essai 2 (avec lui) : le témoin nul au bit, mais le germe à 4,6 mm sous 6,5 cm — la bascule
  entre deux formules quand δ déplace le franchissement. Essai 3 (le point de B prolongé, continu) : 6,9 mm — le point de franchissement
  se divise par la pente de B, nulle aux crêtes : c'est là que 6,5 cm franchit. **Essai 4, retenu** : en mode relatif, le fantôme latéral
  interpole entre les deux colonnes ce que porte leur fantôme vertical (`ρgη′ + ghost_bg_up`, nul au bit à δ nul) — il ne lit plus la
  pression de B au point latéral. `Volume3::set_lateral_own_ghost`, éteint par défaut ; `MER_A324`, `A324_PROPRE`.

