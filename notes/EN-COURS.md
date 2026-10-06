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

Session : S509 — **terminée**. En autonomie (maillons 2), **6.4, l'envoi sous 1 ms** : S508 a ramené le recoupage de 8 à 1,4–1,55 ms par
pas ; l'envoi à la carte (0,4 à 0,65 ms : 450 Ko par pas, géométrie et tableaux de mouvement entiers) en est le plus gros reste.

**Ce que la session fait.** `Linear3` garde une ombre de ce qu'elle a reçu ; `set_motion` n'envoie que les valeurs qui changent, en paires
(indice, valeur) ; un noyau de dispersion les écrit dans la géométrie et le tampon de mouvement au début du pas.

**Ordre de grandeur, écrit avant.** Ce qui change d'un pas à l'autre : les faces et mailles coupées par la surface de la coque — de l'ordre
de la surface de la coque en mailles (≈ 2·(16·6 + 16·4 + 6·4) ≈ 370 mailles de 25 cm, autant de faces par famille), quelques milliers de
valeurs, ≈ 10 à 30 Ko au lieu de 450 ; la comparaison à l'ombre, ≈ 110 000 flottants, ≈ 0,05 ms.

**Critères, écrits avant.** (1) la carte rend les mêmes bits qu'avec l'envoi entier : les bancs de S503 et S504 aux mêmes chiffres, S358
identique ; (2) le coût CPU par pas ≤ 1 ms sur la coque de la porte D (le critère de S508) ; si (1) et (2), **6.4 validée**.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'ombre, les paires, le noyau de dispersion ; (1)–(2).
- [x] **P3** — preuve ; liste 6.4 ; rituel.

### Notes de reprise
- **P2** — ombre + paires + dispersion (module à part) : mêmes bits partout, 882 valeurs par pas, mais envoi 0,33 ms ; profil interne :
  comparaison 0,185 (dont la validation de 110 000 valeurs) + écriture 0,134 (coût fixe de l'appel). Puis : validation sur les valeurs
  changées (ombre mise à jour après la boucle), test bon marché d'abord dans `check_solid_in`, `changed_faces_in_place`, nœuds dans la
  boîte orientée : **0,81–0,82 ms**. Deux scripts correctifs échoués (guillemets imbriqués, ancre absente) — rattrapés à la main, rien de
  faux committé. 665 essais.
- **P3** — preuve ENVOI-S509 ; **6.4 validée** (8 / 120) ; index ; journal.

