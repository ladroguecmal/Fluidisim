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

Session : S594 — **terminée**. En autonomie (ADR-247), **11.2 — de nombreuses régions de mer décrites par descripteur, transitions par
paramètres** (absent ; I-09 : « on interpole des paramètres, jamais des réalisations »).

**Ce que la session fait.** `regions.rs` : un **descripteur** de région (Hs, niveau moyen) sur un rectangle ; `parametres_en(x, y)` : les
poids de chaque région — 1 à l'intérieur, une transition en `smoothstep` sur une bande de largeur donnée à son bord —, normalisés (une
partition de l'unité), et les **paramètres mélangés** ; `echelle(échantillon, s)` : l'échantillon de B mis à l'échelle de `Hs_local/Hs_réf`
(les composantes de B sont les mêmes partout, ADR-004 §2.1 ; seule leur amplitude suit le paramètre) et décalé du niveau moyen local. Ne
fait pas : la période et la direction par région (elles changent les composantes : à faire par pondération spectrale), la marée par
région (la carte cotidale, S578, s'y attache ensuite), le placement des régions sur la planète (11.1).

**Références, calculées avant** (ce script les écrit). Deux régions, Hs = 1.0 et 3.0 m, une bande de 1 000 m. Au milieu de la bande :
le **mélange des paramètres** donne **Hs = 2.0 m** ; le **témoin** — mélanger deux réalisations indépendantes, chacune à son Hs, à
½–½ — donne **Hs = 1.5811 m** (−20.9 %) ; à Hs égal, la perte vaut **29.3 %** (A11).

**Quantum** (ADR-236 D1) : la mesure de Hs (`4·σ(η)` sur 2 h au pas de 0,5 s, 9 points) — son erreur estimée par l'écart entre deux
fenêtres d'une heure, mesuré ; le seuil 5 % doit le dépasser d'un facteur 10 (vérifié dans l'essai avant de juger).
**Critères, écrits avant.** (1) les poids : une partition de l'unité partout (à 10⁻⁶), 1 à l'intérieur d'une région, continus à travers la
bande ; (2) au milieu de la bande, Hs mesuré à 5 % de 2.0 m, et le témoin à 5 % de 1.5811 m (la perte
mesurée) ; (3) loin de la bande, l'échantillon de chaque région au bit de B mis à son échelle ; (4) refus : un rectangle vide, une
bande non positive, un point hors de toute région.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `regions.rs` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 11.2 ; rituel.

### Notes de reprise
- **Première mesure : (1), (3), (4) et (2) pour les paramètres tenus (Hs 2,0009 m) ; (2) pour le témoin manqué — 1,3619 m pour 1,5811.**
  Relu d'abord (ADR-239 D1), par une mesure de chaque mer : Hs 0,2001 et 0,2001, mais **ρ = −0,43** entre les graines 42 et 43. La formule
  avec ρ, `√((1 + 9 + 2·ρ·3)/4)`, redonne 1,361 m : elle est juste ; l'hypothèse d'indépendance ne l'était pas. Deux réalisations aux mêmes
  composantes ont une corrélation fixe `Σaₖ²·cos Δφₖ/Σaₖ²`, nulle **en moyenne sur les tirages** seulement (écart-type ~0,3 pour ~10
  composantes efficaces) — ce n'est pas un défaut de B (SplitMix, bien mélangé). Le critère (2) témoin, posé sur un couple, reste
  **manqué** et publié.
- **Vérification ajoutée en route** (ADR-244 D1, avant l'essai) : l'**ensemble** — la moyenne de Hs² du témoin sur 800 couples de graines
  (un point, 2 h au pas de 2 s) ; attendu `(1 + 9)/4` = 2,5 (Hs 1,5811) ; l'écart-type de la moyenne ≈ 6·0,3/(4·√800) = 0,016 sur Hs² ; un
  seuil de 0,25 sur Hs² (≈ 5 % sur Hs) lui laisse un rapport ≈ 16 (ADR-236).
- **P2 fini** — (1), (3), (4) tenus ; (2) paramètres 2,0009 m tenu ; (2) témoin d'un couple 1,3619 m manqué (ρ = −0,43) ; l'ensemble sur
  800 couples 2,5038 pour 2,5. Suite 770.
- **P3** — preuve REGIONS-S594 ; liste 11.2 (absent → partiel) et décompte ; index ; journal.

