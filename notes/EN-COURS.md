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

Session : S434 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3a**
([preuve](../docs/validation/APIC-CARTE-S416.md) §22.3), A320.

**Ce que la session trouve en entrant.** `extra3` (`delta3d_coupling.rs`) donne à la face d'axe `a` les termes croisés
`U·∇u′_a + u′·∇U_a` : le premier par **différences centrées** de `u′`, le second avec le **gradient analytique** de B — deux
discrétisations qui, ensemble, ne forment plus le gradient discret qu'elles sont pour deux écoulements irrotationnels (l'hypothèse de
S369). Témoin S433 : un germe de 1 mm croît à 0,1151 s⁻¹ sous la houle de 7,5 cm.

**Ce que la session fait.** `Volume3::set_cross_bernoulli(bool)`, éteint par défaut (au bit) : pour B irrotationnel
(`∂_b U_a = ∂_a U_b`), `U·∇u′_a + u′·∇U_a = ∂_a(U·u′) + Σ_b U_b (∂_b u′_a − ∂_a u′_b)`. **G** : la différence, entre les deux mailles
de la face, de `φ = U·u′` pris aux centres (U des échantillons de B, `u′` des moyennes de faces) — un gradient discret exact, que la
projection absorbe ; **R** : `Σ_b U_b (∂_b u′_a − ∂_a u′_b)`, les différences de `u′` aux centres le long de `a`, nul quand δ est
irrotationnel. Aux faces du sommet (une seule maille), l'ancienne forme. Le banc `transfert_oriente mer` : `MER_BERNOULLI=1`.

**Critères, écrits avant** (C7d-3a, preuve §22.3) : (1) le germe de 1 mm sous la houle de **7,5 cm et de 5 cm**, 25 cm, masque 7 :
**taux < 0,01 s⁻¹ sur 95 s** ; (2) le même à **12,5 cm** (au moins 7,5 cm de houle) ; (3) **δ nul reste nul** sous B seul avec la forme
(un essai, sur le modèle de S369) ; (4) le paquet de l'ordre C sous la houle de 5 cm (`mer_paquet`) : δ maximal **au plus 1,5 fois**
l'amplitude du paquet ; (5) sans la forme, S369 au bit (le témoin) ; suite du cœur, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `set_cross_bernoulli` ; l'essai du point fixe ; la clé du banc.
- [ ] **P3** — mesures : germes 7,5 et 5 cm à 25 cm, 12,5 cm ; le paquet ; critères.
- [ ] **P4** — suite ; preuve (MER-S369 §6 et APIC-CARTE §22.5) ; A320 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — `Volume3::set_cross_bernoulli` (éteint par défaut, au bit) : à chaque face prédite entre deux mailles du domaine (pas au sommet, pas dans un ensemble épars), **G** `= (φ(haut) − φ(bas))/dx`, `φ = U·u′` aux centres (U : moyenne des deux faces `w` de B ; `u′` : moyennes de faces), plus **R** `= Σ_{b≠a} U_b (∂_b u′_a − (u′_b(haut) − u′_b(bas))/dx)`, plus le résidu de B s'il n'est pas retiré. Essai `zero_delta_stays_zero_with_the_bernoulli_cross_terms_s434` : δ nul reste nul au bit sous B seul avec la forme ; sur un germe de 1 mm, les deux formes s'écartent (3,3 µm en 1,5 s). L'essai de S369 inchangé. Banc : `MER_BERNOULLI=1`.
