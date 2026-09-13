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

Session : S228 — en cours
Agent : Codex, GPT-6 (fichiers, git, cargo et Python disponibles)
Entrée : « Continue », après S227. Copie principale et trois copies propres à a86bd43,
lignée B archivée. Jeton libre ; maillons 0. Suite déclarée : A266, puis V restaurable.

**Objectif.** Construire une relation volume/plan orienté correcte pour V, compatible avec les
contenants non prismatiques, et la consommer dans le pas réel. Recevoir prisme, cale, extrêmes
et changements de direction contre des références indépendantes. Ne pas réduire l'ambition.

**Critère d'arrêt.** Le test A266 passe sans être ignoré ; la position de la surface préserve le
volume géométrique dans une précision explicitée, y compris près des fonds et plafonds. Une
géométrie insuffisante est refusée explicitement. Le chemin vertical ancien reste reçu dans son
domaine. Les refus restent atomiques ; pas d'allocation ajoutée au pas. La restauration V est le
lot suivant, sauf dépendance technique indispensable découverte ici.

### Plan

- [x] **P1** — amorce, état réel, jeton et plan seuls.
- [x] **P2** — lecture ciblée et choix du contrat géométrique ; ADR remplaçant la disposition incompatible d'ADR-010, oracle et limites déclarés.
- [>] **P3** — construire le calcul géométrique orienté, sans allocation au pas ; tests indépendants des volumes et plans.
- [ ] **P4** — brancher le contrat dans V ; activer la régression A266, préserver l'atomicité et les domaines compatibles.
- [ ] **P5** — réception complète, coût et limites du chemin consommé ; publication concise de la preuve.
- [ ] **P6** — rituel §6 : journal, file, angles/leçons utiles, trajectoire, index, jeton libre et synchronisation des copies.

Chaque étape reste sous quinze minutes ; découpage déclaré ici si nécessaire.

### Notes de reprise

S227 a isolé deux erreurs : table horizontale insuffisante pour les formes inclinées ; hauteur
verticale consommée comme distance normale. Sous pente 0,3, le prisme à mi-remplissage fuit de
506 ml à un hublot central situé à 1,01 m au lieu de zéro. Le test ignoré porte A266 dans son nom.
Base de validation S227 : 387 réussis, 6 ignorés ; quatre tests d'inventaire passent. Pas de
nouvelle suite générale lancée à l'amorce : la base vient d'être vérifiée et les copies sont identiques.

P2 : ADR-139 acté, partition tétraédrique sans recouvrement, capacité géométrique entière ;
inversion du volume coupé, anciennes tables +Z seulement. I-08 amendé explicitement pour les
intermédiaires f64 de V, sans dérogation au déterminisme ; SPEC-001 dérive les fractions et
SPEC-005 corrige la portée de son test d'étanchéité. Oracle boîte par intégrales séparées, cale
par sections linéaires indépendantes ; pas de dépendance ajoutée.
