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

Session : S271 — terminée
Agent : Codex, GPT-6 ; fichiers, git, cargo et Python.
Entrée : « continue et mes a jour la to do list », master propre 9155256, copie unique.
Objectif : éprouver la houle progressive traversante et actualiser LISTE-PROJET-FINI.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — choisir l'oracle indépendant et déclarer les critères ; actualiser la liste sur preuves.
- [x] **P3** — construire le cas progressif et ses contre-épreuves.
- [x] **P4** — réception, limites, mise à jour finale de la liste et décompte.
- [x] **P5** — rituel §6, file entière, journal, index et jeton.

### Notes de reprise

S270 : courant constant reçu, pas houle progressive. La source non linéaire du fond
ne doit pas être confondue avec une erreur parasite. Priorité J2 maintenue devant
l'affinage du rendu accepté : vérifier le passage d'un fond variable est nécessaire.
Liste utilisateur encore datée S263 ; corriger notamment 4.6/4.7, 8.10 et 9.11,
ainsi que l'ancienne absence A288 déjà close S262. Ne pas cocher le périmètre final
à partir d'un reçu local.

P2 : liste actualisée sur S262–S270, périmètres finaux toujours partiels.
Oracle initial W(ζ)-W(0)-U(ζ)ζ_x déclaré avant campagne ; ne pas viser η'=0
pour une houle linéaire. Réception temporelle entière explicitement non acquise.

P3 : trois tests S271 passent. Erreurs spatiales 3,0481 / 1,2370 / 0,5847 %,
témoin fermé 168,9 / 238,9 / 337,7 %. Pas réel : erreur de taux 0,00120251
puis 0,000601253, divisée par deux. Décompte liste vérifié : 119=3+48+68.
