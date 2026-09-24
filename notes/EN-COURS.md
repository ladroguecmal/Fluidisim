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

Session : S351 — **en cours**. **Porte A, la dégradation de rang 1** — le dernier critère de la v1 ; et la décision
de l'utilisateur sur l'après-v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S350 : un domaine δ 3D se déplace et se redimensionne, au bit ; son coût suit sa surface (0,09 + 3,57 ms ×
surface). Reste de la porte A : « la dégradation d'ADR-012 §4 rang 1 existe, donc la famine a une issue ». Demande :
*« Continue, après la V1 ton objectif seras de completer entièrement la to do liste »*.

**La décision.** Après la v1, l'objectif des sessions devient la [liste du projet fini](../docs/LISTE-PROJET-FINI.md)
entière — ses 120 points à leur périmètre final. Elle répond aussi à ADR-189 D2 : le lot 5 reprend après la v1
entière (lecture, réversible sur un mot). → ADR-190.

**Le rang 1, tel qu'ADR-012 l'écrit.** §4 : rétrécir l'emprise des domaines **non focaux**, le focal protégé ; §5 :
descente en une image, remontée rampée en ≈ 1 s, toute décision engagée ≥ 30 images (1 s à 30 Hz). Construction :
dans `scheduler.rs`, une soumission déclare qu'elle peut rétrécir (`Shrink` : échelle minimale, part fixe du coût) ;
quand l'allocation pleine laisse un vivant sans budget, le focal — la plus forte priorité, engagé 1 s — est servi
entier, les autres à une **échelle commune**, la plus grande qui en serve le plus ; `Grant` porte l'échelle, que
l'hôte applique par `Step3::resize`. Le trajet de S344 ne rend jamais deux domaines voulus à la fois (0,039 chacun
au milieu) : à 36 m d'écart, l'œil à x ≈ 20 m voit A à ≈ 0,09 et B à ≈ 0,11 — la pose du banc.

Critères, écrits avant le code :
1. **Cœur** : (a) quand tout tient à pleine échelle, la décision de S278 — essais existants inchangés ; (b) sinon, le
   focal servi entier, les non-focaux à une échelle commune ≥ leur minimum, somme des budgets ≤ profil, et aucun
   vivant qui tient à son échelle minimale n'est laissé sans budget ; (c) anti-pompage : descente immédiate, remontée
   d'au plus 1 par seconde et pas avant 1 s après la dernière descente, focal engagé ≥ 1 s ; (d) indépendant de
   l'ordre de soumission ; (e) aucune allocation ; un substitutif ne déclare pas de rétrécissement.
2. **Banc** — la côte, domaines à 36 m, une pause de 8 s où les deux sont voulus, budget 5 ms : **aucune image
   « vivant mais affamé »** (S344 : 6 par bascule) ; le focal toujours entier ; budget accordé ≤ 5 ms, temps mesuré
   publié ; pompage compté. Témoin : le même trajet sans rang 1.
3. **Le prix, publié sans seuil** : l'amplitude de δ que coupe chaque rétrécissement.
4. Preuve, file, feuille de route, liste ; si les trois critères de la porte A tiennent, **la porte A reçue et la v1
   déclarée**, avec ses réserves (C sur le banc, D sur la référence CPU).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — ADR-190, la décision de l'utilisateur ; file (décisions), REPRISE §5, feuille de route, liste.
- [ ] **P3** — le rang 1 dans le cœur : `Shrink`, focal, échelle commune ; essais (critère 1 a, b, d, e).
- [ ] **P4** — l'anti-pompage : engagement et rampe ; essais (critère 1 c).
- [ ] **P5** — le banc de la côte avec rang 1, et son témoin ; critères 2 et 3.
- [ ] **P6** — preuve, file, feuille de route, liste ; la porte A et la v1 si reçues ; critère 4.
- [ ] **P7** — rituel.

### Notes de reprise
- Écart réglable `ECART=` ajouté à `domaines()` (défaut 60 m, S344 inchangé). Parts d'écran à 36 m : x = 18,8 m →
  A 0,0993, B 0,1069 ; x = 22,5 → 0,0796 / 0,1248 ; à 30 m, x = 15 → 0,1178 chacun.
