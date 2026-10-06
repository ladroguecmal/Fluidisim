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

Session : S530 — **terminée**. En autonomie : **le lot des registres** (dû ; feuille de route S528–S529), puis **5.5 — l'absorption par le
sol** : la pluie tombe dans V (S378), mais rien n'entre dans le sol.

**Ce que la session fait.** Une loi d'arête de V, `Flow::Infiltration { area_mm2, conductivity_nm_s, suction_um, deficit_pm }` : **Green–
Ampt**, `f = K·(1 + (ψ + h₀)·Δθ/F)`, de la flaque (`from`, la surface du sol à la cote de l'arête ; `h₀` la lame au-dessus) vers le sol
(`to`, un nœud dont le remplissage rapporté à l'aire est la lame infiltrée cumulée `F` — aucun état nouveau). Sur un pas, l'équation
s'intègre **exactement** : `t(F) = (F − M ln(1 + F/M))/K`, `M = (ψ + h₀)Δθ`, inversée par bissection en f64 (déterministe). Le sol plein, le
limiteur d'arrivée arrête l'infiltration ; la flaque à sec, rien ne passe. La validation, l'empreinte et l'instantané connaissent la loi.

**Ordre de grandeur, calculé.** Limon sableux : K = 1,09 cm/h, ψ = 11 cm, Δθ = 0,3, h₀ = 1 cm : `F` = **3,74 / 12,68 / 35,71 mm** à 60 s /
10 min / 1 h. **Un Euler explicite** (`F` au début du pas, plancher 1 µm, pas de 0,1 s) donne +212 % / +37 % / +7,6 % — d'où l'intégration
exacte. Le quantum de V (1 ml sur 1 m², 1 µm) : 3·10⁻⁵ de `F` à 1 h.

**Critères, écrits avant.** (1) Les lois d'avant au bit (suite). (2) Flaque à charge quasi constante (1 000 m², 1 cm), 1 m² de sol :
`F(t)` à 10⁻³ de la solution implicite à 60 s, 10 min, 1 h ; la masse exacte. (3) Le sol plein arrête l'infiltration (au millilitre) ;
une flaque à sec n'infiltre rien. (4) Une pluie de 5 mm/h sur 1 m² de sol de K = 10,9 mm/h pendant 1 h : tout entre (5 L au millilitre
près), la flaque reste sous 2 ml.

### Plan

- [x] **P1** — jeton ; le lot (feuille de route S528–S529) ; plan.
- [x] **P2** — la loi, les essais ; (1)–(4).
- [x] **P3** — preuve ; liste 5.5 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — `Flow::Infiltration`, `green_ampt_step` ; essais `s530` : 3·10⁻⁵ / 1·10⁻⁵ / 7·10⁻⁵ ; sol plein exact ; flaque à sec ; pluie
  4 999 ml, flaque ≤ 1 ml. Suite 690.
- **P3** — preuve INFILTRATION-S530 ; liste 5.5 ; index ; journal.

