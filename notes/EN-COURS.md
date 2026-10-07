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

Session : S675 — **en cours**. En autonomie vers la v2 ; une séance visuelle, comme l'utilisateur les a demandées (R41 reçu en S663).
Depuis S664, la côte 2D de B déferle, porte le niveau moyen et le courant de dérive : rien n'en a encore été montré.

**Ce que la session fait.** L'essai ignoré `record_the_breaking_coast_for_the_visual_session_s675` écrit `calculs/s675_cote.bin`
et `calculs/s675_controle.csv`. La mer de S667 est cuite sur la plage de S364, avec et sans déferlement.

L'enregistrement contient :

- **les profils** le long de `s` : la profondeur, `Hrms` avec et sans déferlement, `η̄`, `V` ;
- **la surface vue de dessus** sur les 750 derniers mètres, 48 images à 0,5 s, les deux côtes ;
- **une coupe** à `n` = 0 sur les 950 derniers mètres.

`outils/rendu_cote.py` (numpy, PIL) en tire trois images :

- `s675_dessus.gif`, la surface vue de dessus, avec et sans déferlement ;
- `s675_coupe.gif`, la coupe sur le fond, avec le niveau moyen ;
- `s675_profils.png`, les profils.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : la côte sans déferlement, à côté. La même mer arrive au rivage avec `Hrms` 2,4 m sur 1 m de fond.
- **instrument** : le rendu recalcule depuis le binaire `Hrms` au rivage, `η̄` au rivage et le pic de `V`, et les compare à
  `s675_controle.csv`, que l'essai écrit. Ce qui départagerait : une lecture juste les retrouve au plancher `f32` (10⁻⁶ en relatif) ; un
  décalage d'octets ou un axe permuté les manque de plusieurs ordres.
- **calcul** (ce script) : l'enregistrement fait ≈ 14.1 Mo (asserté sous 20 Mo).
- **ADR** : ADR-216 (le banc visuel), ADR-266.
- **pièges** :
  - la grille de la côte (`n` < 96 m, `s` < 3 950 m : `eval` rend `None` au-delà) ;
  - `SimTime` en microsecondes ;
  - la police (Segoe UI, pour les accents, S663) ;
  - la côte sans déferlement est hors de son domaine près du rivage : le montrer comme témoin, sans le juger.

**Critères, écrits avant.** (1) Les trois images produites et regardées avant l'envoi. (2) Le contrôle relu à 10⁻⁶ en relatif. (3) R42
posé à l'utilisateur, les fichiers envoyés.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'enregistrement ; le rendu ; (1)–(2).
- [ ] **P3** — preuve ; R42 ; rituel.

### Notes de reprise
