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

---

## Session en cours

Session : S273 — terminée
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo et Python.
Entrée : « reprends le projet », master propre 2506015, copie unique, jeton libre.
Objectif : intégrer au transport réel la reconstruction linéaire de la bande
(défaut de quadrature isolé en S272), la recevoir, puis rejouer le résidu S272.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — ADR-166 (quadrature linéaire de la bande, remplace la formule
  d'ADR-152/165), notes datées, critères de réception et de campagne écrits
  avant le code dans BANDE-LINEAIRE-S273.
- [x] **P3** — construction : même règle aux faces intérieures et extérieures ;
  tests de quadrature contre l'intégrale analytique, bandes signées/coupées,
  fond uniforme et fond nul inchangés.
- [x] **P4** — suite complète, valeurs déplacées expliquées, transaction intacte.
- [x] **P5** — rejouer les cinq passages S272, verdict aux critères S272 inchangés,
  qualification dt/amplitude déclarée en P2.
- [x] **P6** — rituel §6, feuille de route/file/liste, journal, index et jeton.

### Notes de reprise

Maillons 2 à l'ouverture : ce lot doit livrer une correction produit reçue,
pas un instrument de plus. Passages S272 : environ 1 min chacun en release.
Anciennes traces S272 conservées dans TEMP (fluidisim-s272-*.log) ; nouvelles
traces fluidisim-s273-*.log. Repos aligné sur une face dans tous les appelants
(tests, exemples) ; le pli ADR-154 à z=0 ne gêne donc pas la règle par couche.

P3 : `band_layer` commun (bord : plancher = fond de colonne ; intérieur : 0,
ouverture conservée). Flux produit 0,4190/0,0892/0,0162 % (= diagnostic S272),
témoin rectangle 8,41/3,86/1,60 %. Affine exact à 3,9e-8. S271 cinématique
initiale 3,048/1,237/0,585 % → 1,509/0,445/0,137 %. Module couplé 29 ok, 4 ignorés.

P4 : suite 480 ok/21 ignorés. S253 release 2,14 %. Campagne S272 rejouée :
9,11/6,18/5,88 % ; a/2 3,23 % ; demi-dt 5,78 %, sens. 0,668 % ; a² 2,89 %.
Richardson a (ajout après la fine : a/2 aux mailles grossière/moyenne, 2 passages)
7,74/2,92/1,77 % ; a² 2,74/2,85/2,89 % indépendant de dx → ordre trois physique.
Rejeu S253 128 colonnes 5/10 cm en cours (fluidisim-s273-s253-*.log, ~26 min).

P5 : refus maintenu aux critères S272 ; cause resserrée : troncature de
l'oracle ∝ a, part d'ordre deux convergente. Suite concrète : protocole N*
déclaré avant mesure, dt 1 ms / 0,5 ms (l'exemple n'accepte pas encore 500 µs).

P6 : rejeu S253 128 colonnes reçu (profil 0,148 / 0,178 %, b₂ 0,35 / 0,53 %).
Jeton rendu, maillons 0, suite S274 = réception sur N* (protocole avant mesure).
