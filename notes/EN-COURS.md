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

Session : S433 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **la conception de C7d-3**
([preuve](../docs/validation/APIC-CARTE-S416.md) §21 : « sa conception d'abord (une session), qui dira l'ordre avec le mode relatif sur
la carte et A320 »).

**Ce que la session trouve en entrant.** Deux solveurs : le **pas couplé de production** (`Volume3`, `delta3d_coupling.rs` — grille MAC
à fonction hauteur, couplée à B+W par ses faces, éponge, épars, niveaux, faces coupées ; sa production GPU, `delta3d_step.wgsl`, au pas
de S297, **sans** le mode relatif) ; **la bande** (`Apic3` + zone + fond + échange + bascule — S398–S432, ≈ 3 100 lignes en référence,
portée sur la carte), qui simule l'eau totale **sans B**. **A320** ouverte : en mode relatif, le terme croisé `u'·∇U`, discrétisé seul,
fait croître une perturbation sous houle raide ; le remède nommé (ADR-198 D4), la forme de Bernoulli `∇(U·u')`, n'est pas essayé.

**Ce que la session fait.** (1) **La conception** (preuve §22) : où vit la bande (dans le pas couplé, A1 de la campagne — non B dans
`Apic3`), ce que portent les particules (la vitesse propre `u'`, advectée par `U + u'`), où vont les termes croisés, l'ordre et les
critères des morceaux — A320 d'abord, le mode relatif sur la carte, la bande relative en référence puis sur la carte. (2) **Le témoin
d'A320 rejoué** (le germe de 1 mm sous la houle de 7,5 cm, masque 7, 25 cm : S369 mesurait 0,115 s⁻¹ de 35 à 59 s) — l'état de départ
de la session qui le traitera, au chiffre près ou l'écart dit.

**Critères, écrits avant.** La conception dit, pour chaque morceau, son contenu, son lieu (référence sans carte ou poste), son « reçu
si » écrit avant, et la dépendance qu'il protège ; elle nomme ce qu'elle laisse ouvert. Le témoin rejoue S369 (même taux à 5 % près),
sinon l'écart est publié avant tout remède.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le témoin d'A320 rejoué.
- [ ] **P3** — la conception de C7d-3 (preuve §22) ; registres.
- [ ] **P4** — rituel.

### Notes de reprise
