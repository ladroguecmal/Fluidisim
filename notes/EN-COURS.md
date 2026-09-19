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

Session : S285 — terminée
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : continuer.
Objectif : attribuer A290 au moyen de trois trajectoires au même temps, pour décider quel
mécanisme corriger avant toute réduction automatique.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — banc consommant Live/Layer : large intact, large préparé, étroit préparé ;
  même préparation jusqu'à permutation, puis arrêt commun. Mesures séparées et contrôles.
- [x] **P3** — exécuter et publier le diagnostic, tests du viewer ; arrêter au mécanisme
  attribué, ne pas inventer une correction physique sur un seul scénario.
- [x] **P4** — rituel §6 : journal, angles morts, file/feuille/index, jeton libre.

### Notes de reprise

Troisième lot spatial comparé à A276 et à la 3D : la cadence découplée débloque le budget,
la 3D débloque les vaguelettes demandées S277 ; une nouvelle optimisation du rétrécissement
est moins prioritaire. Ce lot est donc borné à l'attribution manquante, préalable à choisir
une correction sans fausse promesse de qualité. Ensuite revenir au coût A276.
Critère d'arrêt : isoler hauteur centrale préparé-intact et réduit-préparé, vérifier identité
au bit avant permutation, publier les fenêtres communes et les limites. Le seuil 3 mm reste
un repère de hauteur, jamais une réception perceptive ni une tolérance modifiée pour réussir.
S284 : 507 tests cœur/harnais, 34 viewer, 19 ignorés ; code cœur inchangé prévu ici.
P3 : banc complet reçu, 48 pas préparés identiques, permutation pas 112 ; zéro allocation
sur 963 appels. Maxima P-I 45,517 mm, R-P 25,391 mm, R-I 66,994 mm. À la permutation :
8,118 mm dus à la préparation, zéro écart central de transfert. 34 tests viewer réussis,
un ignoré. Pas de correction physique spéculative ; revenir au coût A276 après ce diagnostic.
