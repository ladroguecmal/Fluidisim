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

Session : S140 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A206 — mesurer le facteur de conservatisme de `slope_envelope`
(`spectral_pressure.rs:346`, `Σ(|kx|+|ky|)·(|Re η|+|Im η|)`) contre la pente réelle du champ de
pression. C'est le préalable à S139-1 : tant que ce facteur est inconnu, le budget de pente
reste hétérogène et aucun seuil ne s'y dérive (ADR-094, L220).

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — monter le champ de pression minimal et échantillonner sa pente réelle. Cas à
      **une seule case**, où le facteur doit valoir *exactement* le produit des deux
      majorations connues — c'est le témoin qui dit que la sonde mesure ce qu'elle croit.
- [x] **P3** — balayer ce dont le facteur dépend : direction de `k`, phase de `η`, nombre de
      cases, taille de l'emprise. Est-il borné, ou l'emprise peut-elle le faire diverger ?
- [ ] **P4** — la borne resserrée `Σ|k_w|·|η|` (norme euclidienne au lieu des deux sommes de
      valeurs absolues) : même coût, majorant rigoureux. Mesurer ce qu'elle récupère, et
      **vérifier qu'elle majore toujours** — c'est une propriété de sûreté, pas de finesse.
- [ ] **P5** — décision et livrable : ce que A206 impose à S139-1.
- [ ] **P6** — rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ 57504f0 = master ; worktree `886155`. 270 tests/cinq ignorés.

Ce qui est établi et n'est pas à remesurer :
- `slope[i] = -Σ weighted_k[i]·(η_re·sin φ_i + η_im·cos φ_i)`, `φ` dépendant du point
  (`spectral_pressure.rs`, `Slot::accumulate`) ; l'enveloppe somme
  `(|kx_w|+|ky_w|)·(|η_re|+|η_im|)`.
- les deux majorations sont indépendantes et chacune vaut 1 à √2 : `|kx|+|ky| ≥ |k|` selon la
  direction, `|Re|+|Im| ≥ |η|` selon la phase. Produit dans [1 ; 2] **pour une seule case**.
- contrairement à `ρ = 1,7950713` (S139), qui est fixé par une forme spectrale figée, ce
  facteur dépend de ce que l'appelant publie : c'est la thèse à confirmer ou à réfuter.

Thèse de travail, à vérifier et non à supposer : le conservatisme se **décompose** en un facteur
de forme (L1 contre euclidien, borné par 2, éliminable sans coût) et un facteur de phase (les
phases ne s'alignent pas toutes sur une emprise bornée, non éliminable). Si c'est vrai, S139-1 a
une voie : resserrer la borne au lieu de chercher un facteur par couche.

Piège à éviter : conclure d'un balayage 2D trop grossier que le maximum est plus bas qu'il ne
l'est — même piège qu'en S139, et il rend la borne trop permissive dans le sens dangereux.
