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

Session : S369 — **en cours**. **Les réponses du 2026-09-26, puis A289** — l'utilisateur a répondu aux quatre questions de
fin de S368 : *« Pas de réseau. Godot sera le moteur entier fais comme bon te semble. Tout flotte/coule des interactions
physiques logique. Pas de terrain realiste avec hydrologie etc.... Pas de météo et son a faire à la fin »* ; A289 : *« Le
choix le plus favorable au realisme ainsi que les performances, simple »* ; seconde cible et serveur : *« Pas encore »* ;
R18 : *« Rendu convaincant »* ; photos d'écume : *« Plus tard »*. Session de physique (alternance d'ADR-191 D3).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Ce qui se décide.** (1) Les réponses deviennent des décisions écrites (ADR-197), et les attentes extérieures de la
liste qu'elles lèvent sont levées ; ce qui reste ambigu (« pas de réseau » : aucun format existant, ou pas de
multijoueur ?) reste en attente, sans réduire l'ambition (ADR-127). (2) **A289** : la voie, par les trois critères de
l'utilisateur. Lecture du pas couplé (`delta3d_coupling.rs`) : δ reçoit comme **sources** trois termes qui ne dépendent
que de B — le résidu de quantité de mouvement de B (`momentum_residual`, SPEC-004 §6.1), le transport de B entre le
plan moyen et sa propre surface (la bande), l'erreur de pression de B à sa propre surface (les fantômes). B linéaire ne
satisfait pas les équations complètes : ces restes nourrissent δ même quand rien ne le perturbe. **Voie examinée,
quatrième** : les retirer — δ relatif à la dynamique de B, B tenu pour exact dans le domaine comme partout ailleurs ;
seuls restent les termes où δ figure (croisés B·δ et propres à δ). Gratuite, locale, sans amortir aucune perturbation
ni recréer de domaine. Elle se prend si la mesure la reçoit.

Critères, écrits avant le code :
1. **E1** (houle 5 cm, λ = 4 m, maille 25 cm, 40 s) : avec les trois retraits, δ reste **nul au bit** (δ nul est un
   point fixe discret) ; un retrait à la fois nomme la source de la croissance de S319.
2. **Stabilité** : un germe de 1 mm posé dans la houle, 40 s — pas de croissance exponentielle (taux < 0,01 s⁻¹, contre
   0,10 en S322) ; sinon, une instabilité des termes croisés, nommée, et la voie ne suffit pas.
3. **E2** : le paquet de l'ordre C (2 cm) dans la houle, **sans témoin** — reçu à droite à 30 % près du paquet seul
   (S317), le signe juste à gauche (critère de S319 §3, manqué alors d'un facteur 18).
4. Aucun test existant ne bouge : le mode est une option, éteinte par défaut, jusqu'à la décision.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les réponses : ADR-197 ; liste (6.7, 8.1, 5.11, 2.8, 7.8 ; 8.7 et R18), données des dépendances,
  file (ligne des décisions), feuille de route, REVUE-VISUELLE §23 (verdict R18).
- [ ] **P3** — `delta3d_coupling.rs` : le mode relatif à B (trois retraits, un drapeau par terme pour la mesure) ;
  E1 ; critère 1.
- [ ] **P4** — germe et E2 ; critères 2 et 3.
- [ ] **P5** — ADR-198, la voie d'A289 ; preuve MER-S369 ; A289, lot 2 (4.8, 4.21), file, dépendances, feuille de route,
  index.
- [ ] **P6** — rituel.

### Notes de reprise
