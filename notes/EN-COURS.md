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

Session : S640 — **en cours**. En autonomie, la campagne du rouleau 3D (accepté par l'utilisateur le 2026-10-07). **Étape 1 bis — les faces
coupées dans APIC 3D** : en S639, le fond en escalier manquait le repos (1,51 cm/s au rivage d'une maille ; à 2,5 cm, 1,18 : la géométrie).

**Ce que la session fait.** `Apic3::set_seabed_lisse(fond)` : le fond **lisse** — linéaire entre les centres des colonnes — et, pour chaque
face, la **fraction ouverte à l'eau** (Batty, Bertails et Bridson 2007, la projection variationnelle) : une face verticale, la part de sa
hauteur au-dessus du fond (exacte en hauteur, seize échantillons le long de la face) ; une face horizontale, la part de son aire au-dessus
du fond (seize par seize échantillons). La projection pondère par ces fractions la divergence et le laplacien ; une face fermée garde une
vitesse nulle. Une maille dont les six faces sont fermées est solide. Les particules sont reposées au-dessus du fond lisse ; la
reconstruction reflète sous ce fond (l'image `2·z_b(x, y) − z`). Au repos hydrostatique, la projection rend une vitesse nulle quelles que
soient les fractions. Ne fait pas : les poches d'air et la zone des colonnes avec les faces coupées (refusées ensemble), un fond sous la
forme d'une distance signée générale.

**Références, calculées avant** (ce script). Le canal de S639 (48 × 4 × 16 mailles de 5 cm ; fond plat à 5 cm puis pente 1:3 depuis 0,805 m ;
eau à 0,4 m), le fond lisse : **5992 particules** posées entre le fond et le niveau (aucune à moins de 0.0017 m du fond : pas
d'égalité, asserté) ; le rivage à 1.855 m.

**Quantum** : la maille ; la vitesse en f32. **Critères, écrits avant** — le repos de S388, S393. (1) 5992 particules, aucune perdue en 2 s,
aucune sous le fond lisse ; (2) **la vitesse parasite ≤ 1 cm/s sur 2 s** — ce que l'escalier manquait ; (3) les essais d'APIC 3D (S388–S639)
inchangés ; (4) refus : longueur fausse, valeur non finie ou hors du domaine ; les faces coupées avec les poches ou les colonnes.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — les faces coupées et leurs essais ; (1)–(4).
- [ ] **P3** — preuve ; listes 4.14, 4.16 ; rituel.

### Notes de reprise
