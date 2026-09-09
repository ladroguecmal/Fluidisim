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

Session : S130 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S129-1 — admission dynamique des sources de pression. Le contrôleur d'ADR-078
emprunte un journal **figé** ; il doit pouvoir admettre une source et republier un champ qui
lui corresponde, ou ne rien changer du tout.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — inventaire avant de décider (L209) : ce que l'emprunt impose, ce qu'un refus
      de recalcul laisse derrière lui, et ce que la saturation entraîne — `from_journal`
      refuse tout journal en attente, donc un `Full` bloque aussi les changements d'instant.
- [x] **P3** — ADR-086 : emprunt mutable, trois issues, retour en arriere interne, saturation dite terminale.
- [ ] **P4** — construire : emprunt mutable du journal, `admit` transactionnel, retour à
      l'état antérieur si le champ n'est pas calculable.
- [ ] **P5** — les tests : admission qui republie, `Unchanged` légitime et `Unchanged`
      interdit, doublon, conflit, époque, saturation et son blocage, refus numérique.
- [ ] **P6** — suite complète, release, **hachages de campagne inchangés**.
- [ ] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ e817d0e = master, après avance rapide de ma copie qui était restée à 52e80a5 (S124) et
n'avait rien d'unique. S125 à S129 ont été faites par Codex sur master ; lues à l'amorce.

Ce que la consigne S129-1 demande explicitement : publication cohérente journal/champ,
attente explicite, ancien état conservé au refus, et **jamais `Unchanged` sur un journal
différent**. Doublons, conflits et saturation à recevoir avant toute transaction mixte.

Ce que la lecture a déjà établi :
- `Controller` détient `&'v Journal` : admettre exige `&'v mut Journal`. Sept appelants, tous
  en tests ou exemples — le changement est peu invasif.
- L'emprunt mutable **garantit structurellement** l'invariant « jamais `Unchanged` sur un
  journal différent » : personne d'autre ne peut muter le journal pendant la vie du contrôleur.
  Préférable à un compteur de version, qui ne ferait que le détecter après coup.
- `admit_authenticated` rend `Added`, `Unchanged`, ou `Epoch`/`Conflict`/`Pending`/`Full`.
  `Full` met la source **en attente** et la conserve.
- `from_journal` refuse tout journal dont l'attente est non vide (`Error::Pending`). Une
  saturation bloque donc aussi les changements d'instant, pas seulement les admissions.
- Le journal n'offre **aucun retrait**. Si le champ n'est pas calculable après une admission
  réussie, il faut soit revenir en arrière (retrait interne, symétrique de l'insertion), soit
  laisser le journal en avance sur le champ. ADR-078 a déjà écrit qu'on ne doit pas présenter
  l'ancien champ comme représentant le journal modifié.

Piège à éviter : élargir l'API du journal avec un retrait public. Ce qu'il faut est le retour
en arrière d'une admission dont on connaît la position, pas une suppression arbitraire.

P2/P3 : inventaire dans ADMISSION-PRESSION-S130 §1, decision ADR-086. Deux points ont oriente
la forme : l emprunt mutable **garantit** l invariant "jamais Unchanged sur un journal
different" (le compilateur, pas la vigilance), et les deux succes d admit_authenticated n ont
pas les memes consequences — Added exige un recalcul, Unchanged non, et confondre les deux
ferait payer une preparation complete a chaque readmission d une source connue.
