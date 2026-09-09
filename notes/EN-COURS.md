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

Session : S124 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A201 — la portée d'un champ d'impact vaut 5,09 λ parce que la table de Bessel
s'arrête à `x = 64`. Mesurer d'abord si la piste asymptotique tient, **puis** décider.

### Plan

- [x] **P1** — amorce, jeton, plan.
- [ ] **P2** — mesurer avant de décider (L209, L210). Trois erreurs à séparer, et la troisième
      est celle qu'on risque d'oublier :
      1. l'asymptotique elle-même, en f64 pur, contre la référence dense existante ;
      2. le raccord en `x = 64` — une discontinuité y ferait un anneau visible ;
      3. **la précision de l'argument** `k·r`, calculé en f32 : à `x = 4000`, un ulp de f32
         vaut déjà 2,4e-4 rad, soit cinquante fois la tolérance actuelle de 4e-6. Si c'est
         la phase qui limite, étendre la table ne servirait à rien.
- [ ] **P3** — ADR-084, sur ce que la mesure aura montré, y compris si elle dit non.
- [ ] **P4** — construire ce que la décision retient.
- [ ] **P5** — tests : précision au-delà de 64, continuité au raccord, et **hachages de
      campagne inchangés** — rien ne doit bouger sous `x = 64`.
- [ ] **P6** — la sonde `impact_envelope` rejouée : de combien la portée gagne-t-elle ?
- [ ] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ ea1669e = master ; trois copies coïncidentes, 5134cd archivée, c107bf sur la ligne S44.

Points d'entrée : `radial_impact::bessel` (interpolation Hermite, refus hors [0,64]),
`bessel_table.rs`, et la référence f64 indépendante déjà écrite —`bessel_angular` plus la
quadrature à 4096 directions du test `bessel_against_series_and_dense_angular_reference`.

Asymptotique visée, à vérifier et non à croire :
`J0(x) ≈ √(2/πx)·[cos θ + sin θ/(8x)]`, `J1(x) ≈ √(2/πx)·[sin θ − cos θ·3/(8x)]`, θ = x − π/4.
L'ordre zéro seul donne ~2e-4 d'erreur absolue à x = 64, cinquante fois la tolérance ; le terme
en 1/(8x) devrait suffire, c'est à mesurer.

Contrainte du dépôt : **pas de libm** dans le chemin de production (ADR-060). Les sinus et
cosinus passent par PhaseQ32, la racine carrée f32 est admise.

Issue possible et parfaitement acceptable : la mesure dit que la limite n'est pas la table mais
la phase, et la décision devient autre chose que « étendre ». Ne pas forcer la piste annoncée.
