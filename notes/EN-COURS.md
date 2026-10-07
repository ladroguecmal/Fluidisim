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

Session : S655 — **terminée**. En autonomie vers la v2. Le corps libre de S653 est lancé plus vite que l'eau par son couplage explicite
(S654 : 4,68 m/s à 10 ms, 2,56 à 5 ms, la force lissée inchangée).

**Le remède.** La masse ajoutée traitée implicitement : `(m + m_a)·aₙ₊₁ = F + m·g + m_a·aₙ`. À l'équilibre (`aₙ₊₁ = aₙ`), c'est
`m·a = F + m·g` : rien ne change. `m_a = ½·ρ·V_imm` (la sphère), `V_imm` les mailles du corps où `φ < 0` — la reconstruction reflète l'eau
à travers la sphère (S393) — fois `dx³`. L'accélération du pas précédent est gardée. Sans masse, au bit.

**Critères, écrits avant.** (1) Sous le rouleau (le montage de S653), la vitesse au plus du corps libre à 10 et à 5 ms **à 20 % l'une de
l'autre**, et chacune au plus la vitesse de l'eau de la colonne ; le corps emporté de plus de 0,5 m ; la masse au bit. (2) La flottaison de
S653 tient toujours (le centre à 2 cm du niveau ; aucune croissance). (3) Les essais d'APIC 3D passent.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — la masse ajoutée ; les deux pas en parallèle ; (1)–(3).
- [ ] **P3** — preuve ; liste 6.7 ; A334 ; rituel.

### Notes de reprise
- **P2 fini** — (1) à moitié : 1,87 (10 ms) contre 2,32 (5 ms), 19–24 % ; à 5 ms au-dessus de la sonde (1,50) — manqué ; la sonde restait
  à la position de départ du corps (un comparant non éprouvé) ; (2) tenu, la flottaison sous 1 cm/s (le critère de S653 tenu en entier) ;
  (3) tenu. L'utilisateur : « Les erreurs que tu réalises viennent d'où ? », puis « Corrige et apprend de tes erreurs » — la mémoire
  persistante écrite ; l'ADR en S656.
