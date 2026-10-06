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

Session : S545 — **en cours**. En autonomie, **C21 en référentiel accéléré**. S544 a écrit que les formes volumiques de V « frôlent le
débordement des entiers en µm³ » à l'échelle d'une mer : **faux, et écrit sans calcul** — `Tetrahedron` borne ses coordonnées à ± 4 096 m
(I-08) et calcule le déterminant en i128 ; le seul obstacle était que les formes n'étaient pas construites. À corriger d'abord.

**Ce que la session fait.** (a) La correction de S544 (sa preuve, son essai, la note de C21). (b) Le scénario de C21 avec des formes
volumiques (tétraèdres) : la mer, une boîte de 100 × 100 × 20 m ; le compartiment, 2,5 × 2 × 2 m (5 m²) ; la brèche d'1 dm² à son fond ;
sous `g_eff` = (1, 0, −9,759) m/s² (incliné de 5,85°) puis vertical ; sans puis avec le domaine δ de S544.

**Ordre de grandeur, calculé.** Capacité de la mer : 2·10⁵ m³ = 2·10¹¹ ml (volume·6 en µm³ : 1,2·10²⁴, dans i128) ; le compartiment en
reçoit ≈ 0,76 m³ en 20 s (S544) — la mer baisse de 0,04 mm. L'angle de `g_eff` : atan(1/9,759) = 5,85°.

**Critères, écrits avant.** (1) Sous `g_eff` incliné, `volume_ml` du compartiment identique à l'entier, sans puis avec δ, aux 200 pas ; le
compartiment reçoit plus de 100 L. (2) Vertical, avec les formes volumiques : de même. (3) La suite au bit.

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — la correction de S544 ; l'essai ; (1)–(3).
- [ ] **P3** — preuve (C21-MASSE-S544 §5) ; C21 ; rituel.

### Notes de reprise
