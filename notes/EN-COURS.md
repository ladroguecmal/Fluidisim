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

Session : S166 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S165-1/A221, fermeture autonome avec information entrante et sortante distincte.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — dériver caractéristiques et référence analytique, protocole avant mesure ; copies synchronisées.
- [x] **P3** — implémenter fermeture aux étages, comparer entrée/sortie et raffinements.
- [x] **P4** — réception, résultats, limites et suivi des actions.
- [ ] **P5** — rituel de fin, journal/leçons/index/décomptes, reprise et copies.

### Notes de reprise

Master 8ecf20b propre, quatre copies alignées. Corpus lu dans cette conversation,
S165 terminée ; 112 ADR,221 angles,247 leçons. Bibliothèques inchangées depuis S163.
Suite BILAN-S145 portée par poursuite B4. Aucun domaine 3D ni absorbeur choisi.
P2 : BORD-AUTONOME-S166 déclaré, invariants dérivés et onde simple avant choc.
Témoin analytique distinct de l'identité discrète S165. Entrée et sortie dans les deux sens.

P3 : six tests propres S166 et huit S165 reçus ; 32 montages/128 évolutions release.
À N240/a0,05 entrée : erreur totale 0,314832 ; écart de frontière 0,001505704.
Perte de crête vers12s 0,1277563, identique au témoin : défaut intérieur observé.
Bilan <=2,04e-15, Courant <=0,214735. Aucun runtime modifié.

P4 : A221 traitée subcritique 1D entrée connue ; A222/S166-1 distingue source physique
et résidu numérique du fond. A50 partielle, aucun ADR. Résultats publiés dans BORD-AUTONOME.
