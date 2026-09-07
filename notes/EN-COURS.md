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

```
Session          : S62
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S58-2 — recenser les instances d'A104 : pour chaque constante partagée entre
                   une mesure et sa référence, vérifier qu'un cas la discrimine.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — inventorier les 41 références des deux jeux de cas et les classer par **degré de dépendance**, pas par présence d'un paramètre commun.
- [x] **P3** — balayer les paramètres balayables et **mesurer** le degré au lieu de le déduire ; le scénario est en TOML, aucun code n'est à recompiler.
- [ ] **P4** — rapport AUDIT-REFERENCES-S62 : les cas aveugles, ceux qui ne le sont pas, et ce que chacun teste réellement.
- [ ] **P5** — appliquer ce qui doit l'être, avec essais de refus et témoins.
- [ ] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, jeton, **fusion dans master**.

### Notes de reprise

Départ 92042b4. **A104 est ouvert depuis S21** — *une règle énoncée dans un fichier n'empêche pas
sa violation dans le même fichier* — et S58 en a trouvé une instance coûteuse : trois références
de C10 construites avec la constante qu'elles devaient contrôler, un arbitrage bloqué
trente-sept sessions par un argument faux (**A180**). Personne n'a compté les autres.

**Le critère de classement n'est pas « partage un paramètre ».** Inventaire fait : trois degrés
apparaissent, et seul le premier est une faute.

1. **Tautologie** — la référence recalcule la mesure. `C10-raideur` compare la dérivée de
   `ρ·g·A·d` à `ρ·g·A`. Écart identiquement nul pour toute valeur du paramètre.
2. **Aller-retour** — la référence est un paramètre d'entrée que la mesure reconstruit par une
   chaîne réelle. `Hs` génère une mer d'après `hs`, la mesure par la variance, et compare à `hs`.
   La chaîne est testée ; ce qui ne l'est pas est une **convention partagée** entre les deux bouts.
3. **Indépendance** — la référence vient d'une solution analytique ou d'une autre mesure.
   `C04` contre Ritter ; `C02-c` confronte `λ/T` mesurés à `√(gλ/2π)`.

**Ces degrés se mesurent, ils ne se déduisent pas** (L75, et S58 l'a payé) : balayer le paramètre
et regarder l'écart. Identiquement nul → tautologie. Stable et non nul → aller-retour. Variable
→ indépendance. `hs`, `tp` et `composantes` sont dans le scénario TOML : le balayage ne demande
aucune recompilation, sur une copie hors du dépôt.

**Ne pas refaire S29.** `AUDIT-ASSERTIONS-S29` a déjà classé ces mêmes assertions en recevable,
symptôme et vacuité — c'est une autre question, celle de ce qu'une assertion peut voir échouer.
Ici la question est ce que sa **référence** peut voir bouger. Lire S29 avant de conclure, et dire
où les deux classements se recouvrent.

P2 : 41 references inventoriees. Trois degres, et la plupart des cas sont sains : les references
a 0 ou 1 ne peuvent rien tirer des parametres ; C04 se compare a Ritter, C02-c confronte lambda/T
mesures a racine(g lambda / 2pi) — independance reelle. Restent le groupe tautologique deja
etabli en S58 (C10, A180) et un groupe aller-retour : Hs, orbitale, pente.

P3, et le balayage a trouve trois choses au lieu d une.
 (a) **Hs est exactement proportionnel a hs** : rapport mesure/reference = 0,914723 pour
     hs = 0,6 / 1,2 / 2,4 / 4,8. Ecart 8,528 pour cent, invariant sur un facteur 8. Le cas est
     aveugle au parametre qu il nomme ; il ne mesure qu un facteur de chaine, faux de 8,5 pour cent.
 (b) **Il est gouverne par tp, qu il ne nomme pas** : 0,018 pour cent a tp=4, 8,528 a tp=6,
     15,576 a tp=9 — echec. Et par le nombre de composantes, sans convergence : 1,42 / 10,24 /
     8,53 / 12,83 / 6,53 / 26,30 pour cent de 8 a 256. **Raffiner la configuration le fait echouer.**
     Cause : A102, la fenetre doit couvrir plusieurs fois la plus longue onde ; jamais mesure.
 (c) **La fenetre est en dur dans main.rs:306** — hs_restitue(bg, t, sc.hs, 128, 3.0), soit 384 m.
     grille_cote et grille_pas_m du scenario ne l atteignent jamais : mon balayage de la fenetre
     n a rien deplace sur un facteur 16, et c est ainsi que le litteral s est fait voir.
     Or l en-tete du scenario affirme *ce fichier est auto-suffisant*, et SPEC-003 par.3 l exige.
